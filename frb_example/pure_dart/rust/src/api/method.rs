use backtrace::Backtrace;
use flutter_rust_bridge::handler::Error::CustomError;
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

pub struct SumWith {
    pub x: u32,
}

impl SumWith {
    pub fn sum(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub fn get_sum_struct() -> SumWith {
    SumWith { x: 21 }
}

pub fn get_sum_array(a: u32, b: u32, c: u32) -> [SumWith; 3] {
    [SumWith { x: a }, SumWith { x: b }, SumWith { x: c }]
}
