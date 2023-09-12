/// This is a struct only used in API block 2 for test, but not defined in block file
pub struct StructOnlyForBlock2 {
    pub id: i16,      // in-built type only used in API block 2 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl StructOnlyForBlock2 {
    /// the parameter type `u16 for `num` is only used for struct
    /// used in a specific API block but not defined in the API block(like `StructOnlyForBlock1`,`StructOnlyForBlock3`),
    /// for testing shared type(`u16`) within a no-shared struct method
    pub fn test_method(&self, message: String, num: u16) -> String {
        format!("{}_{}", message, num)
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}
