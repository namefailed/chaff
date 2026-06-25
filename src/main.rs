#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod update;

use eframe::egui;
use std::collections::HashMap;
use std::io::Write;

fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--ghost") {
        loop { std::thread::sleep(std::time::Duration::from_secs(3600)); }
    }

    eframe::run_native(
        "Chaff",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("Chaff")
                .with_inner_size([560.0, 660.0])
                .with_resizable(false),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

// ── State ─────────────────────────────────────────────────────────────────────

struct CatState {
    reg_count: usize,
    handles: Vec<engine::Handle>,
    ghosts: Vec<engine::GhostProcess>,
}

impl CatState {
    fn process_count(&self) -> usize { self.ghosts.len() }
    fn handle_count(&self) -> usize { self.handles.len() }
}

struct TrayHandle {
    _icon: tray_icon::TrayIcon,
    open_id: tray_icon::menu::MenuId,
    quit_id: tray_icon::menu::MenuId,
}

impl TrayHandle {
    fn set_tooltip(&self, text: &str) {
        let _ = self._icon.set_tooltip(Some(text));
    }
}

struct App {
    db: engine::Db,
    enabled: HashMap<String, bool>,
    active: HashMap<String, CatState>,
    process_count: usize,
    status: String,
    tray: Option<TrayHandle>,
    // True only when Quit is chosen from tray — lets us distinguish from the
    // window X button, which should hide to tray rather than exit.
    quitting: bool,
}

impl App {
    fn new() -> Self {
        let db = engine::load();
        let enabled = db.categories.iter().map(|c| (c.id.clone(), true)).collect();
        log("Chaff started");
        Self {
            db,
            enabled,
            active: HashMap::new(),
            process_count: 5,
            status: String::new(),
            tray: setup_tray(),
            quitting: false,
        }
    }

    fn total_processes(&self) -> usize {
        self.active.values().map(|s| s.process_count()).sum()
    }

    fn total_keys(&self) -> usize {
        self.active.values().map(|s| s.reg_count).sum()
    }

    fn total_handles(&self) -> usize {
        self.active.values().map(|s| s.handle_count()).sum()
    }

    fn update_tray(&self) {
        let Some(tray) = &self.tray else { return };
        let procs = self.total_processes();
        let keys = self.total_keys();
        if procs > 0 || keys > 0 {
            tray.set_tooltip(&format!("Chaff — {procs} processes, {keys} keys active"));
        } else {
            tray.set_tooltip("Chaff — inactive");
        }
    }

    fn remove_all(&mut self) {
        let ids: Vec<_> = self.active.keys().cloned().collect();
        for id in &ids {
            if let Some(cat) = self.db.categories.iter().find(|c| c.id == *id) {
                engine::remove_registry(cat);
            }
        }
        // Clear drops all Handles (CloseHandle) and GhostProcesses (kill+wait)
        self.active.clear();
        // Safe to delete temp dir now that all ghost processes are dead
        engine::cleanup_ghost_temp();
    }
}

// ── Cleanup on exit ───────────────────────────────────────────────────────────

impl eframe::App for App {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.remove_all();
        log("Chaff exited");
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll tray events every frame — needed even when minimized
        ctx.request_repaint_after(std::time::Duration::from_millis(200));

        // ── Tray menu events ──────────────────────────────────────────────────
        if let Ok(ev) = tray_icon::menu::MenuEvent::receiver().try_recv() {
            if let Some(tray) = &self.tray {
                if ev.id == tray.open_id {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                } else if ev.id == tray.quit_id {
                    self.quitting = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }

        // Tray icon left-click → restore window
        if tray_icon::TrayIconEvent::receiver().try_recv().is_ok() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }

        // ── Close button → hide to tray (unless Quit was chosen) ─────────────
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.quitting {
                // Let eframe close normally; on_exit will clean up
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            }
        }

        // ── Pre-compute for borrow checker ────────────────────────────────────
        let cats: Vec<(String, String, String, bool)> = self.db.categories.iter()
            .map(|c| (c.id.clone(), c.name.clone(), c.description.clone(), c.has_processes()))
            .collect();
        let db_version = self.db.version.clone();
        let total_procs = self.total_processes();
        let total_keys = self.total_keys();
        let total_handles = self.total_handles();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);

            // Header
            ui.horizontal(|ui| {
                ui.heading("Chaff");
                ui.add_space(12.0);
                let stats_color = if total_procs > 0 || total_keys > 0 {
                    egui::Color32::from_rgb(80, 200, 120)
                } else {
                    egui::Color32::GRAY
                };
                ui.label(egui::RichText::new(
                    format!("{total_procs} processes  ·  {total_keys} reg keys  ·  {total_handles} handles")
                ).color(stats_color).small());
            });
            ui.label("Makes your machine look like a malware analyst's sandbox.");
            ui.add_space(6.0);

            // Process count slider
            ui.horizontal(|ui| {
                ui.label("Processes per category:");
                ui.add(egui::Slider::new(&mut self.process_count, 1..=20));
            });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            egui::ScrollArea::vertical().max_height(430.0).show(ui, |ui| {
                for (id, name, desc, is_proc_cat) in &cats {
                    let is_active = self.active.contains_key(id);
                    let mut checked = self.enabled.get(id).copied().unwrap_or(true);

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut checked, name);
                        if is_active {
                            let badge = if *is_proc_cat {
                                format!("● {} running", self.active[id].process_count())
                            } else {
                                "● active".to_string()
                            };
                            ui.label(
                                egui::RichText::new(badge)
                                    .color(egui::Color32::from_rgb(80, 200, 120))
                                    .small(),
                            );
                        }
                    });
                    self.enabled.insert(id.clone(), checked);
                    ui.label(egui::RichText::new(desc).small().weak());
                    ui.add_space(4.0);
                }
            });

            ui.separator();
            ui.add_space(6.0);

            let any_enabled = self.enabled.values().any(|&v| v);
            let any_active = !self.active.is_empty();

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(any_enabled && !any_active, egui::Button::new("▶  Apply"))
                    .clicked()
                {
                    let to_apply: Vec<_> = self.db.categories.iter()
                        .filter(|c| self.enabled.get(&c.id).copied().unwrap_or(false))
                        .cloned()
                        .collect();

                    let mut msgs = Vec::new();
                    let proc_limit = self.process_count;

                    for cat in to_apply {
                        let limit = if cat.has_processes() { Some(proc_limit) } else { None };
                        let result = engine::apply(&cat, limit);

                        let ghost_names: Vec<String> = result.ghosts.iter().map(|g| g.name.clone()).collect();

                        if result.fail > 0 {
                            let msg = format!("{}: {} ok, {} failed (admin?)", cat.name, result.ok, result.fail);
                            log(&msg);
                            msgs.push(msg);
                        } else {
                            let msg = format!("{}: {} artifacts", cat.name, result.ok);
                            log(&msg);
                            msgs.push(msg);
                        }

                        if !ghost_names.is_empty() {
                            log(&format!("  processes: {}", ghost_names.join(", ")));
                        }

                        self.active.insert(cat.id.clone(), CatState {
                            reg_count: result.reg_count,
                            handles: result.handles,
                            ghosts: result.ghosts,
                        });
                    }

                    self.status = msgs.join("  |  ");
                    self.update_tray();
                }

                ui.add_space(8.0);

                if ui
                    .add_enabled(any_active, egui::Button::new("■  Remove"))
                    .clicked()
                {
                    // Collect messages before remove_all clears active
                    let ids: Vec<_> = self.active.keys().cloned().collect();
                    let mut msgs = Vec::new();
                    for id in &ids {
                        if let Some(cat) = self.db.categories.iter().find(|c| c.id == *id) {
                            msgs.push(cat.name.clone());
                        }
                    }

                    self.remove_all();
                    log("All artifacts removed");

                    self.status = format!("Removed: {}", msgs.join(", "));
                    self.update_tray();
                }
            });

            if !self.status.is_empty() {
                ui.add_space(6.0);
                ui.label(egui::RichText::new(&self.status).small().weak());
            }

            ui.add_space(4.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("db v{db_version}")).small().weak());
                ui.label(
                    egui::RichText::new("  ·  HKLM keys require admin  ·  processes write to %TEMP%\\chaff\\")
                        .small()
                        .weak(),
                );
            });
        });
    }
}

// ── Tray setup ────────────────────────────────────────────────────────────────

fn setup_tray() -> Option<TrayHandle> {
    use tray_icon::{TrayIconBuilder, Icon};
    use tray_icon::menu::{Menu, MenuItem};

    let size = 32u32;
    let rgba: Vec<u8> = (0..size * size).flat_map(|_| [160u8, 40, 40, 255]).collect();
    let icon = Icon::from_rgba(rgba, size, size).ok()?;

    let open_item = MenuItem::new("Open Chaff", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    let menu = Menu::new();
    let _ = menu.append(&open_item);
    let _ = menu.append(&quit_item);

    let icon = TrayIconBuilder::new()
        .with_tooltip("Chaff — inactive")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()
        .ok()?;

    Some(TrayHandle { _icon: icon, open_id, quit_id })
}

// ── Logging ───────────────────────────────────────────────────────────────────

fn log(msg: &str) {
    let Ok(appdata) = std::env::var("APPDATA") else { return };
    let dir = std::path::Path::new(&appdata).join("chaff");
    let _ = std::fs::create_dir_all(&dir);
    let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open(dir.join("chaff.log")) else { return };
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let _ = writeln!(f, "[{ts}] {msg}");
}
