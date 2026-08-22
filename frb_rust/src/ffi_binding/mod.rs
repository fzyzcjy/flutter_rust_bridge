use crate::for_generated::{into_leak_vec_ptr, new_leak_vec_ptr, vec_from_leak_ptr};

#[cfg(not(target_family = "wasm"))]
mod io;
#[cfg(not(target_family = "wasm"))]
#[allow(unused)]
pub use io::*;

#[cfg(target_family = "wasm")]
mod web;
#[cfg(target_family = "wasm")]
#[allow(unused)]
pub use web::*;

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_new(len: i32) -> *mut u8 {
    new_leak_vec_ptr::<u8>(0, len)
}

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_resize(
    ptr: *mut u8,
    old_len: i32,
    new_len: i32,
) -> *mut u8 {
    let mut vec = vec_from_leak_ptr(ptr, old_len);
    vec_resize(&mut vec, new_len);
    into_leak_vec_ptr(vec).0
}

fn vec_resize(vec: &mut Vec<u8>, new_len: i32) {
    let new_len = new_len as usize;
    if new_len > vec.len() {
        vec.reserve_exact(new_len - vec.len());
    }
    vec.resize(new_len, 0);
    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    debug_assert!(vec.len() == new_len);
    // frb-coverage:ignore-end
}

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_free(ptr: *mut u8, len: i32) {
    vec_from_leak_ptr(ptr, len);
}

#[cfg(test)]
mod tests {
    use super::vec_resize;

    #[test]
    /// Extending preserves bytes and zero-initializes new elements.
    fn vec_resize_extends_with_zeroes() {
        let mut value = vec![3, 5];

        vec_resize(&mut value, 5);

        assert_eq!(value, vec![3, 5, 0, 0, 0]);
    }

    #[test]
    /// Shrinking retains the prefix at the requested length.
    fn vec_resize_shrinks_to_requested_prefix() {
        let mut value = vec![3, 5, 7, 11];

        vec_resize(&mut value, 2);

        assert_eq!(value, vec![3, 5]);
    }

    #[test]
    /// Resizing to zero clears every element.
    fn vec_resize_clears_vector_at_zero_length() {
        let mut value = vec![3, 5];

        vec_resize(&mut value, 0);

        assert!(value.is_empty());
    }
}
