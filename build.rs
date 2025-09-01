extern crate embed_resource;

fn main() {
    // Vérifiez si l'OS est Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        //icone de le l'éxécuteur
        embed_resource::compile("resources/app_icon.rc", embed_resource::NONE)
            .manifest_optional()
            .unwrap();
        // Vérifiez si la fonctionnalité "gui" est activée
        if std::env::var("CARGO_FEATURE_GUI").is_ok() {
            println!("cargo:rustc-link-arg=-Wl,--subsystem,windows"); // Pas de console pour GUI
            println!("cargo:rustc-link-arg=-Wl,--entry=mainCRTStartup");
        } else {
            // Par défaut, ou si "cli" est activé
            println!("cargo:rustc-link-arg=-Wl,--subsystem,console"); // Console ouverte pour CLI
        }
    }
}
