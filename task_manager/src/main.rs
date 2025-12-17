mod backend;
mod frontend;
use crate::backend::gatherer::{InfoGetter, Monitor, SysStats};
use crate::frontend::{
    ViewConfig, overview_drawer::GraphDrawer, table_drawer::table_drawer, tree_drawer::tree_drawer,
};
use ::std::sync::mpsc::{self, Receiver};
use ::std::{collections::HashSet, collections::VecDeque, env::var, thread, thread::sleep, time};
use eframe::egui::{self, CentralPanel, Color32, Context, Visuals};

#[derive(PartialEq, Clone, Copy)]
pub enum SortCriteria {
    Cpu,
    Memory,
    Name,
}

#[derive(PartialEq, Clone, Copy)]
pub enum FilterType {
    All,
    User,
    System,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SortType {
    Ascending,
    Descending,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ViewType {
    Table,
    Tree,
    Graphic,
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
    search: String,
    ram_plot_points: VecDeque<f64>,
    cpu_plot_points: VecDeque<f64>,
    graph_draw: GraphDrawer,
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
                used_mem: 0.0,
            },
            criteria: SortCriteria::Cpu,
            sort_type: SortType::Descending,
            filter: FilterType::User,
            view_type: ViewType::Table,
            user,
            open: HashSet::new(),
            search: String::new(),
            ram_plot_points: VecDeque::new(),
            cpu_plot_points: VecDeque::new(),
            graph_draw: GraphDrawer::new(),
        }
    }
}

impl eframe::App for TaskManager {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.rx.try_recv() {
            Ok(data) => {
                self.stats = data;
                if self.cpu_plot_points.len() >= 120 || self.ram_plot_points.len() >= 120 {
                    self.ram_plot_points.pop_front();
                    self.cpu_plot_points.pop_front();
                    self.ram_plot_points.push_back(self.stats.used_mem);
                    self.cpu_plot_points.push_back(self.stats.cpu as f64);
                } else {
                    self.ram_plot_points.push_back(self.stats.used_mem);
                    self.cpu_plot_points.push_back(self.stats.cpu as f64);
                }
                println!("Refresh done")
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Worker Thread Disconnected. Atempted reconnection...");
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

                self.rx = rx;
                println!("Connection restablished...");
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
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
                    if ui
                        .selectable_label(self.view_type == ViewType::Graphic, "Overview")
                        .clicked()
                    {
                        self.view_type = ViewType::Graphic;
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Start typing to search processes"),
                );

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

                    ui.horizontal(|ui| {
                        let arrow = if self.sort_type == SortType::Ascending {
                            "^"
                        } else {
                            "v"
                        };
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
                            .selectable_label(self.criteria == SortCriteria::Name, name_label)
                            .clicked()
                        {
                            self.criteria = SortCriteria::Name;
                            match self.sort_type {
                                SortType::Ascending => self.sort_type = SortType::Descending,
                                SortType::Descending => self.sort_type = SortType::Ascending,
                            };
                        }

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
                            .selectable_label(self.criteria == SortCriteria::Cpu, cpu_label)
                            .clicked()
                        {
                            self.criteria = SortCriteria::Cpu;
                            match self.sort_type {
                                SortType::Ascending => self.sort_type = SortType::Descending,
                                SortType::Descending => self.sort_type = SortType::Ascending,
                            };
                        };
                        ui.label("Sort by:".to_string());
                    });
                });
            });
            ui.separator();

            let config = ViewConfig {
                criteria: self.criteria,
                sort_type: self.sort_type,
                filter: self.filter,
                username: &self.user,
                search: &self.search,
                open: &mut self.open,
            };

            match self.view_type {
                ViewType::Table => table_drawer(ui, &self.stats, config),
                ViewType::Tree => tree_drawer(ui, &self.stats, config),
                ViewType::Graphic => self.graph_draw.overview_drawer(
                    ui,
                    &self.ram_plot_points,
                    &self.cpu_plot_points,
                ),
            }
        });

        ctx.request_repaint_after(time::Duration::from_millis(1000));
    }
}

fn set_theme(ctx: &Context) {
    let background = Color32::from_hex("#17141A").unwrap_or_default();
    let background_light = Color32::from_hex("#221D26").unwrap_or_default();
    let text = Color32::from_hex("#E6E1E8").unwrap_or_default();
    let accent = Color32::from_hex("#A83256").unwrap_or_default();
    let highlight = Color32::from_hex("#E85D92").unwrap_or_default();

    let mut visuals = Visuals::dark();

    visuals.window_fill = background;
    visuals.panel_fill = background;
    visuals.faint_bg_color = background_light;
    visuals.extreme_bg_color = Color32::from_hex("#0a080c").unwrap_or_default();

    visuals.selection.bg_fill = accent;
    //visuals.selection.stroke = egui::Stroke::new(1.0, highlight);

    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
    visuals.widgets.active.bg_fill = highlight;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Color32::BLACK);

    visuals.widgets.hovered.bg_fill = highlight;

    ctx.set_visuals(visuals);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AICI",
        options,
        Box::new(|cc| {
            set_theme(&cc.egui_ctx);
            Ok(Box::<TaskManager>::default())
        }),
    )
}
