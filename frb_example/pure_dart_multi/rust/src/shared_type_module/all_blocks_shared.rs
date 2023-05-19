use flutter_rust_bridge::ZeroCopyBuffer;

pub enum EnumType {
    Empty,
    Primitives {
        /// Dart field comment
        int32: i32,
        float64: f64,
        boolean: bool,
    },
    Nested(Box<EnumType>),
    Optional(
        /// Comment on anonymous field
        Option<i32>,
        Option<Vec<u8>>,
    ),
    Buffer(ZeroCopyBuffer<Vec<f32>>),
    Enums(Weekdays),
}

pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// This is a struct used in ALL API blocks, NOT defined in any regular block file
pub struct SharedStructInAllBlocks {
    pub id: i32,
    pub num: f64,
    pub name: String,
    pub enum_list: Option<Vec<EnumType>>,
}
impl SharedStructInAllBlocks {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}

// This struct is shared for testing only sync return type specifically
pub struct SharedStructOnlyForSyncTest {
    pub name: String,
    pub score: f64,
}
impl SharedStructOnlyForSyncTest {
    #[allow(unused)]
    pub fn test_method(&self, y: u32) -> u32 {
        self.score as u32 + y
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}

/// This is a struct used in API blocks 1 and 2 for test, but not defined in any regular block file
pub struct SharedStructInBlock1And2 {
    pub id: i32,
    pub num: f64,
    pub name: String,
}
impl SharedStructInBlock1And2 {
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
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
    #[allow(unused)]
    pub fn test_method(&self, message: String) -> String {
        message
    }
    #[allow(unused)]
    pub fn test_static_method(message: String) -> String {
        message
    }
}
