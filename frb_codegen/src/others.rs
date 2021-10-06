use log::warn;

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
        &format!(
            "class {} implements DartRustBridgeWireBase {{",
            dart_wire_class_name
        ),
    )
}

pub fn extract_dart_wire_content(content: &str) -> (String, String) {
    let (mut imports, mut body) = (Vec::new(), Vec::new());
    for line in content.split('\n') {
        (if line.starts_with("import ") {
            &mut imports
        } else {
            &mut body
        })
        .push(line);
    }
    (imports.join("\n"), body.join("\n"))
}

pub fn sanity_check(generated_dart_wire_code: &str, dart_wire_class_name: &str) {
    if !generated_dart_wire_code.contains(dart_wire_class_name) {
        warn!(
            "Nothing is generated for dart wire class. \
        Maybe you forget to put code like `mod the_generated_code;` to your `lib.rs` or `main.rs`?"
        );
    }
}
