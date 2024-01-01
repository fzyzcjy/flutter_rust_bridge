// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsyncSse"]}

use flutter_rust_bridge::frb;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinSync {
    inner: i32,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_twin_sync(arg: NonCloneSimpleTwinSync, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_borrow_twin_sync(arg: &NonCloneSimpleTwinSync, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_sync(
    arg: &mut NonCloneSimpleTwinSync,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_own_twin_sync(initial: i32) -> NonCloneSimpleTwinSync {
    NonCloneSimpleTwinSync { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_sync(
    arg: NonCloneSimpleTwinSync,
) -> NonCloneSimpleTwinSync {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_two_args_twin_sync(a: NonCloneSimpleTwinSync, b: NonCloneSimpleTwinSync) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_sync(a: NonCloneSimpleTwinSync, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinSync {
    fn f(&self) -> &str;
}
impl MyTraitTwinSync for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_arg_twin_sync(arg: Box<dyn MyTraitTwinSync + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_return_twin_sync() -> Box<dyn MyTraitTwinSync + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_arg_twin_sync(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_return_twin_sync() -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinSync: Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinSync {
    inner: String,
}

impl HelloTraitTwinSync for HelloOneStructTwinSync {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinSync {
    A,
    B,
}

impl HelloTraitTwinSync for HelloTwoEnumTwinSync {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinSync::A => "A",
            HelloTwoEnumTwinSync::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_own_twin_sync(
    arg: Box<dyn HelloTraitTwinSync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[allow(clippy::borrowed_box)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sync(
    arg: &Box<dyn HelloTraitTwinSync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync(
    arg: &mut Box<dyn HelloTraitTwinSync>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_return_own_one_twin_sync() -> Box<dyn HelloTraitTwinSync> {
    Box::new(HelloOneStructTwinSync {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_return_own_two_twin_sync() -> Box<dyn HelloTraitTwinSync> {
    Box::new(HelloTwoEnumTwinSync::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_own_twin_sync(arg: NonCloneSimpleTwinSync) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_borrow_twin_sync(arg: &NonCloneSimpleTwinSync) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_mut_borrow_twin_sync(arg: &mut NonCloneSimpleTwinSync) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_return_own_twin_sync() -> NonCloneSimpleTwinSync {
        NonCloneSimpleTwinSync { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinSync {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync() -> NonCloneSimpleTwinSync {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_custom_name_twin_sync() -> NonCloneSimpleTwinSync {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_with_result_twin_sync() -> anyhow::Result<NonCloneSimpleTwinSync> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_own_twin_sync(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_borrow_twin_sync(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_mut_borrow_twin_sync(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_return_own_twin_sync(&self) -> NonCloneSimpleTwinSync {
        Self { inner: 42 }
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSync {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSync,
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync(
    arg: StructWithGoodAndOpaqueFieldTwinSync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync(
    arg: &StructWithGoodAndOpaqueFieldTwinSync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync(
    arg: &mut StructWithGoodAndOpaqueFieldTwinSync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync(
) -> StructWithGoodAndOpaqueFieldTwinSync {
    StructWithGoodAndOpaqueFieldTwinSync {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSync { inner: 42 },
    }
}
