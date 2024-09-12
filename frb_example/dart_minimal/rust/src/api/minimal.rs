pub use std::net::Ipv4Addr;
use flutter_rust_bridge::frb;

#[derive(Debug)]
#[frb(opaque)]
pub struct MyIpv4Addr(Ipv4Addr);

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: Device) -> String {
    format!("Hello, {}!", name.ip.to_string())
}

#[frb(rust2dart(dart_type = "InternetAddress", dart_code = "InternetAddress({})"))]
pub fn encode_ipv4_type(raw: MyIpv4Addr) -> String {
    return raw.to_string();
}

#[frb(dart2rust(dart_type = "InternetAddress", dart_code = "{}.address"))]
pub fn decode_ipv4_type(raw: String) -> MyIpv4Addr {
    raw.parse().unwrap()
}

#[derive(Debug)]
#[frb(non_opaque)]
pub struct Device {
    pub ip: MyIpv4Addr,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

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
