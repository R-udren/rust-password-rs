use eframe::egui;
use rust_password::Data;

use super::theme::{ACCENT, BORDER, COMMAND, PANEL, ROW, SURFACE};

pub(crate) fn header(ui: &mut egui::Ui) -> bool {
    let mut scan = false;
    ui.horizontal(|ui| {
        ui.heading("Rust Password");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let button = egui::Button::new(egui::RichText::new("Scan").strong().size(15.0))
                .fill(ACCENT)
                .min_size(egui::vec2(96.0, 38.0));
            scan = ui.add(button).clicked();
        });
    });
    scan
}

pub(crate) fn empty(ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(PANEL)
        .corner_radius(8)
        .inner_margin(20.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.weak("No data");
        });
}

pub(crate) fn error(ui: &mut egui::Ui, error: &str) {
    ui.colored_label(ui.visuals().error_fg_color, error);
}

pub(crate) fn data(ui: &mut egui::Ui, data: &Data, reveal: &mut bool) {
    code(ui, &data.last_code, reveal);
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

fn code(ui: &mut egui::Ui, code: &str, reveal: &mut bool) {
    egui::Frame::new()
        .fill(PANEL)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Last code").strong().size(16.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = if *reveal { "Hide code" } else { "Reveal code" };
                    if ui
                        .add(egui::Button::new(label).min_size(egui::vec2(104.0, 30.0)))
                        .clicked()
                    {
                        *reveal = !*reveal;
                    }
                });
            });
            ui.add_space(8.0);
            let mut value = code.to_owned();
            ui.add_sized(
                [ui.available_width(), 36.0],
                egui::TextEdit::singleline(&mut value)
                    .font(egui::TextStyle::Monospace)
                    .password(!*reveal)
                    .interactive(false),
            );
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
