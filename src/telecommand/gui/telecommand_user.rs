use crate::telecommand::telecommand::Telecommand;
use eframe::egui;
use egui::Button;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct GuiCommand<'a> {
    pub telecommand: Arc<Telecommand>,
    pub keys: &'a HashMap<&'static str, u16>,
}

impl<'a> GuiCommand<'a> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        ip: String,
        port: String,
        keys: &'a HashMap<&'static str, u16>,
    ) -> Self {
        crate::telecommand::gui::telecommand_builder::set_styles(&cc.egui_ctx);
        Self {
            telecommand: Arc::new(Telecommand::new(ip, port)),
            keys,
        }
    }
}

impl eframe::App for GuiCommand<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let telecommand = Arc::clone(&self.telecommand);
            ui.vertical_centered(|ui| {
                // Taille du bouton (largeur = hauteur)
                let size = egui::Vec2::new(60.0, 60.0);

                // Ajout d’un bouton rond centré
                if ui
                    .add_sized(size, Button::new("ON").corner_radius(size.x / 2.0))
                    .clicked()
                {
                    let val = *self.keys.get("on").unwrap();
                    tokio::spawn(async move {
                        let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                        telecommand.send_request(url).await.expect("Could not send the url");
                    });
                }
            });

            let telecommand = Arc::clone(&self.telecommand);
            ui.vertical_centered(|ui| {
                if ui
                    .add_sized(
                        egui::Vec2::new(80.0, 30.0),
                        Button::new("Menu").corner_radius(10.0),
                    )
                    .clicked()
                {
                    let val = *self.keys.get("menu").unwrap();
                    tokio::spawn(async move {
                        let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                        let resp = telecommand.send_request(url).await.expect("Could not send the url");
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Bouton UP en haut
                let telecommand = Arc::clone(&self.telecommand);
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 30.0);
                    if ui.add_sized(egui::Vec2::new(60.0, 50.0), Button::new("Up")).clicked() {
                        let val = *self.keys.get("up").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }
                });

                let telecommand = Arc::clone(&self.telecommand);
                // Ligne du milieu avec Left, OK, Right
                ui.horizontal(|ui| {
                    let total_width = 50.0 + 60.0 + 50.0; // Left + OK + Right
                    let left_space = (ui.available_width() - total_width - 14.0) / 2.0;
                    ui.add_space(left_space);

                let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("Left")).clicked() {
                        let val = *self.keys.get("left").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });

                    }

                let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(60.0, 50.0), Button::new("OK")).clicked() {
                        let val = *self.keys.get("ok").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }

                let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("Right")).clicked() {
                        let val = *self.keys.get("right").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }
                });

                let telecommand = Arc::clone(&self.telecommand);
                // Bouton DOWN en bas
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 30.0);
                    if ui.add_sized(egui::Vec2::new(60.0, 50.0), Button::new("Down")).clicked() {
                        let val = *self.keys.get("down").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });

                    }
                });
            });

            let telecommand = Arc::clone(&self.telecommand);
            ui.vertical_centered(|ui| {
                if ui
                    .add_sized(
                        egui::Vec2::new(80.0, 30.0),
                        Button::new("Back").corner_radius(10.0),
                    )
                        .clicked()
                {
                    let val = *self.keys.get("back").unwrap();
                    tokio::spawn(async move {
                        let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                        let resp = telecommand.send_request(url).await.expect("Could not send the url");
                    });
                }
            });

            ui.separator();

            ui.horizontal_centered(|ui| {
                // Ajouter un espace pour centrer la matrice
                let matrix_width = 3.0 * 50.0; // 3 colonnes de 50px chacune
                let left_space = (ui.available_width() - matrix_width - 15.0) / 2.0;
                ui.add_space(left_space);

                // Colonne 1 : 1, 4, 7
                ui.vertical(|ui| {
                    for i in ['1', '4', '7'] {
                        let telecommand = Arc::clone(&self.telecommand);
                        if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new(&i.to_string()).corner_radius(25.0)).clicked() {
                            let key = i.to_string();      // key: String
                            let val = *self.keys.get(key.as_str()).unwrap(); // key.as_str(): &str
                            tokio::spawn(async move {
                                let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                                let resp = telecommand.send_request(url).await.expect("Could not send the url");
                            });

                        }
                    }
                });

                // Colonne 2 : 2, 5, 8
                ui.vertical(|ui| {
                    for i in ['2', '5', '8'] {
                        let telecommand = Arc::clone(&self.telecommand);
                        if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new(&i.to_string()).corner_radius(25.0)).clicked() {
                            let key = i.to_string();      // key: String
                            let val = *self.keys.get(key.as_str()).unwrap(); // key.as_str(): &str
                            tokio::spawn(async move {
                                let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                                let resp = telecommand.send_request(url).await.expect("Could not send the url");
                            });
                        }
                    }
                });

                ui.vertical(|ui| {
                    for i in ['3', '6', '9'] {
                        let telecommand = Arc::clone(&self.telecommand);
                        if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new(&i.to_string()).corner_radius(25.0)).clicked() {
                            let key = i.to_string();      // key: String
                            let val = *self.keys.get(key.as_str()).unwrap(); // key.as_str(): &str
                            tokio::spawn(async move {
                                let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                                let resp = telecommand.send_request(url).await.expect("Could not send the url");
                            });
                        }
                    }
                });
           });

            ui.separator();
            
            // Matrice 2x2 pour VOL+, VOL-, CH+, CH-
            ui.horizontal_centered(|ui| {
                let matrix_width = 2.0 * 50.0; // 2 colonnes de 40px chacune
                let left_space = (ui.available_width() - matrix_width - 10.0) / 2.0;
                ui.add_space(left_space);
                
                // Colonne 1 : VOL+, CH+
                ui.vertical(|ui| {
                    let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("VOL+").corner_radius(25.0)).clicked() {
                        let val = *self.keys.get("vol+").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }

                    let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("VOL-").corner_radius(25.0)).clicked() {
                        let val = *self.keys.get("vol-").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }
                });
                
                // Colonne 2 : VOL-, CH-
                ui.vertical(|ui| {

                    let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("CH+").corner_radius(25.0)).clicked() {
                        let val = *self.keys.get("ch+").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });

                    }

                    let telecommand = Arc::clone(&self.telecommand);
                    if ui.add_sized(egui::Vec2::new(50.0, 50.0), Button::new("CH-").corner_radius(25.0)).clicked() {
                        let val = *self.keys.get("ch-").unwrap();
                        tokio::spawn(async move {
                            let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                            let resp = telecommand.send_request(url).await.expect("Could not send the url");
                        });
                    }
                });
            }); 
              
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {

                let telecommand = Arc::clone(&self.telecommand);
                ui.add_space(ui.available_width() / 2.0 - 50.0 - 5.0); // Décale vers le centre
                if ui
                    .add_sized(
                        egui::Vec2::new(50.0, 50.0),
                        Button::new("REC").corner_radius(25.0),
                    )
                    .clicked()
                {
                    let val = *self.keys.get("rec").unwrap();
                    tokio::spawn(async move {
                        let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                        let resp = telecommand.send_request(url).await.expect("Could not send the url");
                    });
                }

                let telecommand = Arc::clone(&self.telecommand);
                if ui
                    .add_sized(
                        egui::Vec2::new(50.0, 50.0),
                        Button::new("PLAY").corner_radius(25.0),
                    )
                        .clicked()
                {
                    let val = *self.keys.get("play").unwrap();
                    tokio::spawn(async move {
                        let url = telecommand.build_url(1, val, 0, 0).expect("Could not build the url");
                        let resp = telecommand.send_request(url).await.expect("Could not send the url");
                    });
                }
            });
        });
    }
}
