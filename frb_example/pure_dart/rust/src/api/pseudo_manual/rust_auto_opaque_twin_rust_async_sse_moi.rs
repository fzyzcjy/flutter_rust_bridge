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
pub struct NonCloneSimpleTwinRustAsyncSseMoi {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinRustAsyncSseMoi {
    Apple,
    Orange,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_own_twin_rust_async_sse_moi(
    arg: NonCloneSimpleTwinRustAsyncSseMoi,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_borrow_twin_rust_async_sse_moi(
    arg: &NonCloneSimpleTwinRustAsyncSseMoi,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_mut_borrow_twin_rust_async_sse_moi(
    arg: &mut NonCloneSimpleTwinRustAsyncSseMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_own_twin_rust_async_sse_moi(
    initial: i32,
) -> NonCloneSimpleTwinRustAsyncSseMoi {
    NonCloneSimpleTwinRustAsyncSseMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_own_and_return_own_twin_rust_async_sse_moi(
    arg: NonCloneSimpleTwinRustAsyncSseMoi,
) -> NonCloneSimpleTwinRustAsyncSseMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_two_args_twin_rust_async_sse_moi(
    a: NonCloneSimpleTwinRustAsyncSseMoi,
    b: NonCloneSimpleTwinRustAsyncSseMoi,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async_sse_moi(
    a: NonCloneSimpleTwinRustAsyncSseMoi,
    b: String,
) {
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
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async_sse_moi(
    arg: Box<dyn MyTraitTwinRustAsyncSseMoi + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async_sse_moi(
) -> Box<dyn MyTraitTwinRustAsyncSseMoi + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_callable_arg_twin_rust_async_sse_moi(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_callable_return_twin_rust_async_sse_moi(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinRustAsyncSseMoi: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinRustAsyncSseMoi {
//     inner: String,
// }
//
// impl HelloTraitTwinRustAsyncSseMoi for HelloOneStructTwinRustAsyncSseMoi {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinRustAsyncSseMoi {
//     A,
//     B,
// }
//
// impl HelloTraitTwinRustAsyncSseMoi for HelloTwoEnumTwinRustAsyncSseMoi {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinRustAsyncSseMoi::A => "A",
//             HelloTwoEnumTwinRustAsyncSseMoi::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async_sse_moi(
//     arg: Box<dyn HelloTraitTwinRustAsyncSseMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async_sse_moi(
//     arg: &Box<dyn HelloTraitTwinRustAsyncSseMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async_sse_moi(
//     arg: &mut Box<dyn HelloTraitTwinRustAsyncSseMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async_sse_moi() -> Box<dyn HelloTraitTwinRustAsyncSseMoi> {
//     Box::new(HelloOneStructTwinRustAsyncSseMoi {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(serialize)] pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async_sse_moi() -> Box<dyn HelloTraitTwinRustAsyncSseMoi> {
//     Box::new(HelloTwoEnumTwinRustAsyncSseMoi::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsyncSseMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_own_twin_rust_async_sse_moi(
        arg: NonCloneSimpleTwinRustAsyncSseMoi,
    ) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_borrow_twin_rust_async_sse_moi(
        arg: &NonCloneSimpleTwinRustAsyncSseMoi,
    ) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_arg_mut_borrow_twin_rust_async_sse_moi(
        arg: &mut NonCloneSimpleTwinRustAsyncSseMoi,
    ) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_return_own_twin_rust_async_sse_moi(
    ) -> NonCloneSimpleTwinRustAsyncSseMoi {
        NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinRustAsyncSseMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse_moi() -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_custom_name_twin_rust_async_sse_moi() -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_with_result_twin_rust_async_sse_moi(
    ) -> anyhow::Result<NonCloneSimpleTwinRustAsyncSseMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_own_twin_rust_async_sse_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_borrow_twin_rust_async_sse_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_arg_mut_borrow_twin_rust_async_sse_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_return_own_twin_rust_async_sse_moi(
        &self,
    ) -> NonCloneSimpleTwinRustAsyncSseMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn instance_method_getter_twin_rust_async_sse_moi(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsyncSseMoi,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinRustAsyncSseMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async_sse_moi(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async_sse_moi(
) -> StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
    StructWithGoodAndOpaqueFieldTwinRustAsyncSseMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsyncSseMoi),
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_rust_async_sse_moi(
    arg: EnumWithGoodAndOpaqueTwinRustAsyncSseMoi,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinRustAsyncSseMoi::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinRustAsyncSseMoi::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_rust_async_sse_moi(
) -> EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
    EnumWithGoodAndOpaqueTwinRustAsyncSseMoi::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_rust_async_sse_moi(
) -> EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
    EnumWithGoodAndOpaqueTwinRustAsyncSseMoi::Opaque(NonCloneSimpleTwinRustAsyncSseMoi {
        inner: 42,
    })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncSseMoi {
    pub good: String,
    opaque: NonCloneSimpleTwinRustAsyncSseMoi,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncSseMoi {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsyncSseMoi),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_dummy_twin_rust_async_sse_moi(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsyncSseMoi,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinRustAsyncSseMoi,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_arg_borrow_twin_rust_async_sse_moi(
    arg: &NonCloneSimpleEnumTwinRustAsyncSseMoi,
) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinRustAsyncSseMoi::Orange));
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_enum_return_own_twin_rust_async_sse_moi(
) -> NonCloneSimpleEnumTwinRustAsyncSseMoi {
    NonCloneSimpleEnumTwinRustAsyncSseMoi::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_stream_sink_twin_rust_async_sse_moi(
    sink: StreamSink<NonCloneSimpleTwinRustAsyncSseMoi, flutter_rust_bridge::SseCodec>,
) {
    sink.add(NonCloneSimpleTwinRustAsyncSseMoi { inner: 42 })
        .unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_arg_vec_own_twin_rust_async_sse_moi(
    arg: Vec<NonCloneSimpleTwinRustAsyncSseMoi>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_vec_own_twin_rust_async_sse_moi(
) -> Vec<NonCloneSimpleTwinRustAsyncSseMoi> {
    vec![
        NonCloneSimpleTwinRustAsyncSseMoi { inner: 10 },
        NonCloneSimpleTwinRustAsyncSseMoi { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_arg_twin_rust_async_sse_moi(
    arg: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinRustAsyncSseMoi>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinRustAsyncSseMoi {
    pub auto_opaque: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinRustAsyncSseMoi>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_struct_twin_rust_async_sse_moi(
    arg: StructWithExplicitAutoOpaqueFieldTwinRustAsyncSseMoi,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_return_struct_twin_rust_async_sse_moi(
) -> StructWithExplicitAutoOpaqueFieldTwinRustAsyncSseMoi {
    StructWithExplicitAutoOpaqueFieldTwinRustAsyncSseMoi {
        normal: 100,
        auto_opaque: crate::frb_generated::RustAutoOpaqueMoi::new(
            NonCloneSimpleTwinRustAsyncSseMoi { inner: 100 },
        ),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_explicit_return_twin_rust_async_sse_moi(
    initial: i32,
) -> crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinRustAsyncSseMoi> {
    crate::frb_generated::RustAutoOpaqueMoi::new(NonCloneSimpleTwinRustAsyncSseMoi {
        inner: initial,
    })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_sleep_twin_rust_async_sse_moi(
    apple: &mut NonCloneSimpleTwinRustAsyncSseMoi,
    orange: &mut NonCloneSimpleTwinRustAsyncSseMoi,
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
pub struct OpaqueOneTwinRustAsyncSseMoi(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinRustAsyncSseMoi(PathBuf);

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_opaque_one_and_two_twin_rust_async_sse_moi(
) -> (OpaqueOneTwinRustAsyncSseMoi, OpaqueTwoTwinRustAsyncSseMoi) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_return_opaque_two_twin_rust_async_sse_moi(
) -> OpaqueTwoTwinRustAsyncSseMoi {
    unimplemented!()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_borrow_and_mut_borrow_twin_rust_async_sse_moi(
    borrow: &NonCloneSimpleTwinRustAsyncSseMoi,
    mut_borrow: &mut NonCloneSimpleTwinRustAsyncSseMoi,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_borrow_and_borrow_twin_rust_async_sse_moi(
    a: &NonCloneSimpleTwinRustAsyncSseMoi,
    b: &NonCloneSimpleTwinRustAsyncSseMoi,
) -> i32 {
    a.inner + b.inner
}
