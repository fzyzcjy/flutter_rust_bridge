# Git Ignore

Since it is asked in [#1765](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1765),
here is suggestions about which files should or should not be in version control.

* Auto-generated code (`lib/src/rust/*`, `rust/src/frb_generated*`, `frb_generated.h`, ...):
  Feel free to gitignore it and execute `flutter_rust_bridge_codegen generate` to generate them back.
  Alternatively, they can also be inside version control and there is also no problem.
* Scaffold code (e.g. `rust_builder`, ...): They need to be version controlled.
