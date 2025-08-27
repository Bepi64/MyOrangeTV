use telecommand::telecommand::telecommand::Telecommand;


#[tokio::main]
async fn main() {
    let ip = "192.168.1.18";
    let port = "8080";
    let telecommand = Telecommand::new(ip, port);

    #[cfg(feature = "cli")]
    {
        telecommand
            .run_cli()
            .await
            .expect("Failed to start CLI");
    }

    #[cfg(feature = "gui")]
    {
        telecommand
            .start_gui()
            .expect("Failed to start GUI");
    }
}
