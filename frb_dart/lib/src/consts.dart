import 'package:meta/meta.dart';

/// borrowed from flutter foundation [kIsWeb](https://api.flutter.dev/flutter/foundation/kIsWeb-constant.html),
/// but allows for using it in a Dart context alike
const bool kIsWeb = identical(0, 0.0);

/// {@template flutter_rust_bridge.internal}
/// The code is used only internally and is not a public API. The comment exists mainly to satisfy the linter.
/// {@endtemplate}
@internal
const kEnableFrbFfiTestTool = bool.fromEnvironment("ENABLE_FRB_FFI_TEST_TOOL");
