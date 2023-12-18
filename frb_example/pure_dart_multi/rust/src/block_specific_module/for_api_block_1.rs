/// This is a struct only used in API block 1 for test, but not defined in block file
pub struct StructOnlyForBlock1 {
    pub id: Option<i8>, // both the inner in-built type and the `Option` wrapper are only used in API block 1
    pub num: Option<f64>, // the inner in-built type is used for all API blocks, BUT `Option` wrapped for it is only used here
    pub name: Option<String>, // the inner struct type is used for all API blocks, BUT `Option` wrapped for it is only used here
}
impl StructOnlyForBlock1 {
    /// the parameter type `u16 for `num` is only used for struct
    /// used in a specific API block but not defined in the API block(like `StructOnlyForBlock2`,`StructOnlyForBlock3`),
    /// for testing shared type(`u16`) within a no-shared struct method
    pub fn test_method(&self, message: String, num: u16) -> String {
        format!("{}_{}", message, num)
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

// TODO: delete? refine?
// #[derive(Debug)]
// pub struct SubTypeForStructDefinedInBlock1 {}
