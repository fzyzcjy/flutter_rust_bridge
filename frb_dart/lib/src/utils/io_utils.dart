// TODO do not put in Wire, since now we have GeneralizedFrbRustBinding and can directly handle it
// /// {@macro flutter_rust_bridge.only_for_generated_code}
// class DartApiDl {
//   static int? _initCode;
//   final int Function(ffi.Pointer<ffi.Void>) _initFn;
//
//   /// {@macro flutter_rust_bridge.only_for_generated_code}
//   DartApiDl(this._initFn);
//
//   /// {@macro flutter_rust_bridge.only_for_generated_code}
//   void initApi() {
//     _initCode ??= _initFn(ffi.NativeApi.initializeApiDLData);
//     if (_initCode != 0) {
//       throw Exception('Failed to initialize Dart API. Code: $_initCode');
//     }
//   }
// }
