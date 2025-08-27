use crate::telecommand::telecommand::Telecommand;

trait GuiTelecommand {
    fn prepare_gui(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn run_gui(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn cleanup_gui(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl GuiTelecommand for Telecommand {
    fn prepare_gui(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Prepare GUI for telecommand");
    }

    fn run_gui(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("Run GUI for telecommand");
    }

    fn cleanup_gui(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Cleanup GUI for telecommand");
    }
}

impl Telecommand {
    pub fn start_gui(&self) {
        if let Err(e) = self.run_gui() {
            eprintln!("Failed to start GUI telecommand: {}", e);
        }
    }
}
