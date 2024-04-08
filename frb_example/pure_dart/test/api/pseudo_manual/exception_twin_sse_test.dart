// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinSse', () async {
      await expectLater(
          () async => funcReturnErrorTwinSse(),
          throwsA(isA<AnyhowException>().having(
              (x) => x.message, 'message', startsWith('deliberate error'))));
    });
    test('call funcTypeFalliblePanicTwinSse', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinSse(), 'TwinSse',
          messageOnNative: 'deliberate panic');
    });
    test('call funcTypeInfalliblePanicTwinSse', () async {
      await expectRustPanic(
          () async => funcTypeInfalliblePanicTwinSse(), 'TwinSse',
          messageOnNative: 'deliberate panic');
    });

    addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinSse, [100]);
    test('call customEnumErrorPanicTwinSse', () async {
      await expectRustPanic(
          () async => customEnumErrorPanicTwinSse(), 'TwinSse',
          messageOnNative: 'deliberate panic');
    });

    test('call funcReturnErrorTwinSse', () async {
      var matcher = isA<CustomEnumErrorTwinSse>()
          .having((x) => x.message, 'message', startsWith('deliberate error'));
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(
          () async => customEnumErrorReturnErrorTwinSse(), throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinSse,
      [
        const CustomNestedErrorOuterTwinSse.one('hello'),
        const CustomNestedErrorOuterTwinSse.two(
            CustomNestedErrorInnerTwinSse.three('hello')),
        const CustomNestedErrorOuterTwinSse.two(
            CustomNestedErrorInnerTwinSse.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinSse,
      [const CustomStructErrorTwinSse(a: 'hello')],
      equals,
    );
  });

  group('example-based tests', () {
    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(() async => returnErrCustomErrorTwinSse(),
            throwsA(isA<CustomErrorTwinSse>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSse>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinSse(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinSse(message: "hello")
                .nonstaticReturnCustomStructErrorTwinSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSse>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinSse(message: "hello")
                .nonstaticReturnCustomStructOkTwinSse(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async =>
                CustomStructTwinSse.staticReturnCustomStructErrorTwinSse(),
            throwsA(isA<CustomStructErrorAnotherTwinSse>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinSse.staticReturnCustomStructOkTwinSse(), 3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinSse(),
            throwsA(CustomNestedError1TwinSse.errorNested(
                CustomNestedError2TwinSse.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(() async => returnCustomNestedError1Variant1TwinSse(),
            throwsA(CustomNestedError1TwinSse.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(() async => returnCustomNestedError2TwinSse(),
            throwsA(CustomNestedError2TwinSse.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(() async => returnErrorVariantTwinSse(variant: 0),
            throwsA(isA<CustomErrorTwinSse>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(() async => returnErrorVariantTwinSse(variant: 1),
            throwsA(isA<CustomErrorTwinSse>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSse(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async => SomeStructTwinSse.staticReturnErrCustomErrorTwinSse(),
            throwsA(isA<CustomErrorTwinSse>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async => SomeStructTwinSse.staticReturnErrCustomErrorTwinSse(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(await SomeStructTwinSse.staticReturnOkCustomErrorTwinSse(), 3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinSse(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinSse(value: 7)
                .nonStaticReturnErrCustomErrorTwinSse(),
            throwsA(isA<CustomErrorTwinSse>()));
        bool didCatch = false;
        try {
          await SomeStructTwinSse(value: 7)
              .nonStaticReturnErrCustomErrorTwinSse();
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
            await SomeStructTwinSse(value: 6)
                .nonStaticReturnOkCustomErrorTwinSse(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(
            () async => throwAnyhowTwinSse(), throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectRustPanicRaw(() async => panicWithCustomResultTwinSse(),
            'TwinSse', throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _ in await streamSinkThrowAnyhowTwinSse()) {}
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
          () async => funcReturnErrorTwinSse(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', matcher)));
    });

    test('when panic', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinSse(), 'TwinSse',
          messageMatcherOnNative: matcher);
    });
  });
}
