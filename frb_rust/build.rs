use build_target::{self, Family};
use rustc_version::*;

fn main() {
    if let Ok(Family::Wasm) = build_target::target_family() {
        println!("cargo:rustc-cfg=wasm");
    }
    if let Ok(VersionMeta {
        channel: Channel::Nightly,
        ..
    }) = version_meta()
    {
        println!("cargo:rustc-cfg=nightly")
    }
}
