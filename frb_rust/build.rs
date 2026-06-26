use build_target::Family;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(wasm)");
    if build_target::target_family().contains(&Family::Wasm) {
        println!("cargo:rustc-cfg=wasm");
    }
}
