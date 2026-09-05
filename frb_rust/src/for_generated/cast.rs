/// Cast a byte buffer into a boxed slice of the target type without making any copies.
/// Panics if the cast fails.
#[cfg(target_family = "wasm")]
pub fn slice_from_byte_buffer<T: bytemuck::Pod>(buffer: Vec<u8>) -> Box<[T]> {
    let buf = Box::leak(buffer.into_boxed_slice());
    match bytemuck::try_cast_slice_mut(buf) {
        Ok(buf) => unsafe { Box::from_raw(buf) },
        Err(err) => {
            // clean up before panicking
            unsafe { core::ptr::drop_in_place(buf) }
            panic!("cast error: {err}");
        }
    }
}

#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub trait JsTypedArray {
    type Item: bytemuck::Pod;

    fn buffer(&self) -> js_sys::ArrayBuffer;

    fn byte_offset(&self) -> u32;

    fn byte_length(&self) -> u32;

    fn length(&self) -> u32;
}

#[cfg(target_family = "wasm")]
macro_rules! impl_js_typed_array {
    ($ty:ty, $item:ty) => {
        impl JsTypedArray for $ty {
            type Item = $item;

            fn buffer(&self) -> js_sys::ArrayBuffer {
                self.buffer()
            }

            fn byte_offset(&self) -> u32 {
                self.byte_offset()
            }

            fn byte_length(&self) -> u32 {
                self.byte_length()
            }

            fn length(&self) -> u32 {
                self.length()
            }
        }
    };
}

#[cfg(target_family = "wasm")]
impl_js_typed_array!(js_sys::BigInt64Array, i64);

#[cfg(target_family = "wasm")]
impl_js_typed_array!(js_sys::BigUint64Array, u64);

#[cfg(target_family = "wasm")]
pub fn slice_from_js_typed_array<T: JsTypedArray>(array: T) -> Box<[T::Item]> {
    let byte_length = array.byte_length();
    assert_eq!(
        byte_length as usize,
        array.length() as usize * std::mem::size_of::<T::Item>()
    );
    let bytes = js_sys::Uint8Array::new_with_byte_offset_and_length(
        &array.buffer(),
        array.byte_offset(),
        byte_length,
    );
    slice_from_byte_buffer(bytes.to_vec())
}
