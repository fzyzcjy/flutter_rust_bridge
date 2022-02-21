import 'dart:ffi';
import 'dart:io';

String get currentPlatform => Platform.operatingSystem;

/// Returns nothing 
DynamicLibrary? openDylib({
  String? path,
  bool processDylib = false,
  bool? module,
}) {
  if (path == null) return processDylib ? DynamicLibrary.process() : DynamicLibrary.executable();
  return DynamicLibrary.open(path);
}
