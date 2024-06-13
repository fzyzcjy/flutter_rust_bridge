use build_target::Family;

fn main() {
    if let Ok(Family::Wasm) = build_target::target_family() {
        println!("cargo:rustc-cfg=wasm");
    }
}
