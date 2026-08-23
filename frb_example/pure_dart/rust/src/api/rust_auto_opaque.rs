// FRB_INTERNAL_GENERATOR: {"enableAll": true}

#[allow(unused_imports)]
use crate::frb_generated::RustAutoOpaque;
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use std::path::PathBuf;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinNormal {
    inner: i32,
}

#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub enum NonCloneSimpleEnumTwinNormal {
    Apple,
    Orange,
}

// ==================================== simple =======================================

pub fn rust_auto_opaque_arg_own_twin_normal(arg: NonCloneSimpleTwinNormal, expect: i32) {
    assert_eq!(arg.inner, expect);
}

pub fn rust_auto_opaque_arg_borrow_twin_normal(arg: &NonCloneSimpleTwinNormal, expect: i32) {
    assert_eq!(arg.inner, expect);
}

pub fn rust_auto_opaque_arg_mut_borrow_twin_normal(
    arg: &mut NonCloneSimpleTwinNormal,
    expect: i32,
    adder: i32,
) {
    assert_eq!(arg.inner, expect);
    arg.inner += adder;
}

pub fn rust_auto_opaque_return_own_twin_normal(initial: i32) -> NonCloneSimpleTwinNormal {
    NonCloneSimpleTwinNormal { inner: initial }
}

// ==================================== with other args =======================================

pub fn rust_auto_opaque_arg_own_and_return_own_twin_normal(
    arg: NonCloneSimpleTwinNormal,
) -> NonCloneSimpleTwinNormal {
    assert_eq!(arg.inner, 42);
    arg
}

pub fn rust_auto_opaque_two_args_twin_normal(
    a: NonCloneSimpleTwinNormal,
    b: NonCloneSimpleTwinNormal,
) {
    assert_eq!(a.inner, 10);
    assert_eq!(b.inner, 20);
}

pub fn rust_auto_opaque_normal_and_opaque_arg_twin_normal(a: NonCloneSimpleTwinNormal, b: String) {
    assert_eq!(a.inner, 42);
    assert_eq!(b, "hello");
}

// ==================================== complex type signatures =======================================

pub trait MyTraitTwinNormal {
    fn f(&self) -> &str;
}
impl MyTraitTwinNormal for String {
    fn f(&self) -> &str {
        self
    }
}

/// "+" inside the type signature
pub fn rust_auto_opaque_plus_sign_arg_twin_normal(arg: Box<dyn MyTraitTwinNormal + Send + Sync>) {
    assert_eq!(arg.f(), "hello");
}

pub fn rust_auto_opaque_plus_sign_return_twin_normal() -> Box<dyn MyTraitTwinNormal + Send + Sync> {
    Box::new("hello".to_owned())
}

pub fn rust_auto_opaque_callable_arg_twin_normal(arg: Box<dyn Fn(String) -> String + Send + Sync>) {
    assert_eq!(&arg("hello".into()), "hellohello");
}

pub fn rust_auto_opaque_callable_return_twin_normal() -> Box<dyn Fn(String) -> String + Send + Sync>
{
    Box::new(|x: String| x.repeat(2))
}

// ==================================== trait object =======================================
//
// pub trait HelloTraitTwinNormal: Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
// #[frb(opaque)]
// pub struct HelloOneStructTwinNormal {
//     inner: String,
// }
//
// impl HelloTraitTwinNormal for HelloOneStructTwinNormal {
//     fn func_hello(&self) -> &str {
//         &self.inner
//     }
// }
//
// pub enum HelloTwoEnumTwinNormal {
//     A,
//     B,
// }
//
// impl HelloTraitTwinNormal for HelloTwoEnumTwinNormal {
//     fn func_hello(&self) -> &str {
//         match self {
//             HelloTwoEnumTwinNormal::A => "A",
//             HelloTwoEnumTwinNormal::B => "B",
//         }
//     }
// }
//
// pub fn rust_auto_opaque_trait_object_arg_own_twin_normal(
//     arg: Box<dyn HelloTraitTwinNormal>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// #[allow(clippy::borrowed_box)]
// pub fn rust_auto_opaque_trait_object_arg_borrow_twin_normal(
//     arg: &Box<dyn HelloTraitTwinNormal>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub fn rust_auto_opaque_trait_object_arg_mut_borrow_twin_normal(
//     arg: &mut Box<dyn HelloTraitTwinNormal>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub fn rust_auto_opaque_trait_object_return_own_one_twin_normal() -> Box<dyn HelloTraitTwinNormal> {
//     Box::new(HelloOneStructTwinNormal {
//         inner: "hello".into(),
//     })
// }
//
// pub fn rust_auto_opaque_trait_object_return_own_two_twin_normal() -> Box<dyn HelloTraitTwinNormal> {
//     Box::new(HelloTwoEnumTwinNormal::B)
// }
//

