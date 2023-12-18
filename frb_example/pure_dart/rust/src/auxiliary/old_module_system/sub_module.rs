pub struct OldSimpleStruct {
    pub field: i32,
}

pub fn use_old_module_system(value: i32) -> OldSimpleStruct {
    OldSimpleStruct { field: value }
}
