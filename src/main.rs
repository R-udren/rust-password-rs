use eframe::egui;
use rust_password::Data;
use tracing_subscriber::EnvFilter;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Password",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
}

#[derive(Default)]
struct App {
    result: Option<Result<Data, String>>,
    reveal: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Rust Password");
            ui.label("Read the latest values saved by Rust.");
            ui.add_space(12.0);

            if ui.button("Scan").clicked() {
                self.reveal = false;
                self.result = Some(rust_password::scan().map_err(|error| {
                    tracing::error!(%error, "scan failed");
                    error.to_string()
                }));
            }
            ui.separator();

            match &self.result {
                None => {
                    ui.weak("Press Scan to read the registry.");
                }
                Some(Err(error)) => {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }
                Some(Ok(data)) => show(ui, data, &mut self.reveal),
            }
        });
    }
}

fn show(ui: &mut egui::Ui, data: &Data, reveal: &mut bool) {
    ui.horizontal(|ui| {
        ui.strong("Last code");
        if ui.button(if *reveal { "Hide" } else { "Show" }).clicked() {
            *reveal = !*reveal;
        }
    });
    let mut code = data.last_code.clone();
    ui.add(
        egui::TextEdit::singleline(&mut code)
            .password(!*reveal)
            .interactive(false)
            .desired_width(f32::INFINITY),
    );
    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.strong("Console history");
        ui.weak(format!("{} entries", data.history.len()));
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        if data.history.is_empty() {
            ui.weak("No commands found.");
            return;
        }
        for command in &data.history {
            ui.monospace(command);
        }
    });
}
