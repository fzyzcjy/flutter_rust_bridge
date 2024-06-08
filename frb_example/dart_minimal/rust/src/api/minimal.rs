use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct MyAudioParam(String);

impl MyAudioParam {
    #[frb(sync)]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn my_method(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNode {
    param_one: MyAudioParam,
}

impl MyNode {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            param_one: MyAudioParam::new("a".to_owned()),
        }
    }

    #[frb(proxy)]
    pub fn param_one(&self) -> &MyAudioParam {
        &self.param_one
    }
}
