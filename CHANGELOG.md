## 1.37.2

* Fix compile errors when using enums within fields #564 (thanks @Desdaemon)

## 1.37.1

* Update doc #552 (thanks @dbsxdbsx)
* Bump dart release since previous one did not release successfully

## 1.37.0

* Allow generating multiple Rust and Dart files #527 (thanks @dbsxdbsx)

## 1.36.0

* Add support for stream sink into any argument #542 (thanks @lattice0)

## 1.35.0

* Fix for `store_dart_post_cobject` signature mismatch with `ffigen` >= 6.0 #536 (thanks @SecondFlight)
* Multiple blocks of files in one command #516 (thanks @dbsxdbsx)

## 1.34.2

* Bump dependency versions

## 1.34.1

* Add suitable `ignore_for_file`

## 1.34.0

* Generate Dart `constMeta` as a field, such that users can refer to it when needed #487

## 1.33.0

* Adding an option to prevent free_WireSyncReturnStruct function from being generated #481 (thanks @sccheruku)

## 1.32.0

* Allow `FlutterRustBridgeTimeoutMixin` to disable timeout

## 1.31.0

* Support `#[frb(metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]` for structs #463 (thanks @alanlzhang)

## 1.30.0

* Support non-final fields in Dart structs #452 (thanks @surban)

## 1.29.0

* Make code generator a lib to be used in build.rs; add error types for codegen; depend on cbindgen directly; update docs #434 (thanks @sagudev)
* Update dep in locks #441 (thanks @sagudev)
* Add support for usize and [T;N] #442 (thanks @trobanga)

## 1.28.1

* Fix LLVM path #431 (thanks @sagudev)
* Bump dependency

## 1.28.0

* Add doc for Android NDK bug #423 (thanks @AlienKevin)
* Update to match current template #426 (thanks @Desdaemon)
* Add LLVM 14 #416 (thanks @sagudev)

## 1.27.2

* Enhance documentation

## 1.27.1

* Format `frb_dart` package line length from 120 to 80

## 1.27.0

* Add brackets to boxed variable #414 (thanks @Syndim)

## 1.26.0

* Add default LLVM installation path for Windows #408 (thanks @Desdaemon)

## 1.25.0

* Ignore prefer_const_constructors in generated code #401 (thanks @Surban)
* Add IntoDartExceptPrimitive impl for enums #404 (thanks @Desdaemon)

## 1.24.0

* Skip unresolvable modules #400 (thanks @Surban)

## 1.23.0

* Fix extra comma in enum structs #391 (thanks @Desdaemon)

## 1.22.2

* Bump dependency versions

## 1.22.1

* Improve dart analyzer ignores

## 1.22.0

* Make mirroring work for more use cases: tuple structs, enum variants, wrapping in Vec and Optional #359 (thanks @Unoqwy)
* Bump dependency versions

## 1.21.1

* Bump dependency version

## 1.21.0

* CLI improvements: Run `build_runner` automatically, and more flags #363 (thanks @Desdaemon)
* Disable import parsing in source_graph #364 (thanks @Secondflight)

## 1.20.1

* More code comments and CI related to Corrosion #358 (thanks @Desdaemon)

## 1.20.0

* Add struct mirrors to use external types #352 (thanks @Unoqwy)
* Add examples to documentation

## 1.19.2

* Avoid converting syn types to strings before parsing #346 (thanks @antonok-edm)

## 1.19.1

* Documentation overhaul (thanks @Desdaemon as well)

## 1.18.0

* Refactor to beautify the implementation #338

## 1.17.0

* Support bridging functions with return types other than `Result` #335 (thanks @antonok-edm)

## 1.16.0

* Allow structs and enums to be imported into the api file from elsewhere in the crate #319 (thanks @SecondFlight)
* Refactor command module to fix Windows build failures #325 (thanks @Desdaemon)
* Quote arguments when calling external commands #322 (thanks @Desdaemon)
* Update examples to fix compile error
* Bump dependency version #314

## 1.15.1

* Bump dependency version #311

## 1.15.0

* Fix potential name collision with port argument #305 (thanks @valeth)

## 1.14.0

* Separate generated definitions from implementations #298

## 1.13.0

* When running codegen, create folders for output paths if they don't exist #286 (thanks @SecondFlight)

## 1.12.0

