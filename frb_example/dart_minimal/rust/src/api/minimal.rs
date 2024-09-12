use flutter_rust_bridge::frb;
pub use std::net::Ipv4Addr;

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

// TODO temp
// pub fn func_using_ipv4_addr(arg: Ipv4Addr) -> Ipv4Addr {
//     arg
// }

pub fn func_using_non_opaque_struct_containing_ipv4_addr(
    arg: NonOpaqueStructContainingIpv4Addr,
) -> NonOpaqueStructContainingIpv4Addr {
    arg
}
