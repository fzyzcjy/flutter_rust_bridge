use crate::frb_generated::StreamSink;
use flutter_rust_bridge::for_generated::SseCodec;

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hi_stream_one(sink: StreamSink<i32>) {
    sink.add(100);
}

pub fn hi_stream_two(sink: StreamSink<i32, SseCodec>) {
    sink.add(100);
}
