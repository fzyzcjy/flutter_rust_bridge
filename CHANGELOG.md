# Changelog

## 2.1.0

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Add the ability to generate plugins from the CLI tool #2144 (thanks @mcmah309)
* Fix codegen halt when having boxed trait objects #2180
* Add attribute `#[frb(dart_async)]` #2181
* Fix Dart closures cannot be encoded when using the new Dart JS interop #2191
* Expose Rust executor's async runtime for customization #2151
* Generate methods of Default trait #2150
* Automatically rename function names to avoid keyword conflict #2150
* Improve parsing trait impl in third party crates #2150
* Support more attributes #2140
* Fix generated use statement pointing to self #2140
* Fix linter warning avoid_return_types_on_setters #2140
* Reduce number of generated files #2140
* Add default_dart_async configuration option #2139
* Handle root module scenario in pub use transformer #2124
* Improve hints when fails to parse a struct or enum #2124

## 2.0.0

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.

## 2.0.0-dev.42

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support brackets and nesting in pub use (such as pub use `a::{b, x::{y,z}}`) #2120
* Support providing configuration both by command line and by config file at the same time #2119
* Tell user reasons when skipping traits #2121
* Update default template comments #2118
* Allow using arbitrary frb attribute as an indicator to include an item #2121
* Improve hints when user specify custom wasm-pack-rustflags but does not contain default one #2122

## 2.0.0-dev.41

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Use dart.library.js_interop rather than dart.library.html #2113 (thanks @aran)
* Auto skip methods in trait definitions that has Self as inputs because it cannot be translated to Dart #2099
* Add type_64bit_int to config #2096
* Improve namespace choices for opaque types #2096
* Hint users when having references in output type, hint when wrong rust_input, and improve docs #2100
* Improve warning message #2098
* Experiment more about translating whole audio crate#2096
* Improve internal code #2096
* Make generated output sorted when using proxy_enum and trait_impl #2115

## 2.0.0-dev.40

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support returning types with non-static lifetime (i.e. borrowed types / reference types) #2088 #2093
* Loosen config field `rust_input` syntax with delimit `,` #2092 (thanks @dbsxdbsx)
* Add prefix for automatically generated get/set methods of `#[frb(opaque)]` types to avoid conflicting with existing methods #2090 (thanks @dbsxdbsx)
* Support adding arbitrary code in generated Rust file via rust_preamble config #2086
* Support ignoring a whole module by `#[frb(ignore)]` on module #2085
* Support `/// frb:...` #2085

## 2.0.0-dev.39

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Do not create empty logs directory when not in verbose mode #2057
* Automatically strip prefix set_/get_ when it is a setter/getter function #2058
* Support adding errors to stream sink #2059
* Support import statements in dart_code #2061
* Add `#[frb(type_64bit_int)]` to pick preferred Dart big integer type #2065
* Support user-defined custom serializers and deserializers #2067
* Support attributes on impl block ; Improve ignore category message ; Fix external methods are wrong ignored #2069
* Support `#[frb(name)]` on fields to rename them ; Automatically rename Dart field names if they conflict with Dart keywords #2070
* Refactor internal code #2064 #2062
* Migrate to new Dart web package #2063

