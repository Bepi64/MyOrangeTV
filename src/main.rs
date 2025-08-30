#[tokio::main]
async fn main() {
    #[cfg(feature = "cli")]
    {
        use telecommand::telecommand::telecommand::Telecommand;
        let ip = String::from("192.168.1.18");
        let port = String::from("8080");
        let telecommand = Telecommand::new(ip, port);

        telecommand.run_cli().await.expect("Failed to start CLI");
    }

    #[cfg(feature = "gui")]
    {
        telecommand::telecommand::gui::gui::start_gui();
    }
}
