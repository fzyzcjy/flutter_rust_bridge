use std::thread::sleep;
use std::time::Duration;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinNormal {
    inner: i32,
}

pub fn rust_auto_opaque_return_own_twin_normal(initial: i32) -> NonCloneSimpleTwinNormal {
    NonCloneSimpleTwinNormal { inner: initial }
}

pub fn rust_auto_opaque_sleep_twin_normal(
    apple: &mut NonCloneSimpleTwinNormal,
    orange: &mut NonCloneSimpleTwinNormal,
) -> i32 {
    sleep(Duration::from_millis(1000));
    apple.inner + orange.inner
}