## 2.0.0-dev.38

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix dashed library names #2034 (thanks @vhdirk)
* Refactor internals to base on mods instead of files #2000
* Support code in mod file #2000
* Support code in lib.rs file #2000
* Fix dart_code cannot generate when containing brackets confusing to mod parser #2000
* Support multiple input path prefixes #2000
* Add dart_type_rename to customize Dart names of opaque types #2006
* Support automatic scanning of third party crates (part 1) #2007
* Enhance usize and isize #2008
* Support syntax of `pub use something::*` #2009
* Improve Dart output directory of third party crates #2010
* Support overriding things in third party crate #2011 #2013
* Automatically mark methods of non-pub structs as ignored #2012
* Refactor macros information encoding and decoding #2014
* Detect and skip functions with generics #2015
* Auto detect as opaque when third party struct/enum has non public field #2016
* Auto convert reference type in return type as unit type #2017
* Automatically mirror for scanned third party types #2018
* Skip generating auto accessors for borrowed fields #2018
* Support slices as arguments (such as `&[u8]`, `&[Something]`) #2019
* Support non-exhaustive enumerations #2020
* Improve Dart import generation #2021
* Support trait methods and trait default implementations #2024
* Improve pub use parsing and trait definition parsing in third party crates #2025
* Add integrate_third_party example #2027
* Support overriding third party methods #2029
* Support adding new methods to third party structs #2029
* Fix third party pub use scanning problem #2030
* Support multiple `#[frb(external)]` impl blocks for a single struct #2030
* Fix scanning third party pub use concrete type for methods #2031
* Make generated Dart opaque class abstract to improve testability #2032
* Refactor HIR (high-level intermediate representation) #2037
* Add stop_on_error configuration #2037
* Refactor override_priority and refine_namespace #2041
* Support trait definitions and translate to Dart abstract classes #2033
* Support third party overriding traits #2033
* Refactor MIR (mid-level intermediate representation) parser #2044
* Support `#[frb(generate_impl_enum)]` #2046
* Fix overriding attributes in third party crates for traits #2046
* Add RustOpaqueInterface to cleanup generated interface #2047
* Refactor to extract early_generator between high-level intermediate representation and mid-level intermediate representation #2049
* Support `#[frb(proxy)]` #2050 #2052 #2053
* Refactor to extract Lockable #2051
* Support &dyn Trait #2054

## 2.0.0-dev.37

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support positional parameters (in addition to named parameters) #1988
* Support overriding environment variable for build-web #1984
* Hint users when a function is not public and is ignored #1985
* Rename generated functions that are not to be used by end users explicitly #1983
* Improve generated user-facing API #1988

## 2.0.0-dev.36

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Automatically generate getters and setters for public fields of opaque types #1975
* Support Dart setters #1972
* Support i128 and u128 #1964
* Improve RustAutoOpaque's API and codegen handling #1970
* Fix when users do not have explicit dependency on anyhow #1967
* Hint users when using opaque types inside non opaque structs #1978
* Hint users when using getters of opaque types #1976

## 2.0.0-dev.35

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support Result type when Rust calls Dart back #1945
* Support type char #1957
* Support renaming functions and methods #1958
* Re-enable MemorySanitizer #1959
* Show hints when functions or methods are ignored #1956
* Add hints to deliberate Exception in RustLib.init in Chrome #1955
* Support when struct type definition is in one file, struct impl is in another file, and the impl needs extra import #1953

## 2.0.0-dev.34

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Add preamble for dart generated files #1949 (thanks @Krysl)
* Support same function name in different files #1944
* Support dart_format_line_length on generated .freezed.dart #1939
* Improve hints when Flutter hot restarts with Streams #1942
* Fix name conflict when using C++ keywords #1943
* Fix warning use of deprecated associated function chrono::NaiveDateTime::from_timestamp_micros #1941
* Bump Dart SDK source #1940

## 2.0.0-dev.33

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Automatically wait when concurrent mutable access, while still preventing deadlocks #1920

## 2.0.0-dev.32

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support self in non-opaque case (&self already supported before) #1880
* Support &SomeStructOrEnum in non-opaque case (SomeStructOrEnum already supported before) #1880
* Support &str type (String already supported before) #1880
* Support instance and static methods for enums without fields #1879
* Revert stream default semantics to not wait for Rust function execution #1877
* Allow users to customize whether to await for Rust function for streams #1877
* Hint users when a type is automatically inferred as both opaque and non-opaque #1876
* Add check to ensure Rust and Dart has in-sync generated code #1878

