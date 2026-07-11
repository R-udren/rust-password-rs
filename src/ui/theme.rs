use eframe::egui;

pub const PANEL: egui::Color32 = egui::Color32::from_gray(25);
pub const SURFACE: egui::Color32 = egui::Color32::from_gray(15);
pub const BORDER: egui::Color32 = egui::Color32::from_gray(48);
pub const ROW: egui::Color32 = egui::Color32::from_gray(22);
pub const ACCENT: egui::Color32 = egui::Color32::from_gray(224);
pub const COMMAND: egui::Color32 = egui::Color32::from_gray(220);

pub fn configure(context: &egui::Context) {
    let mut style = egui::Style::default();
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 8.0);
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(24.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(14.0));
    context.set_global_style(style);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_gray(10);
    visuals.extreme_bg_color = egui::Color32::from_gray(12);
    visuals.text_edit_bg_color = Some(egui::Color32::from_gray(12));
    visuals.override_text_color = Some(egui::Color32::from_gray(235));
    visuals.weak_text_color = Some(egui::Color32::from_gray(145));
    visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(40);
    visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(55);
    visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(68);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_gray(230);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_gray(90);
    context.set_visuals(visuals);
}
