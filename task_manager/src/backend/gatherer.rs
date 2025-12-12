use std::collections::HashMap;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind, Users};
pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: f64,
    pub exe: String,
    pub user: String,
    pub child: Vec<ProcessInfo>,
}

pub struct SysStats {
    pub processes: Vec<ProcessInfo>,
    pub cpu: f32,
    pub mem: f64,
}

pub struct Monitor {
    sys: System,
    users: Users,
}

pub trait InfoGetter {
    fn new() -> Self;
    fn system_info_update(&mut self) -> SysStats;
    fn tree(pid: u32, fam: &mut HashMap<u32, Vec<ProcessInfo>>) -> Vec<ProcessInfo>;
}

impl InfoGetter for Monitor {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cpu()
                .with_user(UpdateKind::OnlyIfNotSet)
                .with_memory()
                .with_exe(UpdateKind::OnlyIfNotSet),
        );

        let users = Users::new_with_refreshed_list();

        Self { sys, users }
    }

    fn system_info_update(&mut self) -> SysStats {
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cpu()
                .with_user(UpdateKind::OnlyIfNotSet)
                .with_memory()
                .with_exe(UpdateKind::OnlyIfNotSet),
        );

        let mut families: HashMap<u32, Vec<ProcessInfo>> = HashMap::new();

        for (pid, process) in self.sys.processes() {
            let mut info = ProcessInfo {
                pid: pid.as_u32(),
                parent_pid: 0u32,
                name: String::new(),
                cpu: process.cpu_usage() / (self.sys.cpus().len() as f32),
                memory: (process.memory() as f64) / 1024.0 / 1024.0,
                exe: String::new(),
                user: "Unknown".to_string(),
                child: Vec::new(),
            };

            info.name = if let Some(correct_name) = process.name().to_str() {
                correct_name.to_string()
            } else {
                "Unknown".to_string()
            };

            match process.exe() {
                Some(path) => match path.to_str() {
                    Some(correct_path) => info.exe = correct_path.to_string(),
                    None => info.exe = "Unknown".to_string(),
                },
                None => info.exe = "Unknown".to_string(),
            }

            if let Some(uid) = process.user_id() {
                'lop: for user in self.users.list() {
                    if uid == user.id() {
                        info.user = user.name().to_string();
                        break 'lop;
                    }
                }
            }

            match process.parent() {
                Some(pid) => info.parent_pid = pid.as_u32(),
                None => info.parent_pid = 0u32,
            }

            families.entry(info.parent_pid).or_default().push(info);
        }

        let mut process_info: Vec<ProcessInfo> = Self::tree(0, &mut families);

        for (_, mut processe) in families {
            for process in &mut processe {
                process.child = Self::tree(process.pid, &mut HashMap::new());
            }
            process_info.append(&mut processe);
        }

        self.sys.refresh_cpu_all();
        let cpu = self.sys.global_cpu_usage();
        let mem = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100.0;

        SysStats {
            processes: process_info,
            cpu,
            mem,
        }
    }

    fn tree(pid: u32, fam: &mut HashMap<u32, Vec<ProcessInfo>>) -> Vec<ProcessInfo> {
        let mut kids = fam.remove(&pid).unwrap_or_default();

        for kid in &mut kids {
            kid.child = Self::tree(kid.pid, fam);
        }

        kids
    }
}
