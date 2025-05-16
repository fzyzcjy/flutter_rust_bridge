// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/exception_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('systematic test', () {
    test('call funcReturnErrorTwinRustAsync', () async {
      await expectLater(
          () async => funcReturnErrorTwinRustAsync(),
          throwsA(isA<AnyhowException>().having(
              (x) => x.message, 'message', startsWith('deliberate error'))));
    });
    test('call funcTypeFalliblePanicTwinRustAsync', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinRustAsync(), 'TwinRustAsync',
          messageOnNative: 'deliberate panic');
    });
    test('call funcTypeInfalliblePanicTwinRustAsync', () async {
      await expectRustPanic(
          () async => funcTypeInfalliblePanicTwinRustAsync(), 'TwinRustAsync',
          messageOnNative: 'deliberate panic');
    });

    addTestsIdentityFunctionCall(customEnumErrorReturnOkTwinRustAsync, [100]);
    test('call customEnumErrorPanicTwinRustAsync', () async {
      await expectRustPanic(
          () async => customEnumErrorPanicTwinRustAsync(), 'TwinRustAsync',
          messageOnNative: 'deliberate panic');
    });

    test('call funcReturnErrorTwinRustAsync', () async {
      var matcher = isA<CustomEnumErrorTwinRustAsync>()
          .having((x) => x.message, 'message', startsWith('deliberate error'));
      if (!kIsWeb)
        matcher = matcher.having((x) => x.backtrace, 'backtrace', isNotEmpty);
      await expectLater(() async => customEnumErrorReturnErrorTwinRustAsync(),
          throwsA(matcher));
    });

    addTestsErrorFunctionCall(
      customNestedErrorReturnErrorTwinRustAsync,
      [
        const CustomNestedErrorOuterTwinRustAsync.one('hello'),
        const CustomNestedErrorOuterTwinRustAsync.two(
            CustomNestedErrorInnerTwinRustAsync.three('hello')),
        const CustomNestedErrorOuterTwinRustAsync.two(
            CustomNestedErrorInnerTwinRustAsync.four(42)),
      ],
      equals,
    );

    addTestsErrorFunctionCall(
      customStructErrorReturnErrorTwinRustAsync,
      [const CustomStructErrorTwinRustAsync(a: 'hello')],
      equals,
    );
  });

  group('example-based tests', () {
    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(() async => returnErrCustomErrorTwinRustAsync(),
            throwsA(isA<CustomErrorTwinRustAsync>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => returnCustomStructErrorTwinRustAsync(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsync>()));
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOkTwinRustAsync(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(
            () async => CustomStructTwinRustAsync(message: "hello")
                .nonstaticReturnCustomStructErrorTwinRustAsync(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsync>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(
            await CustomStructTwinRustAsync(message: "hello")
                .nonstaticReturnCustomStructOkTwinRustAsync(),
            3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(
            () async => CustomStructTwinRustAsync
                .staticReturnCustomStructErrorTwinRustAsync(),
            throwsA(isA<CustomStructErrorAnotherTwinRustAsync>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(
            await CustomStructTwinRustAsync
                .staticReturnCustomStructOkTwinRustAsync(),
            3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(
            () async => returnCustomNestedError1TwinRustAsync(),
            throwsA(CustomNestedError1TwinRustAsync.errorNested(
                CustomNestedError2TwinRustAsync.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => returnCustomNestedError1Variant1TwinRustAsync(),
            throwsA(CustomNestedError1TwinRustAsync.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(() async => returnCustomNestedError2TwinRustAsync(),
            throwsA(CustomNestedError2TwinRustAsync.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(
            () async => returnErrorVariantTwinRustAsync(variant: 0),
            throwsA(isA<CustomErrorTwinRustAsync>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(
            () async => returnErrorVariantTwinRustAsync(variant: 1),
            throwsA(isA<CustomErrorTwinRustAsync>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinRustAsync(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async => SomeStructTwinRustAsync
                .staticReturnErrCustomErrorTwinRustAsync(),
            throwsA(isA<CustomErrorTwinRustAsync>()));
      });

      test('Throw CustomError static method, verifies implements Frb',
          () async {
        await expectLater(
            () async => SomeStructTwinRustAsync
                .staticReturnErrCustomErrorTwinRustAsync(),
            throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(
            await SomeStructTwinRustAsync
                .staticReturnOkCustomErrorTwinRustAsync(),
            3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomErrorTwinRustAsync(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => SomeStructTwinRustAsync(value: 7)
                .nonStaticReturnErrCustomErrorTwinRustAsync(),
            throwsA(isA<CustomErrorTwinRustAsync>()));
        bool didCatch = false;
        try {
          await SomeStructTwinRustAsync(value: 7)
              .nonStaticReturnErrCustomErrorTwinRustAsync();
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
            await SomeStructTwinRustAsync(value: 6)
                .nonStaticReturnOkCustomErrorTwinRustAsync(),
            6);
      });

      test('Throw anyhow error', () async {
        await expectLater(() async => throwAnyhowTwinRustAsync(),
            throwsA(isA<FrbException>()));
      });

      test('Function with custom result panics', () async {
        await expectRustPanicRaw(
            () async => panicWithCustomResultTwinRustAsync(),
            'TwinRustAsync',
            throwsA(isA<FrbException>()));
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _ in await streamSinkThrowAnyhowTwinRustAsync()) {}
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
          () async => funcReturnErrorTwinRustAsync(),
          throwsA(isA<AnyhowException>()
              .having((x) => x.message, 'message', matcher)));
    });

    test('when panic', () async {
      await expectRustPanic(
          () async => funcTypeFalliblePanicTwinRustAsync(), 'TwinRustAsync',
          messageMatcherOnNative: matcher);
    });
  });
}
