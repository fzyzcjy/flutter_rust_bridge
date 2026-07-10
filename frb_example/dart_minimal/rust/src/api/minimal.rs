use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub mod first_store {
    pub struct Model {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct Model {
        pub value: String,
    }
}

pub fn duplicate_named_models() -> Vec<second_store::Model> {
    vec![second_store::Model {
        value: "second".to_owned(),
    }]
}
