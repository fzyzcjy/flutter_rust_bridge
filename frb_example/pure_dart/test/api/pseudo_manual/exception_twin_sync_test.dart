// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('call funcReturnErrorTwinSync', () async {
    await expectLater(() async => funcReturnErrorTwinSync(),
        throwsA(isA<AnyhowException>().having((x) => x.message, 'message', 'deliberate error')));
  });
  test('call funcTypeFalliblePanicTwinSync', () async {
    await expectLater(() async => funcTypeFalliblePanicTwinSync(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });
  test('call funcTypeInfalliblePanicTwinSync', () async {
    await expectLater(() async => funcTypeInfalliblePanicTwinSync(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });

  addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinSync, [100]);
  test('call customEnumErrorPanicTwinSync', () async {
    await expectLater(() async => customEnumErrorPanicTwinSync(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });

  test('call funcReturnErrorTwinSync', () async {
    await expectLater(
        () async => customEnumErrorReturnErrorTwinSync(),
        throwsA(isA<CustomEnumErrorTwinSync>()
            .having((x) => x.message, 'message', 'deliberate error')
            .having((x) => x.backtrace, 'backtrace', isNotEmpty)));
  });

  addTestsErrorFunctionCall(
    customNestedErrorReturnErrorTwinSync,
    [
      const CustomNestedErrorOuterTwinSync.one('hello'),
      const CustomNestedErrorOuterTwinSync.two(CustomNestedErrorInnerTwinSync.three('hello')),
      const CustomNestedErrorOuterTwinSync.two(CustomNestedErrorInnerTwinSync.four(42)),
    ],
    equals,
  );

  addTestsErrorFunctionCall(
    customStructErrorReturnErrorTwinSync,
    [const CustomStructErrorTwinSync(a: 'hello')],
    equals,
  );
}
