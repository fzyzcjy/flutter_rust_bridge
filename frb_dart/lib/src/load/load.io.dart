import 'dart:ffi';
import 'dart:io';

/// agnostic dynamic library loading on native platforms
///
/// see [#898](https://github.com/fzyzcjy/flutter_rust_bridge/pull/898)
DynamicLibrary loadLibForFlutter(String path) => Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS && Abi.current() == Abi.macosX64
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
DynamicLibrary loadLibForDart(String path) =>
    Platform.isIOS ? DynamicLibrary.process() : DynamicLibrary.open(path);
