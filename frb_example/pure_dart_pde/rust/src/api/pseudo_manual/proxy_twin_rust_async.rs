// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinRustAsync(String);

impl MyAudioParamTwinRustAsync {
    pub async fn create_twin_rust_async(value: String) -> Self {
        Self(value)
    }

    pub async fn my_method_twin_rust_async(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinRustAsync {
    param_one: MyAudioParamTwinRustAsync,
    param_two: MyAudioParamTwinRustAsync,
}

impl MyNodeTwinRustAsync {
    pub async fn create_twin_rust_async() -> Self {
        Self {
            param_one: MyAudioParamTwinRustAsync("a".to_owned()),
            param_two: MyAudioParamTwinRustAsync("b".to_owned()),
        }
    }

    #[frb(proxy)]
    pub async fn param_one_twin_rust_async(&self) -> &MyAudioParamTwinRustAsync {
        &self.param_one
    }

    #[frb(proxy)]
    pub async fn param_two_twin_rust_async(&self) -> &MyAudioParamTwinRustAsync {
        &self.param_two
    }
}
