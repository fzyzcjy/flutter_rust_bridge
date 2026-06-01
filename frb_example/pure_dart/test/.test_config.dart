import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> testExecutable(FutureOr<void> Function() testMain) async {
  tearDownAll(RustLib.dispose);
  await testMain();
}