## 2.0.0-dev.31

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support methods in external crates #1861
* Support inserting arbitrary Dart code #1860
* Support disabling default Hash/Eq generation #1860
* Support passing stream object at arbitrary location and arbitrary amount #1867
* Let Rust function finish execution before returning stream object #1867
* Support cases when using Rust conditional compilation #1856
* Fix not exporting some struct types needed for customizing handlers #1865
* Fix ignoring user-provided custom handler objects #1865
* Fix ReceivePort is not closed when subscription is cancelled before Rust closes the stream #1857

## 2.0.0-dev.30

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix class is not generated when having only static methods #1851
* Fix passing non-existent variable to getter causing compilation error #1851
* Fix missing code generation when using enum and methods #1851
* Fix sh permission #1842 (thanks @canxin121)

## 2.0.0-dev.29

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* StreamSink recursive intodart_type #1843 (thanks @SilverMira)

## 2.0.0-dev.28

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix error when using build.rs instead of standard way to trigger code generation #1809

## 2.0.0-dev.27

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Improve procedure macros #1800 #1676 (thanks @Desdaemon)
* Add support for ffigen upgrades #1799 #1757 (thanks @aran)
* Improve hints when structs are in lib.rs #1802 #1579 (thanks @h3x4d3c1m4l)
* Improve scenarios when opaque-by-default is more natural #1805
* Improve behavior of constructor #1804
* Fix when using non-opaque struct + option + opaque inner type #1803

## 2.0.0-dev.26

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Allow specifying pubspec.yaml location in build.rs #1797 (thanks @HalidOdat)
* Hint how to only run build.rs if api directory changes #1794 (thanks @HalidOdat)

## 2.0.0-dev.25

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Generate decl of `store_dart_post_cobject` when full_dep enabled #1773 (thanks @Berrysoft)
* Support type Self (originally needs to specify the concrete type, now can use this shortcut) #160
* Provide hints when struct/enum is never used #1763
* Improve hints when types are exported but not used #1779
* Auto understand when user structs have name conflict with builtin types #1782
* Fix when non-enum + specify default value + dart_enums_style being true #1780
* Fix `Vec<Uuid>` error when using CST codec (the SSE codec does not have this bug) #1762
* Fix build-web cannot find Dart program in some platforms #1758

## 2.0.0-dev.24

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support ignoring all methods of a struct #1730
* Allow Rust structs to become Dart callable objects #1728
* Improve hints when using non-meaningful `&mut` #1739
* Allow Dart GC to collect when a Stream is created but not closed #1737
* Make rust_builder has unique name to avoid conflicts #1738
* Inform users when both config file and command line arguments are used #1731

## 2.0.0-dev.23

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* More friendly error message for Unit Struct #1718 (thanks @alanlzhang)

## 2.0.0-dev.22

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Automatically upgrade flutter_rust_bridge Dart/Rust runtime versions when running generate #1704
* Fix failure of formatting Dart code when path is too long #1702
* Add checks when build_runner needs to be a dependency #1695
* Fix generated code is breaking cargo test because of its doc-test contains a moved import #1706
* Add uuid and chrono for SSE codec (the CST/DCO codec already supports it) #169
* Add example using build.rs #1696
* Add doc about movement of things like StreamSink #1703
* Add tests and doc about multiple Dart Isolates #1699

