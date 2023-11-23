use flutter_rust_bridge::{DartAbi, IntoDart};

pub fn return_dart_dynamic() -> DartAbi {
    vec!["foo".into_dart()].into_dart()
}
