pub struct ArenaBase<T>(typed_arena::Arena<T>);

impl<T> Default for ArenaBase<T> {
    fn default() -> Self {
        Self(typed_arena::Arena::new())
    }
}

impl<'a, T: 'a> ArenaBase<T> {
    pub fn alloc(&'a self, value: T) -> &'a mut T {
        self.0.alloc(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    enum SampleEnum {
        String(String),
        PathBuf(PathBuf),
    }

    #[test]
    fn test_simple() {
        let arena = ArenaBase::default();
        let apple = arena.alloc(SampleEnum::String("Apple".to_owned()));
        let _orange = arena.alloc(SampleEnum::PathBuf(PathBuf::new()));
        assert!(matches!(apple, SampleEnum::String(a) if a == "Apple"));
        drop(arena);
        // assert_eq!(apple, "Apple"); // This will make compile fail (as expected)
    }
}