## 2.0.0-dev.21

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support more fine-grained rust-auto-opaque #1680
* Add RustAutoOpaque as an extra alternative approach to fully automatic arbitrary types #1679
* Fix when primitive enums + has custom discriminator values + using SSE codec #1677
* Fix when StreamSink + rust-auto-opaque types #1682
* Fix when using types in external crate and use HashMap to contain it (continue from @aran's PR) #1684
* Add doc to explain how to configure ios build system to avoid "linker undefined symbols" #1678
* Improve comments for generated code #1685

## 2.0.0-dev.20

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix module parsing when two modules have the same name #1664 (thanks @alanlzhang)
* Fix integrate command's default template has rust_lib hardcoded instead of custom names #1669
* Support `Vec<Vec<u8>>` for CST/DCO codec (SSE codec already supports it) #1663
* Make LLVM dependency optional #1666
* Add checks of versions #1667

## 2.0.0-dev.19

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Use try-lock for rust auto opaque #1653
* Improve hints when running create command #1652
* Add MOI RustOpaqueCodec #1642
* Add more feature flags #1658
* Add tutorial for pure dart scenario #1661
* Add more tests #1657

## 2.0.0-dev.18

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Update cargokit to fix flutter flavor messing up with build mode #1644 (thanks @pixelshot91)
* Update dependency to allo-isolate for NaiveDate implementation #1631 (thanks @TrackerSB)

## 2.0.0-dev.17

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Add org option in create command #1635 (thanks @wxitcode)
* Fix typo in frb_example_pure_dart_exapmle_external_lib #1645 (thanks @aran)

## 2.0.0-dev.16

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Improve performance by avoiding constructing vectors #1636
* Fix some Dart imports are not auto generated when using rust-auto-opaque in some scenarios #1638
* Add test that one struct has multiple impl blocks in multiple files #1639

## 2.0.0-dev.15

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Execute flutter pub get inside build_tool from cargokit on integrate and on create #1614 (thanks @MateusHBR)

## 2.0.0-dev.14

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix compilation error when using custom error type + rust auto opaque #1622
* Fix wrong HTTP headers which causes Rust cannot run on Safari #1620

## 2.0.0-dev.13

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Let rust-auto-opaque support rust async #1617
* Let setup_default_user_utils setup logging in web #1616
* Fix compile error when the user crate does not have anyhow as direct dependency #1603
* Add test when parsing non-existing attribute name #1602

## 2.0.0-dev.12

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support property getters #1600
* Automatically close streams when it is dropped #1601
* Allow to accept more Dart types for `Vec<primitive>` #1597
* Improve `frb_dart/ffigen_generated` #1598
* Allow `#[frb(one, two)]` (originally only allow `#[frb(one)] #[frb(two)]`) #1599

## 2.0.0-dev.11

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support map and set #1565
* Pass Rust stack traces to Dart side during a panic #1586
* Fix the Dart naming of rust opaque types when using records #1593
* Fix utilities cannot set environment variables on web #1590

## 2.0.0-dev.10

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Let integrate update .gitignore #1552
* Fix linter errors when some extra linter rules are enabled #1560
* Add prefix for generated C symbols #1558
* Support isize (mimic existing usize) #1482
* Let flutter_rust_bridge create/integrate support customizing rust crate name and directory #1567
* Improve flutter_rust_bridge_codegen integrate #1566
* Add CI for release mode and generate final artifacts #1572
* No need to let user write AssertUnwindSafe everywhere #1574
* Allow users to quickly define Rust initialization functions (via `#[frb(init)]`) #1580
* Wrap allo-isolate post to return Result instead of bool for better error checking #1584
* Provide setup for logging, backtrace, etc, by default, but allow easy customization #1581
* Fix awaiting DartFn, i.e. Rust callback to Dart #1585

## 2.0.0-dev.9

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Support ignoring a function via `#[frb(ignore)]` #1544 (thanks @JustSimplyKyle)
* Bump dependency to allow building if rust toolchain for esp32 is installed #1498 (thanks @mattiasgronlund)

## 2.0.0-dev.8

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Let codegen report error when provided configuration names are incorrect #1509 (thanks @patmuk)

## 2.0.0-dev.7

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Improve error message when a file ending is not provided in configuration #1540 (thanks @patmuk)

## 2.0.0-dev.6

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Remove double-allocation for enum CST decode #1500 (thanks @Desdaemon)

## 2.0.0-dev.5

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Set the msrv #1463 (thanks @patmuk)
* Toggle off default `clap` feature for `cbindgen` #1503 (thanks @CosmicHorrorDev)

## 2.0.0-dev.4

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* Fix Generated Rust code of `Result<RustOpaque<...>>` is incorrect #1474 (thanks @trobanga)
* (My (@fzyzcjy's) PRs are not listed here)

## 2.0.0-dev.3

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* This is still a separate version to see whether the readme image layout works for pub.dev and crates.io.

## 2.0.0-dev.2

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* This is again a separate version, because I need to trigger the `release` CI to fix the binary for `cargo binstall` (
  that CI did not checkout recursive submodule before).

## 2.0.0-dev.1

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* This is a separate version, because cargo publish seems not to understand frb_codegen's template Cargo.toml files, so
  need to publish version to test.

## 2.0.0-dev.0

* Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/guides/miscellaneous/whats-new for what's changed in V2.
* The 2.0.0-dev.0 will contain bugs and missing features, because I have to have a real (pre-)release before I can start
  working on the last part of tests and functionalities. So it is a chicken-and-egg problem ;) But I expect
  2.0.0-dev.2 (or -dev.3) to be stable.

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
* General CLI improvements: dumping, upgrade clap, yaml config, serialize IR, bump versions, etc #1119 (thanks
  @Desdaemon)
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
* Fix `Optional<DateTime>` #1079 (thanks @alexthe2)

## 1.64.0

* Support `Option<Datetime>` #1066 (thanks @alexthe2)

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

* Refactors for `Boxed` and `Option` (Refactors `EnumRef` to make `Option<FieldlessEnum>` work; `Box<primitive>`
  and `Option<Box<primitive>>` on WASM no longer allocates a Box, but receives a (nullable) value directly from Dart)
  #949 thanks @Desdaemon

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

* Implement opaque types, enabling arbitrary Rust structs to be used as opaque Dart objects, by generating wrappers and
  raw Arc pointers #795 (thanks @rogurotus)

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

* Add support for the Web platform, parallel to the existing mobile/desktop platforms, via WASM and JavaScript as
  intermediate values #589 (thanks @Desdaemon)

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

* Support `#[frb(metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]` for structs #463 (thanks
  @alanlzhang)

