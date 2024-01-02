use crate::handler::error::Error;

/// Listens when error happens
pub trait ErrorListener: Copy + Send + 'static {
    fn on_error(&self, error: Error);
}
