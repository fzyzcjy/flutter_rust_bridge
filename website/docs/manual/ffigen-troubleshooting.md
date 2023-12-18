import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Ffigen Troubleshooting

## LLVM needs to be installed

The [ffigen](https://pub.dev/packages/ffigen),
a popular Flutter official package used by flutter_rust_bridge,
requires the installation of LLVM.

According to its documentation, the commands are:

<Tabs>

<TabItem value="Windows">

1. Install Visual Studio with C++ development support.
2. Install [LLVM](https://releases.llvm.org/download.html) or `winget install -e --id LLVM.LLVM`.

</TabItem>

<TabItem value="MacOS">

1. Install Xcode.
2. Install Xcode command line tools - `xcode-select --install`.
3. Install LLVM - `brew install llvm`.

</TabItem>

<TabItem value="Linux apt-get">

1. Install libclangdev - `sudo apt-get install libclang-dev`.

</TabItem>

<TabItem value="Linux dnf">

1. Install libclangdev - `sudo dnf install clang-devel`.

</TabItem>

</Tabs>

After installation, if the `ffigen` used by `flutter_rust_bridge_codegen` still cannot find LLVM,
you may specify it explicitly via `--llvm-path <YOUR_LLVM_PATH>` (command line)
or `llvm_path: <YOUR_LLVM_PATH>` (configuration file).

## The generated store_dart_post_cobject() has the wrong signature / `'stdarg.h' file not found` in Linux / `stdbool.h` / ...

Try to run code generator with working directory at `/`, or set the environment variable:
```bash
export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
```
as described in [ffigen #257](https://github.com/dart-lang/ffigen/issues/257), or add include path as is described in [#108](https://github.com/fzyzcjy/flutter_rust_bridge/issues/108). This is a problem with Rust's builtin `Command`. See also: [#472](https://github.com/fzyzcjy/flutter_rust_bridge/issues/472) & [#494](https://github.com/fzyzcjy/flutter_rust_bridge/issues/494).

