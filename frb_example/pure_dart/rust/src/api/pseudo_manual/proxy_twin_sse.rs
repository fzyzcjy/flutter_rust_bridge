// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinSse(String);

impl MyAudioParamTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn create_twin_sse(value: String) -> Self {
        Self(value)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn my_method_twin_sse(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinSse {
    param_one: MyAudioParamTwinSse,
    param_two: MyAudioParamTwinSse,
}

impl MyNodeTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn create_twin_sse() -> Self {
        Self {
            param_one: MyAudioParamTwinSse("a".to_owned()),
            param_two: MyAudioParamTwinSse("b".to_owned()),
        }
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn param_one_twin_sse(&self) -> &MyAudioParamTwinSse {
        &self.param_one
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn param_two_twin_sse(&self) -> &MyAudioParamTwinSse {
        &self.param_two
    }
}
