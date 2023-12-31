/// Cast a byte buffer into a boxed slice of the target type without making any copies.
/// Panics if the cast fails.
#[cfg(wasm)]
pub fn slice_from_byte_buffer<T: bytemuck::Pod>(buffer: Vec<u8>) -> Box<[T]> {
    let buf = Box::leak(buffer.into_boxed_slice());
    match bytemuck::try_cast_slice_mut(buf) {
        Ok(buf) => unsafe { Box::from_raw(buf) },
        Err(err) => {
            // clean up before panicking
            unsafe { core::ptr::drop_in_place(buf) }
            panic!("cast error: {}", err);
        }
    }
}
