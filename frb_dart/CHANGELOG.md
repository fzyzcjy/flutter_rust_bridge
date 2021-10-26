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

