pub mod global_state{
    use std::sync::RwLock;
    use state::InitCell;

    pub struct GlobalState{
        pub listen: bool,
        pub run: bool
    }

    pub static GLOBAL_STATE: InitCell<RwLock<GlobalState>> = InitCell::new();
    pub fn init_global_state() {
            let gstate = GlobalState{
                listen : false,
                run : true
            };

        GLOBAL_STATE.set(RwLock::new(gstate));
    }
}

