// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinSyncSse(String);

impl MyAudioParamTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync_sse(value: String) -> Self {
        Self(value)
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn my_method_twin_sync_sse(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinSyncSse {
    param_one: MyAudioParamTwinSyncSse,
    param_two: MyAudioParamTwinSyncSse,
}

impl MyNodeTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync_sse() -> Self {
        Self {
            param_one: MyAudioParamTwinSyncSse("a".to_owned()),
            param_two: MyAudioParamTwinSyncSse("b".to_owned()),
        }
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn param_one_twin_sync_sse(&self) -> &MyAudioParamTwinSyncSse {
        &self.param_one
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn param_two_twin_sync_sse(&self) -> &MyAudioParamTwinSyncSse {
        &self.param_two
    }
}
