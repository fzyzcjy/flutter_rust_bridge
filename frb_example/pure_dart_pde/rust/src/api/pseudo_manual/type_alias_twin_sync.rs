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
