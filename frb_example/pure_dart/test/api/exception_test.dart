import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/exception.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcReturnErrorTwinNormal', () async {
    await expectLater(() async => funcReturnErrorTwinNormal(),
        throwsA(isA<AnyhowException>().having((x) => x.message, 'message', 'deliberate error')));
  });
  test('call funcTypeFalliblePanicTwinNormal', () async {
    await expectLater(() async => funcTypeFalliblePanicTwinNormal(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });
  test('call funcTypeInfalliblePanicTwinNormal', () async {
    await expectLater(() async => funcTypeInfalliblePanicTwinNormal(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });

  addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinNormal, [100]);
  test('call customEnumErrorPanicTwinNormal', () async {
    await expectLater(() async => customEnumErrorPanicTwinNormal(),
        throwsA(isA<PanicException>().having((x) => x.message, 'message', 'deliberate panic')));
  });

  test('call funcReturnErrorTwinNormal', () async {
    await expectLater(
        () async => customEnumErrorReturnErrorTwinNormal(),
        throwsA(isA<CustomEnumErrorTwinNormal>()
            .having((x) => x.message, 'message', 'deliberate error')
            .having((x) => x.backtrace, 'backtrace', isNotEmpty)));
  });

  addTestsErrorFunctionCall(
    customNestedErrorReturnErrorTwinNormal,
    [
      const CustomNestedErrorOuterTwinNormal.one('hello'),
      const CustomNestedErrorOuterTwinNormal.two(CustomNestedErrorInnerTwinNormal.three('hello')),
      const CustomNestedErrorOuterTwinNormal.two(CustomNestedErrorInnerTwinNormal.four(42)),
    ],
    equals,
  );

  addTestsErrorFunctionCall(
    customStructErrorReturnErrorTwinNormal,
    [const CustomStructErrorTwinNormal(a: 'hello')],
    equals,
  );
}
