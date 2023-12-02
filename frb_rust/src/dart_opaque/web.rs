#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: Box<JsValue>,
    drop_port: Option<String>,
}

impl DartOpaqueBase {
    pub fn new(handle: JsValue, port: Option<JsValue>) -> Self {
        Self {
            inner: Box::new(handle),
            drop_port: port.map(|p| p.dyn_ref::<BroadcastChannel>().unwrap().name()),
        }
    }

    pub fn unwrap(self) -> JsValue {
        *self.inner
    }

    pub fn into_raw(self) -> *mut JsValue {
        Box::into_raw(self.inner)
    }

    pub fn channel(&self) -> Option<Channel> {
        Some(Channel::new(PortLike::broadcast(self.drop_port.as_ref()?)))
    }
}
