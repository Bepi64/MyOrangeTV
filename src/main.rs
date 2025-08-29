use telecommand::telecommand::telecommand::Telecommand;

#[tokio::main]
async fn main() {
    let ip = "192.168.1.18";
    let port = "8080";
    let telecommand = Telecommand::new(ip, port);

    #[cfg(feature = "cli")]
    {
        telecommand.run_cli().await.expect("Failed to start CLI");
    }

    #[cfg(feature = "gui")]
    {
        use eframe::{NativeOptions, egui};
        use telecommand::telecommand::gui::gui::TelecommandBuilder;

        let mut options = NativeOptions::default();

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

        eframe::run_native(
            "MyOrangeTV",
            options,
            Box::new(|cc| Ok(Box::new(TelecommandBuilder::new(cc)))),
        );
    }
}
