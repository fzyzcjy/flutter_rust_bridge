pub struct NewSimpleStruct {
    pub field: i32,
}

pub fn use_new_module_system(value: i32) -> NewSimpleStruct {
    NewSimpleStruct { field: value }
}
