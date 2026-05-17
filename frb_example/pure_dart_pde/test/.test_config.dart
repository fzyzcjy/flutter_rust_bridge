// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:async';

import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';

Future<void> testExecutable(FutureOr<void> Function() testMain) async {
  try {
    await testMain();
  } finally {
    RustLib.dispose();
  }
}
