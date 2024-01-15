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
