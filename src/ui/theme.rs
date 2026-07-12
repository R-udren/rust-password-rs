use eframe::egui;

pub const PANEL: egui::Color32 = egui::Color32::from_gray(18);
pub const SURFACE: egui::Color32 = egui::Color32::from_gray(10);
pub const BORDER: egui::Color32 = egui::Color32::from_gray(52);
pub const ROW: egui::Color32 = egui::Color32::from_gray(24);
pub const ACCENT: egui::Color32 = egui::Color32::from_gray(235);
pub const COMMAND: egui::Color32 = egui::Color32::from_gray(222);

pub fn configure(context: &egui::Context) {
    let mut style = egui::Style::default();
    style.spacing.item_spacing = egui::vec2(12.0, 10.0);
    style.spacing.button_padding = egui::vec2(15.0, 9.0);
    style.spacing.indent = 18.0;
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(26.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, egui::FontId::monospace(14.0));
    context.set_global_style(style);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_gray(6);
    visuals.extreme_bg_color = SURFACE;
    visuals.text_edit_bg_color = Some(egui::Color32::from_gray(8));
    visuals.override_text_color = Some(egui::Color32::from_gray(239));
    visuals.weak_text_color = Some(egui::Color32::from_gray(151));
    visuals.faint_bg_color = egui::Color32::from_gray(15);
    visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(29);
    visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(42);
    visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(57);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_gray(226);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.widgets.noninteractive.bg_stroke.color = BORDER;
    visuals.selection.bg_fill = egui::Color32::from_gray(72);
    context.set_visuals(visuals);
}
