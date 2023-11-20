import 'dart:ffi' as ffi;

// TODO rename?
class DartApiDl {
  static int? _initCode;
  final int Function(ffi.Pointer<ffi.Void>) _initFn;

  DartApiDl(this._initFn);

  void initApi() {
    _initCode ??= _initFn(ffi.NativeApi.initializeApiDLData);
    if (_initCode != 0) {
      throw Exception('Failed to initialize Dart API. Code: $_initCode');
    }
  }
}
