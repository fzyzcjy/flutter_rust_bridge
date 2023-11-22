use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn simple_adder_sync(a: i32, b: i32) -> i32 {
    a + b
}
