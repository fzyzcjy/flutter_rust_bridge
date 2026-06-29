pub struct NativeOnlyDto {
    values: Vec<i64>,
    maybe_value: Option<i64>,
}

#[flutter_rust_bridge_macros::frb(semi_serialize)]
pub fn accept_native_only_dto(arg: NativeOnlyDto) {
    drop(arg);
}
