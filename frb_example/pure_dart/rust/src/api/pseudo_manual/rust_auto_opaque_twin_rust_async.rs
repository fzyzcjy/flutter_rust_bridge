// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::{frb, DartSafe};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinRustAsync {
    inner: i32,
}

// ==================================== simple =======================================

pub async fn rust_auto_opaque_arg_own_twin_rust_async(arg: NonCloneSimpleTwinRustAsync, expect: i32) {
    assert_eq!(arg.inner, expect);
}

pub async fn rust_auto_opaque_arg_borrow_twin_rust_async(arg: &NonCloneSimpleTwinRustAsync, expect: i32) {
    assert_eq!(arg.inner, expect);
}

pub async fn rust_auto_opaque_arg_mut_borrow_twin_rust_async(
    arg: &mut NonCloneSimpleTwinRustAsync,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

pub async fn rust_auto_opaque_return_own_twin_rust_async(initial: i32) -> NonCloneSimpleTwinRustAsync {
    NonCloneSimpleTwinRustAsync { inner: initial }
}

// ==================================== with other args =======================================

pub async fn rust_auto_opaque_arg_own_and_return_own_twin_rust_async(
    arg: NonCloneSimpleTwinRustAsync,
) -> NonCloneSimpleTwinRustAsync {
    assert_eq!(arg.inner, 42);
    arg
}

pub async fn rust_auto_opaque_two_args_twin_rust_async(
    a: NonCloneSimpleTwinRustAsync,
    b: NonCloneSimpleTwinRustAsync,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async(a: NonCloneSimpleTwinRustAsync, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinRustAsync: DartSafe {
    fn f(&self) -> &str;
}
impl MyTraitTwinRustAsync for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async(arg: Box<dyn MyTraitTwinRustAsync + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async() -> Box<dyn MyTraitTwinRustAsync + Send + Sync> {
    Box::new("hello".to_owned())
}

pub async fn rust_auto_opaque_callable_arg_twin_rust_async(
    arg: Box<dyn Fn(String) -> String + Send + Sync + UnwindSafe + RefUnwindSafe>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

pub async fn rust_auto_opaque_callable_return_twin_rust_async(
) -> Box<dyn Fn(String) -> String + Send + Sync + UnwindSafe + RefUnwindSafe> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinRustAsync: DartSafe + Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinRustAsync {
    inner: String,
}

impl HelloTraitTwinRustAsync for HelloOneStructTwinRustAsync {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinRustAsync {
    A,
    B,
}

impl HelloTraitTwinRustAsync for HelloTwoEnumTwinRustAsync {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinRustAsync::A => "A",
            HelloTwoEnumTwinRustAsync::B => "B",
        }
    }
}

pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async(
    arg: Box<dyn HelloTraitTwinRustAsync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async(
    arg: &Box<dyn HelloTraitTwinRustAsync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async(
    arg: &mut Box<dyn HelloTraitTwinRustAsync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async() -> Box<dyn HelloTraitTwinRustAsync> {
    Box::new(HelloOneStructTwinRustAsync {
        inner: "hello".into(),
    })
}

pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async() -> Box<dyn HelloTraitTwinRustAsync> {
    Box::new(HelloTwoEnumTwinRustAsync::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsync {
    pub async fn static_method_arg_own_twin_rust_async(arg: NonCloneSimpleTwinRustAsync) {
        assert_eq!(arg.inner, 42);
    }

    pub async fn static_method_arg_borrow_twin_rust_async(arg: &NonCloneSimpleTwinRustAsync) {
        assert_eq!(arg.inner, 42);
    }

    pub async fn static_method_arg_mut_borrow_twin_rust_async(arg: &mut NonCloneSimpleTwinRustAsync) {
        assert_eq!(arg.inner, 42);
    }

    pub async fn static_method_return_own_twin_rust_async() -> NonCloneSimpleTwinRustAsync {
        NonCloneSimpleTwinRustAsync { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinRustAsync {
    /// unnamed constructor
    pub async fn new_twin_rust_async() -> NonCloneSimpleTwinRustAsync {
        Self { inner: 42 }
    }

    /// named constructor
    pub async fn new_custom_name_twin_rust_async() -> NonCloneSimpleTwinRustAsync {
        Self { inner: 42 }
    }

    pub async fn instance_method_arg_own_twin_rust_async(self) {
        assert_eq!(self.inner, 42);
    }

    pub async fn instance_method_arg_borrow_twin_rust_async(&self) {
        assert_eq!(self.inner, 42);
    }

    pub async fn instance_method_arg_mut_borrow_twin_rust_async(&mut self) {
        assert_eq!(self.inner, 42);
    }

    pub async fn instance_method_return_own_twin_rust_async(&self) -> NonCloneSimpleTwinRustAsync {
        Self { inner: 42 }
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsync {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsync,
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_rust_async(
    arg: &StructWithGoodAndOpaqueFieldTwinRustAsync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_rust_async(
    arg: &mut StructWithGoodAndOpaqueFieldTwinRustAsync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async(
) -> StructWithGoodAndOpaqueFieldTwinRustAsync {
    StructWithGoodAndOpaqueFieldTwinRustAsync {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsync { inner: 42 },
    }
}
