/// This is a struct used in all API blocks for test, but not defined in any regular block file
pub struct SharedStruct {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
impl SharedStruct {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct only used in API block 1 for test, but not defined in block file
pub struct OnlyForApi1Struct {
    pub id: i16,      // in-built type only used in API block 1 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl OnlyForApi1Struct {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct only used in API block 2 for test, but not defined in block file
pub struct OnlyForApi2Struct {
    pub id: i64,      // in-built type only used in API block 2 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl OnlyForApi2Struct {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// this struct is used only as parameter in API block1 and used only as return type in API block2, but not
/// defined in either block file
pub struct CrossSharedStruct {
    pub name: String,
}
impl CrossSharedStruct {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}
