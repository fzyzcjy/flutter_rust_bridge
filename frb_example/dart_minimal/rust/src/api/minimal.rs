use flutter_rust_bridge::frb;
use crate::frb_generated::RustAutoOpaqueMoi;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct OpaqueItemTwinNormal(i32);

#[frb(opaque)]
pub struct ItemContainerSolutionOneTwinNormal {
    pub name: String,
    items: Vec<OpaqueItemTwinNormal>,
}

impl ItemContainerSolutionOneTwinNormal {
    pub fn create_twin_normal() -> Self {
        Self {
            name: "hi".to_owned(),
            items: vec![OpaqueItemTwinNormal(100)],
        }
    }

    pub fn get_item_contents_twin_normal(&self) -> Vec<i32> {
        self.items.iter().map(|x| x.0).collect()
    }
}
