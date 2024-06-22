#[frb(ui_state)]
pub struct RustState {
    pub count: i32,
}

impl RustState {
    pub fn new() -> Self {
        Self { count: 100 }
    }

    #[frb(ui_mutation)]
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
