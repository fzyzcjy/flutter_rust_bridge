use flutter_rust_bridge::ZeroCopyBuffer;

pub enum SharedComplexEnumInAllBlocks {
    Empty,
    Primitives {
        /// Dart field comment
        int32: i32,
        float64: f64,
        boolean: bool,
    },
    Nested(Box<SharedComplexEnumInAllBlocks>),
    Optional(
        /// Comment on anonymous field
        Option<i32>,
        Option<Vec<u8>>,
    ),
    Buffer(ZeroCopyBuffer<Vec<f32>>),
    Enums(SharedWeekdaysEnumInAllBlocks),
    BytesArray([u8; 3]),
}
impl SharedComplexEnumInAllBlocks {
    pub fn test_enum_method(&self, message: String) -> String {
        message
    }

    #[allow(unused)]
    pub fn test_mutable_enum_method(mut self) {}

    #[allow(unused)]
    pub fn test_mutable_ref_enum_method(&mut self) {}

    pub fn test_static_enum_method(message: String) -> String {
        message
    }
}

/// Simple enums.
#[derive(Debug, Clone, Copy)]
pub enum SharedWeekdaysEnumInAllBlocks {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    /// Best day of the week.
    Saturday,
    Sunday,
}
impl SharedWeekdaysEnumInAllBlocks {
    pub fn test_enum_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_enum_method(message: String) -> String {
        message
    }

    pub fn print_weekday(&self) -> String {
        match self {
            SharedWeekdaysEnumInAllBlocks::Monday => "Monday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Tuesday => "Tuesday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Wednesday => "Wednesday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Thursday => "Thursday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Friday => "Friday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Saturday => "Saturday".to_string(),
            SharedWeekdaysEnumInAllBlocks::Sunday => "Sunday".to_string(),
        }
    }
}

/// This is a struct used in ALL API blocks, NOT defined in any regular block file
pub struct SharedStructInAllBlocks {
    pub id: i32,
    pub num: f64,
    pub name: String,
    pub enum_list: Option<Vec<SharedComplexEnumInAllBlocks>>,
}
impl SharedStructInAllBlocks {
    /// The parameter type `u32 for `num` is ONLY used in this method for this testing project.
    /// It is for testing type `u32` within a shared struct,
    /// which means that the `u32` should be treated as a shared type, too.
    pub fn test_method(&self, message: String, num: u32) -> String {
        format!("{}_{}", message, num)
    }

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
    pub fn test_method(&self, message: String) -> String {
        message
    }

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

    #[allow(unused)]
    pub fn test_mutable_method(mut self, name: String) {
        self.name = name;
    }

    #[allow(unused)]
    pub fn test_mutable_ref_method(&mut self, name: String) {
        self.name = name;
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}
