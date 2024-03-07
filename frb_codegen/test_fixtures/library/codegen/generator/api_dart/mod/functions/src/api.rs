#[flutter_rust_bridge_macros::frb]
pub fn fn_with_default_arg(#[frb(default = 1)] foo: i32) {
    drop(foo);
}
