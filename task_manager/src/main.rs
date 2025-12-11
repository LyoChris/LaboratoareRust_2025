mod backend;
use backend::{Monitor, InfoGetter, SysStats};
use eframe::egui::{self, CentralPanel, Context, FontFamily, FontId, TextStyle};
use egui_extras::{Column, TableBuilder};
use::std::{thread, thread::sleep, time, cmp::Ordering};
use::std::sync::mpsc::{self, Receiver};

struct TaskManager {
    rx: Receiver<SysStats>,
    stats: SysStats
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

        Self { rx, stats: SysStats {processes: Vec::new(), cpu: 0, mem: 0} }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(data) = self.rx.try_recv() {
            self.stats = data;
            println!("Refresh done");
        }

        //default sort;
        self.stats.processes.sort_by(|a,b | {
            b.cpu.partial_cmp(&a.cpu).unwrap_or(Ordering::Equal)
        });
        set_styles(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from aplication");
            ui.separator();

            let width = ui.available_width();

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
                            ui.label(format!("{}%", self.stats.cpu));
                        });
                        ui.separator();
                    });
                    header.col(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.heading("Memory");
                            ui.label(format!("{}%", self.stats.mem));
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
                    let num = self.stats.processes.len();

                    body.rows(height, num, |mut row| {
                        let index = row.index();
                        let process = &self.stats.processes[index];

                        row.col(|ui| {
                            ui.label(process.name.to_string());
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
                            ui.label(process.exe.to_string());
                        });

                        // Col 5: Username
                        row.col(|ui| {
                            ui.label(process.user.to_string());
                        });
                    });
                });
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
