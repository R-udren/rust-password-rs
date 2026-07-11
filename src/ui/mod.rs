pub(crate) mod theme;
mod view;

use eframe::egui;
use rust_password::Data;

#[derive(Default)]
pub(crate) struct App {
    result: Option<Result<Data, String>>,
    reveal: bool,
}

impl App {
    fn scan(&mut self) {
        self.reveal = false;
        self.result = Some(rust_password::scan().map_err(|error| {
            tracing::error!(%error, "scan failed");
            error.to_string()
        }));
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(ui.style()).inner_margin(20.0))
            .show(ui, |ui| match &self.result {
                None => {
                    if view::scan(ui, None) {
                        self.scan();
                    }
                }
                Some(Err(error)) => {
                    if view::scan(ui, Some(error)) {
                        self.scan();
                    }
                }
                Some(Ok(data)) => view::content(ui, data, &mut self.reveal),
            });
    }
}