// ==================================== static method =======================================

impl NonCloneSimpleTwinNormal {
    pub fn static_method_arg_own_twin_normal(arg: NonCloneSimpleTwinNormal) {
        assert_eq!(arg.inner, 42);
    }

    pub fn static_method_arg_borrow_twin_normal(arg: &NonCloneSimpleTwinNormal) {
        assert_eq!(arg.inner, 42);
    }

    pub fn static_method_arg_mut_borrow_twin_normal(arg: &mut NonCloneSimpleTwinNormal) {
        assert_eq!(arg.inner, 42);
    }

    pub fn static_method_return_own_twin_normal() -> NonCloneSimpleTwinNormal {
        NonCloneSimpleTwinNormal { inner: 42 }
    }
}

// ==================================== instance method =======================================

impl NonCloneSimpleTwinNormal {
    /// unnamed constructor
    pub fn new_twin_normal() -> NonCloneSimpleTwinNormal {
        Self { inner: 42 }
    }

    /// named constructor
    pub fn new_custom_name_twin_normal() -> NonCloneSimpleTwinNormal {
        Self { inner: 42 }
    }

    /// constructor with Result
    pub fn new_with_result_twin_normal() -> anyhow::Result<NonCloneSimpleTwinNormal> {
        Ok(Self { inner: 42 })
    }

    pub fn instance_method_arg_own_twin_normal(self) {
        assert_eq!(self.inner, 42);
    }

    pub fn instance_method_arg_borrow_twin_normal(&self) {
        assert_eq!(self.inner, 42);
    }

    pub fn instance_method_arg_mut_borrow_twin_normal(&mut self) {
        assert_eq!(self.inner, 42);
    }

    pub fn instance_method_return_own_twin_normal(&self) -> NonCloneSimpleTwinNormal {
        Self { inner: 42 }
    }

    #[frb(getter)]
    pub fn instance_method_getter_twin_normal(&self) -> i32 {
        self.inner
    }
}

// ================ struct with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub struct StructWithGoodAndOpaqueFieldTwinNormal {
    pub good: String,
    pub opaque: NonCloneSimpleTwinNormal,
    // Reproduce https://github.com/fzyzcjy/flutter_rust_bridge/issues/1792#issuecomment-1972804379
    pub option_opaque: Option<NonCloneSimpleTwinNormal>,
}

pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_normal(
    arg: StructWithGoodAndOpaqueFieldTwinNormal,
) {
    assert_eq!(&arg.good, "hello");
    assert_eq!(arg.opaque.inner, 42);
    assert_eq!(arg.option_opaque.unwrap().inner, 42);
}

pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_normal(
) -> StructWithGoodAndOpaqueFieldTwinNormal {
    StructWithGoodAndOpaqueFieldTwinNormal {
        good: "hello".to_string(),
        opaque: NonCloneSimpleTwinNormal { inner: 42 },
        option_opaque: Some(NonCloneSimpleTwinNormal { inner: 42 }),
    }
}

// ================ enum with both encodable and opaque fields ===================

#[frb(non_opaque)]
pub enum EnumWithGoodAndOpaqueTwinNormal {
    Good(String),
    Opaque(NonCloneSimpleTwinNormal),
}

pub fn rust_auto_opaque_enum_with_good_and_opaque_arg_own_twin_normal(
    arg: EnumWithGoodAndOpaqueTwinNormal,
) {
    match arg {
        EnumWithGoodAndOpaqueTwinNormal::Good(inner) => assert_eq!(&inner, "hello"),
        EnumWithGoodAndOpaqueTwinNormal::Opaque(inner) => assert_eq!(inner.inner, 42),
    }
}

pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_good_twin_normal(
) -> EnumWithGoodAndOpaqueTwinNormal {
    EnumWithGoodAndOpaqueTwinNormal::Good("hello".to_owned())
}

pub fn rust_auto_opaque_enum_with_good_and_opaque_return_own_opaque_twin_normal(
) -> EnumWithGoodAndOpaqueTwinNormal {
    EnumWithGoodAndOpaqueTwinNormal::Opaque(NonCloneSimpleTwinNormal { inner: 42 })
}

// ================ struct/enum with both encodable and opaque fields, without non_opaque option ===================

