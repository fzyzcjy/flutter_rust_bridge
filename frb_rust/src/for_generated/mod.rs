//! Functions that support auto-generated Rust code.
//! These functions are usually *not* meant to be used by humans directly.

mod cast;
mod pointer;

pub use crate::handler::handler::FfiCallMode;
pub use crate::handler::handler::TaskInfo;
use crate::misc::manual_impl::*;
pub use crate::platform_types::MessagePort;
pub use cast::*;
pub use lazy_static::lazy_static;
pub use pointer::*;
