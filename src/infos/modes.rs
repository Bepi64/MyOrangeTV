use std::collections::HashMap;

pub fn get_map_modes() -> HashMap<&'static str, u16> {
    let modes = HashMap::from([
        ("short", 0),   // appui court
        ("long", 1),    // appui prolongé
        ("release", 2), // relâchement après appui
    ]);
    modes
}
