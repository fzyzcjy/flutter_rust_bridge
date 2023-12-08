pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// TODO only temp add this for debug, should remove later (o/w this example is no longer "minimal"
#[flutter_rust_bridge::frb(serialize)]
pub fn hello(a: i32, b: i32) -> i32 {
    a + b
}
