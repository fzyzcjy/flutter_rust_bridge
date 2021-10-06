use std::{env, fs};
use std::path::{Path, PathBuf};

use log::warn;

use crate::config::*;

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
pub const DUMMY_WIRE_CODE_FOR_BINDGEN: &str = r#"
    // ----------- DUMMY CODE FOR BINDGEN ----------
    
    // copied from: allo-isolate
    pub type DartPort = i64;
    pub type DartPostCObjectFnType = unsafe extern "C" fn(port_id: DartPort, message: *mut std::ffi::c_void) -> bool;
    #[no_mangle] pub unsafe extern "C" fn store_dart_post_cobject(ptr: DartPostCObjectFnType) { panic!("dummy code") }
    
    // ---------------------------------------------
    "#;

pub fn modify_dart_wire_content(content_raw: &str, dart_wire_class_name: &str) -> String {
    content_raw.replace(
        &format!("class {} {{", dart_wire_class_name),
        &format!("class {} implements DartRustBridgeWireBase {{", dart_wire_class_name),
    )
}

pub fn extract_dart_wire_content(content: &str) -> (String, String) {
    let (mut imports, mut body) = (Vec::new(), Vec::new());
    for line in content.split("\n") {
        (if line.starts_with("import ") { &mut imports } else { &mut body }).push(line);
    }
    (imports.join("\n"), body.join("\n"))
}

pub fn sanity_check(generated_dart_wire_code: &str, dart_wire_class_name: &str) {
    if !generated_dart_wire_code.contains(dart_wire_class_name) {
        warn!("Nothing is generated for dart wire class. \
        Maybe you forget to put code like `mod the_generated_code;` to your `lib.rs` or `main.rs`?");
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
                input_path: canon_dir(&self.rust.input_path),
                output_path: canon_dir(&self.rust.output_path),
            },
            dart: ConfigDart {
                output_path: canon_dir(&self.dart.output_path),
                output_class_name: self.dart.output_class_name,
                format_line_length: self.dart.format_line_length,
            },
            c: ConfigC {
                output_path: canon_dir(&self.c.output_path),
            },
        }
    }
}

impl ConfigRust {}

impl ConfigDart {
    pub fn api_class_name(&self) -> String {
        self.output_class_name.clone()
    }

    pub fn api_impl_class_name(&self) -> String {
        format!("_{}Impl", self.output_class_name)
    }

    pub fn wire_class_name(&self) -> String {
        format!("{}Wire", self.output_class_name)
    }
}