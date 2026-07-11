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

pub type ChainedAppResultTwinNormal<T> = AppResultTwinNormal<T>;

pub fn generic_result_alias_chained_ok_twin_normal() -> ChainedAppResultTwinNormal<i32> {
    Ok(43)
}

pub fn generic_result_alias_chained_err_twin_normal() -> ChainedAppResultTwinNormal<i32> {
    Err(GenericAliasErrorTwinNormal::Deliberate)
}

pub type FlexibleResultTwinNormal<T, E> = std::result::Result<T, E>;

pub fn generic_result_alias_two_params_ok_twin_normal(
) -> FlexibleResultTwinNormal<i32, GenericAliasErrorTwinNormal> {
    Ok(44)
}

pub fn generic_result_alias_two_params_err_twin_normal(
) -> FlexibleResultTwinNormal<i32, GenericAliasErrorTwinNormal> {
    Err(GenericAliasErrorTwinNormal::Deliberate)
}

pub type OptionalAliasTwinNormal<T> = Option<T>;

pub fn generic_option_alias_return_twin_normal(input: i32) -> OptionalAliasTwinNormal<i32> {
    if input >= 0 {
        Some(input)
    } else {
        None
    }
}

pub fn generic_option_alias_arg_twin_normal(input: OptionalAliasTwinNormal<i32>) -> i32 {
    input.unwrap_or(-1)
}
