use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct MyAudioParamTwinNormal(String);

impl MyAudioParamTwinNormal {
    pub fn create_twin_normal(value: String) -> Self {
        Self(value)
    }

    pub fn my_method_twin_normal(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinNormal {
    param_one: MyAudioParamTwinNormal,
    param_two: MyAudioParamTwinNormal,
}

impl MyNodeTwinNormal {
    pub fn create_twin_normal() -> Self {
        Self {
            param_one: MyAudioParamTwinNormal("a".to_owned()),
            param_two: MyAudioParamTwinNormal("b".to_owned()),
        }
    }

    #[frb(proxy)]
    pub fn param_one_twin_normal(&self) -> &MyAudioParamTwinNormal {
        &self.param_one
    }

    #[frb(proxy)]
    pub fn param_two_twin_normal(&self) -> &MyAudioParamTwinNormal {
        &self.param_two
    }
}
