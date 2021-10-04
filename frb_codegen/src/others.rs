use crate::config::*;
use std::{env, fs};
use std::path::{Path, PathBuf};

pub fn parse_command_line_args() -> String {
    let mut args = env::args();
    let _ = args.next().unwrap(); // executable name
    let config_path = args.next().expect(
        "Invalid usage of this command. Please provide path to config.yaml as your argument. \
        Example: `/path/to/this/binary /path/to/your/flutter_rust_bridge.yaml`",
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
            "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
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

impl Config {
    pub fn read(config_path: &str) -> Self {
        let raw_config: Config =
            serde_yaml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
        raw_config.canonicalize(PathBuf::from(config_path).parent().unwrap())
    }

    pub fn canonicalize(self, base_dir: &Path) -> Config {
        let canon_dir = |sub_path: &str| {
            let mut path = PathBuf::from(base_dir);
            path.push(sub_path);
            path.into_os_string().into_string().unwrap()
        };

        Config {
            rust: ConfigRust {
                crate_dir: canon_dir(&self.rust.crate_dir),
                api_path: canon_dir(&self.rust.api_path),
                wire_path: canon_dir(&self.rust.wire_path),
            },
            dart: ConfigDart {
                api_class_name: self.dart.api_class_name,
                wire_class_name: self.dart.wire_class_name,
                api_path: canon_dir(&self.dart.api_path),
                wire_path: canon_dir(&self.dart.wire_path),
                format_line_length: self.dart.format_line_length,
            },
            c: ConfigC {
                wire_path: canon_dir(&self.c.wire_path),
            },
        }
    }
}

