use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(rust2dart(dart_type = "String", dart_code = "{}"))]
#[frb(serialize)]
pub fn serializer_my_external_type(raw: flutter_rust_bridge::CatchUnwindWithBacktrace) -> String {
    unimplemented!()
}

#[frb(dart2rust(dart_type = "String", dart_code = "{}"))]
#[frb(serialize)]
pub fn deserializer_my_external_type(raw: String) -> flutter_rust_bridge::CatchUnwindWithBacktrace {
    unimplemented!()
}

pub fn f(
    a: flutter_rust_bridge::CatchUnwindWithBacktrace,
) -> flutter_rust_bridge::CatchUnwindWithBacktrace {
    a
}