## 1.30.0

* Support non-final fields in Dart structs #452 (thanks @surban)

## 1.29.0

* Make code generator a lib to be used in build.rs; add error types for codegen; depend on cbindgen directly; update
  docs #434 (thanks @sagudev)
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

* Make mirroring work for more use cases: tuple structs, enum variants, wrapping in Vec and Optional #359 (thanks
  @Unoqwy)
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
* Improvements #267 (thanks @Desdaemon): Resolve #265, Resolve #266, Fix attributes not working on enum variants, Add
  comments on enum variants and fields, (Internal) unify tuple and normal enum structs
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

* Support more types of the form`Vec<primitive_type>` and `ZeroCopyBuffer<Vec<primitive_type>>`, such as `Vec<f32>`
  and `ZeroCopyBuffer<Vec<f32>>` to be transformed into `Float32List` in Dart. (#162, #153)
* Do not generate unnecessary Dart to Rust wire code to fix bugs such as when `Vec<ZeroCopyBuffer<Vec<u8>>>` is in
  output argument.
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
* Add `FlutterRustBridgeSetupMixin` (an optional helper class), which allows custom setup hooks before ffi can be
  executed.
* Add `hint` parameter in generated Dart code, allowing users to pass custom data to the Dart executor, thus increasing
  flexibility.
* Improve panic handling in extreme cases (avoid panic across languages, which is undefined behavior).
* Refactored `Handler`, now it is much easier to customize your own handler functionality.
* Remove one `Box::new(FnOnce)`, thus enables better inlining for ffi function calls.
* Fix bug: Dart struct(class) is not generated if the struct only appears in the return type #98.
* Add `FlutterRustBridgeTimeoutMixin`. If used, a timeout exception will be thrown for ffi calls that do not return
  within time limit.

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
