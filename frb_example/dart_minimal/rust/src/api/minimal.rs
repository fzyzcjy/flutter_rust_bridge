use crate::frb_generated::StreamSink;

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hi_stream(sink: StreamSink<i32>) {
    sink.add(100);
}
