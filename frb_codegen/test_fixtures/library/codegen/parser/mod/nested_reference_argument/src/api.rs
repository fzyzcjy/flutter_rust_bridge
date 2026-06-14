#[derive(Clone)]
pub enum NestedRefEnum {
    A,
    B,
}

impl NestedRefEnum {
    pub fn is_a(&self) -> bool {
        matches!(self, Self::A)
    }
}

pub struct NestedRefOwner {
    value: Option<NestedRefEnum>,
}

impl NestedRefOwner {
    pub fn from(value: &Option<&NestedRefEnum>) -> Self {
        Self {
            value: value.clone().cloned(),
        }
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}
