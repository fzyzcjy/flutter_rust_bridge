use lib_flutter_rust_bridge_codegen::codegen;
use crate::binary::commands::GenerateCommandArgs;

pub(crate) fn parse_generate_command_args(args: GenerateCommandArgs) -> codegen::Config {
    if no_args {
        todo!("from yaml")
    }

    if args.config_file {
        todo!()
    }

    codegen::Config {
        rust_input: args.rust_input,
        dart_output: args.dart_output,
        dart_decl_output: args.dart_decl_output,
        c_output: args.c_output,
        rust_crate_dir: args.rust_crate_dir,
        rust_output: args.rust_output,
        class_name: args.class_name,
        dart_format_line_length: args.dart_format_line_length,
        dart_enums_style: Some(args.dart_enums_style),
        add_mod_to_lib: Some(!args.no_add_mod_to_lib),
        llvm_path: args.llvm_path,
        llvm_compiler_opts: args.llvm_compiler_opts,
        dart_root: args.dart_root,
        build_runner: Some(!args.no_build_runner),
        use_bridge_in_method: Some(!args.no_use_bridge_in_method),
        extra_headers: args.extra_headers,
        verbose: Some(args.verbose),
        wasm: Some(args.wasm),
        inline_rust: Some(args.inline_rust),
        skip_deps_check: Some(args.skip_deps_check),
        dump: args.dump,
        dart3: Some(!args.no_dart3),
        keep_going: Some(args.keep_going),
    }
}
