// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_alias_id_twin_sync_sse(input: Id) -> Id {
    input
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_nest_alias_id_twin_sync_sse(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinSyncSse {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_alias_model_twin_sync_sse(input: Id) -> TestModelTwinSyncSse {
    TestModelTwinSyncSse {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #1710: user-defined `type Result<T>` must not shadow generated wire code.
pub enum ResultShadowErrorTwinSyncSse {
    Dummy,
}

pub type Result<T> = std::result::Result<T, ResultShadowErrorTwinSyncSse>;

// Infallible API triggers generated `std::result::Result::<_, ()>::Ok` wrapper in wire code.
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn infallible_with_result_shadow_twin_sync_sse() -> i32 {
    42
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinSyncSse {
    Deliberate,
}

pub type AppResultTwinSyncSse<T> = std::result::Result<T, GenericAliasErrorTwinSyncSse>;

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_ok_twin_sync_sse() -> AppResultTwinSyncSse<i32> {
    Ok(42)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_err_twin_sync_sse() -> AppResultTwinSyncSse<i32> {
    Err(GenericAliasErrorTwinSyncSse::Deliberate)
}
