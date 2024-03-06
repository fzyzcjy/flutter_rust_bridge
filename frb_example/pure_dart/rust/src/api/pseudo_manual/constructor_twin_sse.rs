// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `constructor.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

pub struct ConstructorTranslatableStructTwinSse {
    pub one: String,
}

impl ConstructorTranslatableStructTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }
}

#[frb(opaque)]
pub struct ConstructorOpaqueStructTwinSse {
    pub one: String,
}

impl ConstructorOpaqueStructTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }

    #[frb(sync)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn check_twin_sse(&self) {
        assert_eq!(self.one, "hello");
    }
}

pub struct ConstructorTranslatableSyncStructTwinSse {
    pub one: String,
}

impl ConstructorTranslatableSyncStructTwinSse {
    #[frb(sync)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }
}

#[frb(opaque)]
pub struct ConstructorOpaqueSyncStructTwinSse {
    pub one: String,
}

impl ConstructorOpaqueSyncStructTwinSse {
    #[frb(sync)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse() -> Self {
        Self {
            one: "hello".to_owned(),
        }
    }

    #[frb(sync)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn check_twin_sse(&self) {
        assert_eq!(self.one, "hello");
    }
}
