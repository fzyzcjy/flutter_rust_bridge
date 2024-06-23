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
pub struct NonCloneSimpleTwinSyncMoi {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinSyncMoi {
    Apple,
    Orange,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_twin_sync_moi(arg: NonCloneSimpleTwinSyncMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_borrow_twin_sync_moi(arg: &NonCloneSimpleTwinSyncMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_sync_moi(
    arg: &mut NonCloneSimpleTwinSyncMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_own_twin_sync_moi(initial: i32) -> NonCloneSimpleTwinSyncMoi {
    NonCloneSimpleTwinSyncMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_sync_moi(
    arg: NonCloneSimpleTwinSyncMoi,
) -> NonCloneSimpleTwinSyncMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_two_args_twin_sync_moi(
    a: NonCloneSimpleTwinSyncMoi,
    b: NonCloneSimpleTwinSyncMoi,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_sync_moi(
    a: NonCloneSimpleTwinSyncMoi,
    b: String,
) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinSyncMoi {
    fn f(&self) -> &str;
}
impl MyTraitTwinSyncMoi for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_arg_twin_sync_moi(
    arg: Box<dyn MyTraitTwinSyncMoi + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_return_twin_sync_moi() -> Box<dyn MyTraitTwinSyncMoi + Send + Sync>
{
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_arg_twin_sync_moi(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_return_twin_sync_moi(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinSyncMoi: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinSyncMoi {
//     inner: String,
// }
//
// impl HelloTraitTwinSyncMoi for HelloOneStructTwinSyncMoi {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinSyncMoi {
//     A,
//     B,
// }
//
// impl HelloTraitTwinSyncMoi for HelloTwoEnumTwinSyncMoi {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinSyncMoi::A => "A",
//             HelloTwoEnumTwinSyncMoi::B => "B",
//         }
//     }
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_own_twin_sync_moi(
//     arg: Box<dyn HelloTraitTwinSyncMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sync_moi(
//     arg: &Box<dyn HelloTraitTwinSyncMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync_moi(
//     arg: &mut Box<dyn HelloTraitTwinSyncMoi>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_one_twin_sync_moi() -> Box<dyn HelloTraitTwinSyncMoi> {
//     Box::new(HelloOneStructTwinSyncMoi {
//         inner: "hello".into(),
//     })
// }
//
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn rust_auto_opaque_trait_object_return_own_two_twin_sync_moi() -> Box<dyn HelloTraitTwinSyncMoi> {
//     Box::new(HelloTwoEnumTwinSyncMoi::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinSyncMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_own_twin_sync_moi(arg: NonCloneSimpleTwinSyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_borrow_twin_sync_moi(arg: &NonCloneSimpleTwinSyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_mut_borrow_twin_sync_moi(arg: &mut NonCloneSimpleTwinSyncMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_return_own_twin_sync_moi() -> NonCloneSimpleTwinSyncMoi {
        NonCloneSimpleTwinSyncMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinSyncMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_moi() -> NonCloneSimpleTwinSyncMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_custom_name_twin_sync_moi() -> NonCloneSimpleTwinSyncMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_with_result_twin_sync_moi() -> anyhow::Result<NonCloneSimpleTwinSyncMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_own_twin_sync_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_borrow_twin_sync_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_mut_borrow_twin_sync_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_return_own_twin_sync_moi(&self) -> NonCloneSimpleTwinSyncMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_getter_twin_sync_moi(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinSyncMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSyncMoi,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinSyncMoi>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync_moi(
    arg: StructWithGoodAndOpaqueFieldTwinSyncMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync_moi(
) -> StructWithGoodAndOpaqueFieldTwinSyncMoi {
    StructWithGoodAndOpaqueFieldTwinSyncMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSyncMoi { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinSyncMoi { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinSyncMoi {
    Good(String),
    Opaque(NonCloneSimpleTwinSyncMoi),
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_sync_moi(
    arg: EnumWithGoodAndOpaqueTwinSyncMoi,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinSyncMoi::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinSyncMoi::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_sync_moi(
) -> EnumWithGoodAndOpaqueTwinSyncMoi {
    EnumWithGoodAndOpaqueTwinSyncMoi::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_sync_moi(
) -> EnumWithGoodAndOpaqueTwinSyncMoi {
    EnumWithGoodAndOpaqueTwinSyncMoi::Opaque(NonCloneSimpleTwinSyncMoi { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinSyncMoi {
    pub good: String,
    opaque: NonCloneSimpleTwinSyncMoi,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinSyncMoi {
    Good(String),
    Opaque(NonCloneSimpleTwinSyncMoi),
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_dummy_twin_sync_moi(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinSyncMoi,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinSyncMoi,
) {
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_arg_borrow_twin_sync_moi(arg: &NonCloneSimpleEnumTwinSyncMoi) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinSyncMoi::Orange));
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_return_own_twin_sync_moi() -> NonCloneSimpleEnumTwinSyncMoi {
    NonCloneSimpleEnumTwinSyncMoi::Orange
}

// ================ stream sink ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_stream_sink_twin_sync_moi(sink: StreamSink<NonCloneSimpleTwinSyncMoi>) {
    sink.add(NonCloneSimpleTwinSyncMoi { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_vec_own_twin_sync_moi(
    arg: Vec<NonCloneSimpleTwinSyncMoi>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_vec_own_twin_sync_moi() -> Vec<NonCloneSimpleTwinSyncMoi> {
    vec![
        NonCloneSimpleTwinSyncMoi { inner: 10 },
        NonCloneSimpleTwinSyncMoi { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_arg_twin_sync_moi(
    arg: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncMoi>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinSyncMoi {
    pub auto_opaque: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncMoi>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_struct_twin_sync_moi(
    arg: StructWithExplicitAutoOpaqueFieldTwinSyncMoi,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_struct_twin_sync_moi(
) -> StructWithExplicitAutoOpaqueFieldTwinSyncMoi {
    StructWithExplicitAutoOpaqueFieldTwinSyncMoi {
        normal: 100,
        auto_opaque: crate::frb_generated::RustAutoOpaqueMoi::new(NonCloneSimpleTwinSyncMoi {
            inner: 100,
        }),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_twin_sync_moi(
    initial: i32,
) -> crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncMoi> {
    crate::frb_generated::RustAutoOpaqueMoi::new(NonCloneSimpleTwinSyncMoi { inner: initial })
}

// ================ deadlock detection ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_sleep_twin_sync_moi(
    apple: &mut NonCloneSimpleTwinSyncMoi,
    orange: &mut NonCloneSimpleTwinSyncMoi,
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
pub struct OpaqueOneTwinSyncMoi(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinSyncMoi(PathBuf);

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_sync_moi(
) -> (OpaqueOneTwinSyncMoi, OpaqueTwoTwinSyncMoi) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_two_twin_sync_moi() -> OpaqueTwoTwinSyncMoi {
    unimplemented!()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_sync_moi(
    borrow: &NonCloneSimpleTwinSyncMoi,
    mut_borrow: &mut NonCloneSimpleTwinSyncMoi,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_sync_moi(
    a: &NonCloneSimpleTwinSyncMoi,
    b: &NonCloneSimpleTwinSyncMoi,
) -> i32 {
    a.inner + b.inner
}
