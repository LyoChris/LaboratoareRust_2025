mod backend;
use crate::backend::gatherer::{InfoGetter, Monitor, ProcessInfo, SysStats};
use ::std::sync::mpsc::{self, Receiver};
use ::std::{cmp::Ordering, collections::HashSet, env::var, thread, thread::sleep, time};
use eframe::egui::{self, CentralPanel, Context, FontFamily, FontId, TextStyle};
use egui_extras::{Column, TableBuilder};

#[derive(PartialEq, Clone, Copy)]
enum SortCriteria {
    Cpu,
    Memory,
    Name,
}

#[derive(PartialEq, Clone, Copy)]
enum FilterType {
    All,
    User,
    System,
}

#[derive(PartialEq, Clone, Copy)]
enum SortType {
    Ascending,
    Descending,
}

#[derive(PartialEq, Clone, Copy)]
enum ViewType {
    Table,
    Tree,
}

struct TaskManager {
    rx: Receiver<SysStats>,
    stats: SysStats,
    criteria: SortCriteria,
    sort_type: SortType,
    filter: FilterType,
    user: String,
    view_type: ViewType,
    open: HashSet<u32>,
}

impl TaskManager {
    fn data_table_view<'a>(
        processes: &'a [ProcessInfo],
        crit: SortCriteria,
        sort_type: SortType,
        filter: FilterType,
        username: &String,
    ) -> Vec<&'a ProcessInfo> {
        let mut view: Vec<&ProcessInfo> = Vec::new();
        let mut list_from_tree: Vec<&ProcessInfo> = Vec::new();

        fn dfs<'a>(process: &'a [ProcessInfo], res: &mut Vec<&'a ProcessInfo>) {
            for proc in process.iter() {
                res.push(proc);
                dfs(&proc.child, res);
            }
        }

        dfs(processes, &mut list_from_tree);

        for proc in list_from_tree {
            match (filter, &proc.user) {
                (FilterType::All, _) => view.push(proc),
                (FilterType::User, user) if user == username => view.push(proc),
                (FilterType::System, user) if user != username => view.push(proc),
                _ => (),
            }
        }

        view.sort_by(|a, b| match (crit, sort_type) {
            (SortCriteria::Cpu, SortType::Descending) => {
                b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal)
            }
            (SortCriteria::Cpu, SortType::Ascending) => {
                a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal)
            }
            (SortCriteria::Memory, SortType::Descending) => {
                b.memory.partial_cmp(&a.memory).unwrap_or(Ordering::Equal)
            }
            (SortCriteria::Memory, SortType::Ascending) => {
                a.memory.partial_cmp(&b.memory).unwrap_or(Ordering::Equal)
            }
            (SortCriteria::Name, SortType::Ascending) => {
                b.name.partial_cmp(&a.name).unwrap_or(Ordering::Equal)
            }
            (SortCriteria::Name, SortType::Descending) => {
                a.name.partial_cmp(&b.name).unwrap_or(Ordering::Equal)
            }
        });

        view
    }
    fn data_tree_view<'a>(
        processes: &'a [ProcessInfo],
        crit: SortCriteria,
        sort_type: SortType,
        filter: FilterType,
        username: &String,
        open: &HashSet<u32>,
    ) -> Vec<(&'a ProcessInfo, u8)> {
        let mut view: Vec<(&ProcessInfo, u8)> = Vec::new();

        struct SortFilters<'a> {
            crit: SortCriteria,
            sort_type: SortType,
            filter: FilterType,
            username: &'a String,
        }

        let filt: SortFilters = SortFilters {
            crit,
            sort_type,
            filter,
            username
        };

        fn dfs<'a>(
            process: &'a [ProcessInfo],
            depth: u8,
            res: &mut Vec<(&'a ProcessInfo, u8)>,
            open: &HashSet<u32>,
            filt: &SortFilters<'_>
        ) {
            let mut level: Vec<&ProcessInfo> = process.iter().collect();

            level.sort_by(|a, b| match (filt.crit, filt.sort_type) {
                (SortCriteria::Cpu, SortType::Descending) => {
                    b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal)
                }
                (SortCriteria::Cpu, SortType::Ascending) => {
                    a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal)
                }
                (SortCriteria::Memory, SortType::Descending) => {
                    b.memory.partial_cmp(&a.memory).unwrap_or(Ordering::Equal)
                }
                (SortCriteria::Memory, SortType::Ascending) => {
                    a.memory.partial_cmp(&b.memory).unwrap_or(Ordering::Equal)
                }
                (SortCriteria::Name, SortType::Ascending) => {
                    b.name.partial_cmp(&a.name).unwrap_or(Ordering::Equal)
                }
                (SortCriteria::Name, SortType::Descending) => {
                    a.name.partial_cmp(&b.name).unwrap_or(Ordering::Equal)
                }
            });

            for proc in level {
                match (filt.filter, &proc.user) {
                    (FilterType::All, _) => {
                        res.push((proc, depth));
                        if open.contains(&proc.pid) {
                            dfs(
                                &proc.child,
                                depth + 1,
                                res,
                                open,
                                filt
                            );
                        }
                    },
                    (FilterType::User, user) if user == filt.username => {
                        res.push((proc, depth));
                        if open.contains(&proc.pid) {
                            dfs(
                                &proc.child,
                                depth + 1,
                                res,
                                open,
                                filt,
                            );
                        }
                    },
                    (FilterType::System, user) if user != filt.username => {
                        res.push((proc, depth));
                        if open.contains(&proc.pid) {
                            dfs(
                                &proc.child,
                                depth + 1,
                                res,
                                open,
                                filt,
                            );
                        }
                    },
                    _ => {
                        dfs(
                                &proc.child,
                                depth,
                                res,
                                open,
                                filt,
                            )
                    },
                };
            }
        }

        dfs(
            processes, 0, &mut view, open, &filt
        );

        view
    }
    fn table_drawer(ui: &mut egui::Ui, stats: &SysStats, crit: SortCriteria, sort_type: SortType, filter: FilterType, username: &String) {
        let width = ui.available_width();

        let viewer = TaskManager::data_table_view(
                &stats.processes,
                crit,
                sort_type,
                filter,
                username,
            );

        TableBuilder::new(ui)
            .vscroll(true)
            .column(Column::initial(width * 0.2).resizable(true))
            .column(Column::initial(width * 0.1).resizable(true))
            .column(Column::initial(width * 0.2).resizable(true))
            .column(Column::initial(width * 0.3).resizable(true))
            .column(Column::initial(width * 0.15).resizable(true))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Name");
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("CPU");
                        ui.label(format!("{:.1}%", stats.cpu));
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Memory");
                        ui.label(format!("{:.1}%", stats.mem));
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Path");
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Username");
                    });
                    ui.separator();
                });
            })
            .body(|body| {
                let height = 50.0;
                let num = viewer.len();

                body.rows(height, num, |mut row| {
                    let index = row.index();
                    let process = viewer[index];

                    row.col(|ui| {
                        ui.label(process.name.to_string());
                    });

                    row.col(|ui| {
                        ui.label(format!("{:.2}%", process.cpu));
                    });

                    row.col(|ui| {
                        ui.label(format!("{:.1} MB", process.memory));
                    });

                    row.col(|ui| {
                        ui.label(process.exe.to_string());
                    });

                    row.col(|ui| {
                        ui.label(process.user.to_string());
                    });
                });
            });
    }
    fn tree_drawer(ui: &mut egui::Ui, stats: &SysStats,  crit: SortCriteria, sort_type: SortType, filter: FilterType, username: &String, open: &mut HashSet<u32>) {
        let width = ui.available_width();

        let viewer = TaskManager::data_tree_view(
                &stats.processes,
                crit,
                sort_type,
                filter,
                username,
                open
            );

        TableBuilder::new(ui)
            .vscroll(true)
            .column(Column::initial(width * 0.2).resizable(true))
            .column(Column::initial(width * 0.1).resizable(true))
            .column(Column::initial(width * 0.2).resizable(true))
            .column(Column::initial(width * 0.3).resizable(true))
            .column(Column::initial(width * 0.15).resizable(true))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Name");
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("CPU");
                        ui.label(format!("{:.1}%", stats.cpu));
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Memory");
                        ui.label(format!("{:.1}%", stats.mem));
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Path");
                    });
                    ui.separator();
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("Username");
                    });
                    ui.separator();
                });
            })
            .body(|body| {
                let height = 50.0;
                let num = viewer.len();

                body.rows(height, num, |mut row| {
                    let index = row.index();
                    let (process, depth) = viewer[index];

                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(depth as f32 * 20.0);
                            if !process.child.is_empty() {
                                let arrow = if open.contains(&process.pid) {
                                    "v"
                                }
                                else {
                                    ">"
                                };

                                if ui.button(arrow).clicked() {
                                    if open.contains(&process.pid) {
                                        open.remove(&process.pid);
                                    }
                                    else {
                                        open.insert(process.pid);
                                    }
                                }
                            }
                            else {
                                ui.add_space(20.0);
                            }

                            ui.label(process.name.to_string());
                        });
                    });

                    row.col(|ui| {
                        ui.label(format!("{:.2}%", process.cpu));
                    });

                    row.col(|ui| {
                        ui.label(format!("{:.1} MB", process.memory));
                    });

                    row.col(|ui| {
                        ui.label(process.exe.to_string());
                    });

                    row.col(|ui| {
                        ui.label(process.user.to_string());
                    });
                });
            });
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut monitor = Monitor::new();

            loop {
                let processes = monitor.system_info_update();
                if tx.send(processes).is_err() {
                    break;
                }
                sleep(time::Duration::from_millis(1000));
            }
        });

        let user = var("USER").unwrap_or_else(|_| "unknown".to_string());

        Self {
            rx,
            stats: SysStats {
                processes: Vec::new(),
                cpu: 0.0,
                mem: 0.0,
            },
            criteria: SortCriteria::Cpu,
            sort_type: SortType::Descending,
            filter: FilterType::User,
            view_type: ViewType::Table,
            user,
            open: HashSet::new(),
        }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(data) = self.rx.try_recv() {
            self.stats = data;
            println!("Refresh done");
        }

        set_styles(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from aplication");
            ui.separator();

            ui.horizontal(|ui| {
                let arrow = if self.sort_type == SortType::Ascending {
                    "^"
                } else {
                    "v"
                };
                ui.label("Sort by:".to_string());
                let cpu_label = match self.criteria {
                    SortCriteria::Cpu => format!("{} CPU", arrow),
                    _ => "CPU".to_string(),
                };
                let mem_label = match self.criteria {
                    SortCriteria::Memory => format!("{} RAM", arrow),
                    _ => "RAM".to_string(),
                };
                let name_label = match self.criteria {
                    SortCriteria::Name => format!("{} Name", arrow),
                    _ => "Name".to_string(),
                };
                if ui
                    .selectable_label(self.criteria == SortCriteria::Cpu, cpu_label)
                    .clicked()
                {
                    self.criteria = SortCriteria::Cpu;
                    match self.sort_type {
                        SortType::Ascending => self.sort_type = SortType::Descending,
                        SortType::Descending => self.sort_type = SortType::Ascending,
                    };
                };

                if ui
                    .selectable_label(self.criteria == SortCriteria::Memory, mem_label)
                    .clicked()
                {
                    self.criteria = SortCriteria::Memory;
                    match self.sort_type {
                        SortType::Ascending => self.sort_type = SortType::Descending,
                        SortType::Descending => self.sort_type = SortType::Ascending,
                    };
                }

                if ui
                    .selectable_label(self.criteria == SortCriteria::Name, name_label)
                    .clicked()
                {
                    self.criteria = SortCriteria::Name;
                    match self.sort_type {
                        SortType::Ascending => self.sort_type = SortType::Descending,
                        SortType::Descending => self.sort_type = SortType::Ascending,
                    };
                }

                let filter = match self.filter {
                    FilterType::All => "Shown: All processes".to_string(),
                    FilterType::User => "Shown: User processes".to_string(),
                    FilterType::System => "Shown: System processes".to_string(),
                };

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_label("")
                        .selected_text(filter)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.filter, FilterType::All, "All processes");
                            ui.selectable_value(
                                &mut self.filter,
                                FilterType::User,
                                "User processes",
                            );
                            ui.selectable_value(
                                &mut self.filter,
                                FilterType::System,
                                "System processes",
                            );
                        });

                    if ui
                        .selectable_label(self.view_type == ViewType::Table, "Table")
                        .clicked()
                    {
                        self.view_type = ViewType::Table;
                    }

                    if ui
                        .selectable_label(self.view_type == ViewType::Tree, "Tree")
                        .clicked()
                    {
                        self.view_type = ViewType::Tree;
                    }
                });
            });

            if self.view_type == ViewType::Table {
                Self::table_drawer(ui, &self.stats, self.criteria, self.sort_type, self.filter, &self.user);
            } else {
                Self::tree_drawer(ui, &self.stats, self.criteria, self.sort_type, self.filter, &self.user, &mut self.open);
            }
        });

        ctx.request_repaint_after(time::Duration::from_millis(1000));
    }
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(TextStyle::Heading, FontId::new(20.0, FontFamily::Monospace));

    ctx.set_style(style);
}

fn main() -> Result<(), eframe::Error> {
    // let mut moni: Monitor = Monitor::new();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AICI",
        options,
        Box::new(|_cc| Ok(Box::<TaskManager>::default())),
    )
}
