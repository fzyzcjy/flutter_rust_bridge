// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

#[allow(unused_imports)]
use crate::frb_generated::RustAutoOpaque;
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinRustAsyncSse {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinRustAsyncSse {
    Apple,
    Orange,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_own_twin_rust_async_sse(
    arg: NonCloneSimpleTwinRustAsyncSse,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_borrow_twin_rust_async_sse(
    arg: &NonCloneSimpleTwinRustAsyncSse,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_mut_borrow_twin_rust_async_sse(
    arg: &mut NonCloneSimpleTwinRustAsyncSse,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_own_twin_rust_async_sse(
    initial: i32,
) -> NonCloneSimpleTwinRustAsyncSse {
    NonCloneSimpleTwinRustAsyncSse { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_own_and_return_own_twin_rust_async_sse(
    arg: NonCloneSimpleTwinRustAsyncSse,
) -> NonCloneSimpleTwinRustAsyncSse {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_two_args_twin_rust_async_sse(
    a: NonCloneSimpleTwinRustAsyncSse,
    b: NonCloneSimpleTwinRustAsyncSse,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async_sse(
    a: NonCloneSimpleTwinRustAsyncSse,
    b: String,
) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinRustAsyncSse {
    fn f(&self) -> &str;
}
impl MyTraitTwinRustAsyncSse for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async_sse(
    arg: Box<dyn MyTraitTwinRustAsyncSse + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async_sse(
) -> Box<dyn MyTraitTwinRustAsyncSse + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_callable_arg_twin_rust_async_sse(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_callable_return_twin_rust_async_sse(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinRustAsyncSse: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinRustAsyncSse {
//     inner: String,
// }
//
// impl HelloTraitTwinRustAsyncSse for HelloOneStructTwinRustAsyncSse {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinRustAsyncSse {
//     A,
//     B,
// }
//
// impl HelloTraitTwinRustAsyncSse for HelloTwoEnumTwinRustAsyncSse {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinRustAsyncSse::A => "A",
//             HelloTwoEnumTwinRustAsyncSse::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async_sse(
//     arg: Box<dyn HelloTraitTwinRustAsyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async_sse(
//     arg: &Box<dyn HelloTraitTwinRustAsyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async_sse(
//     arg: &mut Box<dyn HelloTraitTwinRustAsyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async_sse() -> Box<dyn HelloTraitTwinRustAsyncSse> {
//     Box::new(HelloOneStructTwinRustAsyncSse {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async_sse() -> Box<dyn HelloTraitTwinRustAsyncSse> {
//     Box::new(HelloTwoEnumTwinRustAsyncSse::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_own_twin_rust_async_sse(arg: NonCloneSimpleTwinRustAsyncSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_borrow_twin_rust_async_sse(
        arg: &NonCloneSimpleTwinRustAsyncSse,
    ) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_mut_borrow_twin_rust_async_sse(
        arg: &mut NonCloneSimpleTwinRustAsyncSse,
    ) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_return_own_twin_rust_async_sse() -> NonCloneSimpleTwinRustAsyncSse {
        NonCloneSimpleTwinRustAsyncSse { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinRustAsyncSse {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse() -> NonCloneSimpleTwinRustAsyncSse {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_custom_name_twin_rust_async_sse() -> NonCloneSimpleTwinRustAsyncSse {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_with_result_twin_rust_async_sse(
    ) -> anyhow::Result<NonCloneSimpleTwinRustAsyncSse> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_own_twin_rust_async_sse(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_borrow_twin_rust_async_sse(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_mut_borrow_twin_rust_async_sse(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_return_own_twin_rust_async_sse(
        &self,
    ) -> NonCloneSimpleTwinRustAsyncSse {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_getter_twin_rust_async_sse(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsyncSse {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsyncSse,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinRustAsyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async_sse(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsyncSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async_sse(
) -> StructWithGoodAndOpaqueFieldTwinRustAsyncSse {
    StructWithGoodAndOpaqueFieldTwinRustAsyncSse {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsyncSse { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinRustAsyncSse { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinRustAsyncSse {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsyncSse),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_rust_async_sse(
    arg: EnumWithGoodAndOpaqueTwinRustAsyncSse,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinRustAsyncSse::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinRustAsyncSse::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_rust_async_sse(
) -> EnumWithGoodAndOpaqueTwinRustAsyncSse {
    EnumWithGoodAndOpaqueTwinRustAsyncSse::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_rust_async_sse(
) -> EnumWithGoodAndOpaqueTwinRustAsyncSse {
    EnumWithGoodAndOpaqueTwinRustAsyncSse::Opaque(NonCloneSimpleTwinRustAsyncSse { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncSse {
    pub good: String,
    opaque: NonCloneSimpleTwinRustAsyncSse,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncSse {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsyncSse),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_dummy_twin_rust_async_sse(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncSse,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncSse,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_arg_borrow_twin_rust_async_sse(
    arg: &NonCloneSimpleEnumTwinRustAsyncSse,
) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinRustAsyncSse::Orange));
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_return_own_twin_rust_async_sse(
) -> NonCloneSimpleEnumTwinRustAsyncSse {
    NonCloneSimpleEnumTwinRustAsyncSse::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_stream_sink_twin_rust_async_sse(
    sink: StreamSink<NonCloneSimpleTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
) {
    sink.add(NonCloneSimpleTwinRustAsyncSse { inner: 42 })
        .unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_vec_own_twin_rust_async_sse(
    arg: Vec<NonCloneSimpleTwinRustAsyncSse>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_vec_own_twin_rust_async_sse(
) -> Vec<NonCloneSimpleTwinRustAsyncSse> {
    vec![
        NonCloneSimpleTwinRustAsyncSse { inner: 10 },
        NonCloneSimpleTwinRustAsyncSse { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_arg_twin_rust_async_sse(
    arg: RustAutoOpaque<NonCloneSimpleTwinRustAsyncSse>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinRustAsyncSse {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinRustAsyncSse>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_struct_twin_rust_async_sse(
    arg: StructWithExplicitAutoOpaqueFieldTwinRustAsyncSse,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_return_struct_twin_rust_async_sse(
) -> StructWithExplicitAutoOpaqueFieldTwinRustAsyncSse {
    StructWithExplicitAutoOpaqueFieldTwinRustAsyncSse {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinRustAsyncSse { inner: 100 }),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_return_twin_rust_async_sse(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinRustAsyncSse> {
    RustAutoOpaque::new(NonCloneSimpleTwinRustAsyncSse { inner: initial })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_sleep_twin_rust_async_sse(
    apple: &mut NonCloneSimpleTwinRustAsyncSse,
    orange: &mut NonCloneSimpleTwinRustAsyncSse,
) -> i32 {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    apple.inner + orange.inner
}

// ================ misc ===================

// #1577 - this should generate valid Dart code without name collisions
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueOneTwinRustAsyncSse(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinRustAsyncSse(PathBuf);

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_opaque_one_and_two_twin_rust_async_sse(
) -> (OpaqueOneTwinRustAsyncSse, OpaqueTwoTwinRustAsyncSse) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_opaque_two_twin_rust_async_sse() -> OpaqueTwoTwinRustAsyncSse {
    unimplemented!()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_borrow_and_mut_borrow_twin_rust_async_sse(
    borrow: &NonCloneSimpleTwinRustAsyncSse,
    mut_borrow: &mut NonCloneSimpleTwinRustAsyncSse,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_borrow_and_borrow_twin_rust_async_sse(
    a: &NonCloneSimpleTwinRustAsyncSse,
    b: &NonCloneSimpleTwinRustAsyncSse,
) -> i32 {
    a.inner + b.inner
}
