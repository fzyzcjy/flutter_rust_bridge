use std::cfg;
#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::path::PathBuf;

#[cfg(all(not(feature = "docs-only"), windows))]
fn find_dart_sdk() -> Option<PathBuf> {
    if let Ok(path) = env::var("dart_sdk") {
        Some(path.into())
    } else {
        //We'll look for either a flutter sdk _or_ a choco tool path.
        if let Ok(path) = env::var("ChocolateyToolsLocation") {
            let path = PathBuf::from(path).join("dart-sdk");
            if let Ok(x) = std::fs::metadata(&path) {
                if x.is_dir() {
                    return Some(path);
                }
            }
        } else {
            let paths: Vec<PathBuf> = env::split_paths(
                &std::env::var("Path")
                    .ok()
                    .expect("Could not find $PATH variable."),
            )
            .collect();
            for i in &paths {
                if i.components().any(|x| x.as_os_str() == "dart-sdk") {
                    let mut path = i.clone();
                    while !path.is_dir() || path.file_name().unwrap() != "dart-sdk" {
                        path.pop();
                    }
                    if !path.as_os_str().is_empty() {
                        return Some(path);
                    }
                }
            }
            for i in &paths {
                if i.components().any(|x| x.as_os_str() == "flutter") {
                    let mut path = i.clone();
                    while !path.is_dir() || path.file_name().unwrap() != "flutter" {
                        path.pop();
                    }
                    if !path.as_os_str().is_empty() {
                        println!(
                            "cargo:warning=Using Flutter path by default. This is not recommended!"
                        );
                        return Some(path);
                    }
                }
            }
        }
        None
    }
}

#[cfg(windows)]
fn emit_compiler_flags() {
    let dart_path = match find_dart_sdk() {
        Some(x) => x,
        None => panic!("Could not find dart sdk!"),
    };
    let dart_path = PathBuf::from(dart_path);
    println!(
        r#"cargo:rustc-link-search=native={}"#,
        dart_path.join("bin").to_str().unwrap()
    );
    println!(r"cargo:rustc-link-lib=dart");
    // We're using the precompiled ones instead of using bindgen each time
    // let bindings = bindgen::Builder::default()
    //     .header("./bindgen/wrapper.h")
    //     .clang_arg(format!("--include-directory={}", dart_path.join("include").to_str().unwrap()))
    //     .clang_arg("-DDART_SHARED_LIB")
    //     .generate()
    //     .expect("Unable to generate bindings.");
    // let out_path = PathBuf::from(env::var("OUT_DIR").expect("Could not find OUT_DIR"));
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
    // panic!("OUT_DIR: {:?}", env::var("OUT_DIR").expect("Could not find OUT_DIR"));
}

#[cfg(unix)]
fn emit_compiler_flags() {}

fn main() {
    #[cfg(not(feature = "docs-only"))]
    fn _main() {
        emit_compiler_flags();
    }

    #[cfg(feature = "docs-only")]
    fn _main() {}

    _main();
}
