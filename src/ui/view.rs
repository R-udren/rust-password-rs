use eframe::egui;
use rust_password::Data;

use super::theme::{ACCENT, BORDER, COMMAND, PANEL, ROW, SURFACE};

pub fn scan(ui: &mut egui::Ui, error: Option<&str>) -> bool {
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading("Rust Pass Reveal");
            ui.add_space(20.0);
            if let Some(error) = error {
                ui.colored_label(ui.visuals().error_fg_color, error);
                ui.add_space(8.0);
            }
            let text = egui::RichText::new("Scan")
                .strong()
                .size(17.0)
                .color(SURFACE);
            ui.add(
                egui::Button::new(text)
                    .fill(ACCENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(7)
                    .min_size(egui::vec2(196.0, 52.0)),
            )
            .clicked()
        })
        .inner
    })
    .inner
}

pub fn content(ui: &mut egui::Ui, data: &Data, reveal: &mut bool) {
    ui.vertical_centered(|ui| {
        ui.heading("Rust Pass Reveal");
    });
    ui.add_space(18.0);
    code(ui, &data.last_code, reveal);
    ui.add_space(12.0);
    status(ui, data);
    ui.add_space(20.0);
    history(ui, &data.history);
}

fn code(ui: &mut egui::Ui, code: &str, reveal: &mut bool) {
    frame()
        .inner_margin(egui::Margin::symmetric(18, 16))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Last code").size(15.0).strong());
                ui.add_space(11.0);
                let mut value = code.to_owned();
                ui.horizontal(|ui| {
                    const FIELD_WIDTH: f32 = 118.0;
                    const BUTTON_WIDTH: f32 = 104.0;
                    let width = FIELD_WIDTH + BUTTON_WIDTH + ui.spacing().item_spacing.x;
                    ui.add_space(((ui.available_width() - width) / 2.0).max(0.0));
                    ui.add_sized(
                        [FIELD_WIDTH, 40.0],
                        egui::TextEdit::singleline(&mut value)
                            .font(egui::FontId::monospace(21.0))
                            .horizontal_align(egui::Align::Center)
                            .vertical_align(egui::Align::Center)
                            .password(!*reveal)
                            .interactive(false),
                    );
                    if ui
                        .add_sized(
                            [BUTTON_WIDTH, 40.0],
                            egui::Button::new(if *reveal { "Hide" } else { "Reveal" })
                                .corner_radius(6),
                        )
                        .clicked()
                    {
                        *reveal = !*reveal;
                    }
                });
            });
        });
}

fn status(ui: &mut egui::Ui, data: &Data) {
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| steam(ui, data));
        card(&mut columns[1], |ui| rust(ui, data));
    });
}

fn card(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    frame()
        .inner_margin(egui::Margin::symmetric(16, 15))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(106.0);
            content(ui);
        });
}

fn steam(ui: &mut egui::Ui, data: &Data) {
    ui.label(egui::RichText::new("Steam").size(15.0).strong());
    ui.add_space(7.0);
    ui.label(egui::RichText::new(&data.steam.last_game).size(19.0));
    ui.add_space(9.0);
    let steam_id = data
        .steam
        .steam_id
        .map_or_else(|| "No active user".to_owned(), |id| id.to_string());
    ui.weak(egui::RichText::new(steam_id).monospace());
}

fn rust(ui: &mut egui::Ui, data: &Data) {
    ui.label(egui::RichText::new("Game").size(15.0).strong());
    ui.add_space(7.0);
    ui.label(egui::RichText::new(&data.rust.name).size(19.0));
    ui.add_space(9.0);
    ui.horizontal(|ui| {
        badge(
            ui,
            if data.rust.installed {
                "Installed"
            } else {
                "Not installed"
            },
            data.rust.installed,
        );
        badge(
            ui,
            if data.rust.running {
                "Running"
            } else {
                "Not running"
            },
            data.rust.running,
        );
    });
}

fn badge(ui: &mut egui::Ui, label: &str, active: bool) {
    let (fill, text) = if active {
        (ACCENT, SURFACE)
    } else {
        (SURFACE, egui::Color32::from_gray(175))
    };
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(10)
        .inner_margin(egui::Margin::symmetric(9, 4))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(label).size(12.0).color(text));
        });
}

fn history(ui: &mut egui::Ui, history: &[String]) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Console history").size(16.0).strong());
        ui.weak(history.len().to_string());
    });
    ui.add_space(2.0);
    frame().inner_margin(4.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(ui.available_height().max(160.0))
            .show(ui, |ui| commands(ui, history));
    });
}

fn frame() -> egui::Frame {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
}

fn commands(ui: &mut egui::Ui, history: &[String]) {
    if history.is_empty() {
        ui.add_space(12.0);
        ui.centered_and_justified(|ui| ui.weak("No commands found."));
        return;
    }

    for (index, command) in history.iter().rev().enumerate() {
        let fill = if index % 2 == 0 {
            ROW
        } else {
            egui::Color32::TRANSPARENT
        };
        egui::Frame::new()
            .fill(fill)
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(10, 7))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(egui::RichText::new(command).monospace().color(COMMAND));
            });
    }
}
