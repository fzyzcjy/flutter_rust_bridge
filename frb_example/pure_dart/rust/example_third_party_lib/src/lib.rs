//! Examples of types scanned directly from a third-party crate via `rust_input` (not mirrored)

#[derive(Debug, Clone)]
pub enum ThirdPartyEnumWithFields {
    Unit,
    Named { id: i32, name: String },
    Tuple(i32),
}
