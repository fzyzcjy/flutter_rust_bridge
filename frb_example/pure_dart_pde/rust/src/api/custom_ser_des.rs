// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

#[frb(opaque)]
pub struct MessageWithCustomSerializerTwinNormal(i32);

#[frb(rust2dart(dart_type = "int", dart_code = "int.parse({})"))]
#[frb(serialize)]
pub fn serializer_my_type(raw: MessageWithCustomSerializerTwinNormal) -> String {
    raw.0.to_string()
}

#[frb(dart2rust(dart_type = "int", dart_code = "{}.toString()"))]
#[frb(serialize)]
pub fn deserializer_my_type(raw: String) -> MessageWithCustomSerializerTwinNormal {
    MessageWithCustomSerializerTwinNormal(raw.parse().unwrap())
}

#[frb(serialize)]
pub fn function_using_type_with_custom_serializer(
    arg: MessageWithCustomSerializerTwinNormal,
) -> MessageWithCustomSerializerTwinNormal {
    arg
}
