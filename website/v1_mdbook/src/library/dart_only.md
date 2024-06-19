# Dart-only base
This page details how to set up the initial structure of our monorepo,
including the crucial Dart-only base package.

## Initialization script
This script creates a new monorepo named `$LIBNAME` in the current working directory
and initializes it with some needed files.
The following script assumes a bash shell, which you should make sure to use to run it.
Also, the script generates some ffi convenience files in your Dart `src/` which you should
check out after the script completes.

After the script runs, change the `flutter_rust_bridge` dependency in `/packages/library_name/pubspec.yaml` to the following:
```yaml
  flutter_rust_bridge: "1.62.1"
```
Note: If you so choose, you can update the `flutter_rust_bridge` versions in
`/packages/library_name/native/Cargo.toml` and `/packages/library_name/pubspec.yaml`
to the latest version available, but newer versions are not guaranteed to work
with this section of the guide due to a lack of CI testing.
Version 1.62.1 is known to work with this guide as-is.
CI testing is planned once the Native Assets feature is released.

Finally, change the variables at the top of the script to fit your needs.

```bash
LIBNAME=library_name # snake_case
DART_CLASS_NAME=LibraryName # probably is PascalCase version of $LIBNAME

# Monorepo setup
mkdir -p $LIBNAME/packages
cd $LIBNAME
git init

cat << EOF >> Cargo.toml
[workspace]
members = ["packages/$LIBNAME/native"]
EOF

cat << EOF >> analysis_options.yaml
# TODO change the below options/lints as you see fit
analyzer:
  exclude:
    - '**.freezed.dart'
    - '**.g.dart'
  language:
    strict-inference: true
    strict-raw-types: true
  errors:
    invalid_annotation_target: ignore

linter:
  rules:
    # Custom lints
    - prefer_single_quotes

    # Core Dart lints
    - avoid_empty_else
    - avoid_relative_lib_imports
    - avoid_shadowing_type_parameters
    - avoid_types_as_parameter_names
    - await_only_futures
    - camel_case_extensions
    - camel_case_types
    - curly_braces_in_flow_control_structures
    - depend_on_referenced_packages
    - empty_catches
    - file_names
    - hash_and_equals
    - iterable_contains_unrelated_type
    - list_remove_unrelated_type
    - no_duplicate_case_values
    - non_constant_identifier_names
    - null_check_on_nullable_type_parameter
    - package_prefixed_library_names
    - prefer_generic_function_type_aliases
    - prefer_is_empty
    - prefer_is_not_empty
    - prefer_iterable_whereType
    - prefer_typing_uninitialized_variables
    - provide_deprecation_message
    - unnecessary_overrides
    - unrelated_type_equality_checks
    - valid_regexps
    - void_checks

    # Recommended Dart lints
    - always_require_non_null_named_parameters
    - annotate_overrides
    - avoid_function_literals_in_foreach_calls
    - avoid_init_to_null
    - avoid_null_checks_in_equality_operators
    - avoid_renaming_method_parameters
    - avoid_return_types_on_setters
    - avoid_returning_null_for_void
    - avoid_single_cascade_in_expression_statements
    - constant_identifier_names
    - control_flow_in_finally
    - empty_constructor_bodies
    - empty_statements
    - exhaustive_cases
    - implementation_imports
    - library_names
    - library_prefixes
    - library_private_types_in_public_api
    - no_leading_underscores_for_library_prefixes
    - no_leading_underscores_for_local_identifiers
    - null_closures
    - overridden_fields
    - package_names
    - prefer_adjacent_string_concatenation
    - prefer_collection_literals
    - prefer_conditional_assignment
    - prefer_contains
    - prefer_equal_for_default_values
    - prefer_final_fields
    - prefer_for_elements_to_map_fromIterable
    - prefer_function_declarations_over_variables
    - prefer_if_null_operators
    - prefer_initializing_formals
    - prefer_inlined_adds
    - prefer_interpolation_to_compose_strings
    - prefer_is_not_operator
    - prefer_null_aware_operators
    - prefer_spread_collections
    - prefer_void_to_null
    - recursive_getters
    - slash_for_doc_comments
    - type_init_formals
    - unnecessary_brace_in_string_interps
    - unnecessary_const
    - unnecessary_constructor_name
    - unnecessary_getters_setters
    - unnecessary_late
    - unnecessary_new
    - unnecessary_null_aware_assignments
    - unnecessary_null_in_if_null_operators
    - unnecessary_nullable_for_final_variable_declarations
    - unnecessary_string_escapes
    - unnecessary_string_interpolations
    - unnecessary_this
    - use_function_type_syntax_for_parameters
    - use_rethrow_when_possible

    # Flutter lints
    - avoid_print
    - avoid_unnecessary_containers
    - avoid_web_libraries_in_flutter
    - no_logic_in_create_state
    - prefer_const_constructors
    - prefer_const_constructors_in_immutables
    - prefer_const_declarations
    - prefer_const_literals_to_create_immutables
    - sized_box_for_whitespace
    - sort_child_properties_last
    - use_build_context_synchronously
    - use_full_hex_values_for_flutter_colors
    - use_key_in_widget_constructors
EOF

cat << EOF >> .gitignore
# Miscellaneous
*.class
*.log
*.pyc
*.swp
.DS_Store
.atom/
.buildlog/
.history
.svn/

# IntelliJ related
*.iml
*.ipr
*.iws
.idea/

# The .vscode folder contains launch configuration and tasks you configure in
# VS Code which you may wish to be included in version control, so this line
# is commented out by default.
#.vscode/

# Flutter/Dart/Pub related
pubspec.lock
pubspec_overrides.yaml
**/doc/api/
.dart_tool/
.packages
build/
.pub-cache/
.pub/
.flutter-plugins
.flutter-plugins-dependencies

# Rust related
/target/
/Cargo.lock
/platform-build
EOF

# Dart setup
DART_BASE=packages/$LIBNAME
dart create --template=package $DART_BASE
(cd $DART_BASE && dart pub add flutter_rust_bridge ffi && dart pub add ffigen --dev)
rm $DART_BASE/analysis_options.yaml # we provide our own in repo root
( # ffi setup
cd $DART_BASE
mkdir -p lib/src/ffi

cat << EOF >> lib/src/ffi/stub.dart
import 'package:$LIBNAME/src/bridge_generated.dart';

/// Represents the external library for $LIBNAME
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

$DART_CLASS_NAME createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();
EOF

cat << EOF >> lib/src/ffi/io.dart
import 'dart:ffi';

import 'package:$LIBNAME/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

$DART_CLASS_NAME createWrapperImpl(ExternalLibrary dylib) =>
    ${DART_CLASS_NAME}Impl(dylib);
EOF

cat << EOF >> lib/src/ffi/web.dart
import 'package:$LIBNAME/src/bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

typedef ExternalLibrary = WasmModule;

$DART_CLASS_NAME createWrapperImpl(ExternalLibrary module) =>
    ${DART_CLASS_NAME}Impl.wasm(module);
EOF

cat << EOF >> lib/src/ffi.dart
import 'bridge_generated.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.js_interop) 'ffi/web.dart';

$DART_CLASS_NAME? _wrapper;

$DART_CLASS_NAME createWrapper(ExternalLibrary lib) {
  _wrapper ??= createWrapperImpl(lib);
  return _wrapper!;
}
EOF

echo "export 'src/ffi.dart';" >> lib/$LIBNAME.dart
)

# Rust setup
RUST_BASE=$DART_BASE/native
mkdir -p $RUST_BASE/src

cat << EOF >> $RUST_BASE/build.rs
use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

const RUST_INPUT: &str = "src/api.rs";
const DART_OUTPUT: &str = "../lib/src/bridge_generated.dart";

const IOS_C_OUTPUT: &str = "../../flutter_$LIBNAME/ios/Classes/frb.h";
const MACOS_C_OUTPUT_DIR: &str = "../../flutter_$LIBNAME/macos/Classes/";

fn main() {
    // Tell Cargo that if the input Rust code changes, rerun this build script
    println!("cargo:rerun-if-changed={}", RUST_INPUT);

    // Options for frb_codegen
    let raw_opts = RawOpts {
        rust_input: vec![RUST_INPUT.to_string()],
        dart_output: vec![DART_OUTPUT.to_string()],
        c_output: Some(vec![IOS_C_OUTPUT.to_string()]),
        extra_c_output_path: Some(vec![MACOS_C_OUTPUT_DIR.to_string()]),
        inline_rust: true,
        wasm: true,
        ..Default::default()
    };

    // Generate Rust & Dart ffi bridges
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    // Format the generated Dart code
    _ = std::process::Command::new("flutter")
        .arg("format")
        .arg("..")
        .spawn();
}
EOF

cat << EOF >> $RUST_BASE/.gitignore
# Rust library related
Cargo.lock
target
EOF

cat << EOF >> $RUST_BASE/Cargo.toml
[package]
name = "$LIBNAME"
version = "0.0.0"
edition = "2018"

[lib]
crate-type = ["staticlib", "cdylib"]

[build-dependencies]
flutter_rust_bridge_codegen = "1.62.*"

[dependencies]
flutter_rust_bridge = "1.62.*"
EOF

touch $RUST_BASE/src/api.rs

cat << EOF >> $RUST_BASE/src/lib.rs
mod api;
EOF

cargo build
```
