// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:async';

import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> testExecutable(FutureOr<void> Function() testMain) async {
  tearDownAll(RustLib.dispose);
  await testMain();
}
