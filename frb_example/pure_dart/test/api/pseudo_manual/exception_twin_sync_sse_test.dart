// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinSyncSse', () async {
      await expectLater(
          () async => funcReturnErrorTwinSyncSse(),
          throwsA(isA<AnyhowException>().having(
              (x) => x.message, 'message', startsWith('deliberate error'))));
    });
    test('call funcTypeFalliblePanicTwinSyncSse', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinSyncSse(), 'TwinSyncSse',
          messageOnNative: 'deliberate panic');
    });
    test('call funcTypeInfalliblePanicTwinSyncSse', () async {
      await expectRustPanic(
          () async => funcTypeInfalliblePanicTwinSyncSse(), 'TwinSyncSse',
          messageOnNative: 'deliberate panic');
    });

    addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinSyncSse, [100]);
    test('call customEnumErrorPanicTwinSyncSse', () async {
      await expectRustPanic(
          () async => customEnumErrorPanicTwinSyncSse(), 'TwinSyncSse',
          messageOnNative: 'deliberate panic');
    });

    test('call funcReturnErrorTwinSyncSse', () async {
      var matcher = isA<CustomEnumErrorTwinSyncSse>()
          .having((x) => x.message, 'message', startsWith('deliberate error'));
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(() async => customEnumErrorReturnErrorTwinSyncSse(),
          throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinSyncSse,
      [
        const CustomNestedErrorOuterTwinSyncSse.one('hello'),
        const CustomNestedErrorOuterTwinSyncSse.two(
            CustomNestedErrorInnerTwinSyncSse.three('hello')),
        const CustomNestedErrorOuterTwinSyncSse.two(
            CustomNestedErrorInnerTwinSyncSse.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinSyncSse,
      [const CustomStructErrorTwinSyncSse(a: 'hello')],
      equals,
    );
  });

  group('example-based tests', () {
    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(() async => returnErrCustomErrorTwinSyncSse(),
            throwsA(isA<CustomErrorTwinSyncSse>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinSyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSyncSse>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinSyncSse(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinSyncSse(message: "hello")
                .nonstaticReturnCustomStructErrorTwinSyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSyncSse>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinSyncSse(message: "hello")
                .nonstaticReturnCustomStructOkTwinSyncSse(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async => CustomStructTwinSyncSse
                .staticReturnCustomStructErrorTwinSyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSyncSse>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinSyncSse
                .staticReturnCustomStructOkTwinSyncSse(),
            3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinSyncSse(),
            throwsA(CustomNestedError1TwinSyncSse.errorNested(
                CustomNestedError2TwinSyncSse.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => returnCustomNestedError1Variant1TwinSyncSse(),
            throwsA(CustomNestedError1TwinSyncSse.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(() async => returnCustomNestedError2TwinSyncSse(),
            throwsA(CustomNestedError2TwinSyncSse.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(() async => returnErrorVariantTwinSyncSse(variant: 0),
            throwsA(isA<CustomErrorTwinSyncSse>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(() async => returnErrorVariantTwinSyncSse(variant: 1),
            throwsA(isA<CustomErrorTwinSyncSse>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSyncSse(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async =>
                SomeStructTwinSyncSse.staticReturnErrCustomErrorTwinSyncSse(),
            throwsA(isA<CustomErrorTwinSyncSse>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async =>
                SomeStructTwinSyncSse.staticReturnErrCustomErrorTwinSyncSse(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(
            await SomeStructTwinSyncSse.staticReturnOkCustomErrorTwinSyncSse(),
            3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSyncSse(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinSyncSse(value: 7)
                .nonStaticReturnErrCustomErrorTwinSyncSse(),
            throwsA(isA<CustomErrorTwinSyncSse>()));
        bool didCatch = false;
        try {
          await SomeStructTwinSyncSse(value: 7)
              .nonStaticReturnErrCustomErrorTwinSyncSse();
        } catch (e) {
          final FrbBacktracedException ex = e as FrbBacktracedException;
          print("backtrace: ${ex.backtrace}");
          if (!kIsWeb) expect(ex.backtrace, isNotEmpty);
          didCatch = true;
        }
        expect(didCatch, true);
      });

      test('Do not throw CustomError non-static method', () async {
        expect(
            await SomeStructTwinSyncSse(value: 6)
                .nonStaticReturnOkCustomErrorTwinSyncSse(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(
            () async => throwAnyhowTwinSyncSse(), throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectRustPanicRaw(() async => panicWithCustomResultTwinSyncSse(),
            'TwinSyncSse', throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _ in await streamSinkThrowAnyhowTwinSyncSse()) {}
          },
          throwsA(isA<AnyhowException>().having((e) => e.toString(), 'toString',
              startsWith('AnyhowException(anyhow error'))),
        );
      });
    });
  });

  group('has backtraces', skip: kIsWeb, () {
    final matcher = anyOf(contains('.rs'), contains('::'),
        contains('<unknown>'), contains('fn:'));

    test('when error (Result::Err)', () async {
      await expectLater(
          () async => funcReturnErrorTwinSyncSse(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', matcher)));
    });

    test('when panic', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinSyncSse(), 'TwinSyncSse',
          messageMatcherOnNative: matcher);
    });
  });
}
