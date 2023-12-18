use crate::auxiliary::new_module_system::{use_new_module_system, NewSimpleStruct};
use crate::auxiliary::old_module_system::{use_old_module_system, OldSimpleStruct};
use crate::auxiliary::sample_types::{MyEnum, MyStruct};

// Function that uses imported struct (from within this crate)
pub fn use_imported_struct_twin_normal(my_struct: MyStruct) -> bool {
    my_struct.content
}

// Function that uses imported enum (from within this crate)
pub fn use_imported_enum_twin_normal(my_enum: MyEnum) -> bool {
    match my_enum {
        MyEnum::False => false,
        MyEnum::True => true,
    }
}

pub fn call_old_module_system_twin_normal() -> OldSimpleStruct {
    use_old_module_system(2)
}

pub fn call_new_module_system_twin_normal() -> NewSimpleStruct {
    use_new_module_system(1)
}
