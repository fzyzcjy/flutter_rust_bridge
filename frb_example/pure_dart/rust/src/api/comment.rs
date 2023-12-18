/// This is single line comment
pub fn function_with_comments_triple_slash_single_line_twin_normal() {}

/// This is first line
/// This is second line
pub fn function_with_comments_triple_slash_multi_line_twin_normal() {}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.
 Newlines are preserved.
*/
pub fn function_with_comments_slash_star_star_twin_normal() {}

/// Comments on simple enums
pub enum SimpleEnumWithCommentsTwinNormal {
    Apple,
    /// Comments on enum variants
    Orange,
}

/// Comments on complex enums
pub enum ComplexEnumWithCommentsTwinNormal {
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
pub struct StructWithCommentsTwinNormal {
    /// Documentation on a struct field
    pub field_with_comments: i32,
}

impl StructWithCommentsTwinNormal {
    /// Documentation on a static method
    pub fn static_method_twin_normal() {}

    /// Documentation on an instance method
    pub fn instance_method_twin_normal(&self) {}
}
