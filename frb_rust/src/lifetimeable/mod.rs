use std::any::Any;

pub struct Lifetimeable<T> {
    // NOTE: The borrowed value must be *before* the dependency values to have correct *drop order*
    value: T,
    dependencies: Vec<Box<dyn Any>>,
}
