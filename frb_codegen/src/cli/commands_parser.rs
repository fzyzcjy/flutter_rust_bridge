use lib_flutter_rust_bridge_codegen::codegen;
use crate::cli::commands::GenerateCommandArgs;

impl GenerateCommandArgs {
    pub(crate) fn into_config(self) -> codegen::Config {
        codegen::Config {
            rust_input: self.rust_input,
            dart_output: self.dart_output,
            dart_decl_output: self.dart_decl_output,
            c_output: self.c_output,
            rust_crate_dir: self.rust_crate_dir,
            rust_output: self.rust_output,
            class_name: self.class_name,
            dart_format_line_length: self.dart_format_line_length,
            dart_enums_style: Some(self.dart_enums_style),
            add_mod_to_lib: Some(!self.no_add_mod_to_lib),
            llvm_path: self.llvm_path,
            llvm_compiler_opts: self.llvm_compiler_opts,
            dart_root: self.dart_root,
            build_runner: Some(!self.no_build_runner),
            use_bridge_in_method: Some(!self.no_use_bridge_in_method),
            extra_headers: self.extra_headers,
            verbose: Some(self.verbose),
            wasm: Some(self.wasm),
            inline_rust: Some(self.inline_rust),
            skip_deps_check: Some(self.skip_deps_check),
            dump: self.dump,
            dart3: Some(!self.no_dart3),
            keep_going: Some(self.keep_going),
        }
    }
}

