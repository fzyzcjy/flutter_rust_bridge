// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::{EnumAlias, Id, MyStruct, StructAlias, UserIdAlias};

pub fn handle_type_alias_id_twin_normal(input: Id) -> Id {
    input
}

pub fn handle_type_nest_alias_id_twin_normal(input: UserIdAlias) -> Id {
    input
}

pub struct TestModelTwinNormal {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

pub fn handle_type_alias_model_twin_normal(input: Id) -> TestModelTwinNormal {
    TestModelTwinNormal {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

// Regression for #1710: user-defined `type Result<T>` must not shadow generated wire code.
pub enum ResultShadowErrorTwinNormal {
    Dummy,
}

pub type Result<T> = std::result::Result<T, ResultShadowErrorTwinNormal>;

// Infallible API triggers generated `std::result::Result::<_, ()>::Ok` wrapper in wire code.
pub fn infallible_with_result_shadow_twin_normal() -> i32 {
    42
}

// Regression for #3071: a generic type alias used in an exported signature must
// be expanded to its underlying `Result`, so the function is fallible and both
// the value and the Dart exception path are available.
pub enum GenericAliasErrorTwinNormal {
    Deliberate,
}

pub type AppResultTwinNormal<T> = std::result::Result<T, GenericAliasErrorTwinNormal>;

pub fn generic_result_alias_ok_twin_normal() -> AppResultTwinNormal<i32> {
    Ok(42)
}

pub fn generic_result_alias_err_twin_normal() -> AppResultTwinNormal<i32> {
    Err(GenericAliasErrorTwinNormal::Deliberate)
}
