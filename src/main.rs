mod ui;

#[path = "../branding.rs"]
#[allow(dead_code)]
mod branding;

use eframe::egui;
use tracing_subscriber::EnvFilter;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

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
