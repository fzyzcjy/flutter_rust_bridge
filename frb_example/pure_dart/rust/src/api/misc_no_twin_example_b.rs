// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "syncSse", "rustAsyncSse"]}

use crate::api::misc_no_twin_example_a::StructInMiscNoTwinExampleA;
use flutter_rust_bridge::frb;

// Reproduce #1630
#[frb(opaque)]
pub struct StructInMiscNoTwinExampleB;

impl StructInMiscNoTwinExampleB {
    pub async fn get_struct_in_misc_no_twin_example_a(&self) -> StructInMiscNoTwinExampleA {
        StructInMiscNoTwinExampleA
    }
}
