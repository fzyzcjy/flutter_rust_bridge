// NOTE: Multi-file is supported (e.g. put in submodules of crate::app, or configure frb input mod)

use flutter_rust_bridge::frb;

#[frb(ui_state)]
pub struct RustState {
    pub count: i32,
}

impl RustState {
    pub fn new() -> Self {
        Self {
            count: 100,
            base_state: Default::default(),
        }
    }

    #[frb(ui_mutation)]
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
