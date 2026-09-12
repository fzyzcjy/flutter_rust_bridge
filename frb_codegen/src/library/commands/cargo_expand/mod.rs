mod real;

use crate::codegen::dumper::Dumper;
use crate::utils::crate_name::CrateName;
use anyhow::Result;
use std::path::Path;

pub(crate) const CODEGEN_RUNNING_ENV: &str = "_FRB_CODEGEN_IS_RUNNING";

pub(crate) fn run_cargo_expand(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    dumper: &Dumper,
    features: Option<&[String]>,
) -> Result<syn::File> {
    real::run(rust_crate_dir, interest_crate_name, dumper, features)
}
