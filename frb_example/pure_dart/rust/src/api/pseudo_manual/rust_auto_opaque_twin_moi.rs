// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use flutter_rust_bridge::rust_async::RwLock;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinMoi {
    inner: i32,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_arg_own_twin_moi(arg: NonCloneSimpleTwinMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_arg_borrow_twin_moi(arg: &NonCloneSimpleTwinMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_moi(
    arg: &mut NonCloneSimpleTwinMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_return_own_twin_moi(initial: i32) -> NonCloneSimpleTwinMoi {
    NonCloneSimpleTwinMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_moi(
    arg: NonCloneSimpleTwinMoi,
) -> NonCloneSimpleTwinMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_two_args_twin_moi(a: NonCloneSimpleTwinMoi, b: NonCloneSimpleTwinMoi) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_moi(a: NonCloneSimpleTwinMoi, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinMoi {
    fn f(&self) -> &str;
}
impl MyTraitTwinMoi for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_plus_sign_arg_twin_moi(arg: Box<dyn MyTraitTwinMoi + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_plus_sign_return_twin_moi() -> Box<dyn MyTraitTwinMoi + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_callable_arg_twin_moi(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_callable_return_twin_moi() -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinMoi: Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinMoi {
    inner: String,
}

impl HelloTraitTwinMoi for HelloOneStructTwinMoi {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinMoi {
    A,
    B,
}

impl HelloTraitTwinMoi for HelloTwoEnumTwinMoi {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinMoi::A => "A",
            HelloTwoEnumTwinMoi::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_trait_object_arg_own_twin_moi(
    arg: Box<dyn HelloTraitTwinMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[allow(clippy::borrowed_box)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_trait_object_arg_borrow_twin_moi(
    arg: &Box<dyn HelloTraitTwinMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_moi(
    arg: &mut Box<dyn HelloTraitTwinMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_trait_object_return_own_one_twin_moi() -> Box<dyn HelloTraitTwinMoi> {
    Box::new(HelloOneStructTwinMoi {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_trait_object_return_own_two_twin_moi() -> Box<dyn HelloTraitTwinMoi> {
    Box::new(HelloTwoEnumTwinMoi::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn static_method_arg_own_twin_moi(arg: NonCloneSimpleTwinMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn static_method_arg_borrow_twin_moi(arg: &NonCloneSimpleTwinMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn static_method_arg_mut_borrow_twin_moi(arg: &mut NonCloneSimpleTwinMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn static_method_return_own_twin_moi() -> NonCloneSimpleTwinMoi {
        NonCloneSimpleTwinMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn new_twin_moi() -> NonCloneSimpleTwinMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn new_custom_name_twin_moi() -> NonCloneSimpleTwinMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn new_with_result_twin_moi() -> anyhow::Result<NonCloneSimpleTwinMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn instance_method_arg_own_twin_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn instance_method_arg_borrow_twin_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn instance_method_arg_mut_borrow_twin_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn instance_method_return_own_twin_moi(&self) -> NonCloneSimpleTwinMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    pub fn instance_method_getter_twin_moi(&self) -> i32 {
        self.inner
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinMoi,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_moi(
    arg: StructWithGoodAndOpaqueFieldTwinMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_moi(
    arg: &StructWithGoodAndOpaqueFieldTwinMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_moi(
    arg: &mut StructWithGoodAndOpaqueFieldTwinMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_moi(
) -> StructWithGoodAndOpaqueFieldTwinMoi {
    StructWithGoodAndOpaqueFieldTwinMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinMoi { inner: 42 },
    }
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_explicit_arg_twin_moi(
    arg: RustAutoOpaque<NonCloneSimpleTwinMoi>,
    expect: i32,
) {
    assert_eq!((*arg).try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinMoi {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinMoi>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_explicit_struct_twin_moi(arg: StructWithExplicitAutoOpaqueFieldTwinMoi) {
    assert_eq!((*arg.auto_opaque).try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_explicit_return_twin_moi(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinMoi> {
    RustAutoOpaque::new(RwLock::new(NonCloneSimpleTwinMoi { inner: initial }))
}

// ================ misc ===================

// #1577 - this should generate valid Dart code without name collisions
pub struct OpaqueOneTwinMoi(PathBuf);
pub struct OpaqueTwoTwinMoi(PathBuf);
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_moi() -> (OpaqueOneTwinMoi, OpaqueTwoTwinMoi)
{
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_return_opaque_two_twin_moi() -> OpaqueTwoTwinMoi {
    unimplemented!()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_moi(
    borrow: &NonCloneSimpleTwinMoi,
    mut_borrow: &mut NonCloneSimpleTwinMoi,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_moi(
    a: &NonCloneSimpleTwinMoi,
    b: &NonCloneSimpleTwinMoi,
) -> i32 {
    a.inner + b.inner
}
