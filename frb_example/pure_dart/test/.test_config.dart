import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<void> testExecutable(FutureOr<void> Function() testMain) async {
  try {
    await testMain();
  } finally {
    RustLib.dispose();
  }
}
