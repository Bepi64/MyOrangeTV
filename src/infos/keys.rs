use std::collections::HashMap;

pub fn get_map_keys() -> HashMap<&'static str, u16>
{
    let keys = HashMap::from ([
        ("on", 116),
        ("0", 512),
        ("1", 513),
        ("2", 514),
        ("3", 515),
        ("4", 516),
        ("5", 517),
        ("6", 518),
        ("7", 519),
        ("8", 520),
        ("9", 521),
        ("ch+", 402),
        ("ch-", 403),
        ("vol+", 115),
        ("vol-", 114),
        ("mute", 113),
        ("up", 103),
        ("down", 108),
        ("left", 105),
        ("right", 106),
        ("ok", 352),
        ("back", 158),
        ("menu", 139),
        ("play", 164),
        ("rec", 167),
        ]);
    keys
}