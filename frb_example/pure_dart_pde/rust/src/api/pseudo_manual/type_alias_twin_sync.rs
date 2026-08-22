// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_alias_id_twin_sync(input: Id) -> Id {
    input
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_nest_alias_id_twin_sync(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinSync {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_type_alias_model_twin_sync(input: Id) -> TestModelTwinSync {
    TestModelTwinSync {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinSync {
    Deliberate,
}

pub type AppResultTwinSync<T> = std::result::Result<T, GenericAliasErrorTwinSync>;

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_ok_twin_sync() -> AppResultTwinSync<i32> {
    Ok(42)
}

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_err_twin_sync() -> AppResultTwinSync<i32> {
    Err(GenericAliasErrorTwinSync::Deliberate)
}

pub type ChainedAppResultTwinSync<T> = AppResultTwinSync<T>;

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_chained_ok_twin_sync() -> ChainedAppResultTwinSync<i32> {
    Ok(43)
}

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_chained_err_twin_sync() -> ChainedAppResultTwinSync<i32> {
    Err(GenericAliasErrorTwinSync::Deliberate)
}

pub type FlexibleResultTwinSync<T, E> = std::result::Result<T, E>;

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_two_params_ok_twin_sync(
) -> FlexibleResultTwinSync<i32, GenericAliasErrorTwinSync> {
    Ok(44)
}

#[flutter_rust_bridge::frb(sync)]
pub fn generic_result_alias_two_params_err_twin_sync(
) -> FlexibleResultTwinSync<i32, GenericAliasErrorTwinSync> {
    Err(GenericAliasErrorTwinSync::Deliberate)
}

pub type OptionalAliasTwinSync<T> = Option<T>;

#[flutter_rust_bridge::frb(sync)]
pub fn generic_option_alias_return_twin_sync(input: i32) -> OptionalAliasTwinSync<i32> {
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generic_option_alias_arg_twin_sync(input: OptionalAliasTwinSync<i32>) -> i32 {
    input.unwrap_or(-1)
}
