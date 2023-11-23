use flutter_rust_bridge::{spawn, StreamSink};

pub struct ConcatenateWith {
    pub a: String,
}

impl ConcatenateWith {
    pub fn new(a: String) -> ConcatenateWith {
        ConcatenateWith { a }
    }

    pub fn concatenate(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    pub fn concatenate_static(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub fn handle_some_stream_sink(&self, key: u32, max: u32, sink: StreamSink<Log2>) {
        let a = self.a.clone();
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: format!("{a}{i}"),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_stream_sink_at_1(&self, sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink(key: u32, max: u32, sink: StreamSink<Log2>) {
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink_single_arg(sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }
}

#[derive(Debug, Clone)]
pub struct Log2 {
    pub key: u32,
    pub value: String,
}

pub struct SomeStruct {
    pub value: u32,
}

impl SomeStruct {
    pub fn new(value: u32) -> SomeStruct {
        SomeStruct { value }
    }
    pub fn static_return_err_custom_error() -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub fn static_return_ok_custom_error() -> Result<u32, CustomError> {
        Ok(3)
    }

    pub fn non_static_return_err_custom_error(&self) -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub fn non_static_return_ok_custom_error(&self) -> Result<u32, CustomError> {
        Ok(self.value)
    }
}

pub struct CustomStruct {
    pub message: String,
}

impl CustomStruct {
    pub fn new(message: String) -> CustomStruct {
        CustomStruct { message }
    }

    pub fn static_return_custom_struct_error() -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: "error message".to_string(),
        })
    }

    pub fn static_return_custom_struct_ok() -> Result<u32, CustomStructError> {
        Ok(3)
    }

    pub fn nonstatic_return_custom_struct_error(&self) -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: self.message.clone(),
        })
    }

    pub fn nonstatic_return_custom_struct_ok(&self) -> Result<u32, CustomStructError> {
        Ok(3)
    }
}
