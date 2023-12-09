// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsyncSse"]}

use flutter_rust_bridge::{frb, DartSafe};
pub use std::panic::{RefUnwindSafe, UnwindSafe};

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinSse {
    inner: i32,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_arg_own_twin_sse(arg: NonCloneSimpleTwinSse, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_arg_borrow_twin_sse(arg: &NonCloneSimpleTwinSse, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_sse(
    arg: &mut NonCloneSimpleTwinSse,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_return_own_twin_sse(initial: i32) -> NonCloneSimpleTwinSse {
    NonCloneSimpleTwinSse { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_sse(
    arg: NonCloneSimpleTwinSse,
) -> NonCloneSimpleTwinSse {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_two_args_twin_sse(a: NonCloneSimpleTwinSse, b: NonCloneSimpleTwinSse) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_sse(a: NonCloneSimpleTwinSse, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinSse: DartSafe {
    fn f(&self) -> &str;
}
impl MyTraitTwinSse for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_plus_sign_arg_twin_sse(arg: Box<dyn MyTraitTwinSse + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_plus_sign_return_twin_sse() -> Box<dyn MyTraitTwinSse + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_callable_arg_twin_sse(
    arg: Box<dyn Fn(String) -> String + Send + Sync + UnwindSafe + RefUnwindSafe>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_callable_return_twin_sse(
) -> Box<dyn Fn(String) -> String + Send + Sync + UnwindSafe + RefUnwindSafe> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinSse: DartSafe + Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinSse {
    inner: String,
}

impl HelloTraitTwinSse for HelloOneStructTwinSse {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinSse {
    A,
    B,
}

impl HelloTraitTwinSse for HelloTwoEnumTwinSse {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinSse::A => "A",
            HelloTwoEnumTwinSse::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_trait_object_arg_own_twin_sse(
    arg: Box<dyn HelloTraitTwinSse>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sse(
    arg: &Box<dyn HelloTraitTwinSse>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sse(
    arg: &mut Box<dyn HelloTraitTwinSse>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_trait_object_return_own_one_twin_sse() -> Box<dyn HelloTraitTwinSse> {
    Box::new(HelloOneStructTwinSse {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_trait_object_return_own_two_twin_sse() -> Box<dyn HelloTraitTwinSse> {
    Box::new(HelloTwoEnumTwinSse::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_arg_own_twin_sse(arg: NonCloneSimpleTwinSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_arg_borrow_twin_sse(arg: &NonCloneSimpleTwinSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_arg_mut_borrow_twin_sse(arg: &mut NonCloneSimpleTwinSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_return_own_twin_sse() -> NonCloneSimpleTwinSse {
        NonCloneSimpleTwinSse { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinSse {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse() -> NonCloneSimpleTwinSse {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_custom_name_twin_sse() -> NonCloneSimpleTwinSse {
        Self { inner: 42 }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_arg_own_twin_sse(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_arg_borrow_twin_sse(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_arg_mut_borrow_twin_sse(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_return_own_twin_sse(&self) -> NonCloneSimpleTwinSse {
        Self { inner: 42 }
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSse {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSse,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sse(
    arg: StructWithGoodAndOpaqueFieldTwinSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sse(
    arg: &StructWithGoodAndOpaqueFieldTwinSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sse(
    arg: &mut StructWithGoodAndOpaqueFieldTwinSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sse(
) -> StructWithGoodAndOpaqueFieldTwinSse {
    StructWithGoodAndOpaqueFieldTwinSse {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSse { inner: 42 },
    }
}
