use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct OpaqueItem(i32);

pub struct ItemContainerSolutionTwo {
    pub name: String,
    pub items: Vec<RustAutoOpaque<OpaqueItem>>,
}

impl ItemContainerSolutionTwo {
    pub fn create() -> Self {
        Self {
            name: "hi".to_owned(),
            items: vec![RustAutoOpaque::new(OpaqueItem(100))],
        }
    }

    pub fn get_item_contents(&self) -> Vec<i32> {
        self.items.iter().map(|x| x.blocking_read().0).collect()
    }
}
