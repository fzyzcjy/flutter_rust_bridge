use super::primitive_list_misc::*;
use flutter_rust_bridge::*;

#[frb(sync)]
pub fn handle_zero_copy_vec_of_primitive_sync_twin_normal(
    n: i32,
) -> ZeroCopyVecOfPrimitivePackTwinNormal {
    handle_zero_copy_vec_of_primitive_twin_normal(n)
}
