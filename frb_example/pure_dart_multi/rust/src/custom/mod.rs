/// This is a struct used in ALL API blocks, NOT defined in any regular block file
pub struct SharedStructInAllBlocks {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
impl SharedStructInAllBlocks {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

// This struct is shared for testing only sync return type specifically
pub struct SharedStructOnlyForSyncTest {
    pub default_score: f64,
}
impl SharedStructOnlyForSyncTest {
    pub fn sum(&self, y: u32) -> u32 {
        self.default_score as u32 + y
    }
    pub fn sum_static(x: u32, y: u32) -> u32 {
        x + y
    }
}

/// This is a struct used in API blocks 1 and 2 for test, but not defined in any regular block file
pub struct SharedStructInBlock1And2 {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
impl SharedStructInBlock1And2 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct used in all API blocks 2 and 3 for test, but not defined in any regular block file
pub struct SharedStructInBlock2And3 {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
impl SharedStructInBlock2And3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

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

/// This is a struct only used in API block 1 for test, but not defined in block file
pub struct StructOnlyForBlock1 {
    pub id: i8,       // in-built type only used in API block 1 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl StructOnlyForBlock1 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct only used in API block 2 for test, but not defined in block file
pub struct StructOnlyForBlock2 {
    pub id: i16,      // in-built type only used in API block 2 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl StructOnlyForBlock2 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct only used in API block 3 for test, but not defined in block file
pub struct StructOnlyForBlock3 {
    pub id: i64,      // in-built type only used in API block 3 for test
    pub num: f64,     // in-built type in all API blocks for test
    pub name: String, // struct type used in all API blocks for test
}
impl StructOnlyForBlock3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }
    pub fn test_static_method(message: String) -> String {
        message
    }
}
