use super::*;
use egui_plot::{GridMark, Line, Plot, PlotPoints};
use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum GraphType {
    Cpu,
    Ram,
}

#[derive(PartialEq)]
enum ViewType {
    General,
    Cpu,
    Ram,
}

pub struct GraphDrawer {
    current_view: ViewType,
}

impl GraphDrawer {
    pub fn new() -> Self {
        GraphDrawer {
            current_view: ViewType::General,
        }
    }

    fn draw_table(
        ui: &mut egui::Ui,
        plot_points: &VecDeque<f64>,
        graph_type: GraphType,
        height: f32,
        name: &String,
    ) {
        let points: PlotPoints = plot_points
            .iter()
            .enumerate()
            .map(|(i, &y)| [i as f64, y])
            .collect();

        let (graph_name, color, max_dim) = match graph_type {
            GraphType::Cpu => (
                "CPU",
                egui::Color32::from_hex("#C70067").unwrap_or_default(),
                100.0,
            ),
            GraphType::Ram => (
                "Memory",
                egui::Color32::from_hex("#C70067").unwrap_or_default(),
                30.8,
            ),
        };

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(graph_name)
                    .strong()
                    .size(20.0)
                    .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(name)
                        .strong()
                        .size(20.0)
                        .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                );
            });
        });

        ui.add_space(10.0);

        let line = Line::new(points)
            .name(graph_name)
            .color(color)
            .width(2.0)
            .fill(0.0);

        Plot::new(graph_name.to_string())
            .height(height)
            .include_y(0.0)
            .include_y(max_dim)
            .show_axes([false, true])
            .show_grid([false, true])
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .y_grid_spacer(move |_| {
                let mut markers = Vec::new();

                for step in 0..=4 {
                    markers.push(GridMark {
                        value: max_dim * (step as f64 / 4.0),
                        step_size: max_dim / 4.0,
                    });
                }
                markers
            })
            .y_axis_formatter(|marker, _range| match graph_type {
                GraphType::Ram => format!("{:.1} Gib", marker.value),
                GraphType::Cpu => format!("{:.0}%", marker.value),
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });

        ui.add_space(3.0);

        match graph_type {
            GraphType::Cpu => {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(
                            format! {"Total Usage: {:.2}%", plot_points.back().unwrap_or(&0.0)},
                        )
                        .size(14.0)
                        .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                    );
                });
            }
            GraphType::Ram => {
                ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(format!{"Used Physical Memory: {:.1} GiB", plot_points.back().unwrap_or(&0.0)}).size(14.0).color(Color32::from_hex("#E6E1E8").unwrap_or_default()));
    });
            }
        };
    }

    pub fn overview_drawer(
        &mut self,
        ui: &mut egui::Ui,
        ram_points: &VecDeque<f64>,
        cpu_points: &VecDeque<f64>,
        sys_specifics: &SysSpecifics,
    ) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(self.current_view == ViewType::General, "Overview")
                .clicked()
            {
                self.current_view = ViewType::General;
            }

            if ui
                .selectable_label(self.current_view == ViewType::Cpu, "CPU")
                .clicked()
            {
                self.current_view = ViewType::Cpu;
            }

            if ui
                .selectable_label(self.current_view == ViewType::Ram, "Memory")
                .clicked()
            {
                self.current_view = ViewType::Ram;
            }
        });
        ui.separator();

        match self.current_view {
            ViewType::General => {
                let height = (ui.available_height() - 180.0) / 2.0;

                GraphDrawer::draw_table(
                    ui,
                    cpu_points,
                    GraphType::Cpu,
                    height,
                    &sys_specifics.cpu_specifics.name,
                );

                ui.add_space(10.0);
                ui.separator();

                GraphDrawer::draw_table(
                    ui,
                    ram_points,
                    GraphType::Ram,
                    height,
                    &format!("{:.1} GB", sys_specifics.mem_specifics.total),
                );
            }
            ViewType::Cpu => {
                let height = (ui.available_height() - 180.0) / 2.0;

                GraphDrawer::draw_table(
                    ui,
                    cpu_points,
                    GraphType::Cpu,
                    height,
                    &sys_specifics.cpu_specifics.name,
                );

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(30.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Processes")
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}",
                                        sys_specifics.cpu_specifics.processes
                                    ))
                                    .size(25.0)
                                    .strong()
                                    .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                                );
                            });
                            ui.add_space(20.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Logical Cores")
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}",
                                        sys_specifics.cpu_specifics.logical_cores
                                    ))
                                    .size(25.0)
                                    .strong()
                                    .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                                );
                            });
                        });

                        ui.add_space(30.0);
                        ui.vertical(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Speed")
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{:.2} GHz",
                                        sys_specifics.cpu_specifics.current_speed
                                    ))
                                    .size(25.0)
                                    .strong()
                                    .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                                );
                            });

                            ui.add_space(20.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("Physical Cores")
                                        .size(16.0)
                                        .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}",
                                        sys_specifics.cpu_specifics.physical_cores
                                    ))
                                    .size(25.0)
                                    .strong()
                                    .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                                );
                            });
                        });
                    });
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        let seconds = sys_specifics.cpu_specifics.up_time;
                        ui.label(
                            egui::RichText::new("Up time(D:H:M:S)")
                                .size(16.0)
                                .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "{}:{}:{}:{}",
                                seconds / 86400,
                                (seconds % 86400) / 3600,
                                (seconds % 3600) / 60,
                                seconds % 60
                            ))
                            .size(25.0)
                            .strong()
                            .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                        );
                    });
                });
            }
            ViewType::Ram => {
                let height = (ui.available_height() - 180.0) / 2.0;

                GraphDrawer::draw_table(
                    ui,
                    ram_points,
                    GraphType::Ram,
                    height,
                    &format!("{:.1} GB", sys_specifics.mem_specifics.total),
                );

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Used memory")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.used
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                        ui.add_space(30.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Used swap")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.swap_used
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                    });
                    ui.add_space(30.0);
                    ui.vertical(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Available memory")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.available
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                        ui.add_space(30.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Available swap")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.swap_available
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                    });
                    ui.add_space(30.0);
                    ui.vertical(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Total memory")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.total
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                        ui.add_space(30.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Total swap")
                                    .size(16.0)
                                    .color(Color32::LIGHT_GRAY.linear_multiply(0.5)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2} GiB",
                                    sys_specifics.mem_specifics.swap_total
                                ))
                                .size(25.0)
                                .strong()
                                .color(Color32::from_hex("#E6E1E8").unwrap_or_default()),
                            );
                        });
                    });
                    ui.add_space(30.0);
                });
            }
        }
    }
}
