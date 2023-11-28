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
