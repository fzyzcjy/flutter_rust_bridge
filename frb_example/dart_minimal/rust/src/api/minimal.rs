use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO temp demo
#[frb(opaque)]
pub struct MyStruct {}
impl MyStruct {
    pub fn f(&mut self, a: i32) -> i32 {
        a
    }

    pub fn my_static_method() -> i32 {
        42
    }

    #[frb(sync)]
    pub fn new() -> Self {
        Self {}
    }
}

pub enum HiEnum {
    A,
    B,
}
pub fn func_using_enum() -> HiEnum {
    HiEnum::B
}
