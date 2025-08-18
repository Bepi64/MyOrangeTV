use std::collections::HashMap;
use crate::infos;

pub fn get_all_infos() -> (HashMap<&'static str, u16>, HashMap<&'static str, u16>
    , HashMap<&'static str, u16>, HashMap<&'static str, u16>)
{

    let operations = infos::operations::get_map_operations();
    let keys = infos::keys::get_map_keys();
    let modes = infos::modes::get_map_modes();
    let epgs = infos::epg_ids::get_map_epgs();

    (operations, keys, modes, epgs)
}
