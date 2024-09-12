use std::net::Ipv4Addr;

use flutter_rust_bridge::frb;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: Device) -> String {
    format!("Hello, {}!", name.ip.to_string())
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
pub struct Device {
    pub ip: Ipv4Addr,
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}