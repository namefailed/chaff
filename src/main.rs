#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod update;

use eframe::egui;
use std::collections::HashMap;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Chaff",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("Chaff")
                .with_inner_size([520.0, 480.0])
                .with_resizable(false),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

struct App {
    db: engine::Db,
    enabled: HashMap<String, bool>,
    active: HashMap<String, bool>,
    // Live mutex/pipe handles per category — dropped on Remove to close them
    handles: HashMap<String, Vec<engine::Handle>>,
    status: String,
}

impl App {
    fn new() -> Self {
        let db = engine::load();
        let enabled = db.categories.iter().map(|c| (c.id.clone(), true)).collect();
        let active = db.categories.iter().map(|c| (c.id.clone(), false)).collect();
        Self { db, enabled, active, handles: HashMap::new(), status: String::new() }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let cats: Vec<(String, String, String)> = self.db.categories.iter()
            .map(|c| (c.id.clone(), c.name.clone(), c.description.clone()))
            .collect();
        let db_version = self.db.version.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("Chaff");
            ui.label("Makes your machine look like a malware analyst's sandbox.");
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            for (id, name, desc) in &cats {
                let active = self.active.get(id).copied().unwrap_or(false);
                let mut checked = self.enabled.get(id).copied().unwrap_or(true);

                ui.horizontal(|ui| {
                    ui.checkbox(&mut checked, name);
                    if active {
                        ui.label(
                            egui::RichText::new("● active")
                                .color(egui::Color32::from_rgb(80, 200, 120))
                                .small(),
                        );
                    }
                });
                self.enabled.insert(id.clone(), checked);
                ui.label(egui::RichText::new(desc).small().weak());
                ui.add_space(6.0);
            }

            ui.separator();
            ui.add_space(6.0);

            let any_enabled = self.enabled.values().any(|&v| v);
            let any_active = self.active.values().any(|&v| v);

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
                    for cat in to_apply {
                        let (ok, fail, new_handles) = engine::apply(&cat);
                        self.active.insert(cat.id.clone(), ok > 0);
                        self.handles.insert(cat.id.clone(), new_handles);
                        if fail > 0 {
                            msgs.push(format!("{}: {ok} ok, {fail} failed (run as admin?)", cat.name));
                        } else {
                            msgs.push(format!("{}: {ok} artifacts", cat.name));
                        }
                    }
                    self.status = msgs.join("  |  ");
                }

                ui.add_space(8.0);

                if ui
                    .add_enabled(any_active, egui::Button::new("■  Remove"))
                    .clicked()
                {
                    let to_remove: Vec<_> = self.db.categories.iter()
                        .filter(|c| self.active.get(&c.id).copied().unwrap_or(false))
                        .cloned()
                        .collect();

                    let mut msgs = Vec::new();
                    for cat in to_remove {
                        let reg_removed = engine::remove_registry(&cat);
                        let handle_count = self.handles.remove(&cat.id).map(|v| v.len()).unwrap_or(0);
                        self.active.insert(cat.id.clone(), false);
                        msgs.push(format!("{}: {} removed", cat.name, reg_removed + handle_count));
                    }
                    self.status = msgs.join("  |  ");
                }
            });

            if !self.status.is_empty() {
                ui.add_space(6.0);
                ui.label(egui::RichText::new(&self.status).small().weak());
            }

            ui.add_space(4.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Artifacts db v".to_string() + &db_version)
                        .small()
                        .weak(),
                );
                ui.label(
                    egui::RichText::new("  ·  HKLM keys require admin  ·  HKCU keys work without elevation")
                        .small()
                        .weak(),
                );
            });
        });
    }
}
