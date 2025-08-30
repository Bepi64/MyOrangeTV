use crate::telecommand::gui::{telecommand_user::GuiCommand, telecommand_builder::TelecommandBuilder};
use eframe::{egui, NativeOptions, wgpu::{Backends, InstanceDescriptor}};
use std::rc::Rc;
use std::cell::RefCell;

pub fn start_gui() {

    let mut options1 = NativeOptions {
        viewport: {
            egui::viewport::ViewportBuilder::default()
                .with_resizable(true)
                .with_inner_size([600.0, 600.0])
        },
        ..Default::default()
    };

    let mut options2 = NativeOptions {
        viewport: {
            egui::viewport::ViewportBuilder::default()
                .with_inner_size([200.0, 1800.0])
                .with_close_button(true)
        },
        ..Default::default()
    };


    if cfg!(target_os = "linux") {

        {
            options1.renderer = eframe::Renderer::Wgpu;
            let backend =
                eframe::egui_wgpu::WgpuSetup::CreateNew(eframe::egui_wgpu::WgpuSetupCreateNew {
                    instance_descriptor: {
                        let mut hey = InstanceDescriptor::default();
                        hey.backends = Backends::VULKAN;
                        hey
                    },
                    ..Default::default()
                });
            options1.wgpu_options.wgpu_setup = backend;
        }

        {
            options2.renderer = eframe::Renderer::Wgpu;
            let backend =
                eframe::egui_wgpu::WgpuSetup::CreateNew(eframe::egui_wgpu::WgpuSetupCreateNew {
                    instance_descriptor: {
                        let mut hey = InstanceDescriptor::default();
                        hey.backends = Backends::VULKAN;
                        hey
                    },
                    ..Default::default()
                });
            options2.wgpu_options.wgpu_setup = backend;
        }
    }

    let ip = Rc::new(RefCell::new(String::new()));
    let port = Rc::new(RefCell::new(String::new()));
    let next = Rc::new(RefCell::new(false));
                                           
    let _ = eframe::run_native(
        "My Orange Decoder Information",
        options1,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(TelecommandBuilder::new(cc, Rc::clone(&ip), Rc::clone(&port), Rc::clone(&next))))
        }),
    );

    if *next.borrow() {
        let (operations, keys, modes, epg_id) = crate::infos::all_infos::get_all_infos();
        let _ = eframe::run_native(
            "My OrangeTV Telecommand",
            options2,
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);

                Ok(Box::new(GuiCommand::new(cc, (*ip.borrow()).clone(), (*port.borrow()).clone(), &keys)))
            }),
        );
    }
}

