// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinNormal {
    inner: i32,
}

pub fn rust_auto_opaque_arg_own(arg: NonCloneSimpleTwinNormal) {
    assert_eq!(arg.inner, 42);
}

pub fn rust_auto_opaque_arg_borrow(arg: &NonCloneSimpleTwinNormal) {
    assert_eq!(arg.inner, 42);
}

pub fn rust_auto_opaque_arg_mut_borrow(arg: &mut NonCloneSimpleTwinNormal) {
    assert_eq!(arg.inner, 42);
    arg.inner += 1;
}

pub fn rust_auto_opaque_return_own() -> NonCloneSimpleTwinNormal {
    NonCloneSimpleTwinNormal { inner: 42 }
}

pub fn rust_auto_opaque_arg_own_and_return_own(
    arg: NonCloneSimpleTwinNormal,
) -> NonCloneSimpleTwinNormal {
    assert_eq!(arg.inner, 42);
    arg
}

pub fn rust_auto_opaque_two_args(a: NonCloneSimpleTwinNormal, b: NonCloneSimpleTwinNormal) {
    assert_eq!(a.inner, 42);
    assert_eq!(b.inner, 42);
}

pub fn rust_auto_opaque_normal_and_opaque_arg(a: NonCloneSimpleTwinNormal, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// TODO static methods
// TODO instance methods
// TODO dyn trait
// TODO complex type signature
// TODO opaque *inside* other objects, especially ref/mutref ---- a bit hard, do it later
// TODO return borrow ---- hard?
