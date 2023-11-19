use std::fs;
use std::ops::Add;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::{info, warn};
use pathdiff::diff_paths;

pub fn sanity_check(generated_dart_wire_code: &str, dart_wire_class_name: &str) -> crate::Result {
    if !generated_dart_wire_code.contains(dart_wire_class_name) {
        bail!(
            "Nothing is generated for dart wire class. \
            Maybe you forget to put code like `mod the_generated_bridge_code;` to your `lib.rs`?",
        );
    }
    Ok(())
}
