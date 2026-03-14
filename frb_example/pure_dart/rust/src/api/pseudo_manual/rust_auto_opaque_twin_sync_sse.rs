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
pub struct NonCloneSimpleTwinSyncSse {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinSyncSse {
    Apple,
    Orange,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_twin_sync_sse(arg: NonCloneSimpleTwinSyncSse, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_borrow_twin_sync_sse(arg: &NonCloneSimpleTwinSyncSse, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_sync_sse(
    arg: &mut NonCloneSimpleTwinSyncSse,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_own_twin_sync_sse(initial: i32) -> NonCloneSimpleTwinSyncSse {
    NonCloneSimpleTwinSyncSse { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_sync_sse(
    arg: NonCloneSimpleTwinSyncSse,
) -> NonCloneSimpleTwinSyncSse {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_two_args_twin_sync_sse(
    a: NonCloneSimpleTwinSyncSse,
    b: NonCloneSimpleTwinSyncSse,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_sync_sse(
    a: NonCloneSimpleTwinSyncSse,
    b: String,
) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinSyncSse {
    fn f(&self) -> &str;
}
impl MyTraitTwinSyncSse for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_arg_twin_sync_sse(
    arg: Box<dyn MyTraitTwinSyncSse + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_return_twin_sync_sse() -> Box<dyn MyTraitTwinSyncSse + Send + Sync>
{
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_arg_twin_sync_sse(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_return_twin_sync_sse(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinSyncSse: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinSyncSse {
//     inner: String,
// }
//
// impl HelloTraitTwinSyncSse for HelloOneStructTwinSyncSse {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinSyncSse {
//     A,
//     B,
// }
//
// impl HelloTraitTwinSyncSse for HelloTwoEnumTwinSyncSse {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinSyncSse::A => "A",
//             HelloTwoEnumTwinSyncSse::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_own_twin_sync_sse(
//     arg: Box<dyn HelloTraitTwinSyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sync_sse(
//     arg: &Box<dyn HelloTraitTwinSyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync_sse(
//     arg: &mut Box<dyn HelloTraitTwinSyncSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_one_twin_sync_sse() -> Box<dyn HelloTraitTwinSyncSse> {
//     Box::new(HelloOneStructTwinSyncSse {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(serialize)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_two_twin_sync_sse() -> Box<dyn HelloTraitTwinSyncSse> {
//     Box::new(HelloTwoEnumTwinSyncSse::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_own_twin_sync_sse(arg: NonCloneSimpleTwinSyncSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_borrow_twin_sync_sse(arg: &NonCloneSimpleTwinSyncSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_mut_borrow_twin_sync_sse(arg: &mut NonCloneSimpleTwinSyncSse) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_return_own_twin_sync_sse() -> NonCloneSimpleTwinSyncSse {
        NonCloneSimpleTwinSyncSse { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinSyncSse {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_sse() -> NonCloneSimpleTwinSyncSse {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_custom_name_twin_sync_sse() -> NonCloneSimpleTwinSyncSse {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_with_result_twin_sync_sse() -> anyhow::Result<NonCloneSimpleTwinSyncSse> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_own_twin_sync_sse(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_borrow_twin_sync_sse(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_mut_borrow_twin_sync_sse(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_return_own_twin_sync_sse(&self) -> NonCloneSimpleTwinSyncSse {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_getter_twin_sync_sse(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSyncSse {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSyncSse,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinSyncSse>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync_sse(
    arg: StructWithGoodAndOpaqueFieldTwinSyncSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync_sse(
) -> StructWithGoodAndOpaqueFieldTwinSyncSse {
    StructWithGoodAndOpaqueFieldTwinSyncSse {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSyncSse { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinSyncSse { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinSyncSse {
    Good(String),
    Opaque(NonCloneSimpleTwinSyncSse),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_sync_sse(
    arg: EnumWithGoodAndOpaqueTwinSyncSse,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinSyncSse::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinSyncSse::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_sync_sse(
) -> EnumWithGoodAndOpaqueTwinSyncSse {
    EnumWithGoodAndOpaqueTwinSyncSse::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_sync_sse(
) -> EnumWithGoodAndOpaqueTwinSyncSse {
    EnumWithGoodAndOpaqueTwinSyncSse::Opaque(NonCloneSimpleTwinSyncSse { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinSyncSse {
    pub good: String,
    opaque: NonCloneSimpleTwinSyncSse,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinSyncSse {
    Good(String),
    Opaque(NonCloneSimpleTwinSyncSse),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_dummy_twin_sync_sse(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinSyncSse,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinSyncSse,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_arg_borrow_twin_sync_sse(arg: &NonCloneSimpleEnumTwinSyncSse) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinSyncSse::Orange));
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_return_own_twin_sync_sse() -> NonCloneSimpleEnumTwinSyncSse {
    NonCloneSimpleEnumTwinSyncSse::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_stream_sink_twin_sync_sse(
    sink: StreamSink<NonCloneSimpleTwinSyncSse, flutter_rust_bridge::SseCodec>,
) {
    sink.add(NonCloneSimpleTwinSyncSse { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_vec_own_twin_sync_sse(
    arg: Vec<NonCloneSimpleTwinSyncSse>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_vec_own_twin_sync_sse() -> Vec<NonCloneSimpleTwinSyncSse> {
    vec![
        NonCloneSimpleTwinSyncSse { inner: 10 },
        NonCloneSimpleTwinSyncSse { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_arg_twin_sync_sse(
    arg: RustAutoOpaque<NonCloneSimpleTwinSyncSse>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinSyncSse {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinSyncSse>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_struct_twin_sync_sse(
    arg: StructWithExplicitAutoOpaqueFieldTwinSyncSse,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_struct_twin_sync_sse(
) -> StructWithExplicitAutoOpaqueFieldTwinSyncSse {
    StructWithExplicitAutoOpaqueFieldTwinSyncSse {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinSyncSse { inner: 100 }),
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_twin_sync_sse(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinSyncSse> {
    RustAutoOpaque::new(NonCloneSimpleTwinSyncSse { inner: initial })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_sleep_twin_sync_sse(
    apple: &mut NonCloneSimpleTwinSyncSse,
    orange: &mut NonCloneSimpleTwinSyncSse,
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
pub struct OpaqueOneTwinSyncSse(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinSyncSse(PathBuf);

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_sync_sse(
) -> (OpaqueOneTwinSyncSse, OpaqueTwoTwinSyncSse) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_two_twin_sync_sse() -> OpaqueTwoTwinSyncSse {
    unimplemented!()
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_sync_sse(
    borrow: &NonCloneSimpleTwinSyncSse,
    mut_borrow: &mut NonCloneSimpleTwinSyncSse,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_sync_sse(
    a: &NonCloneSimpleTwinSyncSse,
    b: &NonCloneSimpleTwinSyncSse,
) -> i32 {
    a.inner + b.inner
}
