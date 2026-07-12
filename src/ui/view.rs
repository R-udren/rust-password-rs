use eframe::egui;
use rust_password::Data;

use super::theme::{ACCENT, BORDER, COMMAND, PANEL, ROW, SURFACE};

pub fn scan(ui: &mut egui::Ui, error: Option<&str>) -> bool {
    let mut scan = false;
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Rust Pass Reveal");
            ui.add_space(18.0);
            if let Some(error) = error {
                ui.colored_label(ui.visuals().error_fg_color, error);
                ui.add_space(12.0);
            }
            let text = egui::RichText::new("Scan")
                .strong()
                .size(18.0)
                .color(egui::Color32::from_gray(18));
            let button = egui::Button::new(text)
                .fill(ACCENT)
                .stroke(egui::Stroke::NONE)
                .min_size(egui::vec2(180.0, 54.0));
            scan = ui.add(button).clicked();
        });
    });
    scan
}

pub fn content(ui: &mut egui::Ui, data: &Data, reveal: &mut bool) {
    ui.vertical_centered(|ui| {
        ui.heading("Rust Pass Reveal");
    });
    ui.add_space(16.0);
    code(ui, &data.last_code, reveal);
    ui.add_space(12.0);
    status(ui, data);
    ui.add_space(18.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Console history").strong().size(16.0));
        ui.weak(data.history.len().to_string());
    });
    ui.add_space(8.0);

    egui::Frame::new()
        .fill(SURFACE)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
        .inner_margin(4.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| commands(ui, &data.history));
        });
}

fn status(ui: &mut egui::Ui, data: &Data) {
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| steam(ui, data));
        card(&mut columns[1], |ui| rust(ui, data));
    });
}

fn card(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_height(96.0);
            content(ui);
        });
}

fn steam(ui: &mut egui::Ui, data: &Data) {
    ui.label(egui::RichText::new("Steam").strong().size(16.0));
    ui.add_space(5.0);
    ui.label(
        egui::RichText::new(&data.steam.last_game)
            .strong()
            .size(20.0),
    );
    ui.add_space(8.0);
    let steam_id = data
        .steam
        .steam_id
        .map_or_else(|| "No active user".to_owned(), |id| id.to_string());
    ui.weak(egui::RichText::new(steam_id).monospace());
}

fn rust(ui: &mut egui::Ui, data: &Data) {
    ui.label(egui::RichText::new("Game").strong().size(16.0));
    ui.add_space(5.0);
    ui.label(egui::RichText::new(&data.rust.name).strong().size(20.0));
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        pill(
            ui,
            if data.rust.installed {
                "Installed"
            } else {
                "Not installed"
            },
            data.rust.installed,
        );
        pill(
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

fn pill(ui: &mut egui::Ui, label: &str, active: bool) {
    let (fill, text) = if active {
        (ACCENT, egui::Color32::from_gray(18))
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

fn code(ui: &mut egui::Ui, code: &str, reveal: &mut bool) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Last code").strong().size(16.0));
            });
            ui.add_space(10.0);
            let mut value = code.to_owned();
            ui.horizontal(|ui| {
                const FIELD_WIDTH: f32 = 120.0;
                const BUTTON_WIDTH: f32 = 104.0;
                let group_width = FIELD_WIDTH + BUTTON_WIDTH + ui.spacing().item_spacing.x;
                ui.add_space(((ui.available_width() - group_width) / 2.0).max(0.0));
                ui.add_sized(
                    [FIELD_WIDTH, 42.0],
                    egui::TextEdit::singleline(&mut value)
                        .font(egui::FontId::monospace(22.0))
                        .horizontal_align(egui::Align::Center)
                        .vertical_align(egui::Align::Center)
                        .password(!*reveal)
                        .interactive(false),
                );
                let label = if *reveal { "Hide code" } else { "Reveal code" };
                if ui
                    .add(egui::Button::new(label).min_size(egui::vec2(BUTTON_WIDTH, 42.0)))
                    .clicked()
                {
                    *reveal = !*reveal;
                }
            });
        });
}

fn commands(ui: &mut egui::Ui, history: &[String]) {
    if history.is_empty() {
        ui.add_space(12.0);
        ui.centered_and_justified(|ui| {
            ui.weak("No commands found.");
        });
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
