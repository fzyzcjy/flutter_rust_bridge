use flutter_rust_bridge::frb;
use std::net::Ipv4Addr;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(rust2dart(dart_type = "InternetAddress", dart_code = "InternetAddress({})"))]
pub fn encode_ipv4_type(raw: Ipv4Addr) -> String {
    return raw.to_string();
}

#[frb(dart2rust(dart_type = "InternetAddress", dart_code = "{}.address"))]
pub fn decode_ipv4_type(raw: String) -> Ipv4Addr {
    raw.parse().unwrap()
}

#[derive(Debug)]
pub struct StructWithIpv4AddrField {
    pub ip: Ipv4Addr,
}

impl StructWithIpv4AddrField {
    pub fn f(&self) {}
}
