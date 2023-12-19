* [Desdaemon](https://github.com/Desdaemon): Support not only simple enums but also enums with fields which gets translated to native enum or sealed freezed class in Dart. Support the Option type as nullable types in Dart. Support Vec of Strings type. Support tuple type. Support comments in code. Add marker attributes for future usage. Add Linux and Windows support for with-flutter example, and make CI works for that. Avoid parameter collision. Overhaul the documentation and add several chapters to demonstrate configuring a Flutter+Rust project in all five platforms. Refactor command module. Precompiled binary CI workflow. Fix bugs. Add support for the Web platform, parallel to the existing mobile/desktop platforms, via WASM and JavaScript as intermediate values. GitHub retry actions. Implement draft of opaque types. Refactor Boxed and Option. Impl list of dates and optionals. Parameter defaults. Refactor CLI. Refactor codegen errors.
* [rogurotus](https://github.com/rogurotus): Add Rust opaque types, enabling arbitrary Rust structs to be used as opaque Dart objects by generating wrappers and raw Arc pointers. Also add Dart opaque types, allowing to use any Dart objects in Rust code. Extend `SyncReturn` for more types. Fix generation bug. Fix SyncReturn. Migrate to dart-sys. Update CI. Fix linters. Fix SyncReturn bug.
* [ngasull](https://github.com/ngasull): Make sync mode support whatever types that classical async mode supports. Bump sdk.
* [SecondFlight](https://github.com/SecondFlight): Allow structs and enums to be imported from other files within the crate by creating source graph. Auto-create relevant dir. Fix `store_dart_post_cobject` error with ffigen 6.0.
* [lattice0](https://github.com/lattice0): Implement hierarchy of exceptions. Support methods, such that Rust struct impls can be converted to Dart class methods. StreamSink at any argument.
* [Unoqwy](https://github.com/Unoqwy): Add struct mirrors, such that types in the external crates can be imported and used without redefining and copying.
* [antonok-edm](https://github.com/antonok-edm): Avoid converting syn types to strings before parsing to improve code and be more robust.
* [sagudev](https://github.com/sagudev): Make code generator a `lib`. Add error types. Depend on `cbindgen`. Fix LLVM paths. Update deps. Fix CI errors.
* [surban](https://github.com/surban): Support unit return type. Skip unresolvable modules. Ignore prefer_const_constructors. Non-final Dart fields.
* [Roms1383](https://github.com/Roms1383): Fix build_runner calling bug. Remove global `ffigen` dependency. Improve version check. Fix enum name-variant conflicts. Support Chrono date time and UUID types. Migrate to Rust 1.64 workspace. Update and refactor CI. Update header comments. Code cleanup.
* [dbsxdbsx](https://github.com/dbsxdbsx): Allow generating multiple Rust and Dart files. Fix lint. Update doc. Add logging.
* [GregoryConrad](https://github.com/GregoryConrad): Add doc to setup frb inside a Dart/Flutter library.
* [huang12zheng](https://github.com/huang12zheng): Support type aliases and nested ones. Tweak code generation. Fix rust_build_and_test on Mac. Improve CI logic and cache. Remove bridge field in model.
* [trobanga](https://github.com/trobanga): Add support for `[T;N]` structs. Add `usize` support. Add a cmd argument. Separate dart tests. Fix fallible list case.
* [MnlPhlp](https://github.com/MnlPhlp): Support macros and will auto expand. Allow mirror types in streams.
* [SoLongAndThanksForAllThePizza](https://github.com/SoLongAndThanksForAllThePizza): Refactor and enhance SyncReturn to support more types. Refactor post-release CI.
* [hsfzxjy](https://github.com/hsfzxjy): Fix SyncReturn use-after-free bug.
* [Cupnfish](https://github.com/Cupnfish): Support arrays as function parameters. Allow multi mirror.
* [alanlzhang](https://github.com/alanlzhang): Add generation for Dart metadata. Enhance module parser. Fix enum in struct. Fix linter.
* [erikas-taroza](https://github.com/erikas-taroza): Support list of primitive enums. Make enum camelCase. Warn wrong path. Fix cargo expand.
* [SiongSng](https://github.com/SiongSng): Finish implementing exception hierarchy. Fix SyncReturn bug.
* [JustSimplyKyle](https://github.com/JustSimplyKyle): Also finish implementing exception hierarchy.
* [Zaitam](https://github.com/Zaitam): Fix when method return struct. Partial migration to Dart 3.
* [coder0xff](https://github.com/coder0xff): Discuss binding unmodified Rust. Refactor SupportedInnerType. Extra codegen tester.
* [nitn3lav](https://github.com/nitn3lav): Nested `struct`s without `Box`.
* [alexthe2](https://github.com/alexthe2): Add Option Datetime. Add empty structs. Improve doc. Add `r#`. Fix mirror enum bug.
* [AlienKevin](https://github.com/AlienKevin): Add flutter example for macOS. Add doc for Android NDK bug.
* [banool](https://github.com/banool): Fix pubspec parsing. Fix symbol-stripping doc.
* [anlumo](https://github.com/anlumo): Fix freezed + methods. Non-clone RustOpaque.
* [temeddix](https://github.com/temeddix): Fix broken CI. Custom num workers. Fix MacOS doc steps. Update doc. Make zero-copy defaultable.
* [NightFeather0615](https://github.com/NightFeather0615): Fix Vec bool.
* [OfficialBoyfriend](https://github.com/OfficialBoyfriend): Fix error display.
* [w-ensink](https://github.com/w-ensink): Improve doc. Fix CI. Refactor. Add tests.
* [smw-wagnerma](https://github.com/smw-wagnerma): Improve Windows encoding handling.
* [powpingdone](https://github.com/powpingdone): Document JNI init and libc++_static linking.
* [valeth](https://github.com/valeth): Rename callFfi's port.
* [sccheruku](https://github.com/sccheruku): Prevent double-generating utility.
* [upsuper](https://github.com/upsuper): Refactor delegate-attr.
* [jsonmona](https://github.com/jsonmona): Add import.
* [codercengiz](https://github.com/codercengiz): Fix mirroring bug.
* [Michael-F-Bryan](https://github.com/Michael-F-Bryan): Detect broken bindings.
* [phlip9](https://github.com/phlip9): Fix no-serde compilation.
* [bus710](https://github.com/bus710): Add a case in troubleshooting.
* [Demezy](https://github.com/Demezy): Mention troubleshooting.
* [gutenfries](https://github.com/gutenfries): Bump proc-macros.
* [anstadnik](https://github.com/anstadnik): Check keywords.
* [aran](https://github.com/aran): Fix pubspec. Bump version.
* [syndim](https://github.com/syndim): Add a bracket to box.
* [TENX-S](https://github.com/TENX-S): Improve doc. Reproduce a bug.
* [polypixeldev](https://github.com/polypixeldev): Improve doc.
* [CicadaCinema](https://github.com/CicadaCinema): Bump version. Improve doc.
* [w1th0utnam3](https://github.com/w1th0utnam3): Improve message.
* [vincent-herlemont](https://github.com/vincent-herlemont): Loosen version.
* [zaynetro](https://github.com/zaynetro): Improve doc.
* [raphaelrobert](https://github.com/raphaelrobert): Remove oudated doc.
* [DMouayad](https://github.com/DMouayad): Improve doc.
* [elliotsayes](https://github.com/elliotsayes): Improve doc.
* [tmpfs](https://github.com/tmpfs): Improve doc.
* [thomas725](https://github.com/thomas725): Improve doc.
* [juzi5201314](https://github.com/juzi5201314): Improve doc.
* [Voklen](https://github.com/Voklen): Improve doc.
* [svenstaro](https://github.com/svenstaro): Improve doc.
* [utilForever](https://github.com/utilForever): Fix typos.
* [not-holar](https://github.com/not-holar): Fix typos.
* [Stonks3141](https://github.com/Stonks3141): Fix doc credit.
* [vimaxwell](https://github.com/vimaxwell): Fix doc link.
* [lker-dev](https://github.com/lker-dev): Fix doc link.
* [jaiakash](https://github.com/jaiakash): Fix doc link.
* [feber](https://github.com/feber): Fix doc link.
* [satvikpendem](https://github.com/satvikpendem): Little co-work #989.
* [wxitcode](https://github.com/wxitcode): Fix a typo.
* [rustui](https://github.com/rustui): Fix a typo.
* [escwxyz](https://github.com/escwxyz): Fix a typo.
* [eltociear](https://github.com/eltociear): Fix a typo.
* [thesimplekid](https://github.com/thesimplekid): Fix a typo.
