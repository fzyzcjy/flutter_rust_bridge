pub struct StructWithZeroField {}

pub fn func_struct_with_zero_field_twin_normal(arg: StructWithZeroField) -> StructWithZeroField {
    arg
}

pub struct StructWithOneField {
    pub a: i32,
}

pub fn func_struct_with_one_field_twin_normal(arg: StructWithOneField) -> StructWithOneField {
    arg
}

pub struct StructWithTwoField {
    pub a: i32,
    pub b: i32,
}

pub fn func_struct_with_two_field_twin_normal(arg: StructWithTwoField) -> StructWithTwoField {
    arg
}

pub struct TupleStructWithOneField(pub i32);

pub fn func_tuple_struct_with_one_field_twin_normal(
    arg: TupleStructWithOneField,
) -> TupleStructWithOneField {
    arg
}

pub struct TupleStructWithTwoField(pub i32, pub i32);

pub fn func_tuple_struct_with_two_field_twin_normal(
    arg: TupleStructWithTwoField,
) -> TupleStructWithTwoField {
    arg
}
