## 2.0.0-dev.1

* Please refer to TODO for what's changed in V2.

## 1.82.6

* Bump README (since before 2.0 stable version, the 1.x will be the default page on pub.dev etc)

## 1.82.5

* Fix `rustfmt::skip` #1420 (thanks @rogurotus)

## 1.82.4

* Fix `Result<SyncReturn(T), E>` #1407 (thanks @rogurotus)

## 1.82.3

* Fix Rust error string not displayed in Dart #1390 (thanks @OfficialBoyfriend)

## 1.82.2

* Refactor `Vec<Option<_>>` #1388 (thanks @Desdaemon)

## 1.82.1

* Fix cargo expand missing code #1372 (thanks @erikas-taroza)
* Use delegate-attr to simplify code #1364 (thanks @upsuper)

## 1.82.0

* Support for Result<T, E> with exception throwing on Dart (thanks @lattice0, @SiongSng, @JustSimplyKyle)
* Fix invalid sdk spec syntax #1331 (thanks @aran)
* Document some common android fixes #1335 (thanks @powpingdone)
* Add support of `Vec<bool>` (thanks @NightFeather0615)
* Update ffigen version (thanks @aran)

## 1.81.0

* Allow macros to be used and will be expanded #1320 (thanks @MnlPhlp)

## 1.80.2

* Fix and test for SyncReturn of mirrored type #1315 (thanks @MnlPhlp)
* Fix better check for whether the file need to generate freezed structs/enums #1312 (thanks @SiongSng)

## 1.80.1

* Manually bump proc-macro2 -> 1.0.66 #1306 (thanks @gutenfries)

## 1.80.0

* Reinforce error handling and reduce usage of panic in codegen #1298 (thanks @Desdaemon)
* fix: sync return with Freezed metadata #1304 (thanks @SiongSng)

## 1.79.0

* Update example tab on pub.dev and remove low-quality dependency #1263
* Adjust type constraints to allow mirror types in Streams #1285 (thanks @MnlPhlp)

## 1.78.0

* Translate tuples to records and enums to sealed classes #1238 (thanks @Desdaemon)

## 1.77.1

* Fix on zero-copy cargo feature #1229 (thanks @temeddix)

## 1.77.0

* Provide `zero-copy` cargo feature #1228 （thanks @temeddix)
* Add steps in macOS integration in docs #1227 (thanks @temeddix)

## 1.76.0

* Support Dart 3 and Flutter 3.10 (thanks @Zaitam for partial impl)

## 1.75.3

* Fix bridge access for Rust opaques #1206 (thanks @Desdaemon)

## 1.75.2

* Warn user about using lib.rs as a Rust input #1193 (thanks @erikas-taroza)
* Allow enum member in struct, and fix enum errors in dart strict dynamic checker #1202 (thanks @alanlzhang)

## 1.75.1

* Fix compile error when building frb_codegen w/o serde feature #1185 (thanks @phlip9)

## 1.75.0

* Allow removing `bridge` field in dart model #1170 (thanks @huang12zheng)

## 1.74.0

* Allow RustOpaque to be non-Clone #989 (thanks @anlumo)

## 1.73.0

* Support generating Dart structs with freezed + methods #970 (thanks @anlumo)

## 1.72.2

* Fix: Handle fallible case for TypeGeneralListGenerator. #1149 (thanks @trobanga)

## 1.72.1

* Fix for enums that contain mirrored structs Enum1(MyStruct) and Unit Tests #1144 (thanks @alexthe2)

## 1.72.0

* Added support for raw strings #1139 (thanks @alexthe2)

## 1.71.2

* Code refactor #1137
* Remove SupportedInnerType #1117 (thanks @coder0xff)

## 1.71.1

* Fix log_initial for panic! and refine a panic! info #1124 (thanks @dbsxdbsx)

## 1.71.0

* Support for list of primitive enums #1128 (thanks @erikas-taroza)

## 1.70.0

* Add tests to gen, build, run pure_dart #1106 (thanks @coder0xff)
* Generate Dart enum variants in camelCase #1112 (thanks @erikas-taroza)
* General CLI improvements: dumping, upgrade clap, yaml config, serialize IR, bump versions, etc #1119 (thanks @Desdaemon)
* Fixes for serde-yml #1122 (thanks @Desdaemon)

## 1.69.0

* Check for dart keywords #958 (thanks @anstadnik)

## 1.68.0

* feat: return dynamic data as `DartAbi` #1102 (thanks @Desdaemon)
* fix: incorrect ffi.Usize type #1102 (thanks @Desdaemon)
* fix: default on freezed structs not applied #1101 (thanks @Desdaemon)

## 1.67.0

* Refractor and refine for frb_codegen #1093 (thanks @dbsxdbsx)
* Translate Rust method comments #1094 (thanks @Desdaemon)
* Add documentation for Rust versions 1.68 and above with the latest Android NDK #1096 (thanks @polypixeldev)
* Parameter defaults, make structs const-constructible #1095 (thanks @Desdaemon)

## 1.66.0

* Implement `List<DateTime>`, `List<Duration>` #972 (thanks @Desdaemon)

## 1.65.1

* Fix the duplicated "dummy_method_to_enforce_bundling" in C header within multi-blocks #1024 (thanks @dbsxdbsx)

## 1.65.0

* Support empty structs #1071 (thanks @alexthe2)
* Fix Optional<DateTime> #1079 (thanks @alexthe2)

