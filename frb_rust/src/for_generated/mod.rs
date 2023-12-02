//! Functions that support auto-generated Rust code.
//! These functions are usually *not* meant to be used by humans directly.

mod cast;
mod pointer;

pub use crate::handler::handler::FfiCallMode;
pub use crate::handler::handler::TaskInfo;
pub use crate::platform_types::MessagePort;
pub use cast::*;
pub use pointer::*;
