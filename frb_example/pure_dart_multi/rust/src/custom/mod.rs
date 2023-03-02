// struct used in all API blocks for test
pub struct SharedStruct {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
// struct only used in API block 1 for test
pub struct OnlyForApi1Struct {
    pub id: i16,      // in-built type only used in API block 1 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
// struct only used in API block 2 for test
pub struct OnlyForApi2Struct {
    pub id: i64,      // in-built type only used in API block 2 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}

// this struct is used only as parameter in API block1, and used only as return type in API block2
pub struct CrossSharedStruct {
    pub name: String,
}
