use super::*;

fn data_tree_view<'a>(
    processes: &'a [ProcessInfo],
    config: &ViewConfig,
) -> Vec<(&'a ProcessInfo, u8)> {
    let mut view: Vec<(&ProcessInfo, u8)> = Vec::new();

    fn dfs<'a>(
        process: &'a [ProcessInfo],
        depth: u8,
        res: &mut Vec<(&'a ProcessInfo, u8)>,
        open: &HashSet<u32>,
        config: &ViewConfig,
    ) {
        let search = config.search.to_lowercase();
        let mut level: Vec<&ProcessInfo> = process.iter().collect();

        level.sort_by(|a, b| match (config.criteria, config.sort_type) {
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
            match (config.filter, &proc.user) {
                (FilterType::All, _)
                    if (proc.name.to_lowercase().contains(&search)
                        || proc.user.to_lowercase().contains(&search)) =>
                {
                    res.push((proc, depth));
                    if open.contains(&proc.pid) {
                        dfs(&proc.child, depth + 1, res, open, config);
                    }
                }
                (FilterType::User, user)
                    if user == config.username
                        && (proc.name.to_lowercase().contains(&search)
                            || proc.user.to_lowercase().contains(&search)
                            || proc.pid.to_string().to_lowercase().contains(&search)) =>
                {
                    res.push((proc, depth));
                    if open.contains(&proc.pid) {
                        dfs(&proc.child, depth + 1, res, open, config);
                    }
                }
                (FilterType::System, user)
                    if user != config.username
                        && (proc.name.to_lowercase().contains(&search)
                            || proc.user.to_lowercase().contains(&search)
                            || proc.pid.to_string().to_lowercase().contains(&search)) =>
                {
                    res.push((proc, depth));
                    if open.contains(&proc.pid) {
                        dfs(&proc.child, depth + 1, res, open, config);
                    }
                }
                _ => dfs(&proc.child, depth, res, open, config),
            };
        }
    }

    dfs(processes, 0, &mut view, config.open, config);

    view
}

pub fn tree_drawer(ui: &mut egui::Ui, stats: &SysStats, config: ViewConfig) {
    let width = ui.available_width();

    let viewer = data_tree_view(&stats.processes, &config);

    TableBuilder::new(ui)
        .vscroll(true)
        .column(Column::initial(width * 0.2).resizable(true))
        .column(Column::initial(width * 0.1).resizable(true))
        .column(Column::initial(width * 0.2).resizable(true))
        .column(Column::initial(width * 0.3).resizable(true))
        .column(Column::initial(width * 0.15).resizable(true))
        .header(25.0, |mut header| {
            header.col(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("Name")
                            .strong()
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_hex("#e85d92").unwrap_or_default()),
                    );
                });
                ui.separator();
            });
            header.col(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("CPU")
                            .strong()
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_hex("#e85d92").unwrap_or_default()),
                    );
                    match stats.cpu {
                        0.0..=50.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.cpu))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::GREEN),
                            );
                        }
                        50.0..=80.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.cpu))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::YELLOW),
                            );
                        }
                        80.0..=100.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.cpu))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::RED),
                            );
                        }
                        _ => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.cpu))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::RED),
                            );
                        }
                    }
                });
                ui.separator();
            });
            header.col(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("Memory")
                            .strong()
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_hex("#e85d92").unwrap_or_default()),
                    );
                    match stats.mem {
                        0.0..=60.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.mem))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::GREEN),
                            );
                        }
                        60.0..=90.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.mem))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::YELLOW),
                            );
                        }
                        90.0..=100.0 => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.mem))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::RED),
                            );
                        }
                        _ => {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", stats.mem))
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(egui::Color32::RED),
                            );
                        }
                    }
                });
                ui.separator();
            });
            header.col(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("Path")
                            .strong()
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_hex("#e85d92").unwrap_or_default()),
                    );
                });
                ui.separator();
            });
            header.col(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("Username")
                            .strong()
                            .family(egui::FontFamily::Monospace)
                            .color(egui::Color32::from_hex("#e85d92").unwrap_or_default()),
                    );
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
                        let painter = ui.painter();

                        painter.line_segment(
                            [
                                egui::pos2(
                                    ui.available_rect_before_wrap().left()
                                        + depth as f32 * 20.0
                                        + 7.0,
                                    ui.available_rect_before_wrap().top(),
                                ),
                                egui::pos2(
                                    ui.available_rect_before_wrap().left()
                                        + depth as f32 * 20.0
                                        + 7.0,
                                    ui.available_rect_before_wrap().bottom() + 35.0,
                                ),
                            ],
                            egui::Stroke::new(
                                1.0,
                                egui::Color32::from_hex("#e85d92").unwrap_or_default(),
                            ),
                        );
                        painter.line_segment(
                            [
                                ui.available_rect_before_wrap().left_center()
                                    + egui::vec2((depth + 1) as f32 * 20.0 - 12.0, 0.0),
                                ui.available_rect_before_wrap().left_center()
                                    + egui::vec2((depth + 1) as f32 * 20.0 - 2.0, 0.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                egui::Color32::from_hex("#e85d92").unwrap_or_default(),
                            ),
                        );
                        ui.add_space(depth as f32 * 20.0);
                        if !process.child.is_empty() {
                            let arrow = if config.open.contains(&process.pid) {
                                "v"
                            } else {
                                ">"
                            };

                            if ui.button(arrow).clicked() {
                                if config.open.contains(&process.pid) {
                                    config.open.remove(&process.pid);
                                } else {
                                    config.open.insert(process.pid);
                                }
                            }
                        } else {
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
                    ui.add(egui::Label::new(&process.exe).truncate());
                });

                row.col(|ui| {
                    ui.label(process.user.to_string());
                });

                row.response().context_menu(|ui| {
                    if ui
                        .add(egui::Button::new("Kill process").min_size(egui::vec2(140.0, 25.0)))
                        .clicked()
                    {
                        match Command::new("kill").arg(process.pid.to_string()).status() {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Failed to execute: {e}");
                            }
                        }
                        ui.close_menu();
                    }
                    if ui
                        .add(egui::Button::new("Copy path").min_size(egui::vec2(140.0, 25.0)))
                        .clicked()
                    {
                        ui.output_mut(|o| o.copied_text = process.exe.clone());
                        ui.close_menu();
                    }
                    if ui
                        .add(
                            egui::Button::new("Copy process name")
                                .min_size(egui::vec2(140.0, 25.0)),
                        )
                        .clicked()
                    {
                        ui.output_mut(|o| o.copied_text = process.name.clone());
                        ui.close_menu();
                    }
                    if ui
                        .add(egui::Button::new("Open location").min_size(egui::vec2(140.0, 25.0)))
                        .clicked()
                    {
                        let process_path = Path::new(&process.exe);

                        if let Some(path) = process_path.parent() {
                            match Command::new("xdg-open").arg(path).status() {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                        } else {
                            println!("Couldn't open directory");
                        }

                        ui.close_menu();
                    }
                    ui.menu_button("Send Signal", |ui| {
                        ui.set_min_height(25.00);
                        if ui
                            .add(
                                egui::Button::new("Suspend (STOP)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-STOP")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Continue (CONT)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-CONT")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Hangup (HUP)").min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-HUP")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Interrupt (INT)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-INT")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Terminate (TERM)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-TERM")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(egui::Button::new("Kill (KILL)").min_size(egui::vec2(140.0, 25.0)))
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-KILL")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("User 1 (USR1)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-USR1")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("User 2 (USR2)")
                                    .min_size(egui::vec2(140.0, 25.0)),
                            )
                            .clicked()
                        {
                            match Command::new("kill")
                                .arg("-USR2")
                                .arg(process.pid.to_string())
                                .status()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to execute: {e}");
                                }
                            }
                            ui.close_menu();
                        }
                    });
                });
            });
        });
}
