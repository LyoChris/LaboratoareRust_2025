mod backend;
use backend::{Monitor, ProcessInfo, InfoGetter};
use eframe::egui::{CentralPanel, Context, FontFamily, FontId, TextStyle};
use egui_extras::{Column, TableBuilder};

struct TaskManager {
    monitor: Monitor,
    processes: Vec<ProcessInfo>,
}

impl Default for TaskManager {
    fn default() -> Self {
        let mut monitor = Monitor::new();
        let processes = monitor.system_info_update();

        Self { monitor, processes }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.processes = self.monitor.system_info_update();
        set_styles(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from aplication");
            ui.separator();

            let width = ui.available_width();

            TableBuilder::new(ui)
                .vscroll(true)
                .column(Column::initial(width * 0.2).resizable(true))
                .column(Column::initial(width * 0.1).resizable(true))
                .column(Column::initial(width * 0.1).resizable(true))
                .column(Column::initial(width * 0.3).resizable(true))
                .column(Column::initial(width * 0.2).resizable(true))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Process Name");
                        ui.separator();
                    });
                    header.col(|ui| {
                        ui.heading("CPU usage");
                        ui.separator();
                    });
                    header.col(|ui| {
                        ui.heading("Memory usage");
                        ui.separator();
                    });
                    header.col(|ui| {
                        ui.heading("Path");
                        ui.separator();
                    });
                    header.col(|ui| {
                        ui.heading("Username");
                        ui.separator();
                    });
                })
                .body(|body| {
                    let height = 50.0;
                    let num = self.processes.len();

                    body.rows(height, num, |mut row| {
                        let index = row.index();
                        let process = &self.processes[index];

                        row.col(|ui| {
                            ui.label(format!("{}", process.name));
                        });

                        // Col 2: CPU
                        row.col(|ui| {
                            ui.label(format!("{}%", process.cpu));
                        });

                        // Col 3: Memory
                        row.col(|ui| {
                            ui.label(format!("{:.1} MB", process.memory));
                        });

                        // Col 4: Path
                        row.col(|ui| {
                            ui.label(format!("{}", process.exe));
                        });

                        // Col 5: Username
                        row.col(|ui| {
                            ui.label(format!("{}", process.user));
                        });
                    });
                });
        });
    }
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace));

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

    // for process in moni.system_info_update() {
    //     println!("PID: {} | Parent PID: {} | Name: {} | CPU: {:.2}% | Memory: {} bytes | EXE: {} | User: {}",
    //              process.pid,
    //              process.parent_pid,
    //              process.name,
    //              process.cpu,
    //              process.memory,
    //              process.exe,
    //              process.user);
    // }
}
