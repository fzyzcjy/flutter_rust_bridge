pub type WireSyncRust2DartDco = *mut allo_isolate::ffi::DartCObject;

#[repr(C)]
pub struct WireSyncRust2DartSse {
    pub ptr: *mut u8,
    pub len: i32,
}

pub type MessagePort = i64;

pub type DartAbi = allo_isolate::ffi::DartCObject;

pub type SendableMessagePortHandle = MessagePort;

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    *port
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    *handle
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    raw.parse().unwrap()
}

pub type PlatformGeneralizedUint8ListPtr = *mut u8;
