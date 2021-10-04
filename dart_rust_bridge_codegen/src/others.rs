use crate::config::Config;
use std::{env, fs};

pub fn parse_command_line_args() -> String {
    let mut args = env::args();
    let _ = args.next().unwrap(); // executable name
    let config_path = args.next().expect(
        "Please provide command line argument which is the path to your configuration file",
    );
    println!("Use config file at: {}", config_path);
    config_path
}

// NOTE [DartPostCObjectFnType] was originally [*mut DartCObject] but I changed it to [*mut c_void]
// because cannot automatically generate things related to [DartCObject]. Anyway this works fine.
pub const DUMMY_WIRE_CODE_FOR_BINDGEN: &'static str = r#"
    // ----------- DUMMY CODE FOR BINDGEN ----------
    
    // copied from: allo-isolate
    pub type DartPort = i64;
    pub type DartPostCObjectFnType = unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool;
    #[no_mangle] pub unsafe extern "C" fn store_dart_post_cobject(ptr: DartPostCObjectFnType) { panic!("dummy code") }
    
    // ---------------------------------------------
    "#;

pub fn modify_dart_wire_content(dart_wire_path: &str, dart_wire_class_name: &str) {
    let content_raw = fs::read_to_string(dart_wire_path).unwrap();
    let content_modified = content_raw.replace(
        &format!("class {} {{", dart_wire_class_name),
        &format!(
            "import 'package:dart_rust_bridge/dart_rust_bridge.dart';
            class {} implements DartRustBridgeWireBase {{",
            dart_wire_class_name
        ),
    );
    fs::write(dart_wire_path, content_modified).unwrap();
}

pub fn sanity_check(config: &Config) {
    if !fs::read_to_string(&config.dart.wire_path)
        .unwrap()
        .contains(&config.dart.wire_class_name)
    {
        panic!("Nothing is generated for dart wire class. \
        Maybe you forget to put code like `mod generated_wire;` to your `lib.rs` or `main.rs`? (file path: {})", &config.dart.wire_path);
    }
}
