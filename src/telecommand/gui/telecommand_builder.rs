use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TelecommandBuilder {
    pub ip_input: Rc<RefCell<String>>,
    pub port_input: Rc<RefCell<String>>,
    ip_check: bool,
    port_check: bool,
    submited: Rc<RefCell<bool>>,
}

impl TelecommandBuilder {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        ip_input: Rc<RefCell<String>>,
        port_input: Rc<RefCell<String>>,
        submited: Rc<RefCell<bool>>,
    ) -> Self {
        //see if we can do something else later
        set_styles(&cc.egui_ctx);
        let ip_check = false;
        let port_check = false;
        Self {
            ip_input,
            port_input,
            ip_check,
            port_check,
            submited,
        }
    }
}

impl eframe::App for TelecommandBuilder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut submited = self.submited.borrow_mut();
        let mut ip_input = self.ip_input.borrow_mut();
        let mut port_input = self.port_input.borrow_mut();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Please enter the informations !");
            ui.separator();
            ui.collapsing("New Orange TV", |ui| {
                ui.add(egui::Label::new(
                    "Here you'll enter and register the ip and port of your decoder",
                ));
                ui.horizontal(|ui| {
                    ui.label("Its IPv4 : ");
                    ui.text_edit_singleline(&mut *ip_input);
                });
                ui.horizontal(|ui| {
                    ui.label("Its port : ");
                    ui.text_edit_singleline(&mut *port_input);
                });

                ui.horizontal(|ui| {
                    if ui.button("Next").clicked() {
                        *submited = true;

                        if ip_input.is_empty() || ip_input.chars().any(|c| c.is_alphabetic()) {
                            //use regex
                            self.ip_check = false;
                        } else {
                            self.ip_check = true;
                        }

                        if port_input.is_empty() || port_input.chars().any(|c| c.is_alphabetic()) {
                            *submited = false;
                            self.port_check = false;
                        } else {
                            self.port_check = true;
                        }

                        if self.ip_check && self.port_check {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                    if ui.button("Clear").clicked() {
                        ip_input.clear();
                        port_input.clear();
                        *submited = false;
                    }
                });
            });
            ui.separator();
            ui.collapsing("Existant Orange TV", |ui| ui.label("todo()!"));
        });
    }
}

pub fn set_styles(ctx: &egui::Context) {
    use egui::FontFamily;
    use egui::{FontId, TextStyle::*};
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, FontFamily::Proportional)),
        (Body, FontId::new(18.0, FontFamily::Proportional)),
        (Button, FontId::new(15.0, FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
