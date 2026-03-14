// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `comment.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

/// This is single line comment
#[flutter_rust_bridge::frb(serialize)]
pub fn function_with_comments_triple_slash_single_line_twin_sse() {}

/// This is first line
/// This is second line
#[flutter_rust_bridge::frb(serialize)]
pub fn function_with_comments_triple_slash_multi_line_twin_sse() {}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.
 Newlines are preserved.
*/
#[flutter_rust_bridge::frb(serialize)]
pub fn function_with_comments_slash_star_star_twin_sse() {}

/// Comments on simple enums
pub enum SimpleEnumWithCommentsTwinSse {
    Apple,
    /// Comments on enum variants
    Orange,
}

/// Comments on complex enums
pub enum ComplexEnumWithCommentsTwinSse {
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
pub struct StructWithCommentsTwinSse {
    /// Documentation on a struct field
    pub field_with_comments: i32,
}

impl StructWithCommentsTwinSse {
    /// Documentation on a static method
    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_twin_sse() {}

    /// Documentation on an instance method
    #[flutter_rust_bridge::frb(serialize)]
    pub fn instance_method_twin_sse(&self) {}
}
