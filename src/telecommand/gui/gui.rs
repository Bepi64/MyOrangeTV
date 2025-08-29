use crate::telecommand::telecommand::Telecommand;
use eframe::{egui, glow::Context};

#[derive(Default)]
pub struct GuiTelecommand {}

impl GuiTelecommand {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GuiTelecommand {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }


    fn on_exit(&mut self, _gl: Option<&Context>) {
        // Perform any necessary cleanup here
    }
}
