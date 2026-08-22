// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

#[allow(unused_imports)]
use crate::frb_generated::RustAutoOpaque;
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinRustAsync {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinRustAsync {
    Apple,
    Orange,
}

// ==================================== simple =======================================

pub async fn rust_auto_opaque_arg_own_twin_rust_async(
    arg: NonCloneSimpleTwinRustAsync,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

pub async fn rust_auto_opaque_arg_borrow_twin_rust_async(
    arg: &NonCloneSimpleTwinRustAsync,
    expect: i32,
) {
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

pub async fn rust_auto_opaque_return_own_twin_rust_async(
    initial: i32,
) -> NonCloneSimpleTwinRustAsync {
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

pub async fn rust_auto_opaque_normal_and_opaque_arg_twin_rust_async(
    a: NonCloneSimpleTwinRustAsync,
    b: String,
) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinRustAsync {
    fn f(&self) -> &str;
}
impl MyTraitTwinRustAsync for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
pub async fn rust_auto_opaque_plus_sign_arg_twin_rust_async(
    arg: Box<dyn MyTraitTwinRustAsync + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

pub async fn rust_auto_opaque_plus_sign_return_twin_rust_async(
) -> Box<dyn MyTraitTwinRustAsync + Send + Sync> {
    Box::new("hello".to_owned())
}

pub async fn rust_auto_opaque_callable_arg_twin_rust_async(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

pub async fn rust_auto_opaque_callable_return_twin_rust_async(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinRustAsync: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinRustAsync {
//     inner: String,
// }
//
// impl HelloTraitTwinRustAsync for HelloOneStructTwinRustAsync {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinRustAsync {
//     A,
//     B,
// }
//
// impl HelloTraitTwinRustAsync for HelloTwoEnumTwinRustAsync {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinRustAsync::A => "A",
//             HelloTwoEnumTwinRustAsync::B => "B",
//         }
//     }
// }
//
// pub async fn rust_auto_opaque_trait_object_arg_own_twin_rust_async(
//     arg: Box<dyn HelloTraitTwinRustAsync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// pub async fn rust_auto_opaque_trait_object_arg_borrow_twin_rust_async(
//     arg: &Box<dyn HelloTraitTwinRustAsync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub async fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_rust_async(
//     arg: &mut Box<dyn HelloTraitTwinRustAsync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub async fn rust_auto_opaque_trait_object_return_own_one_twin_rust_async() -> Box<dyn HelloTraitTwinRustAsync> {
//     Box::new(HelloOneStructTwinRustAsync {
//         inner: "hello".into(),
//     })
// }
//
// pub async fn rust_auto_opaque_trait_object_return_own_two_twin_rust_async() -> Box<dyn HelloTraitTwinRustAsync> {
//     Box::new(HelloTwoEnumTwinRustAsync::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinRustAsync {
    pub async fn static_method_arg_own_twin_rust_async(arg: NonCloneSimpleTwinRustAsync) {
        assert_eq!(arg.inner, 42);
    }

    pub async fn static_method_arg_borrow_twin_rust_async(arg: &NonCloneSimpleTwinRustAsync) {
        assert_eq!(arg.inner, 42);
    }

    pub async fn static_method_arg_mut_borrow_twin_rust_async(
        arg: &mut NonCloneSimpleTwinRustAsync,
    ) {
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

    /// constructor with Result
    pub async fn new_with_result_twin_rust_async() -> anyhow::Result<NonCloneSimpleTwinRustAsync> {
        Ok(Self { inner: 42 })
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

    #[frb(getter)]
    pub async fn instance_method_getter_twin_rust_async(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinRustAsync {
    pub good: String,
    pub opaque: NonCloneSimpleTwinRustAsync,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinRustAsync>,
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_rust_async(
    arg: StructWithGoodAndOpaqueFieldTwinRustAsync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

pub async fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_rust_async(
) -> StructWithGoodAndOpaqueFieldTwinRustAsync {
    StructWithGoodAndOpaqueFieldTwinRustAsync {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinRustAsync { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinRustAsync { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinRustAsync {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsync),
}

pub async fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_rust_async(
    arg: EnumWithGoodAndOpaqueTwinRustAsync,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinRustAsync::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinRustAsync::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_rust_async(
) -> EnumWithGoodAndOpaqueTwinRustAsync {
    EnumWithGoodAndOpaqueTwinRustAsync::Good("hello".to_owned())
}

pub async fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_rust_async(
) -> EnumWithGoodAndOpaqueTwinRustAsync {
    EnumWithGoodAndOpaqueTwinRustAsync::Opaque(NonCloneSimpleTwinRustAsync { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsync {
    pub good: String,
    opaque: NonCloneSimpleTwinRustAsync,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinRustAsync {
    Good(String),
    Opaque(NonCloneSimpleTwinRustAsync),
}

#[allow(unused_variables)]
pub async fn rust_auto_opaque_dummy_twin_rust_async(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinRustAsync,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinRustAsync,
) {
}

// ================ enum opaque type ===================

pub async fn rust_auto_opaque_enum_arg_borrow_twin_rust_async(
    arg: &NonCloneSimpleEnumTwinRustAsync,
) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinRustAsync::Orange));
}

pub async fn rust_auto_opaque_enum_return_own_twin_rust_async() -> NonCloneSimpleEnumTwinRustAsync {
    NonCloneSimpleEnumTwinRustAsync::Orange
}

// ================ stream sink ===================

pub async fn rust_auto_opaque_stream_sink_twin_rust_async(
    sink: StreamSink<NonCloneSimpleTwinRustAsync>,
) {
    sink.add(NonCloneSimpleTwinRustAsync { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

pub async fn rust_auto_opaque_arg_vec_own_twin_rust_async(
    arg: Vec<NonCloneSimpleTwinRustAsync>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

pub async fn rust_auto_opaque_return_vec_own_twin_rust_async() -> Vec<NonCloneSimpleTwinRustAsync> {
    vec![
        NonCloneSimpleTwinRustAsync { inner: 10 },
        NonCloneSimpleTwinRustAsync { inner: 20 },
    ]
}

// ================ use explicit type ===================

pub async fn rust_auto_opaque_explicit_arg_twin_rust_async(
    arg: RustAutoOpaque<NonCloneSimpleTwinRustAsync>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinRustAsync {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinRustAsync>,
    pub normal: i32,
}

pub async fn rust_auto_opaque_explicit_struct_twin_rust_async(
    arg: StructWithExplicitAutoOpaqueFieldTwinRustAsync,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

pub async fn rust_auto_opaque_explicit_return_struct_twin_rust_async(
) -> StructWithExplicitAutoOpaqueFieldTwinRustAsync {
    StructWithExplicitAutoOpaqueFieldTwinRustAsync {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinRustAsync { inner: 100 }),
    }
}

pub async fn rust_auto_opaque_explicit_return_twin_rust_async(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinRustAsync> {
    RustAutoOpaque::new(NonCloneSimpleTwinRustAsync { inner: initial })
}

// ================ deadlock detection ===================

pub async fn rust_auto_opaque_sleep_twin_rust_async(
    apple: &mut NonCloneSimpleTwinRustAsync,
    orange: &mut NonCloneSimpleTwinRustAsync,
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
pub struct OpaqueOneTwinRustAsync(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinRustAsync(PathBuf);

pub async fn rust_auto_opaque_return_opaque_one_and_two_twin_rust_async(
) -> (OpaqueOneTwinRustAsync, OpaqueTwoTwinRustAsync) {
    unimplemented!()
}
pub async fn rust_auto_opaque_return_opaque_two_twin_rust_async() -> OpaqueTwoTwinRustAsync {
    unimplemented!()
}

pub async fn rust_auto_opaque_borrow_and_mut_borrow_twin_rust_async(
    borrow: &NonCloneSimpleTwinRustAsync,
    mut_borrow: &mut NonCloneSimpleTwinRustAsync,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

pub async fn rust_auto_opaque_borrow_and_borrow_twin_rust_async(
    a: &NonCloneSimpleTwinRustAsync,
    b: &NonCloneSimpleTwinRustAsync,
) -> i32 {
    a.inner + b.inner
}
