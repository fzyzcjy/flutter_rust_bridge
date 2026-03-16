use build_target::Family;

fn main() {
    for family in build_target::target_family() {
        if family == Family::Wasm {
            println!("cargo:rustc-cfg=wasm");
        }
    }
}
