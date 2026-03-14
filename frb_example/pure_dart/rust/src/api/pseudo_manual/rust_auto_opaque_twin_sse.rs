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
pub struct NonCloneSimpleTwinSse {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinSse {
    Apple,
    Orange,
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

pub trait MyTraitTwinSse {
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
pub fn rust_auto_opaque_callable_arg_twin_sse(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_callable_return_twin_sse() -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinSse: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinSse {
//     inner: String,
// }
//
// impl HelloTraitTwinSse for HelloOneStructTwinSse {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinSse {
//     A,
//     B,
// }
//
// impl HelloTraitTwinSse for HelloTwoEnumTwinSse {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinSse::A => "A",
//             HelloTwoEnumTwinSse::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn rust_auto_opaque_trait_object_arg_own_twin_sse(
//     arg: Box<dyn HelloTraitTwinSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(serialize)] pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sse(
//     arg: &Box<dyn HelloTraitTwinSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sse(
//     arg: &mut Box<dyn HelloTraitTwinSse>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn rust_auto_opaque_trait_object_return_own_one_twin_sse() -> Box<dyn HelloTraitTwinSse> {
//     Box::new(HelloOneStructTwinSse {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(serialize)] pub fn rust_auto_opaque_trait_object_return_own_two_twin_sse() -> Box<dyn HelloTraitTwinSse> {
//     Box::new(HelloTwoEnumTwinSse::B)
// }
//

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

    /// constructor with Result
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_with_result_twin_sse() -> anyhow::Result<NonCloneSimpleTwinSse> {
        Ok(Self { inner: 42 })
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

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_getter_twin_sse(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSse {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSse,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sse(
    arg: StructWithGoodAndOpaqueFieldTwinSse,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sse(
) -> StructWithGoodAndOpaqueFieldTwinSse {
    StructWithGoodAndOpaqueFieldTwinSse {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSse { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinSse { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinSse {
    Good(String),
    Opaque(NonCloneSimpleTwinSse),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_sse(
    arg: EnumWithGoodAndOpaqueTwinSse,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinSse::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinSse::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_sse(
) -> EnumWithGoodAndOpaqueTwinSse {
    EnumWithGoodAndOpaqueTwinSse::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_sse(
) -> EnumWithGoodAndOpaqueTwinSse {
    EnumWithGoodAndOpaqueTwinSse::Opaque(NonCloneSimpleTwinSse { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinSse {
    pub good: String,
    opaque: NonCloneSimpleTwinSse,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinSse {
    Good(String),
    Opaque(NonCloneSimpleTwinSse),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_dummy_twin_sse(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinSse,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinSse,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_enum_arg_borrow_twin_sse(arg: &NonCloneSimpleEnumTwinSse) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinSse::Orange));
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_enum_return_own_twin_sse() -> NonCloneSimpleEnumTwinSse {
    NonCloneSimpleEnumTwinSse::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_stream_sink_twin_sse(
    sink: StreamSink<NonCloneSimpleTwinSse, flutter_rust_bridge::SseCodec>,
) {
    sink.add(NonCloneSimpleTwinSse { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_arg_vec_own_twin_sse(arg: Vec<NonCloneSimpleTwinSse>, expect: Vec<i32>) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_return_vec_own_twin_sse() -> Vec<NonCloneSimpleTwinSse> {
    vec![
        NonCloneSimpleTwinSse { inner: 10 },
        NonCloneSimpleTwinSse { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_explicit_arg_twin_sse(
    arg: RustAutoOpaque<NonCloneSimpleTwinSse>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinSse {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinSse>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_explicit_struct_twin_sse(arg: StructWithExplicitAutoOpaqueFieldTwinSse) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_explicit_return_struct_twin_sse() -> StructWithExplicitAutoOpaqueFieldTwinSse
{
    StructWithExplicitAutoOpaqueFieldTwinSse {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinSse { inner: 100 }),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_explicit_return_twin_sse(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinSse> {
    RustAutoOpaque::new(NonCloneSimpleTwinSse { inner: initial })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_sleep_twin_sse(
    apple: &mut NonCloneSimpleTwinSse,
    orange: &mut NonCloneSimpleTwinSse,
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
pub struct OpaqueOneTwinSse(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinSse(PathBuf);

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_sse() -> (OpaqueOneTwinSse, OpaqueTwoTwinSse)
{
    unimplemented!()
}
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_return_opaque_two_twin_sse() -> OpaqueTwoTwinSse {
    unimplemented!()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_sse(
    borrow: &NonCloneSimpleTwinSse,
    mut_borrow: &mut NonCloneSimpleTwinSse,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_sse(
    a: &NonCloneSimpleTwinSse,
    b: &NonCloneSimpleTwinSse,
) -> i32 {
    a.inner + b.inner
}
