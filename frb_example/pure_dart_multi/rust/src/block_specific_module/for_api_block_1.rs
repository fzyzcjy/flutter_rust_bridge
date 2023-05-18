/// This is a struct only used in API block 1 for test, but not defined in block file
pub struct StructOnlyForBlock1 {
    pub id: Option<i8>, // both the inner in-built type and the `Option` wrapper are only used in API block 1
    pub num: Option<f64>, // the inner in-built type is used for all API blocks, BUT `Option` wrapped for it is only used here
    pub name: Option<String>, // the inner struct type is used for all API blocks, BUT `Option` wrapped for it is only used here
}
impl StructOnlyForBlock1 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}
