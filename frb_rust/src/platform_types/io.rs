pub type WireSyncReturnDco = *mut allo_isolate::ffi::DartCObject;
pub type WireSyncReturnSse = WireSyncReturnSseStruct;

pub type MessagePort = i64;

pub type DartAbi = allo_isolate::ffi::DartCObject;

pub type SendableMessagePortHandle = MessagePort;

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    *port
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    *handle
}

#[repr(C)]
pub struct WireSyncReturnSseStruct {
    pub ptr: *const u8,
    pub len: i32,
}
