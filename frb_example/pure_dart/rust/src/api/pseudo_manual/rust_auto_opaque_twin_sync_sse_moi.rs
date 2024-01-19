// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

#[allow(unused_imports)]
use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use flutter_rust_bridge::rust_async::RwLock;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinSyncSseMoi {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinSyncSseMoi {
    Apple,
    Orange,
}

// ==================================== simple =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_twin_sync_sse_moi(arg: NonCloneSimpleTwinSyncSseMoi, expect: i32) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_borrow_twin_sync_sse_moi(
    arg: &NonCloneSimpleTwinSyncSseMoi,
    expect: i32,
) {
    assert_eq!(arg.inner, expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_mut_borrow_twin_sync_sse_moi(
    arg: &mut NonCloneSimpleTwinSyncSseMoi,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_own_twin_sync_sse_moi(initial: i32) -> NonCloneSimpleTwinSyncSseMoi {
    NonCloneSimpleTwinSyncSseMoi { inner: initial }
}

// ==================================== with other args =======================================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_own_and_return_own_twin_sync_sse_moi(
    arg: NonCloneSimpleTwinSyncSseMoi,
) -> NonCloneSimpleTwinSyncSseMoi {
    assert_eq!(arg.inner, 42);
    arg
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_two_args_twin_sync_sse_moi(
    a: NonCloneSimpleTwinSyncSseMoi,
    b: NonCloneSimpleTwinSyncSseMoi,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_normal_and_opaque_arg_twin_sync_sse_moi(
    a: NonCloneSimpleTwinSyncSseMoi,
    b: String,
) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinSyncSseMoi {
    fn f(&self) -> &str;
}
impl MyTraitTwinSyncSseMoi for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_arg_twin_sync_sse_moi(
    arg: Box<dyn MyTraitTwinSyncSseMoi + Send + Sync>,
) {
    assert_eq!(arg.f(), "hello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_plus_sign_return_twin_sync_sse_moi(
) -> Box<dyn MyTraitTwinSyncSseMoi + Send + Sync> {
    Box::new("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_arg_twin_sync_sse_moi(
    arg: Box<dyn Fn(String) -> String + Send + Sync>,
) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_callable_return_twin_sync_sse_moi(
) -> Box<dyn Fn(String) -> String + Send + Sync> {
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================

pub trait HelloTraitTwinSyncSseMoi: Send + Sync {
    fn func_hello(&self) -> &str;
}

pub struct HelloOneStructTwinSyncSseMoi {
    inner: String,
}

impl HelloTraitTwinSyncSseMoi for HelloOneStructTwinSyncSseMoi {
    fn func_hello(&self) -> &str {
        &self.inner
    }
}

pub enum HelloTwoEnumTwinSyncSseMoi {
    A,
    B,
}

impl HelloTraitTwinSyncSseMoi for HelloTwoEnumTwinSyncSseMoi {
    fn func_hello(&self) -> &str {
        match self {
            HelloTwoEnumTwinSyncSseMoi::A => "A",
            HelloTwoEnumTwinSyncSseMoi::B => "B",
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_own_twin_sync_sse_moi(
    arg: Box<dyn HelloTraitTwinSyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[allow(clippy::borrowed_box)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_borrow_twin_sync_sse_moi(
    arg: &Box<dyn HelloTraitTwinSyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync_sse_moi(
    arg: &mut Box<dyn HelloTraitTwinSyncSseMoi>,
    expect: String,
) {
    assert_eq!(arg.func_hello(), expect);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_return_own_one_twin_sync_sse_moi(
) -> Box<dyn HelloTraitTwinSyncSseMoi> {
    Box::new(HelloOneStructTwinSyncSseMoi {
        inner: "hello".into(),
    })
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_trait_object_return_own_two_twin_sync_sse_moi(
) -> Box<dyn HelloTraitTwinSyncSseMoi> {
    Box::new(HelloTwoEnumTwinSyncSseMoi::B)
}

// ==================================== static method =======================================

impl NonCloneSimpleTwinSyncSseMoi {
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_own_twin_sync_sse_moi(arg: NonCloneSimpleTwinSyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_borrow_twin_sync_sse_moi(arg: &NonCloneSimpleTwinSyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_arg_mut_borrow_twin_sync_sse_moi(arg: &mut NonCloneSimpleTwinSyncSseMoi) {
        assert_eq!(arg.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_return_own_twin_sync_sse_moi() -> NonCloneSimpleTwinSyncSseMoi {
        NonCloneSimpleTwinSyncSseMoi { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinSyncSseMoi {
    /// unnamed constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_sse_moi() -> NonCloneSimpleTwinSyncSseMoi {
        Self { inner: 42 }
    }

    /// named constructor
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_custom_name_twin_sync_sse_moi() -> NonCloneSimpleTwinSyncSseMoi {
        Self { inner: 42 }
    }

    /// constructor with Result
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_with_result_twin_sync_sse_moi() -> anyhow::Result<NonCloneSimpleTwinSyncSseMoi> {
        Ok(Self { inner: 42 })
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_own_twin_sync_sse_moi(self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_borrow_twin_sync_sse_moi(&self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_arg_mut_borrow_twin_sync_sse_moi(&mut self) {
        assert_eq!(self.inner, 42);
    }

    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_return_own_twin_sync_sse_moi(&self) -> NonCloneSimpleTwinSyncSseMoi {
        Self { inner: 42 }
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn instance_method_getter_twin_sync_sse_moi(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

pub struct StructWithGoodAndOpaqueFieldTwinSyncSseMoi {
    pub good: String,
    pub opaque: NonCloneSimpleTwinSyncSseMoi,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync_sse_moi(
    arg: StructWithGoodAndOpaqueFieldTwinSyncSseMoi,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync_sse_moi(
) -> StructWithGoodAndOpaqueFieldTwinSyncSseMoi {
    StructWithGoodAndOpaqueFieldTwinSyncSseMoi {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinSyncSseMoi { inner: 42 },
    }
}

// ================ enum with both encodable and opaque fields ===================

pub enum EnumWithGoodAndOpaqueTwinSyncSseMoi {
    Good(String),
    Opaque(NonCloneSimpleTwinSyncSseMoi),
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_sync_sse_moi(
    arg: EnumWithGoodAndOpaqueTwinSyncSseMoi,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinSyncSseMoi::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinSyncSseMoi::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_sync_sse_moi(
) -> EnumWithGoodAndOpaqueTwinSyncSseMoi {
    EnumWithGoodAndOpaqueTwinSyncSseMoi::Good("hello".to_owned())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_sync_sse_moi(
) -> EnumWithGoodAndOpaqueTwinSyncSseMoi {
    EnumWithGoodAndOpaqueTwinSyncSseMoi::Opaque(NonCloneSimpleTwinSyncSseMoi { inner: 42 })
}

// ================ enum opaque type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_arg_borrow_twin_sync_sse_moi(arg: &NonCloneSimpleEnumTwinSyncSseMoi) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinSyncSseMoi::Orange));
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_enum_return_own_twin_sync_sse_moi() -> NonCloneSimpleEnumTwinSyncSseMoi {
    NonCloneSimpleEnumTwinSyncSseMoi::Orange
}

// ================ vec of opaque ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_arg_vec_own_twin_sync_sse_moi(
    arg: Vec<NonCloneSimpleTwinSyncSseMoi>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_vec_own_twin_sync_sse_moi() -> Vec<NonCloneSimpleTwinSyncSseMoi> {
    vec![
        NonCloneSimpleTwinSyncSseMoi { inner: 10 },
        NonCloneSimpleTwinSyncSseMoi { inner: 20 },
    ]
}

// ================ use explicit type ===================

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_arg_twin_sync_sse_moi(
    arg: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncSseMoi>,
    expect: i32,
) {
    assert_eq!((*arg).try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinSyncSseMoi {
    pub auto_opaque: crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncSseMoi>,
    pub normal: i32,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_struct_twin_sync_sse_moi(
    arg: StructWithExplicitAutoOpaqueFieldTwinSyncSseMoi,
) {
    assert_eq!((*arg.auto_opaque).try_read().unwrap().inner, arg.normal);
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_explicit_return_twin_sync_sse_moi(
    initial: i32,
) -> crate::frb_generated::RustAutoOpaqueMoi<NonCloneSimpleTwinSyncSseMoi> {
    crate::frb_generated::RustAutoOpaqueMoi::new(RwLock::new(NonCloneSimpleTwinSyncSseMoi {
        inner: initial,
    }))
}

// ================ misc ===================

// #1577 - this should generate valid Dart code without name collisions
#[frb(opaque)]
pub struct OpaqueOneTwinSyncSseMoi(PathBuf);
#[frb(opaque)]
pub struct OpaqueTwoTwinSyncSseMoi(PathBuf);

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_one_and_two_twin_sync_sse_moi(
) -> (OpaqueOneTwinSyncSseMoi, OpaqueTwoTwinSyncSseMoi) {
    unimplemented!()
}
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_return_opaque_two_twin_sync_sse_moi() -> OpaqueTwoTwinSyncSseMoi {
    unimplemented!()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_sync_sse_moi(
    borrow: &NonCloneSimpleTwinSyncSseMoi,
    mut_borrow: &mut NonCloneSimpleTwinSyncSseMoi,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_auto_opaque_borrow_and_borrow_twin_sync_sse_moi(
    a: &NonCloneSimpleTwinSyncSseMoi,
    b: &NonCloneSimpleTwinSyncSseMoi,
) -> i32 {
    a.inner + b.inner
}
