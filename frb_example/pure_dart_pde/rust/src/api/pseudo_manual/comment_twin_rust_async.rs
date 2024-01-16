// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `comment.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

/// This is single line comment
pub async fn function_with_comments_triple_slash_single_line_twin_rust_async() {}

/// This is first line
/// This is second line
pub async fn function_with_comments_triple_slash_multi_line_twin_rust_async() {}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.
 Newlines are preserved.
*/
pub async fn function_with_comments_slash_star_star_twin_rust_async() {}

/// Comments on simple enums
pub enum SimpleEnumWithCommentsTwinRustAsync {
    Apple,
    /// Comments on enum variants
    Orange,
}

/// Comments on complex enums
pub enum ComplexEnumWithCommentsTwinRustAsync {
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
pub struct StructWithCommentsTwinRustAsync {
    /// Documentation on a struct field
    pub field_with_comments: i32,
}

impl StructWithCommentsTwinRustAsync {
    /// Documentation on a static method
    pub async fn static_method_twin_rust_async() {}

    /// Documentation on an instance method
    pub async fn instance_method_twin_rust_async(&self) {}
}
