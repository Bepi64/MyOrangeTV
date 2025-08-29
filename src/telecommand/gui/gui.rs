use eframe::egui;

#[derive(Default)]
pub struct TelecommandBuilder {
    ip_input : String,
    port_input : String,
}

impl TelecommandBuilder {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //see if we can do something else later
        set_styles(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for TelecommandBuilder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Please enter the informations !");
            ui.collapsing("New Orange TV", |ui| {});
            ui.collapsing("Existant Orange TV", |ui| {});
        });
    }
}

fn set_styles(ctx: &egui::Context) {
    use egui::FontFamily;
    use egui::{FontId, TextStyle::*};
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, FontFamily::Proportional)),
        (Body, FontId::new(18.0, FontFamily::Proportional)),
        (Button, FontId::new(18.0, FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
