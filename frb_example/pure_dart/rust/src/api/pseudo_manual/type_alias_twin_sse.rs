// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_type_alias_id_twin_sse(input: Id) -> Id {
    input
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_type_nest_alias_id_twin_sse(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinSse {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_type_alias_model_twin_sse(input: Id) -> TestModelTwinSse {
    TestModelTwinSse {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinSse {
    Deliberate,
}

pub type AppResultTwinSse<T> = std::result::Result<T, GenericAliasErrorTwinSse>;

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_ok_twin_sse() -> AppResultTwinSse<i32> {
    Ok(42)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_err_twin_sse() -> AppResultTwinSse<i32> {
    Err(GenericAliasErrorTwinSse::Deliberate)
}

pub type ChainedAppResultTwinSse<T> = AppResultTwinSse<T>;

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_chained_ok_twin_sse() -> ChainedAppResultTwinSse<i32> {
    Ok(43)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_chained_err_twin_sse() -> ChainedAppResultTwinSse<i32> {
    Err(GenericAliasErrorTwinSse::Deliberate)
}

pub type FlexibleResultTwinSse<T, E> = std::result::Result<T, E>;

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_two_params_ok_twin_sse(
) -> FlexibleResultTwinSse<i32, GenericAliasErrorTwinSse> {
    Ok(44)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_result_alias_two_params_err_twin_sse(
) -> FlexibleResultTwinSse<i32, GenericAliasErrorTwinSse> {
    Err(GenericAliasErrorTwinSse::Deliberate)
}

pub type OptionalAliasTwinSse<T> = Option<T>;

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_option_alias_return_twin_sse(input: i32) -> OptionalAliasTwinSse<i32> {
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn generic_option_alias_arg_twin_sse(input: OptionalAliasTwinSse<i32>) -> i32 {
    input.unwrap_or(-1)
}
