
mod backend;
use backend::{Monitor, InfoGetter};

fn main() {
    let mut moni: Monitor = Monitor::new();

    for process in moni.system_info_update() {
        println!("PID: {} | Parent PID: {} | Name: {} | CPU: {:.2}% | Memory: {} bytes | EXE: {} | User: {}", 
                 process.pid, 
                 process.parent_pid, 
                 process.name, 
                 process.cpu, 
                 process.memory, 
                 process.exe, 
                 process.user);
    }   
}
