use std::sync::RwLock;
use std::collections::HashMap;
use enigo::Key;
use state::InitCell;

use crate::key_funcs;

pub struct GlobalState{
    pub listen: bool,
    pub run: bool,
    pub binds: HashMap<String, Key>,
    pub mod_keys: Vec<Key>,
}

pub static GLOBAL_STATE: InitCell<RwLock<GlobalState>> = InitCell::new();
pub fn init_global_state() {
    let gstate = GlobalState{
        listen : false,
        run : true,
        binds: HashMap::new(),
        mod_keys: Vec::new(),
    };

    GLOBAL_STATE.set(RwLock::new(gstate));
    key_funcs::init_binds();
}
