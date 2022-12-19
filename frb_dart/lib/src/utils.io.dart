import 'dart:ffi';
import 'dart:io';

DynamicLibrary loadDylib(String path) => Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS && Abi.current() == Abi.macosX64
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
