use build_target::{self, Family};

fn main() {
    if let Ok(Family::Wasm) = build_target::target_family() {
        println!("cargo:rustc-cfg=wasm");
    } else {
        println!("cargo:rerun-if-changed=src/dart_api_dl/trampoline.c");
        cc::Build::new()
            .file("src/dart_api_dl/trampoline.c")
            .compile("trampoline");
    }
}
