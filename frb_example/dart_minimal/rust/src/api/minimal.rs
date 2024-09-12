pub use std::net::Ipv4Addr;
use flutter_rust_bridge::frb;

#[frb(rust2dart(dart_type = "InternetAddress", dart_code = "InternetAddress({})"))]
pub fn serializer_ipv4_addr(raw: Ipv4Addr) -> String {
    return raw.to_string();
}

#[frb(dart2rust(dart_type = "InternetAddress", dart_code = "{}.address"))]
pub fn deserializer_ipv4_addr(raw: String) -> Ipv4Addr {
    raw.parse().unwrap()
}

pub fn func_using_ipv4_addr(arg: Ipv4Addr) -> Ipv4Addr {
    arg
}

#[derive(Debug)]
#[frb(non_opaque)]
pub struct NonOpaqueStructContainingIpv4Addr {
    pub inner: Ipv4Addr,
}
