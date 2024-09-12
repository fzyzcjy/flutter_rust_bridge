use flutter_rust_bridge::frb;
pub use frb_example_pure_dart_example_external_lib::SimpleTranslatableExternalStructWithMethod;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(rust2dart(dart_type = "String", dart_code = "{}"))]
#[frb(serialize)]
pub fn serializer_my_external_type(raw: SimpleTranslatableExternalStructWithMethod) -> String {
    unimplemented!()
}

#[frb(dart2rust(dart_type = "String", dart_code = "{}"))]
#[frb(serialize)]
pub fn deserializer_my_external_type(raw: String) -> SimpleTranslatableExternalStructWithMethod {
    unimplemented!()
}

pub fn f(a: SimpleTranslatableExternalStructWithMethod) -> SimpleTranslatableExternalStructWithMethod {
    a
}
