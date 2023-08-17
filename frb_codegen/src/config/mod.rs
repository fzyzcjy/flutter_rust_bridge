pub(crate) mod opts;
pub mod opts_parser;
pub(crate) mod raw_opts;
pub(crate) mod raw_opts_parser;
pub(crate) mod refine_c_output;

mod error;
pub(crate) use error::Error;
// pub(crate) type ConfigResult<T = (), E = Error> = core::result::Result<T, E>;
