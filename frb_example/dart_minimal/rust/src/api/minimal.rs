use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

use std::fmt;
use std::fmt::Debug;

pub trait Progress {
    fn update(&self);
}

pub struct ProgressHolder {
    pub progress: Box<dyn Progress>,
}

impl Debug for ProgressHolder {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
