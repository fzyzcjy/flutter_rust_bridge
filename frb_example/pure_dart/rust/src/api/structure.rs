use flutter_rust_bridge::frb;

pub struct StructWithZeroFieldTwinNormal {}

pub fn func_struct_with_zero_field_twin_normal(
    arg: StructWithZeroFieldTwinNormal,
) -> StructWithZeroFieldTwinNormal {
    arg
}

pub struct StructWithOneFieldTwinNormal {
    pub a: i32,
}

pub fn func_struct_with_one_field_twin_normal(
    arg: StructWithOneFieldTwinNormal,
) -> StructWithOneFieldTwinNormal {
    arg
}

pub struct StructWithTwoFieldTwinNormal {
    pub a: i32,
    pub b: i32,
}

pub fn func_struct_with_two_field_twin_normal(
    arg: StructWithTwoFieldTwinNormal,
) -> StructWithTwoFieldTwinNormal {
    arg
}

pub struct TupleStructWithOneFieldTwinNormal(pub i32);

pub fn func_tuple_struct_with_one_field_twin_normal(
    arg: TupleStructWithOneFieldTwinNormal,
) -> TupleStructWithOneFieldTwinNormal {
    arg
}

pub struct TupleStructWithTwoFieldTwinNormal(pub i32, pub i32);

pub fn func_tuple_struct_with_two_field_twin_normal(
    arg: TupleStructWithTwoFieldTwinNormal,
) -> TupleStructWithTwoFieldTwinNormal {
    arg
}

#[frb]
pub struct StructWithFieldRenameTwinNormal {
    #[frb(name = "renamed_field")]
    pub class: i32,
}

pub fn func_for_struct_with_field_rename_twin_normal(
    arg: StructWithFieldRenameTwinNormal,
) -> StructWithFieldRenameTwinNormal {
    arg
}

pub struct StructWithDartKeywordFieldTwinNormal {
    pub class: i32,
    pub interface: i64,
}

pub fn func_for_struct_with_dart_keyword_field_twin_normal(
    arg: StructWithDartKeywordFieldTwinNormal,
) -> StructWithDartKeywordFieldTwinNormal {
    arg
}
