// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcReturnErrorTwinSync', () async {
    await expectLater(() async => funcReturnErrorTwinSync(),
        throwsA(isA<AnyhowException>().having((x) => x.message, 'anyhow', 'TODO')));
  });

  test('call funcReturnPanicTwinSync', () async {
    await expectLater(() async => funcReturnPanicTwinSync(),
        throwsA(isA<PanicException>().having((x) => x.message, 'error', 'TODO')));
  });
}
