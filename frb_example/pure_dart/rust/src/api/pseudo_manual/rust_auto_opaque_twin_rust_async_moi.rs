// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

use flutter_rust_bridge::frb;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinRustAsyncMoi {
    inner: i32,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_arg_own_twin_rust_async_moi(arg: NonCloneSimpleTwinRustAsyncMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_arg_borrow_twin_rust_async_moi(arg: &NonCloneSimpleTwinRustAsyncMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_arg_mut_borrow_twin_rust_async_moi(
    arg: &mut NonCloneSimpleTwinRustAsyncMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_return_own_twin_rust_async_moi(initial: i32) -> NonCloneSimpleTwinRustAsyncMoi {
    NonCloneSimpleTwinRustAsyncMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_arg_own_and_return_own_twin_rust_async_moi(
    arg: NonCloneSimpleTwinRustAsyncMoi,
) -> NonCloneSimpleTwinRustAsyncMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_two_args_twin_rust_async_moi(
    a: NonCloneSimpleTwinRustAsyncMoi,
    b: NonCloneSimpleTwinRustAsyncMoi,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async_moi(a: NonCloneSimpleTwinRustAsyncMoi, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinRustAsyncMoi {
    fn f(&self) -> &str;
}
impl MyTraitTwinRustAsyncMoi for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async_moi(arg: Box<dyn MyTraitTwinRustAsyncMoi + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async_moi() -> Box<dyn MyTraitTwinRustAsyncMoi + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_callable_arg_twin_rust_async_moi(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_callable_return_twin_rust_async_moi() -> Box<dyn Fn(String) -> String + Send + Sync>
{
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinRustAsyncMoi: Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinRustAsyncMoi {
    inner: String,
}

impl HelloTraitTwinRustAsyncMoi for HelloOneStructTwinRustAsyncMoi {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinRustAsyncMoi {
    A,
    B,
}

impl HelloTraitTwinRustAsyncMoi for HelloTwoEnumTwinRustAsyncMoi {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinRustAsyncMoi::A => "A",
            HelloTwoEnumTwinRustAsyncMoi::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async_moi(
    arg: Box<dyn HelloTraitTwinRustAsyncMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[allow(clippy::borrowed_box)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async_moi(
    arg: &Box<dyn HelloTraitTwinRustAsyncMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async_moi(
    arg: &mut Box<dyn HelloTraitTwinRustAsyncMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async_moi() -> Box<dyn HelloTraitTwinRustAsyncMoi> {
    Box::new(HelloOneStructTwinRustAsyncMoi {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async_moi() -> Box<dyn HelloTraitTwinRustAsyncMoi> {
    Box::new(HelloTwoEnumTwinRustAsyncMoi::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsyncMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn static_method_arg_own_twin_rust_async_moi(arg: NonCloneSimpleTwinRustAsyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn static_method_arg_borrow_twin_rust_async_moi(arg: &NonCloneSimpleTwinRustAsyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn static_method_arg_mut_borrow_twin_rust_async_moi(arg: &mut NonCloneSimpleTwinRustAsyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn static_method_return_own_twin_rust_async_moi() -> NonCloneSimpleTwinRustAsyncMoi {
        NonCloneSimpleTwinRustAsyncMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinRustAsyncMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn new_twin_rust_async_moi() -> NonCloneSimpleTwinRustAsyncMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn new_custom_name_twin_rust_async_moi() -> NonCloneSimpleTwinRustAsyncMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn new_with_result_twin_rust_async_moi() -> anyhow::Result<NonCloneSimpleTwinRustAsyncMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn instance_method_arg_own_twin_rust_async_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn instance_method_arg_borrow_twin_rust_async_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn instance_method_arg_mut_borrow_twin_rust_async_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn instance_method_return_own_twin_rust_async_moi(&self) -> NonCloneSimpleTwinRustAsyncMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn instance_method_getter_twin_rust_async_moi(&self) -> i32 {
        self.inner
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsyncMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsyncMoi,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async_moi(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsyncMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_rust_async_moi(
    arg: &StructWithGoodAndOpaqueFieldTwinRustAsyncMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_rust_async_moi(
    arg: &mut StructWithGoodAndOpaqueFieldTwinRustAsyncMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async_moi(
) -> StructWithGoodAndOpaqueFieldTwinRustAsyncMoi {
    StructWithGoodAndOpaqueFieldTwinRustAsyncMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsyncMoi { inner: 42 },
    }
}

// ================ misc ===================

// #1577 - this should generate valid Dart code without name collisions
pub struct OpaqueOneTwinRustAsyncMoi(PathBuf);
pub struct OpaqueTwoTwinRustAsyncMoi(PathBuf);
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_return_opaque_one_and_two_twin_rust_async_moi(
) -> (OpaqueOneTwinRustAsyncMoi, OpaqueTwoTwinRustAsyncMoi) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] pub async fn rust_auto_opaque_return_opaque_two_twin_rust_async_moi() -> OpaqueTwoTwinRustAsyncMoi {
    unimplemented!()
}
