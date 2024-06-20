use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

use flutter_rust_bridge::for_generated::anyhow;

#[frb(opaque)]
pub struct MyStructWithTryFromTwinNormal(String);

// #2103
impl TryFrom<String> for MyStructWithTryFromTwinNormal {
    type Error = anyhow::Error;

    #[frb(sync)]
    fn try_from(value: String) -> anyhow::Result<Self> {
        Ok(Self(value))
    }
}
