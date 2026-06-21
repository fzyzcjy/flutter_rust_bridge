use std::env;

use build_target::Family;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(frb_sanitize_memory)");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=NIX_FRB_RUSTFLAGS");

    if let Ok(Family::Wasm) = build_target::target_family() {
        println!("cargo:rustc-cfg=wasm");
    }

    if has_memory_sanitizer_flag("RUSTFLAGS") || has_memory_sanitizer_flag("NIX_FRB_RUSTFLAGS") {
        println!("cargo:rustc-cfg=frb_sanitize_memory");
    }
}

fn has_memory_sanitizer_flag(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            value
                .split_whitespace()
                .any(|part| part == "-Zsanitizer=memory")
        })
        .unwrap_or(false)
}
