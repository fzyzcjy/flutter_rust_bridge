#[cfg(feature = "rust-async-tokio")]
mod tokio_impl;

#[cfg(feature = "rust-async-async-std")]
mod async_std_impl;

#[cfg(feature = "rust-async-tokio")]
pub mod tokio_runtime {
    pub use super::tokio_impl::*;
}

#[cfg(feature = "rust-async-async-std")]
pub mod async_std_runtime {
    pub use super::async_std_impl::*;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "rust-async-tokio")] {
        pub use tokio_runtime::*;
    } else if #[cfg(feature = "rust-async-async-std")] {
        pub use async_std_runtime::*;
    }
}
