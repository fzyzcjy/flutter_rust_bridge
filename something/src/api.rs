use flutter_rust_bridge::StreamSink;

pub fn tick(a: u32, b: u32, sink: StreamSink<i32>) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn tick_2(a: u32, sink: StreamSink<i32>, b: u32) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn tick_3(sink: StreamSink<i32>, a: u32, b: u32) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn tick_4(sink: StreamSink<i32>) -> Result<(), anyhow::Error> {
    todo!()
}