// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

pub use std::net::Ipv4Addr;

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

#[frb(rust2dart(dart_type = "InternetAddress", dart_code = "InternetAddress({})"))]
pub fn serializer_ipv4_addr(raw: Ipv4Addr) -> String {
    return raw.to_string();
}

#[frb(dart2rust(dart_type = "InternetAddress", dart_code = "{}.address"))]
pub fn deserializer_ipv4_addr(raw: String) -> Ipv4Addr {
    raw.parse().unwrap()
}

#[derive(Debug)]
#[frb(non_opaque)]
pub struct NonOpaqueStructContainingIpv4Addr {
    pub inner: Ipv4Addr,
}

pub fn func_using_ipv4_addr(arg: Ipv4Addr) -> Ipv4Addr {
    arg
}

pub fn func_using_non_opaque_struct_containing_ipv4_addr(
    arg: NonOpaqueStructContainingIpv4Addr,
) -> NonOpaqueStructContainingIpv4Addr {
    arg
}
