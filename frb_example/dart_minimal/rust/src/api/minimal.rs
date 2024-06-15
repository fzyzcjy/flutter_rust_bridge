use flutter_rust_bridge::frb;

// #[frb(init)]
// pub fn init_app() {
//     flutter_rust_bridge::setup_default_user_utils();
// }
//
// pub fn minimal_adder(a: i32, b: i32) -> i32 {
//     a + b
// }

#[frb(opaque)]
pub struct LifetimeTesterOneTwinNormal(String);

#[frb(opaque)]
pub struct LifetimeTesterTwoTwinNormal<'a> {
    one: &'a LifetimeTesterOneTwinNormal,
}

impl LifetimeTesterOneTwinNormal {
    #[allow(clippy::needless_lifetimes)]
    pub fn compute_two<'a>(&'a self) -> LifetimeTesterTwoTwinNormal<'a> {
        LifetimeTesterTwoTwinNormal { one: self }
    }
}

// impl<'a> LifetimeTesterTwoTwinNormal<'a> {
//     pub fn greet(&self) -> String {
//         self.one.0.to_string()
//     }
// }
