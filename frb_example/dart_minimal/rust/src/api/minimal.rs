use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct MessageWithCustomSerializerTwinNormal(i32);

#[frb(rust2dart(dart_type = "int", dart_code = "int.parse({})"))]
pub fn serializer_my_type(raw: MessageWithCustomSerializerTwinNormal) -> String {
    raw.0.to_string()
}

#[frb(dart2rust(dart_type = "int", dart_code = "{}.toString()"))]
pub fn deserializer_my_type(raw: String) -> MessageWithCustomSerializerTwinNormal {
    MessageWithCustomSerializerTwinNormal(raw.parse().unwrap())
}

pub fn function_using_type_with_custom_serializer(
    arg: MessageWithCustomSerializerTwinNormal,
) -> MessageWithCustomSerializerTwinNormal {
    arg
}
