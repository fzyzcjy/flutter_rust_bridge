use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