* Redesign documentation and make it a mdBook #272
* Remove `syn` dependency from macros to speed up #277 (thanks @Desdaemon)

## 1.11.0

* Marker attributes for expressiveness #261 (thanks @Desdaemon)
* Improvements #267 (thanks @Desdaemon): Resolve #265, Resolve #266, Fix attributes not working on enum variants, Add comments on enum variants and fields, (Internal) unify tuple and normal enum structs
* Avoid user parameter collision in wire functions #270 (thanks @Desdaemon)

## 1.10.0

* Support enum structs (Rust's expressive enums with fields) #256 (thanks @Desdaemon)

## 1.9.1

* Bump versions of dependencies

## 1.9.0

* Support field-less enum types #239 (thanks @Desdaemon)

## 1.8.2

* Fix bug that returning u32 from Rust results in wrong value received by Dart #234

## 1.8.1

* Update example and doc.

## 1.8.0

* Add support for unit return type #198 (thanks @surban)
* Add flutter example for macOS and add macOS instructions in README #211 (thanks @AlienKevin)

## 1.7.0

* Enrich metadata of generated ffi calls - now we can have more "reflection" information.
* Add llvm-compiler-opts as command-line argument #210 (thanks @trobanga)

## 1.6.1

* Remove extra newline on empty comment #203 (thanks @Desdaemon)

## 1.6.0

* Implement `Vec<String>` #193 (thanks @Desdaemon)
* Add logging for FlutterRustBridgeSetupMixin for users to debug easily.
* Set names of threads for `ThreadPoolExecutor` to make debugging easier when looking at threads.

## 1.5.0

* Copy Rust comments over to Dart generated file (#182, thanks @Desdaemon)

## 1.4.0

* Support synchronous function calls in addition to existing asynchronous Future and Stream approaches (#175, #176)
* Remove unnecessary dependency of `lint` for the Dart package.

## 1.3.0

* Support more types of the form`Vec<primitive_type>` and `ZeroCopyBuffer<Vec<primitive_type>>`, such as `Vec<f32>` and `ZeroCopyBuffer<Vec<f32>>` to be transformed into `Float32List` in Dart. (#162, #153)
* Do not generate unnecessary Dart to Rust wire code to fix bugs such as when `Vec<ZeroCopyBuffer<Vec<u8>>>` is in output argument.
* Warn when `ffigen` emits any `[SEVERE]` log messages.
* Make outputs change less when input of codegen changes.
* Simplify `Wire2Api<Option<T>>` generated code.

## 1.2.2

* Add Linux and Windows out-of-the-box support for the `with_flutter` example.
* Improve linter hints

## 1.2.1

* Add `--skip-add-mod-to-lib` flag.
* Allow Rust input file in directories besides root directory of the crate.
* Warn when command's output seems to indicate errors.
* Do not include `stdarg.h` automatically (related: #108 and #53).
* Fix windows path handling problem (#119, thanks @smw-wagnerma).
* Add `--llvm-path` flag.

## 1.2.0

* Enable `Option<T>` types to be transformed (thanks @Desdaemon)
* Support `Stream`s: call function once, "return" multiple times with different data.
* Add `FlutterRustBridgeSetupMixin` (an optional helper class), which allows custom setup hooks before ffi can be executed.
* Add `hint` parameter in generated Dart code, allowing users to pass custom data to the Dart executor, thus increasing flexibility.
* Improve panic handling in extreme cases (avoid panic across languages, which is undefined behavior).
* Refactored `Handler`, now it is much easier to customize your own handler functionality.
* Remove one `Box::new(FnOnce)`, thus enables better inlining for ffi function calls.
* Fix bug: Dart struct(class) is not generated if the struct only appears in the return type #98.
* Add `FlutterRustBridgeTimeoutMixin`. If used, a timeout exception will be thrown for ffi calls that do not return within time limit.

## 1.1.0

* Generate `dummy_method_to_enforce_bundling` to avoid "symbols not found" problems in iOS release build
* Allow customizations for generated Dart classes
* Add pure-Dart tutorial
* Update examples and tutorials, and fix outdated documentations
* Formatting problems for generated code

## 1.0.3

* Fix bugs and add features (details to be written later)

## 1.0.2

* Fix bugs and add features (details to be written later)

## 1.0.1

* Fix bugs and add features (details to be written later)

## 1.0.0

* Initial release
