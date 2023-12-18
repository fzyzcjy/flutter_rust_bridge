/// this struct is used only as parameter in API block1 and used only as return type in API block2, but not
/// defined in either block file
pub struct CrossSharedStructInBlock1And2 {
    pub name: String,
}
impl CrossSharedStructInBlock1And2 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// this struct is used only as parameter in API block2 and used only as return type in API block3, but not
/// defined in either block file
pub struct CrossSharedStructInBlock2And3 {
    pub name: String,
}
impl CrossSharedStructInBlock2And3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}
