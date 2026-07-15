// really bad attempt at kinda replicating gtk looks
use crate::messages::WorkerMsg;
use crate::{brave, desktop, launcher, paths, setup, theme};
use eframe::egui;
use std::sync::mpsc::{Receiver, Sender};

pub fn run_gui(startup_error: Option<String>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 500.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "prime-wine",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(App::new(startup_error)))
        }),
    );
    if let Err(e) = result {
        eprintln!("could not open the window: {}", e);
    }
}

// blue button, like a gtk suggested action
fn big_button(ui: &mut egui::Ui, label: &str, width: f32) -> bool {
    let text = egui::RichText::new(label)
        .size(15.0)
        .family(theme::semibold())
        .color(egui::Color32::WHITE);
    let button = egui::Button::new(text).fill(theme::ACCENT);
    ui.add_sized([width, 44.0], button).clicked()
}

const SMALL_BUTTON_WIDTH: f32 = 110.0;

// gray button with a fixed size so rows are easy to center
fn small_button(ui: &mut egui::Ui, label: &str) -> bool {
    ui.add_sized(
        [SMALL_BUTTON_WIDTH, 34.0],
        egui::Button::new(egui::RichText::new(label).size(13.5)),
    )
    .clicked()
}

// centered section header with lines on both sides
fn section_header(ui: &mut egui::Ui, label: &str) {
    let font = egui::FontId::new(12.5, theme::semibold());
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), font, theme::DIM_TEXT);
    let spacing = ui.spacing().item_spacing.x;
    let line_width = ((ui.available_width() - galley.size().x) / 2.0 - spacing).max(0.0);
    ui.horizontal(|ui| {
        ui.add_sized([line_width, 1.0], egui::Separator::default().horizontal());
        ui.label(
            egui::RichText::new(label)
                .size(12.5)
                .family(theme::semibold())
                .color(theme::DIM_TEXT),
        );
        ui.add_sized([line_width, 1.0], egui::Separator::default().horizontal());
    });
}

struct App {
    busy: bool,
    progress: f32,
    log_lines: Vec<String>,
    error: Option<String>,
    confirm_uninstall: bool,
    worker_rx: Option<Receiver<WorkerMsg>>,
}

impl App {
    fn new(startup_error: Option<String>) -> Self {
        Self {
            busy: false,
            progress: 0.0,
            log_lines: vec!["hello im a terminal".to_string()],
            error: startup_error,
            confirm_uninstall: false,
            worker_rx: None,
        }
    }

    // runs a job on a thread so the window does not freeze
    fn start_worker(&mut self, job: fn(&Sender<WorkerMsg>) -> Result<(), String>) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.worker_rx = Some(rx);
        self.busy = true;
        self.progress = 0.0;
        self.error = None;
        std::thread::spawn(move || {
            let result = job(&tx);
            match result {
                Ok(()) => {
                    let _ = tx.send(WorkerMsg::Done);
                }
                Err(e) => {
                    let _ = tx.send(WorkerMsg::Failed(e));
                }
            }
        });
    }

    fn poll_worker(&mut self) {
        let Some(rx) = &self.worker_rx else {
            return;
        };
        while let Ok(msg) = rx.try_recv() {
            match msg {
                WorkerMsg::Log(line) => self.log_lines.push(line),
                WorkerMsg::Progress(value) => self.progress = value,
                WorkerMsg::Done => {
                    self.busy = false;
                    self.worker_rx = None;
                    return;
                }
                WorkerMsg::Failed(e) => {
                    self.busy = false;
                    self.worker_rx = None;
                    self.error = Some(e);
                    return;
                }
            }
        }
    }

    fn uninstall(&mut self) {
        let _ = std::fs::remove_dir_all(paths::data_dir());
        desktop::remove_desktop_entry();
        self.log_lines.push("everything removed".to_string());
        self.confirm_uninstall = false;
    }

    fn draw_buttons(&mut self, ui: &mut egui::Ui) {
        if paths::is_installed() {
            ui.vertical_centered(|ui| {
                if big_button(ui, "watch prime video", 200.0) {
                    if let Err(e) = launcher::launch_prime_detached() {
                        self.error = Some(e);
                    } else {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });

            // pad the row by hand so it sits in the middle
            let row_width = 4.0 * SMALL_BUTTON_WIDTH + 3.0 * ui.spacing().item_spacing.x;
            let pad = ((ui.available_width() - row_width) / 2.0).max(0.0);
            ui.horizontal(|ui| {
                ui.add_space(pad);
                if small_button(ui, "update brave") {
                    self.start_worker(brave::update_brave);
                }
                if small_button(ui, "kill wine") {
                    match launcher::kill_wine() {
                        Ok(()) => self.log_lines.push("wine killed".to_string()),
                        Err(e) => self.error = Some(e),
                    }
                }
                if small_button(ui, "reinstall") {
                    // wipe the prefix but keep the wine download
                    let _ = std::fs::remove_dir_all(paths::prefix_dir());
                    self.start_worker(setup::run_install);
                }
                if self.confirm_uninstall {
                    if small_button(ui, "confirm?") {
                        self.uninstall();
                    }
                } else if small_button(ui, "uninstall") {
                    self.confirm_uninstall = true;
                }
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.label("this will download wine and brave, then set everything up for you.");
                ui.add_space(4.0);
                if big_button(ui, "install", 200.0) {
                    self.start_worker(setup::run_install);
                }
            });
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_worker();
        if self.busy {
            // keep polling while the worker runs
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }

        // icon in the top right corner
        let icon_size = 48.0;
        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(
                ui.max_rect().right() - icon_size - 14.0,
                ui.max_rect().top() + 14.0,
            ),
            egui::vec2(icon_size, icon_size),
        );
        ui.put(
            icon_rect,
            egui::Image::new(egui::include_image!("../assets/icon.png")),
        );

        ui.add_space(4.0);
        ui.vertical_centered(|ui| {
            ui.heading("prime-wine");
            ui.label(
                egui::RichText::new("watch prime video in hd with brave and wine")
                    .color(theme::DIM_TEXT),
            );
        });
        ui.add_space(10.0);

        if let Some(e) = &self.error {
            let text = e.clone();
            ui.colored_label(theme::ERROR_TEXT, text);
            ui.add_space(8.0);
        }

        if self.busy {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("working...");
            });
            ui.add(egui::ProgressBar::new(self.progress).show_percentage());
        } else {
            self.draw_buttons(ui);
        }

        ui.add_space(10.0);
        section_header(ui, "terminal");
        egui::Frame::new()
            .fill(ui.visuals().extreme_bg_color)
            .corner_radius(egui::CornerRadius::same(10))
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for line in &self.log_lines {
                            ui.monospace(line);
                        }
                    });
            });
    }
}
