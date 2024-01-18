use std::any::Any;
use std::marker::PhantomData;

// TODO improve performance later
#[derive(Default)]
pub struct Arena<'a>(typed_arena::Arena<Box<dyn Any>>, PhantomData<&'a ()>);

impl<'a> Arena<'a> {
    pub fn alloc<T: 'a>(&'a self, value: T) -> &'a mut T {
        todo!()
        // let ans = self.0.alloc(Box::new(value));
        // ans.downcast_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_simple() {
        let arena = Arena::default();
        let apple = arena.alloc("Apple".to_owned());
        let orange = arena.alloc(PathBuf::new());
        assert_eq!(apple, "Apple");
        drop(arena);
        // assert_eq!(apple, "Apple"); // This will make compile fail (as expected)
    }
}
