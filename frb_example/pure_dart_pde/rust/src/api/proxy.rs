// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MyAudioParamTwinNormal(String);

impl MyAudioParamTwinNormal {
    #[frb(serialize)]
    pub fn create_twin_normal(value: String) -> Self {
        Self(value)
    }

    #[frb(serialize)]
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
    #[frb(serialize)]
    pub fn create_twin_normal() -> Self {
        Self {
            param_one: MyAudioParamTwinNormal("a".to_owned()),
            param_two: MyAudioParamTwinNormal("b".to_owned()),
        }
    }

    #[frb(proxy)]
    #[frb(serialize)]
    pub fn param_one_twin_normal(&self) -> &MyAudioParamTwinNormal {
        &self.param_one
    }

    #[frb(proxy)]
    #[frb(serialize)]
    pub fn param_two_twin_normal(&self) -> &MyAudioParamTwinNormal {
        &self.param_two
    }
}
