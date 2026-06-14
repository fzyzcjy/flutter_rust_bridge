// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

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
