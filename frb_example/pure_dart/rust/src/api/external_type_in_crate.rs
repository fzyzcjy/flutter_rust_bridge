use crate::data::{EnumAlias, Id, MyEnum, MyStruct, StructAlias, UserIdAlias};

// Function that uses imported struct (from within this crate)
pub fn use_imported_struct(my_struct: MyStruct) -> bool {
    my_struct.content
}

// Function that uses imported enum (from within this crate)
pub fn use_imported_enum(my_enum: MyEnum) -> bool {
    match my_enum {
        MyEnum::False => false,
        MyEnum::True => true,
    }
}
