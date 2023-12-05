use std::ops::Deref;

pub struct DartFn<F> {
    inner: F,
}

// https://github.com/rust-lang/rust/issues/29625#issuecomment-1692602873
impl<F> Deref for DartFn<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
