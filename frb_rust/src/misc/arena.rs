use std::any::Any;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ArenaBase<'a, T: 'a>(typed_arena::Arena<T>);

impl<'a, T: 'a> ArenaBase<'a, T> {
    pub fn alloc(&'a self, value: T) -> &'a mut T {
        self.0.alloc(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_simple() {
        let arena = ArenaBase::default();
        let apple = arena.alloc("Apple".to_owned());
        let orange = arena.alloc(PathBuf::new());
        assert_eq!(apple, "Apple");
        drop(arena);
        // assert_eq!(apple, "Apple"); // This will make compile fail (as expected)
    }
}
