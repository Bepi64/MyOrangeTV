trait GuiTelecommand {
    fn prepare_gui(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn start_gui(&self, gui: bool) -> Result<(), Box<dyn std::error::Error>>;
    fn cleanup_gui(&self) -> Result<(), Box<dyn std::error::Error>>;
}
