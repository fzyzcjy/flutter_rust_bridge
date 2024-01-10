// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "syncSse", "rustAsyncSse"]}

use flutter_rust_bridge::frb;

// Reproduce #1630
#[frb(opaque)]
pub struct StructInMiscNoTwinExample;

impl StructInMiscNoTwinExample {
    pub async fn get_struct_in_other_files(&self) -> FeatureA {
        TODO
    }
}
