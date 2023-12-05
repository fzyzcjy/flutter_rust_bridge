pub type WireSyncReturn = *mut allo_isolate::ffi::DartCObject;

pub type MessagePort = i64;

pub type DartAbi = allo_isolate::ffi::DartCObject;

pub type SendableMessagePort = MessagePort;

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePort {
    port
}

pub fn handle_to_message_port(handle: &SendableMessagePort) -> MessagePort {
    handle
}
