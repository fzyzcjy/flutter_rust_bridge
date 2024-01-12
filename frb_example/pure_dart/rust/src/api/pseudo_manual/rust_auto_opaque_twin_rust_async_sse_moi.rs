// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

use flutter_rust_bridge::frb;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinRustAsyncSseMoi {
    inner: i32,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_arg_own_twin_rust_async_sse_moi(arg: NonCloneSimpleTwinRustAsyncSseMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_arg_borrow_twin_rust_async_sse_moi(arg: &NonCloneSimpleTwinRustAsyncSseMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_arg_mut_borrow_twin_rust_async_sse_moi(
    arg: &mut NonCloneSimpleTwinRustAsyncSseMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_return_own_twin_rust_async_sse_moi(initial: i32) -> NonCloneSimpleTwinRustAsyncSseMoi {
    NonCloneSimpleTwinRustAsyncSseMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_arg_own_and_return_own_twin_rust_async_sse_moi(
    arg: NonCloneSimpleTwinRustAsyncSseMoi,
) -> NonCloneSimpleTwinRustAsyncSseMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_two_args_twin_rust_async_sse_moi(
    a: NonCloneSimpleTwinRustAsyncSseMoi,
    b: NonCloneSimpleTwinRustAsyncSseMoi,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async_sse_moi(a: NonCloneSimpleTwinRustAsyncSseMoi, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinRustAsyncSseMoi {
    fn f(&self) -> &str;
}
impl MyTraitTwinRustAsyncSseMoi for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async_sse_moi(arg: Box<dyn MyTraitTwinRustAsyncSseMoi + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async_sse_moi() -> Box<dyn MyTraitTwinRustAsyncSseMoi + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_callable_arg_twin_rust_async_sse_moi(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_callable_return_twin_rust_async_sse_moi() -> Box<dyn Fn(String) -> String + Send + Sync>
{
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinRustAsyncSseMoi: Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinRustAsyncSseMoi {
    inner: String,
}

impl HelloTraitTwinRustAsyncSseMoi for HelloOneStructTwinRustAsyncSseMoi {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinRustAsyncSseMoi {
    A,
    B,
}

impl HelloTraitTwinRustAsyncSseMoi for HelloTwoEnumTwinRustAsyncSseMoi {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinRustAsyncSseMoi::A => "A",
            HelloTwoEnumTwinRustAsyncSseMoi::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async_sse_moi(
    arg: Box<dyn HelloTraitTwinRustAsyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[allow(clippy::borrowed_box)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async_sse_moi(
    arg: &Box<dyn HelloTraitTwinRustAsyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async_sse_moi(
    arg: &mut Box<dyn HelloTraitTwinRustAsyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async_sse_moi() -> Box<dyn HelloTraitTwinRustAsyncSseMoi> {
    Box::new(HelloOneStructTwinRustAsyncSseMoi {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async_sse_moi() -> Box<dyn HelloTraitTwinRustAsyncSseMoi> {
    Box::new(HelloTwoEnumTwinRustAsyncSseMoi::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsyncSseMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn static_method_arg_own_twin_rust_async_sse_moi(arg: NonCloneSimpleTwinRustAsyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn static_method_arg_borrow_twin_rust_async_sse_moi(arg: &NonCloneSimpleTwinRustAsyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn static_method_arg_mut_borrow_twin_rust_async_sse_moi(arg: &mut NonCloneSimpleTwinRustAsyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn static_method_return_own_twin_rust_async_sse_moi() -> NonCloneSimpleTwinRustAsyncSseMoi {
        NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinRustAsyncSseMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn new_twin_rust_async_sse_moi() -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn new_custom_name_twin_rust_async_sse_moi() -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn new_with_result_twin_rust_async_sse_moi() -> anyhow::Result<NonCloneSimpleTwinRustAsyncSseMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn instance_method_arg_own_twin_rust_async_sse_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn instance_method_arg_borrow_twin_rust_async_sse_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn instance_method_arg_mut_borrow_twin_rust_async_sse_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn instance_method_return_own_twin_rust_async_sse_moi(&self) -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn instance_method_getter_twin_rust_async_sse_moi(&self) -> i32 {
        self.inner
    }
}

// ================ types with both encodable and opaque fields ===================

#[frb(opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsyncSseMoi,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async_sse_moi(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_rust_async_sse_moi(
    arg: &StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_rust_async_sse_moi(
    arg: &mut StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async_sse_moi(
) -> StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
    StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 },
    }
}

// ================ misc ===================

// #1577 - this should generate valid Dart code without name collisions
pub struct OpaqueOneTwinRustAsyncSseMoi(PathBuf);
pub struct OpaqueTwoTwinRustAsyncSseMoi(PathBuf);
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_return_opaque_one_and_two_twin_rust_async_sse_moi(
) -> (OpaqueOneTwinRustAsyncSseMoi, OpaqueTwoTwinRustAsyncSseMoi) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_return_opaque_two_twin_rust_async_sse_moi() -> OpaqueTwoTwinRustAsyncSseMoi {
    unimplemented!()
}
