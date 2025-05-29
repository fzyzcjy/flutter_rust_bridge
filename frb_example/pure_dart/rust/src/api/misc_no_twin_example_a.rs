// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use crate::api::rust_opaque::NonCloneDataRaw;
use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
pub use std::any::Any;
use std::sync::{Arc, Mutex};

// Reproduce #1630
#[frb(opaque)]
pub struct StructInMiscNoTwinExampleA {}

impl StructInMiscNoTwinExampleA {
    pub async fn sample_function_a(&self) {}
}

// Reproduce "multi impl block" in #1630
#[frb(opaque)]
pub struct StructWithImplBlockInMultiFile {}

impl StructWithImplBlockInMultiFile {
    pub fn method_in_a(&self) {}
}

// Please keep exactly the *same* name in two files to test #1913
pub fn same_function_name_in_different_files() {}

// #1933
pub struct StructWithImplBlockInAnotherFile {}

pub struct StructWithCustomNameMethodTwinNormal(pub i32);

impl StructWithCustomNameMethodTwinNormal {
    #[frb(name = "renamedMethod", sync)]
    pub fn method_with_custom_name_twin_normal(&self) {}
}

#[frb(name = "renamedFunction")]
pub fn function_with_custom_name_twin_normal() {}

#[frb(opaque)]
pub struct StructWithSimpleSetterTwinNormal(i32);

impl StructWithSimpleSetterTwinNormal {
    #[frb(sync)]
    pub fn new() -> Self {
        Self(100)
    }

    #[frb(getter, sync)]
    pub fn simple_getter(&self) -> i32 {
        self.0
    }

    #[frb(setter, sync)]
    pub fn simple_setter(&mut self, value: i32) {
        self.0 = value;
    }

    // should auto strip prefix "get_" and "set_"
    #[frb(getter, sync)]
    pub fn get_something(&self) -> i32 {
        self.0
    }

    #[frb(setter, sync)]
    pub fn set_something(&mut self, value: i32) {
        self.0 = value;
    }
}

// #1937
// Suppose this is opaque
#[frb(opaque)]
#[derive(Clone)]
pub struct OpaqueItemTwinNormal(i32);

// #1937
#[frb(opaque)]
pub struct ItemContainerSolutionOneTwinNormal {
    // TODO auto generate getter/setter
    pub name: String,
    items: Vec<OpaqueItemTwinNormal>,
}

impl ItemContainerSolutionOneTwinNormal {
    pub fn create_twin_normal() -> Self {
        Self {
            name: "hi".to_owned(),
            items: vec![OpaqueItemTwinNormal(100)],
        }
    }

    pub fn get_item_contents_twin_normal(&self) -> Vec<i32> {
        self.items.iter().map(|x| x.0).collect()
    }
}

// #1937
#[frb]
pub struct ItemContainerSolutionTwoTwinNormal {
    #[frb(non_final)]
    pub name: String,
    pub items: Vec<RustAutoOpaque<OpaqueItemTwinNormal>>,
}

impl ItemContainerSolutionTwoTwinNormal {
    pub fn create_twin_normal() -> Self {
        Self {
            name: "hi".to_owned(),
            items: vec![RustAutoOpaque::new(OpaqueItemTwinNormal(100))],
        }
    }

    pub fn get_item_contents_twin_normal(&self) -> Vec<i32> {
        self.items.iter().map(|x| x.try_read().unwrap().0).collect()
    }
}

#[frb(opaque)]
pub struct DeliberateFailSanityCheckTwinNormal {
    pub good_field_a: String,
    pub good_field_b: i32,
    pub good_field_c: RustAutoOpaque<OpaqueItemTwinNormal>,
    pub deliberate_bad_field_a: Vec<u8>,
    pub deliberate_bad_field_b: OpaqueItemTwinNormal,
    pub deliberate_bad_field_c: Vec<OpaqueItemTwinNormal>,
}

impl DeliberateFailSanityCheckTwinNormal {
    pub fn dummy_function_twin_normal() {}
}

pub fn function_with_arg_type_name_override(a: Box<dyn Any + Send + Sync + 'static>) {
    let _ = a;
}

#[derive(Debug, Clone)]
#[frb(opaque)]
pub struct SimpleLogger(Arc<Mutex<Vec<String>>>);

impl SimpleLogger {
    #[frb(sync)]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(vec![])))
    }

    pub(crate) fn log(&self, message: &str) {
        self.0.lock().unwrap().push(message.to_owned());
    }

    #[frb(sync)]
    pub fn get_and_reset(&self) -> Vec<String> {
        self.0.lock().unwrap().drain(..).collect()
    }
}

