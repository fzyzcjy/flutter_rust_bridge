// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_type_alias_id_twin_rust_async_sse(input: Id) -> Id {
    input
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_type_nest_alias_id_twin_rust_async_sse(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinRustAsyncSse {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_type_alias_model_twin_rust_async_sse(input: Id) -> TestModelTwinRustAsyncSse {
    TestModelTwinRustAsyncSse {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #1710: user-defined `type Result<T>` must not shadow generated wire code.
pub enum ResultShadowErrorTwinRustAsyncSse {
    Dummy,
}

pub type Result<T> = std::result::Result<T, ResultShadowErrorTwinRustAsyncSse>;

// Infallible API triggers generated `std::result::Result::<_, ()>::Ok` wrapper in wire code.
#[flutter_rust_bridge::frb(serialize)]
pub async fn infallible_with_result_shadow_twin_rust_async_sse() -> i32 {
    42
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinRustAsyncSse {
    Deliberate,
}

pub type AppResultTwinRustAsyncSse<T> = std::result::Result<T, GenericAliasErrorTwinRustAsyncSse>;

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_ok_twin_rust_async_sse() -> AppResultTwinRustAsyncSse<i32> {
    Ok(42)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_err_twin_rust_async_sse() -> AppResultTwinRustAsyncSse<i32> {
    Err(GenericAliasErrorTwinRustAsyncSse::Deliberate)
}

pub type ChainedAppResultTwinRustAsyncSse<T> = AppResultTwinRustAsyncSse<T>;

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_chained_ok_twin_rust_async_sse(
) -> ChainedAppResultTwinRustAsyncSse<i32> {
    Ok(43)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_chained_err_twin_rust_async_sse(
) -> ChainedAppResultTwinRustAsyncSse<i32> {
    Err(GenericAliasErrorTwinRustAsyncSse::Deliberate)
}

pub type FlexibleResultTwinRustAsyncSse<T, E> = std::result::Result<T, E>;

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_two_params_ok_twin_rust_async_sse(
) -> FlexibleResultTwinRustAsyncSse<i32, GenericAliasErrorTwinRustAsyncSse> {
    Ok(44)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_result_alias_two_params_err_twin_rust_async_sse(
) -> FlexibleResultTwinRustAsyncSse<i32, GenericAliasErrorTwinRustAsyncSse> {
    Err(GenericAliasErrorTwinRustAsyncSse::Deliberate)
}

pub type OptionalAliasTwinRustAsyncSse<T> = Option<T>;

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_option_alias_return_twin_rust_async_sse(
    input: i32,
) -> OptionalAliasTwinRustAsyncSse<i32> {
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn generic_option_alias_arg_twin_rust_async_sse(
    input: OptionalAliasTwinRustAsyncSse<i32>,
) -> i32 {
    input.unwrap_or(-1)
}
