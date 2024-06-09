// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinRustAsyncSse(String);

impl MyAudioParamTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn create_twin_rust_async_sse(value: String) -> Self {
        Self(value)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn my_method_twin_rust_async_sse(&self) -> String {
        self.0.repeat(2)
    }
}

#[frb(opaque)]
pub struct MyNodeTwinRustAsyncSse {
    param_one: MyAudioParamTwinRustAsyncSse,
    param_two: MyAudioParamTwinRustAsyncSse,
}

impl MyNodeTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn create_twin_rust_async_sse() -> Self {
        Self {
            param_one: MyAudioParamTwinRustAsyncSse("a".to_owned()),
            param_two: MyAudioParamTwinRustAsyncSse("b".to_owned()),
        }
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn param_one_twin_rust_async_sse(&self) -> &MyAudioParamTwinRustAsyncSse {
        &self.param_one
    }

    #[frb(proxy)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn param_two_twin_rust_async_sse(&self) -> &MyAudioParamTwinRustAsyncSse {
        &self.param_two
    }
}
