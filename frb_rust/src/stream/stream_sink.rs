use std::marker::PhantomData;
use crate::platform_types::MessagePort;

pub(crate) struct StreamSink<T> {
    port: MessagePort,
    _phantom: PhantomData<T>,
}

impl<T> StreamSink<T> {
    pub fn deserialize() -> Self {
        todo!()
    }
}
