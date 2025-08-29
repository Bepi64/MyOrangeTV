use eframe::egui;
use colored::*;
use std::path::Path;

use eframe::glow::Context;

#[derive(Debug)]
pub struct TelecommandBuilder {
    pub ip_input: String,
    pub port_input: String,
    ip_check : bool,
    port_check : bool,
    submited : bool,
}

pub fn start_gui() {
    use eframe::NativeOptions;

    let mut options = NativeOptions {
        viewport: {
            egui::viewport::ViewportBuilder::default()
                .with_resizable(true)
                .with_inner_size([200.0, 200.0])
        },
        ..Default::default()
    };

    use eframe::wgpu::{Backends, InstanceDescriptor};
    if cfg!(target_os = "linux") {
        options.renderer = eframe::Renderer::Wgpu;
        let backend =
            eframe::egui_wgpu::WgpuSetup::CreateNew(eframe::egui_wgpu::WgpuSetupCreateNew {
                instance_descriptor: {
                    let mut hey = InstanceDescriptor::default();
                    hey.backends = Backends::VULKAN;
                    hey
                },
                ..Default::default()
            });
        options.wgpu_options.wgpu_setup = backend;
    }
    let _ = eframe::run_native(
        "My OrangeTV",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(TelecommandBuilder::new(cc)))
        }),
    );
}

impl Default for TelecommandBuilder{
    fn default() -> Self{
        let ip_input = String::new();
        let port_input = String::new();
        let ip_check = false;
        let port_check = false;
        let submited = false;
        Self {ip_input, port_input, ip_check, port_check, submited}
    }
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
                    ui.text_edit_singleline(&mut self.ip_input);
                });
                ui.horizontal(|ui| {
                    ui.label("Its port : ");
                    ui.text_edit_singleline(&mut self.port_input);
                });

                ui.horizontal(|ui| {
                    if ui.button("Submit").clicked() {
                        self.submited = true;

                        if self.ip_input.is_empty() || self.ip_input.chars().any(|c| c.is_alphabetic()) {
                            println!("{}", "Error for ip".red());
                            //use regex
                            self.ip_check = false;
                        }
                        else{
                            self.ip_check = true;
                        }

                        if self.port_input.is_empty() || self.port_input.chars().any(|c| c.is_alphabetic()) {
                            println!("{}", "Error for port".red());
                            self.submited = false;
                            self.port_check = false;
                        }
                        else{
                            self.port_check = true;
                        }

                        if self.ip_check && self.port_check
                        {
                            dbg!(&self);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        
                    }
                    if ui.button("Clear").clicked() {
                        self.ip_input.clear();
                        self.port_input.clear();
                        self.submited = false;
                    }
                });
            });
            ui.separator();
            ui.collapsing("Existant Orange TV", |ui| {});
        });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        if self.submited && self.ip_check && self.port_check{
            todo!("Lancement de l'application");
        }
        else{
            println!("Nothing to do!");
        }
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
