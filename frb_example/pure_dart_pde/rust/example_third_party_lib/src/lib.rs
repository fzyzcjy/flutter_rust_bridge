// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

//! Examples of types scanned directly from a third-party crate via `rust_input` (not mirrored)

#[derive(Debug, Clone)]
pub enum ThirdPartyEnumWithFields {
    Unit,
    Named { id: i32, name: String },
    Tuple(i32),
}

/// Other crates cannot construct this with a struct expression, so it must be opaque
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ThirdPartyNonExhaustiveStruct {
    pub value: i32,
}

impl ThirdPartyNonExhaustiveStruct {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