#[allow(dead_code)]
pub struct StructWithGoodAndOpaqueFieldWithoutOptionTwinNormal {
    pub good: String,
    opaque: NonCloneSimpleTwinNormal,
}

pub enum EnumWithGoodAndOpaqueWithoutOptionTwinNormal {
    Good(String),
    Opaque(NonCloneSimpleTwinNormal),
}

#[allow(unused_variables)]
pub fn rust_auto_opaque_dummy_twin_normal(
    a: StructWithGoodAndOpaqueFieldWithoutOptionTwinNormal,
    b: EnumWithGoodAndOpaqueWithoutOptionTwinNormal,
) {
}

// ================ enum opaque type ===================

pub fn rust_auto_opaque_enum_arg_borrow_twin_normal(arg: &NonCloneSimpleEnumTwinNormal) {
    assert!(matches!(arg, NonCloneSimpleEnumTwinNormal::Orange));
}

pub fn rust_auto_opaque_enum_return_own_twin_normal() -> NonCloneSimpleEnumTwinNormal {
    NonCloneSimpleEnumTwinNormal::Orange
}

// ================ stream sink ===================

pub fn rust_auto_opaque_stream_sink_twin_normal(sink: StreamSink<NonCloneSimpleTwinNormal>) {
    sink.add(NonCloneSimpleTwinNormal { inner: 42 }).unwrap();
}

// ================ vec of opaque ===================

pub fn rust_auto_opaque_arg_vec_own_twin_normal(
    arg: Vec<NonCloneSimpleTwinNormal>,
    expect: Vec<i32>,
) {
    for i in 0..expect.len() {
        assert_eq!(arg[i].inner, expect[i]);
    }
}

pub fn rust_auto_opaque_return_vec_own_twin_normal() -> Vec<NonCloneSimpleTwinNormal> {
    vec![
        NonCloneSimpleTwinNormal { inner: 10 },
        NonCloneSimpleTwinNormal { inner: 20 },
    ]
}

// ================ use explicit type ===================

pub fn rust_auto_opaque_explicit_arg_twin_normal(
    arg: RustAutoOpaque<NonCloneSimpleTwinNormal>,
    expect: i32,
) {
    assert_eq!(arg.try_read().unwrap().inner, expect);
}

pub struct StructWithExplicitAutoOpaqueFieldTwinNormal {
    pub auto_opaque: RustAutoOpaque<NonCloneSimpleTwinNormal>,
    pub normal: i32,
}

pub fn rust_auto_opaque_explicit_struct_twin_normal(
    arg: StructWithExplicitAutoOpaqueFieldTwinNormal,
) {
    assert_eq!(arg.auto_opaque.try_read().unwrap().inner, arg.normal);
}

pub fn rust_auto_opaque_explicit_return_struct_twin_normal(
) -> StructWithExplicitAutoOpaqueFieldTwinNormal {
    StructWithExplicitAutoOpaqueFieldTwinNormal {
        normal: 100,
        auto_opaque: RustAutoOpaque::new(NonCloneSimpleTwinNormal { inner: 100 }),
    }
}

pub fn rust_auto_opaque_explicit_return_twin_normal(
    initial: i32,
) -> RustAutoOpaque<NonCloneSimpleTwinNormal> {
    RustAutoOpaque::new(NonCloneSimpleTwinNormal { inner: initial })
}

// ================ deadlock detection ===================

pub fn rust_auto_opaque_sleep_twin_normal(
    apple: &mut NonCloneSimpleTwinNormal,
    orange: &mut NonCloneSimpleTwinNormal,
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
pub struct OpaqueOneTwinNormal(PathBuf);
#[allow(dead_code)]
#[frb(opaque)]
pub struct OpaqueTwoTwinNormal(PathBuf);

pub fn rust_auto_opaque_return_opaque_one_and_two_twin_normal(
) -> (OpaqueOneTwinNormal, OpaqueTwoTwinNormal) {
    unimplemented!()
}
pub fn rust_auto_opaque_return_opaque_two_twin_normal() -> OpaqueTwoTwinNormal {
    unimplemented!()
}

pub fn rust_auto_opaque_borrow_and_mut_borrow_twin_normal(
    borrow: &NonCloneSimpleTwinNormal,
    mut_borrow: &mut NonCloneSimpleTwinNormal,
) -> i32 {
    borrow.inner + mut_borrow.inner
}

pub fn rust_auto_opaque_borrow_and_borrow_twin_normal(
    a: &NonCloneSimpleTwinNormal,
    b: &NonCloneSimpleTwinNormal,
) -> i32 {
    a.inner + b.inner
}
