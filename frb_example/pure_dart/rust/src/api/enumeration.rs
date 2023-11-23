pub enum EnumSimpleTwinNormal {
    A,
    B,
}

pub fn func_enum_simple_twin_normal(arg: EnumSimpleTwinNormal) -> EnumSimpleTwinNormal {
    arg
}

pub enum EnumWithItemMixedTwinNormal {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

pub fn func_enum_with_item_mixed_twin_normal(
    arg: EnumWithItemMixedTwinNormal,
) -> EnumWithItemMixedTwinNormal {
    arg
}

pub enum EnumWithItemTupleTwinNormal {
    A(Vec<u8>),
    B(Vec<i32>),
}

pub fn func_enum_with_item_tuple_twin_normal(
    arg: EnumWithItemTupleTwinNormal,
) -> EnumWithItemTupleTwinNormal {
    arg
}

pub enum EnumWithItemStructTwinNormal {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

pub fn func_enum_with_item_struct_twin_normal(
    arg: EnumWithItemStructTwinNormal,
) -> EnumWithItemStructTwinNormal {
    arg
}
