use std::ops::Deref;

pub struct DartFn<F> {
    inner: F,
}

impl<F> DartFn<F> {
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
}

// https://github.com/rust-lang/rust/issues/29625#issuecomment-1692602873
impl<F> Deref for DartFn<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
