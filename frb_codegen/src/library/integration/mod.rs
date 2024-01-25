//! Integrate Flutter with Rust

mod creator;
pub(crate) mod integrator;
mod utils;

pub use creator::{create, CreateConfig};
pub use integrator::{integrate, IntegrateConfig};
