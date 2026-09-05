use std::mem;

// ref https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
pub fn new_leak_vec_ptr<T: Clone>(fill: T, length: i32) -> *mut T {
    into_leak_vec_ptr(vec![fill; length as usize]).0
}

pub fn into_leak_vec_ptr<T: Clone>(mut v: Vec<T>) -> (*mut T, i32) {
    v.shrink_to_fit();
    assert_eq!(v.len(), v.capacity());
    let ptr = v.as_mut_ptr();
    let len = v.len() as i32;
    mem::forget(v);
    (ptr, len)
}

/// # Safety
/// Use it in pair with [new_leak_vec_ptr].
pub unsafe fn vec_from_leak_ptr<T>(ptr: *mut T, len: i32) -> Vec<T> {
    Vec::from_raw_parts(ptr, len as usize, len as usize)
}

/// Convert `Vec<T>` to array length `N`.
///
/// # Panics
///
/// Panics if length of `Vec<T>` != `N`.
pub fn from_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    core::convert::TryInto::try_into(v)
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// ref: doc of [Box::into_raw]
pub fn new_leak_box_ptr<T>(t: T) -> *mut T {
    let x: Box<T> = Box::new(t);
    Box::into_raw(x)
}

/// # Safety
/// Use it in pair with [new_leak_box_ptr].
pub unsafe fn box_from_leak_ptr<T>(ptr: *mut T) -> Box<T> {
    Box::from_raw(ptr)
}

#[cfg(test)]
mod tests {
    use super::{
        box_from_leak_ptr, from_vec_to_array, into_leak_vec_ptr, new_leak_box_ptr,
        new_leak_vec_ptr, vec_from_leak_ptr,
    };

    /// Restores a vector leaked from an existing allocation.
    #[test]
    fn restores_leaked_vector() {
        let (ptr, len) = into_leak_vec_ptr(vec![10, 20, 30]);

        let recovered = unsafe { vec_from_leak_ptr(ptr, len) };

        assert_eq!(recovered, vec![10, 20, 30]);
    }

    /// Restores a filled vector leaked through the convenience function.
    #[test]
    fn restores_filled_leaked_vector() {
        let ptr = new_leak_vec_ptr("value", 3);

        let recovered = unsafe { vec_from_leak_ptr(ptr, 3) };

        assert_eq!(recovered, vec!["value", "value", "value"]);
    }

    /// Converts a vector with the requested length into an array.
    #[test]
    fn converts_vector_with_matching_length() {
        assert_eq!(from_vec_to_array::<_, 3>(vec![1, 2, 3]), [1, 2, 3]);
    }

    /// Rejects a vector whose length differs from the target array.
    #[test]
    #[should_panic(expected = "Expected a Vec of length 2 but it was 3")]
    fn rejects_vector_with_mismatched_length() {
        let _ = from_vec_to_array::<_, 2>(vec![1, 2, 3]);
    }

    /// Restores a boxed value leaked through its raw pointer.
    #[test]
    fn restores_leaked_box() {
        let ptr = new_leak_box_ptr(String::from("value"));

        let recovered = unsafe { box_from_leak_ptr(ptr) };

        assert_eq!(*recovered, "value");
    }
}
