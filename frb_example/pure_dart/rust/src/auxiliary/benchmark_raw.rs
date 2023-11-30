use crate::api::benchmark_api::{
    benchmark_input_bytes_twin_normal, benchmark_output_bytes_twin_normal,
};
use crate::frb_generated::wire_list_prim_u_8;

#[no_mangle]
pub extern "C" fn benchmark_raw_void_sync() {}

#[no_mangle]
pub unsafe extern "C" fn benchmark_raw_input_bytes_sync(ptr: *mut u8, len: i32) -> i32 {
    // TODO wrong
    // "it is normally not safe to build a Vec<u8> from a pointer to a C char array with length size_t,
    // doing so is only safe if the array was initially allocated by a Vec or String"
    let vec = Vec::from_raw_parts(ptr, len as usize, len as usize);
    benchmark_input_bytes_twin_normal(vec)
}

#[no_mangle]
pub extern "C" fn benchmark_raw_output_bytes_sync(size: i32) -> wire_list_prim_u_8 {
    let vec = benchmark_output_bytes_twin_normal(size);
    todo!()
}