#[frb(opaque)]
pub struct MyStructWithTryFromTwinNormal(String);

// #2103
impl TryFrom<String> for MyStructWithTryFromTwinNormal {
    type Error = flutter_rust_bridge::for_generated::anyhow::Error;

    #[frb]
    fn try_from(value: String) -> flutter_rust_bridge::for_generated::anyhow::Result<Self> {
        Ok(Self(value))
    }
}

impl MyStructWithTryFromTwinNormal {
    pub fn value_twin_normal(&self) -> String {
        self.0.to_owned()
    }
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/2170
pub trait Issue2170Trait {
    fn method(&self);
}

pub struct Issue2170Struct {
    pub field: Box<dyn Issue2170Trait>,
}

impl std::fmt::Debug for Issue2170Struct {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub struct MyStructWithSync {}
impl MyStructWithSync {
    // #2194
    #[frb(name = "sync")]
    pub fn sync(&self) {}
}

#[derive(Debug, Clone, Default)]
#[frb(opaque)]
pub struct StructWithRustAutoOpaqueFieldWithManyDeriveInner {}

#[derive(Debug, Clone, Default)]
pub struct StructWithRustAutoOpaqueFieldWithManyDerive {
    pub content:
        crate::frb_generated::RustAutoOpaque<StructWithRustAutoOpaqueFieldWithManyDeriveInner>,
}

impl StructWithRustAutoOpaqueFieldWithManyDerive {
    pub fn f(&self) {}
}

#[derive(Clone)]
pub struct StructWithRustAutoOpaqueWithNonCloneData {
    pub content: crate::frb_generated::RustAutoOpaque<NonCloneDataRaw>,
}

impl StructWithRustAutoOpaqueWithNonCloneData {
    pub fn f(&self) {}
}

#[cfg(feature = "internal_feature_for_testing")]
pub fn feature_gated_function() -> String {
    "test".to_owned()
}

pub struct StructWithRawNameField {
    pub r#type: String,
}

impl StructWithRawNameField {
    #[frb(serialize)]
    pub fn dummy_function() {}
}

#[frb(serialize)]
pub fn r#for(r#type: String) {
    let _ = r#type;
}

pub const CONST_INT_TWIN_NORMAL: i32 = 42;
pub const CONST_ARRAY_TWIN_NORMAL: [f32; 3] = [1.5, 3.0, 6.0];

// These consts should be ignored
#[allow(dead_code)]
pub(crate) const CONST_PUB_CRATE_SHOULD_IGNORE: i32 = 42;
#[allow(dead_code)]
const CONST_PRIVATE_SHOULD_IGNORE: i32 = 42;
#[allow(dead_code)]
#[frb(ignore)]
pub const CONST_WITH_EXPLICIT_IGNORE_SHOULD_IGNORE: i32 = 42;

#[frb(json_serializable)]
pub enum MyEnumWithJsonSerializableTwinNormal {
    Apple(String),
    Orange { a: i32 },
}

impl MyEnumWithJsonSerializableTwinNormal {
    pub fn f(&self) {}
}

#[frb(json_serializable)]
pub struct MyStructWithJsonSerializableTwinNormal {
    pub field_one: String,
}

impl MyStructWithJsonSerializableTwinNormal {
    pub fn f(&self) {}
}

#[frb(unignore)]
pub struct MyStructWithoutFnWithUnignoreTwinNormal {
    pub a: String,
}

#[frb(unignore)]
pub enum MyEnumWithoutFnWithUnignoreTwinNormal {
    One(String),
}

#[frb(unignore, json_serializable)]
pub struct MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal {
    pub a: String,
}

#[flutter_rust_bridge::frb(opaque)]
pub struct TypeForIgnore {
    #[frb(ignore)]
    pub field_1: u32,
}

impl TypeForIgnore {
    pub fn new() -> Self {
        Self { field_1: 0 }
    }

    pub fn field_1(&self) -> u32 {
        1
    }
}

impl Default for TypeForIgnore {
    fn default() -> Self {
        Self::new()
    }
}

#[flutter_rust_bridge::frb(opaque, ignore_all)]
pub struct TypeForIgnoreAll {
    pub field_1: u32,
    #[frb(unignore)]
    pub field_2: u32,
}

impl TypeForIgnoreAll {
    pub fn new() -> Self {
        Self {
            field_1: 0,
            field_2: 0,
        }
    }

    pub fn field_1(&self) -> u32 {
        1
    }
}

impl Default for TypeForIgnoreAll {
    fn default() -> Self {
        Self::new()
    }
}
