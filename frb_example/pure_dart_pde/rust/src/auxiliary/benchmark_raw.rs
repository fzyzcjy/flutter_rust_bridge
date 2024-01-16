// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#![allow(non_camel_case_types)]

use crate::api::pseudo_manual::benchmark_api::benchmark_input_bytes_twin_normal;
use flutter_rust_bridge::for_generated::{new_leak_vec_ptr, vec_from_leak_ptr};

#[no_mangle]
pub extern "C" fn benchmark_raw_void_sync() {}

#[repr(C)]
#[derive(Clone)]
pub struct benchmark_raw_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[no_mangle]
pub extern "C" fn benchmark_raw_new_list_prim_u_8(len: i32) -> benchmark_raw_list_prim_u_8 {
    benchmark_raw_list_prim_u_8 {
        ptr: new_leak_vec_ptr(Default::default(), len),
        len,
    }
}

// NOTE: `Vec::from_raw_parts` says:
// "it is normally not safe to build a Vec<u8> from a pointer to a C char array with length size_t,
// doing so is only safe if the array was initially allocated by a Vec or String"
// Thus, the input vec *should* be allocated by our `new_leak_vec_ptr` only.
#[no_mangle]
pub unsafe extern "C" fn benchmark_raw_input_bytes(bytes: benchmark_raw_list_prim_u_8) -> i32 {
    let vec = vec_from_leak_ptr(bytes.ptr, bytes.len);
    benchmark_input_bytes_twin_normal(vec)
}

#[no_mangle]
#[allow(unused)]
pub extern "C" fn benchmark_raw_output_bytes(port: i64, message_id: i32, size: i32) {
    #[cfg(target_arch = "wasm32")]
    unimplemented!();

    #[cfg(not(target_arch = "wasm32"))]
    {
        use byteorder::{BigEndian, WriteBytesExt};
        use flutter_rust_bridge::for_generated::Channel;
        use flutter_rust_bridge::{IntoDart, ZeroCopyBuffer};
        use std::io::Cursor;

        let vec = {
            let mut cursor = Cursor::new(vec![0; size as usize + 4]);
            cursor.write_i32::<BigEndian>(message_id).unwrap();
            cursor.into_inner()
        };

        Channel::new(port).post(ZeroCopyBuffer(vec).into_dart());
    }
}
