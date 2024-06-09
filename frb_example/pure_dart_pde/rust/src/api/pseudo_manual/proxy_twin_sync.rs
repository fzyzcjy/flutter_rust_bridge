// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinSync(String);

impl MyAudioParamTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync(value: String) -> Self {
        Self(value)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn my_method_twin_sync(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinSync {
    param_one: MyAudioParamTwinSync,
    param_two: MyAudioParamTwinSync,
}

impl MyNodeTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync() -> Self {
        Self {
            param_one: MyAudioParamTwinSync("a".to_owned()),
            param_two: MyAudioParamTwinSync("b".to_owned()),
        }
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn param_one_twin_sync(&self) -> &MyAudioParamTwinSync {
        &self.param_one
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn param_two_twin_sync(&self) -> &MyAudioParamTwinSync {
        &self.param_two
    }
}
