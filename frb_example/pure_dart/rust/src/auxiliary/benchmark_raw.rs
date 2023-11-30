#[no_mangle]
pub extern "C" fn benchmark_raw_void_sync() {}

#[no_mangle]
pub extern "C" fn benchmark_raw_input_bytes_sync(ptr: *mut u8, len: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn benchmark_raw_output_bytes_sync() -> TODO {
    todo!()
}
