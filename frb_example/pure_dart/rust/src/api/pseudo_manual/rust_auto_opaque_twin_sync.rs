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
pub struct NonCloneSimpleTwinSync {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinSync {
    Apple,
    Orange,
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
//
// pub trait HelloTraitTwinSync: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinSync {
//     inner: String,
// }
//
// impl HelloTraitTwinSync for HelloOneStructTwinSync {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinSync {
//     A,
//     B,
// }
//
// impl HelloTraitTwinSync for HelloTwoEnumTwinSync {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinSync::A => "A",
//             HelloTwoEnumTwinSync::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_own_twin_sync(
//     arg: Box<dyn HelloTraitTwinSync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sync(
//     arg: &Box<dyn HelloTraitTwinSync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync(
//     arg: &mut Box<dyn HelloTraitTwinSync>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_one_twin_sync() -> Box<dyn HelloTraitTwinSync> {
//     Box::new(HelloOneStructTwinSync {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_two_twin_sync() -> Box<dyn HelloTraitTwinSync> {
//     Box::new(HelloTwoEnumTwinSync::B)
// }
//

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

    #[frb(getter)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_getter_twin_sync(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSync {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSync,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinSync>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync(
    arg: StructWithGoodAndOpaqueFieldTwinSync,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync(
) -> StructWithGoodAndOpaqueFieldTwinSync {
    StructWithGoodAndOpaqueFieldTwinSync {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSync { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinSync { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinSync {
    Good(String),
    Opaque(NonCloneSimpleTwinSync),
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_sync(
    arg: EnumWithGoodAndOpaqueTwinSync,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinSync::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinSync::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_sync(
) -> EnumWithGoodAndOpaqueTwinSync {
    EnumWithGoodAndOpaqueTwinSync::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_sync(
) -> EnumWithGoodAndOpaqueTwinSync {
    EnumWithGoodAndOpaqueTwinSync::Opaque(NonCloneSimpleTwinSync { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinSync {
    pub good: String,
    opaque: NonCloneSimpleTwinSync,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinSync {
    Good(String),
    Opaque(NonCloneSimpleTwinSync),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_dummy_twin_sync(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinSync,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinSync,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_arg_borrow_twin_sync(arg: &NonCloneSimpleEnumTwinSync) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinSync::Orange));
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_return_own_twin_sync() -> NonCloneSimpleEnumTwinSync {
    NonCloneSimpleEnumTwinSync::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_stream_sink_twin_sync(sink: StreamSink<NonCloneSimpleTwinSync>) {
    sink.add(NonCloneSimpleTwinSync { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_vec_own_twin_sync(arg: Vec<NonCloneSimpleTwinSync>, expect: Vec<i32>) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_vec_own_twin_sync() -> Vec<NonCloneSimpleTwinSync> {
    vec![
        NonCloneSimpleTwinSync { inner: 10 },
        NonCloneSimpleTwinSync { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_arg_twin_sync(
    arg: RustAutoOpaque<NonCloneSimpleTwinSync>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinSync {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinSync>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_struct_twin_sync(arg: StructWithExplicitAutoOpaqueFieldTwinSync) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_struct_twin_sync(
) -> StructWithExplicitAutoOpaqueFieldTwinSync {
    StructWithExplicitAutoOpaqueFieldTwinSync {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinSync { inner: 100 }),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_twin_sync(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinSync> {
    RustAutoOpaque::new(NonCloneSimpleTwinSync { inner: initial })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_sleep_twin_sync(
    apple: &mut NonCloneSimpleTwinSync,
    orange: &mut NonCloneSimpleTwinSync,
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
pub struct OpaqueOneTwinSync(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinSync(PathBuf);

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_sync(
) -> (OpaqueOneTwinSync, OpaqueTwoTwinSync) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_two_twin_sync() -> OpaqueTwoTwinSync {
    unimplemented!()
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_sync(
    borrow: &NonCloneSimpleTwinSync,
    mut_borrow: &mut NonCloneSimpleTwinSync,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_sync(
    a: &NonCloneSimpleTwinSync,
    b: &NonCloneSimpleTwinSync,
) -> i32 {
    a.inner + b.inner
}