## 1.64.0

* Support Option<Datetime> #1066 (thanks @alexthe2)

## 1.63.1

* Fix the problem with using worker-max feature in WASM #1056 (thanks @temeddix)

## 1.63.0

* Provide cargo features to change the number of pool workers #1026 (thanks @temeddix)

## 1.62.1

* Add logging to disk for frb_codegen #1019 (thanks @dbsxdbsx)
* Upgrade dart sys, upgrade ci version, and fix linters #1023 (thanks @rogurotus)

## 1.62.0

* Migration to dart-sys #998 (thanks @rogurotus)

## 1.61.1

* Add import package:meta/meta.dart to all generated files #980 (thanks @jsonmona)

## 1.61.0

* Allow opt-out of WASM initializer #963 (thanks @Desdaemon)

## 1.60.0

* Refactors for `Boxed` and `Option` (Refactors `EnumRef` to make `Option<FieldlessEnum>` work; `Box<primitive>` and `Option<Box<primitive>>` on WASM no longer allocates a Box, but receives a (nullable) value directly from Dart) #949 thanks @Desdaemon

## 1.59.0

* Fix unexpected content in c output header #937 (thanks @dbsxdbsx)

## 1.58.2

* Fix `SyncReturn<RustOpaque<T>>` #934 (thanks @rogurotus)

## 1.58.1

* fix SyncReturn restrictions and update CI #936 (thanks @rogurotus)

## 1.58.0

* Support nest type alias with topology sort #929 (thanks @huang12zheng)

## 1.57.0

* Make sync mode support whatever types that classical async mode supports #882 (thanks @ngasull)
* docs: add Dart/Flutter library setup documentation #899 (thanks @GregoryConrad)
* Fix lookup symbol store dart post cobject #898 (thanks @Roms1383)

## 1.56.0

* Return error when rust input file cannot be read #912 (thanks @w1th0utnam3)

## 1.55.1

* Fix mirroring to support `Result` return type and `Option<T>` field #907 (thanks @codercengiz)
* Bump Dart SDK to 2.15 #906 (thanks @ngasull)

## 1.55.0

* Bump chrono #905 (thanks @Roms1383)
* Support type aliases #900 (thanks @huang12zheng)

## 1.54.1

* Delete dart_sys #890 (thanks @rogurotus)

## 1.54.0

* Extend `SyncReturn` to support `RustOpaque`, `DartOpaque`, `Option` and so on #876 (thanks @rogurotus)

## 1.53.0

* Add Dart opaque types, allowing to use any Dart objects in Rust code #853 (thanks @rogurotus)

## 1.52.0

* Move semantics of opaque rust for Dart #869 (thanks @rogurotus)

## 1.51.1

* Fix function generation related to opaque rust #867 (thanks @rogurotus)

## 1.51.0

* support wasm with no decl set #861 (thanks @huang12zheng)

## 1.50.0

* Implement opaque types, enabling arbitrary Rust structs to be used as opaque Dart objects, by generating wrappers and raw Arc pointers #795 (thanks @rogurotus)

## 1.49.2

* Fix parsing of packages in pubspec.yaml that have no explicit version specification #846 (thanks @banool)

## 1.49.1

* Bump constraint on ffigen #823 (thanks @CicadaCinema)
* Set default version strategy requirement for chrono #821 (thanks @vincent-herlemont)

## 1.49.0

* Fix return for struct with methods #764 (thanks @Zaitam)
* Support array as parameter types #623 (thanks @Cupnfish)

## 1.48.1

* Pass JS BigInt to wire #747 (thanks @Desdaemon)

## 1.48.0

* Support uuid #728 (thanks @Roms1383)

## 1.47.1

* Allow streaming functions to omit return type #730 (thanks @Desdaemon)

## 1.47.0

* Support chrono date time #694 (thanks @Roms1383)

## 1.46.0

* Fix WireSyncReturnStruct should be freed after buffer being consumed #720 (thanks @hsfzxjy)

## 1.45.0

* Add support for the Web platform, parallel to the existing mobile/desktop platforms, via WASM and JavaScript as intermediate values #589 (thanks @Desdaemon)

## 1.44.0

* Bump dependency versions

## 1.43.0

* Add crate version to generated code header #666 (thanks @Roms1383)

## 1.42.0

* Refactor and enhance SyncReturn to support more types #663 (thanks @SoLongAndThanksForAllThePizza)

## 1.41.3

* Fix "Skipping unresolvable module" by align latest #651 (thanks @alanlzhang)

## 1.41.2

* Add cli arg to skip dependencies check #640 (thanks @Roms1383)

## 1.41.1

* Fix bug with conflicting names in enum name and variant #604 (thanks @Roms1383)

## 1.41.0

* Allow multi-`mirror` #619 (thanks @Cupnfish)

## 1.40.0

* Improve version check #613 (thanks @Roms1383)

## 1.39.0

* Avoid global ffigen #605 (thanks @Roms1383)
* Code cleanup #603 (thanks @Roms1383)

## 1.38.2

* Run build_runner either for Dart or Flutter #599 (thanks @Roms1383)

## 1.38.1

* Fix case when returning a struct with a method in a non method function #587 (thanks @lattice0)

## 1.38.0

* Support methods, such that Rust struct impls can be converted to Dart class methods #543 (thanks @lattice0)

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
