pub struct SomeStructWithDefaultValue {
    pub val: u32
}

impl Default for SomeStructWithDefaultValue {
    fn default() -> Self {
        SomeStructWithDefaultValue {
            val: 42
        }
    }
}