use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/2170
pub trait Issue2170Trait {
    fn method(&self);
}

pub struct Issue2170Struct {
    pub field: Box<dyn Issue2170Trait>,
}

impl std::fmt::Debug for Issue2170Struct {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
