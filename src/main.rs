//! Native GUI entry point.
#![cfg_attr(windows, windows_subsystem = "windows")]

mod ui;

#[path = "../branding.rs"]
mod branding;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_title(branding::PRODUCT)
            .with_icon(egui::IconData {
                rgba: branding::icon_rgba(),
                width: branding::ICON_SIZE,
                height: branding::ICON_SIZE,
            })
            .with_inner_size([760.0, 600.0])
            .with_min_inner_size([560.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        branding::PRODUCT,
        options,
        Box::new(|context| {
            ui::theme::configure(&context.egui_ctx);
            Ok(Box::<ui::App>::default())
        }),
    )
}
