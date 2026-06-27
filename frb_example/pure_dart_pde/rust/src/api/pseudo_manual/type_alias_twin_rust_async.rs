// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

pub async fn handle_type_alias_id_twin_rust_async(input: Id) -> Id {
    input
}

pub async fn handle_type_nest_alias_id_twin_rust_async(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinRustAsync {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

pub async fn handle_type_alias_model_twin_rust_async(input: Id) -> TestModelTwinRustAsync {
    TestModelTwinRustAsync {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #1710: user-defined `type Result<T>` must not shadow generated wire code.
pub enum ResultShadowErrorTwinRustAsync {
    Dummy,
}

pub type Result<T> = std::result::Result<T, ResultShadowErrorTwinRustAsync>;

// Infallible API triggers generated `std::result::Result::<_, ()>::Ok` wrapper in wire code.
pub async fn infallible_with_result_shadow_twin_rust_async() -> i32 {
    42
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinRustAsync {
    Deliberate,
}

pub type AppResultTwinRustAsync<T> = std::result::Result<T, GenericAliasErrorTwinRustAsync>;

pub async fn generic_result_alias_ok_twin_rust_async() -> AppResultTwinRustAsync<i32> {
    Ok(42)
}

pub async fn generic_result_alias_err_twin_rust_async() -> AppResultTwinRustAsync<i32> {
    Err(GenericAliasErrorTwinRustAsync::Deliberate)
}

pub type ChainedAppResultTwinRustAsync<T> = AppResultTwinRustAsync<T>;

pub async fn generic_result_alias_chained_ok_twin_rust_async() -> ChainedAppResultTwinRustAsync<i32>
{
    Ok(43)
}

pub async fn generic_result_alias_chained_err_twin_rust_async() -> ChainedAppResultTwinRustAsync<i32>
{
    Err(GenericAliasErrorTwinRustAsync::Deliberate)
}

pub type FlexibleResultTwinRustAsync<T, E> = std::result::Result<T, E>;

pub async fn generic_result_alias_two_params_ok_twin_rust_async(
) -> FlexibleResultTwinRustAsync<i32, GenericAliasErrorTwinRustAsync> {
    Ok(44)
}

pub async fn generic_result_alias_two_params_err_twin_rust_async(
) -> FlexibleResultTwinRustAsync<i32, GenericAliasErrorTwinRustAsync> {
    Err(GenericAliasErrorTwinRustAsync::Deliberate)
}

pub type OptionalAliasTwinRustAsync<T> = Option<T>;

pub async fn generic_option_alias_return_twin_rust_async(
    input: i32,
) -> OptionalAliasTwinRustAsync<i32> {
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}

pub async fn generic_option_alias_arg_twin_rust_async(
    input: OptionalAliasTwinRustAsync<i32>,
) -> i32 {
    input.unwrap_or(-1)
}
