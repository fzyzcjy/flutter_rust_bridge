use std::cfg;
use std::env;
use std::path::PathBuf;

fn find_dart_sdk() -> Option<PathBuf> {
    if let Ok(path) = env::var("dart_sdk") {
        // Check for the dart SDK in the dart_sdk environment variable
        Some(path.into())
    } else {
        // Check for the dart SDK in the PATH variable
        let paths: Vec<PathBuf> = env::split_paths(
            &std::env::var("PATH")
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
                    println!("cargo:warning=INFO: Defaulting to Flutter-provided dart SDK.");
                    return Some(path.join("bin/cache/dart-sdk"));
                }
            }
        }
        None
    }
}

fn emit_compiler_flags() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => {
            let dart_path = match find_dart_sdk() {
                Some(x) => {
                    println!("cargo:warning=INFO: Found dart sdk at: {}", x.to_str().unwrap());
                    x
                },
                None => panic!("Could not find a dart SDK.\nPlease install the Dart SDK or set the dart_sdk environment variable to the path of the Dart SDK.\nIf you have Flutter installed, the dart SDK will be found automatically."),
            };
            let dart_path = PathBuf::from(dart_path);
            println!(
                r#"cargo:rustc-link-search=native={}"#,
                dart_path.join("bin").to_str().unwrap()
            );
            println!(r"cargo:rustc-link-lib=dart");
        }
        _ => println!("INFO: Target OS is not windows"),
    }
}

fn main() {
    #[cfg(not(feature = "docs-only"))]
    emit_compiler_flags();
}
