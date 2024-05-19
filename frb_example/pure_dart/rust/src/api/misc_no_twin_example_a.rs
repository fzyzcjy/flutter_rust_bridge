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

// #1906
pub struct StructWithImplBlockOutsideApiFolder {}

// #1933
pub struct StructWithImplBlockInAnotherFile {}
