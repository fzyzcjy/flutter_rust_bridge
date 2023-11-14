/// Like `also` in Kotlin
// ref https://docs.rs/apply/latest/src/apply/lib.rs.html
pub(crate) trait Also : Sized {
    /// Apply a function to this value and return the (possibly) modified value.
    fn also<F: FnOnce(&mut Self)>(mut self, block: F) -> Self {
        block(&mut self);
        self
    }
}

impl <T: Sized> Also for T {}
