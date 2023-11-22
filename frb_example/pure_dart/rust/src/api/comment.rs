/// This is single line comment
pub fn function_with_comments_triple_slash_single_line() {}

/// This is first line
/// This is second line
pub fn function_with_comments_triple_slash_multi_line() {}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.
 Newlines are preserved.
*/
pub fn function_with_comments_slash_star_star() {}

/// Comments on simple enums
pub enum SimpleEnumWithComments {
    Apple,
    /// Comments on enum variants
    Orange,
}

/// Comments on complex enums
pub enum ComplexEnumWithComments {
    Apple,
    Orange {
        /// Comments on enum variant's fields
        a_field: i32,
    },
    Raspberry(
        /// Comments on enum variant's anonymous fields
        i32,
    ),
}

/// Comments on structs
pub struct StructWithComments {
    /// Documentation on a struct field
    field_with_comments: i32,
}

pub struct MethodWithComments;

impl MethodWithComments {
    /// Documentation on a static method
    pub fn static_method() {}

    /// Documentation on an instance method
    pub fn instance_method(&self) {}
}
