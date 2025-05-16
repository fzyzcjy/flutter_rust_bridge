// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/exception_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinRustAsyncSse', () async {
      await expectLater(
          () async => funcReturnErrorTwinRustAsyncSse(),
          throwsA(isA<AnyhowException>().having(
              (x) => x.message, 'message', startsWith('deliberate error'))));
    });
    test('call funcTypeFalliblePanicTwinRustAsyncSse', () async {
      await expectRustPanic(() async => funcTypeFalliblePanicTwinRustAsyncSse(),
          'TwinRustAsyncSse',
          messageOnNative: 'deliberate panic');
    });
    test('call funcTypeInfalliblePanicTwinRustAsyncSse', () async {
      await expectRustPanic(
          () async => funcTypeInfalliblePanicTwinRustAsyncSse(),
          'TwinRustAsyncSse',
          messageOnNative: 'deliberate panic');
    });

    addTestsIdentityFunctionCall(
        customEnumErrorReturnOkTwinRustAsyncSse, [100]);
    test('call customEnumErrorPanicTwinRustAsyncSse', () async {
      await expectRustPanic(() async => customEnumErrorPanicTwinRustAsyncSse(),
          'TwinRustAsyncSse',
          messageOnNative: 'deliberate panic');
    });

    test('call funcReturnErrorTwinRustAsyncSse', () async {
      var matcher = isA<CustomEnumErrorTwinRustAsyncSse>()
          .having((x) => x.message, 'message', startsWith('deliberate error'));
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(
          () async => customEnumErrorReturnErrorTwinRustAsyncSse(),
          throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinRustAsyncSse,
      [
        const CustomNestedErrorOuterTwinRustAsyncSse.one('hello'),
        const CustomNestedErrorOuterTwinRustAsyncSse.two(
            CustomNestedErrorInnerTwinRustAsyncSse.three('hello')),
        const CustomNestedErrorOuterTwinRustAsyncSse.two(
            CustomNestedErrorInnerTwinRustAsyncSse.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinRustAsyncSse,
      [const CustomStructErrorTwinRustAsyncSse(a: 'hello')],
      equals,
    );
  });

  group('example-based tests', () {
    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(() async => returnErrCustomErrorTwinRustAsyncSse(),
            throwsA(isA<CustomErrorTwinRustAsyncSse>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinRustAsyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsyncSse>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinRustAsyncSse(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinRustAsyncSse(message: "hello")
                .nonstaticReturnCustomStructErrorTwinRustAsyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsyncSse>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinRustAsyncSse(message: "hello")
                .nonstaticReturnCustomStructOkTwinRustAsyncSse(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async => CustomStructTwinRustAsyncSse
                .staticReturnCustomStructErrorTwinRustAsyncSse(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsyncSse>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinRustAsyncSse
                .staticReturnCustomStructOkTwinRustAsyncSse(),
            3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinRustAsyncSse(),
            throwsA(CustomNestedError1TwinRustAsyncSse.errorNested(
                CustomNestedError2TwinRustAsyncSse.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => returnCustomNestedError1Variant1TwinRustAsyncSse(),
            throwsA(
                CustomNestedError1TwinRustAsyncSse.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(
            () async => returnCustomNestedError2TwinRustAsyncSse(),
            throwsA(
                CustomNestedError2TwinRustAsyncSse.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(
            () async => returnErrorVariantTwinRustAsyncSse(variant: 0),
            throwsA(isA<CustomErrorTwinRustAsyncSse>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(
            () async => returnErrorVariantTwinRustAsyncSse(variant: 1),
            throwsA(isA<CustomErrorTwinRustAsyncSse>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinRustAsyncSse(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async => SomeStructTwinRustAsyncSse
                .staticReturnErrCustomErrorTwinRustAsyncSse(),
            throwsA(isA<CustomErrorTwinRustAsyncSse>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async => SomeStructTwinRustAsyncSse
                .staticReturnErrCustomErrorTwinRustAsyncSse(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(
            await SomeStructTwinRustAsyncSse
                .staticReturnOkCustomErrorTwinRustAsyncSse(),
            3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinRustAsyncSse(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinRustAsyncSse(value: 7)
                .nonStaticReturnErrCustomErrorTwinRustAsyncSse(),
            throwsA(isA<CustomErrorTwinRustAsyncSse>()));
        bool didCatch = false;
        try {
          await SomeStructTwinRustAsyncSse(value: 7)
              .nonStaticReturnErrCustomErrorTwinRustAsyncSse();
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
            await SomeStructTwinRustAsyncSse(value: 6)
                .nonStaticReturnOkCustomErrorTwinRustAsyncSse(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(() async => throwAnyhowTwinRustAsyncSse(),
            throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectRustPanicRaw(
            () async => panicWithCustomResultTwinRustAsyncSse(),
            'TwinRustAsyncSse',
            throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _
                in await streamSinkThrowAnyhowTwinRustAsyncSse()) {}
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
          () async => funcReturnErrorTwinRustAsyncSse(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', matcher)));
    });

    test('when panic', () async {
      await expectRustPanic(() async => funcTypeFalliblePanicTwinRustAsyncSse(),
          'TwinRustAsyncSse',
          messageMatcherOnNative: matcher);
    });
  });
}
