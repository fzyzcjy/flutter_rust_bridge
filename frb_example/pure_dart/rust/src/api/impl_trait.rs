pub struct StructWithDefaultValue {
    pub val: u32,
}

pub trait TestTrait {
    fn trait_fun() -> Self;
}

impl TestTrait for StructWithDefaultValue {
    fn trait_fun() -> Self {
        StructWithDefaultValue { val: 42 }
    }
}
