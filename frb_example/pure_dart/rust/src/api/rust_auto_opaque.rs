use flutter_rust_bridge::{frb, DartSafe};
pub use std::panic::AssertUnwindSafe;

// TODO auto determine it is opaque or not later
#[frb(opaque)]
// Do *NOT* make it Clone or serializable
pub struct NonCloneSimpleTwinNormal {
    inner: i32,
}

// ==================================== simple =======================================
//
// pub fn rust_auto_opaque_arg_own(arg: NonCloneSimpleTwinNormal) {
//     assert_eq!(arg.inner, 42);
// }

pub fn rust_auto_opaque_arg_borrow(arg: &NonCloneSimpleTwinNormal) {
    assert_eq!(arg.inner, 42);
}

// pub fn rust_auto_opaque_arg_mut_borrow(arg: &mut NonCloneSimpleTwinNormal) {
//     assert_eq!(arg.inner, 42);
//     arg.inner += 1;
// }

pub fn rust_auto_opaque_return_own() -> NonCloneSimpleTwinNormal {
    NonCloneSimpleTwinNormal { inner: 42 }
}

// // ==================================== with other args =======================================
//
// pub fn rust_auto_opaque_arg_own_and_return_own(
//     arg: NonCloneSimpleTwinNormal,
// ) -> NonCloneSimpleTwinNormal {
//     assert_eq!(arg.inner, 42);
//     arg
// }
//
// pub fn rust_auto_opaque_two_args(a: NonCloneSimpleTwinNormal, b: NonCloneSimpleTwinNormal) {
//     assert_eq!(a.inner, 42);
//     assert_eq!(b.inner, 42);
// }
//
// pub fn rust_auto_opaque_normal_and_opaque_arg(a: NonCloneSimpleTwinNormal, b: String) {
//     assert_eq!(a.inner, 42);
//     assert_eq!(b, "hello");
// }
//
// // ==================================== complex type signatures =======================================
//
// pub trait MyTraitTwinNormal: DartSafe {
//     fn f(&self) -> &str;
// }
// impl MyTraitTwinNormal for String {
//     fn f(&self) -> &str {
//         self
//     }
// }
//
// /// "+" inside the type signature
// pub fn rust_auto_opaque_plus_sign_arg(arg: Box<dyn MyTraitTwinNormal + Send + Sync>) {
//     assert_eq!(arg.f(), "hello");
// }
//
// pub fn rust_auto_opaque_plus_sign_return() -> Box<dyn MyTraitTwinNormal + Send + Sync> {
//     Box::new("hello".to_owned())
// }
//
// pub fn rust_auto_opaque_callable_arg(arg: AssertUnwindSafe<Box<dyn Fn(String) -> String>>) {
//     assert_eq!(&arg("hello".into()), "hellohello");
// }
//
// pub fn rust_auto_opaque_callable_return() -> AssertUnwindSafe<Box<dyn Fn(String) -> String>> {
//     AssertUnwindSafe(Box::new(|x: String| x.repeat(2)))
// }
//
// // ==================================== trait object =======================================
//
// pub trait HelloTraitTwinNormal: DartSafe + Send + Sync {
//     fn func_hello(&self) -> &str;
// }
//
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
// pub fn rust_auto_opaque_trait_object_arg_own(arg: Box<dyn HelloTraitTwinNormal>, expect: String) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub fn rust_auto_opaque_trait_object_arg_borrow(
//     arg: &Box<dyn HelloTraitTwinNormal>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub fn rust_auto_opaque_trait_object_arg_mut_borrow(
//     arg: &mut Box<dyn HelloTraitTwinNormal>,
//     expect: String,
// ) {
//     assert_eq!(arg.func_hello(), expect);
// }
//
// pub fn rust_auto_opaque_trait_object_return_own_one() -> Box<dyn HelloTraitTwinNormal> {
//     Box::new(HelloOneStructTwinNormal {
//         inner: "hello".into(),
//     })
// }
//
// pub fn rust_auto_opaque_trait_object_return_own_two() -> Box<dyn HelloTraitTwinNormal> {
//     Box::new(HelloTwoEnumTwinNormal::B)
// }
//
// // ==================================== static method =======================================
//
// impl NonCloneSimpleTwinNormal {
//     pub fn static_method_arg_own(arg: NonCloneSimpleTwinNormal) {
//         assert_eq!(arg.inner, 42);
//     }
//
//     pub fn static_method_arg_borrow(arg: &NonCloneSimpleTwinNormal) {
//         assert_eq!(arg.inner, 42);
//     }
//
//     pub fn static_method_arg_mut_borrow(arg: &mut NonCloneSimpleTwinNormal) {
//         assert_eq!(arg.inner, 42);
//         arg.inner += 1;
//     }
//
//     pub fn static_method_return_own() -> NonCloneSimpleTwinNormal {
//         NonCloneSimpleTwinNormal { inner: 42 }
//     }
// }
//
// // ==================================== instance method =======================================
//
// impl NonCloneSimpleTwinNormal {
//     /// unnamed constructor
//     pub fn new() -> NonCloneSimpleTwinNormal {
//         Self { inner: 42 }
//     }
//
//     /// named constructor
//     pub fn new_custom_name() -> NonCloneSimpleTwinNormal {
//         Self { inner: 42 }
//     }
//
//     pub fn instance_method_arg_own(self) {
//         assert_eq!(self.inner, 42);
//     }
//
//     pub fn instance_method_arg_borrow(&self) {
//         assert_eq!(self.inner, 42);
//     }
//
//     pub fn instance_method_arg_mut_borrow(&mut self) {
//         assert_eq!(self.inner, 42);
//         self.inner += 1;
//     }
//
//     pub fn instance_method_return_own(&self) -> NonCloneSimpleTwinNormal {
//         Self { inner: 42 }
//     }
// }
//
// // ================ types with both encodable and opaque fields ===================
//
// #[frb(opaque)]
// pub struct StructWithGoodAndOpaqueFieldTwinNormal {
//     pub good: String,
//     pub opaque: NonCloneSimpleTwinNormal,
// }
//
// pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_own(
//     arg: StructWithGoodAndOpaqueFieldTwinNormal,
// ) {
//     assert_eq!(&arg.good, "hello");
//     assert_eq!(arg.opaque.inner, 42);
// }
//
// pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow(
//     arg: &StructWithGoodAndOpaqueFieldTwinNormal,
// ) {
//     assert_eq!(&arg.good, "hello");
//     assert_eq!(arg.opaque.inner, 42);
// }
//
// pub fn rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow(
//     arg: &mut StructWithGoodAndOpaqueFieldTwinNormal,
// ) {
//     assert_eq!(&arg.good, "hello");
//     assert_eq!(arg.opaque.inner, 42);
//     arg.good += "world";
//     arg.opaque.inner += 1;
// }
//
// pub fn rust_auto_opaque_struct_with_good_and_opaque_field_return_own(
// ) -> StructWithGoodAndOpaqueFieldTwinNormal {
//     StructWithGoodAndOpaqueFieldTwinNormal {
//         good: "hello".to_string(),
//         opaque: NonCloneSimpleTwinNormal { inner: 42 },
//     }
// }
