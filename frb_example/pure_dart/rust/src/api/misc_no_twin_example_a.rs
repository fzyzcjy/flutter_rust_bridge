// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "syncSse", "rustAsyncSse"]}

use crate::api::misc_no_twin_example_b::StructInMiscNoTwinExampleB;
use flutter_rust_bridge::frb;

// Reproduce #1630
#[frb(opaque)]
pub struct StructInMiscNoTwinExampleA;

impl StructInMiscNoTwinExampleB {
    pub async fn sample_function_a(&self) {}
}
