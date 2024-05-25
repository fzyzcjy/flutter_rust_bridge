// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

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
}
