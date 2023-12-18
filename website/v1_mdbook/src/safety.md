# Safety concerns

This library has CI that runs [Valgrind](https://www.valgrind.org/) automatically on the setup that a Dart program calls a Rust program using this package, so memory problems should be found by Valgrind. <sub>(Notice that, even when running a simple hello-world Dart program, Valgrind will report hundreds of errors. See [this Dart lang issue](https://github.com/dart-lang/sdk/issues/47346) for more details. Therefore, I both look at "definitely lost" in Valgrind, and manually search things related to this library - if all reported errors are unrelated to this library then we are safe.)</sub>

In addition, Flutter integration tests are also done in CI. This ensures a real Flutter application using this library does not suffer from problems.

Most of the code are written in safe Rust. The `unsafe` code mainly comes from `support::box_from_leak_ptr` and `support::vec_from_leak_ptr`. They are used for pointers and arrays, and I follow the high-upvoted answers and official doc when writing down that few lines of code.

I use this library heavily in my own Flutter project (`yplusplus`, or `why++`). That app is in production and it works quite well. If I observe any problems, I will fix it in this library.

The CI also runs the `run_codegen` workflow, which ensure that the code generator can compile and generate desired results. Lastly, the CI also runs formatters and linters (`fmt`, `clippy`, `dart analyze`, `dart format`), and linters can also catch some common problems.