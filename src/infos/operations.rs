use std::collections::HashMap;

pub fn get_map_operations() -> HashMap<&'static str, u16> {
    let operations = HashMap::from([
        ("send", 1),   // operation=01 : envoi touche télécommande
        ("zap", 9),    // operation=09 : zap direct par epg_id
        ("state", 10), // operation=10 : événement reprise
    ]);
    operations
}
