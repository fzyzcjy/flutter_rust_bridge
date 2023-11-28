// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `primitive_list_sync_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use super::primitive_list_misc::*;
use flutter_rust_bridge::*;

#[frb(sync)]
#[flutter_rust_bridge::frb(sync)] pub fn handle_zero_copy_vec_of_primitive_sync_twin_sync(
    n: i32,
) -> ZeroCopyVecOfPrimitivePackTwinSync {
    handle_zero_copy_vec_of_primitive_twin_normal(n)
}
