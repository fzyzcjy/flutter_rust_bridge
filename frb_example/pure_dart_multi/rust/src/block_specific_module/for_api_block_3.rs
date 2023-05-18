/// This is a struct only used in API block 3 for test, but not defined in block file
pub struct StructOnlyForBlock3 {
    pub id: i64,      // in-built type only used in API block 3 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl StructOnlyForBlock3 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}
