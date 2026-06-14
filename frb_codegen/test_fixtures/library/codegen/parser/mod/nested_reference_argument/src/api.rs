#[derive(Clone)]
pub enum NestedRefEnum {
    A,
    B,
}

pub enum RecursiveEnum {
    Empty,
    Nested(Box<RecursiveEnum>),
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

pub fn accept_recursive_enum(value: RecursiveEnum) -> RecursiveEnum {
    value
}

pub fn accept_two_nested_refs(first: &Option<&NestedRefEnum>, second: &Option<&NestedRefEnum>) {
    let _ = (first, second);
}

pub fn accept_known_struct_mut(value: &mut NestedRefOwner) {
    let _ = value;
}
