# Command line arguments

Simply add `--help` to see full documentation. The following is a (possibly outdated) snapshot when running the command with `--help`:

```shell
flutter_rust_bridge_codegen 1.20.1

USAGE:
    flutter_rust_bridge_codegen [FLAGS] [OPTIONS] --dart-output <dart-output> --rust-input <rust-input>

FLAGS:
        --skip-add-mod-to-lib    Skip automatically adding `mod bridge_generated;` to `lib.rs`
        --no-build-runner        Skip running build_runner even when codegen-capable code is detected
    -v, --verbose                Show debug messages
    -h, --help                   Prints help information
    -V, --version                Prints version information

OPTIONS:
    -r, --rust-input <rust-input>                              Path of input Rust code
    -d, --dart-output <dart-output>                            Path of output generated Dart code
        --dart-decl-output <dart-decl-output>
            If provided, generated Dart declaration code to this separate file

    -c, --c-output <c-output>...                               Path of output generated C header
        --rust-crate-dir <rust-crate-dir>                      Crate directory for your Rust project
        --rust-output <rust-output>                            Path of output generated Rust code
        --class-name <class-name>                              Generated class name
        --dart-format-line-length <dart-format-line-length>    Line length for dart formatting
        --llvm-path <llvm-path>...                             Path to the installed LLVM
        --llvm-compiler-opts <llvm-compiler-opts>              LLVM compiler opts
        --dart-root <dart-root>
            Path to root of Dart project, otherwise inferred from --dart-output
```

