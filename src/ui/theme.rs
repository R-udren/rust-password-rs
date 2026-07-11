use eframe::egui;

pub(crate) const PANEL: egui::Color32 = egui::Color32::from_rgb(24, 29, 39);
pub(crate) const SURFACE: egui::Color32 = egui::Color32::from_rgb(17, 21, 29);
pub(crate) const BORDER: egui::Color32 = egui::Color32::from_rgb(45, 53, 68);
pub(crate) const ROW: egui::Color32 = egui::Color32::from_rgb(22, 27, 36);
pub(crate) const ACCENT: egui::Color32 = egui::Color32::from_rgb(68, 119, 246);
pub(crate) const COMMAND: egui::Color32 = egui::Color32::from_rgb(215, 220, 230);

pub(crate) fn configure(context: &egui::Context) {
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
    visuals.panel_fill = egui::Color32::from_rgb(12, 15, 21);
    visuals.extreme_bg_color = egui::Color32::from_rgb(13, 16, 22);
    visuals.text_edit_bg_color = Some(egui::Color32::from_rgb(13, 16, 22));
    visuals.override_text_color = Some(egui::Color32::from_rgb(235, 238, 245));
    visuals.weak_text_color = Some(egui::Color32::from_rgb(150, 158, 174));
    visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(38, 45, 58);
    visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(51, 61, 78);
    visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(62, 73, 94);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(230, 234, 242);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.selection.bg_fill = ACCENT;
    context.set_visuals(visuals);
}
