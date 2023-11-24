import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/exception.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  group('systematic test', () {
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
  });

  group('example-based tests', () {
    // Test if sync return is working as expected.
    test('dart call handle_sync_return', () async {
      expect(handleSyncReturn(mode: 'NORMAL'), List.filled(100, 42));

      try {
        handleSyncReturn(mode: 'RESULT_ERR');
        fail("exception not thrown");
      } on FrbAnyhowException catch (e) {
        print('dart catch anyhow e: $e');
      }

      try {
        handleSyncReturn(mode: 'PANIC');
        fail("exception not thrown");
      } on PanicException catch (e) {
        print('dart catch panic e: $e');
      }
    });

    test('dart call returnErr', () async {
      try {
        await returnErr();
        fail("exception not thrown");
      } on FrbAnyhowException catch (e) {
        print('dart catch e: $e');
      }
    });

    test('dart call returnPanic', () async {
      try {
        await returnPanic();
        fail("exception not thrown");
      } catch (e) {
        print('dart catch e: $e');
        expect(e, isA<PanicException>());
      }
    });

    group('custom errors', () {
      // The first time a backtrace is created, symbol resolution
      // takes a significant amount of time.
      test('Throw CustomError', timeout: Timeout.factor(5), () {
        expect(returnErrCustomError(), throwsA(isA<CustomError>()));
      });

      test('Throw CustomStructError', () async {
        await expectLater(() async => await returnCustomStructError(), throwsA(isA<CustomStructError>()));
      });

      test('Throw sync CustomStructError', () {
        try {
          syncReturnCustomStructError();
        } on CustomStructError catch (e) {
          expect(e.message, "error message");
        }
      });

      test('Do not throw CustomStructError', () async {
        expect(await returnCustomStructOk(), 3);
      });

      test('Throw CustomStructError non static method', () async {
        await expectLater(() async => await CustomStruct(message: "hello").nonstaticReturnCustomStructError(),
            throwsA(isA<CustomStructError>()));
      });

      test('Do not throw CustomStructError non static method', () async {
        expect(await CustomStruct(message: "hello").nonstaticReturnCustomStructOk(), 3);
      });

      test('Throw CustomStructError static method', () async {
        await expectLater(() async => await CustomStruct.staticReturnCustomStructError(bridge: api),
            throwsA(isA<CustomStructError>()));
      });

      test('Do not throw CustomStructError static method', () async {
        expect(await CustomStruct.staticReturnCustomStructOk(bridge: api), 3);
      });

      test('Throw CustomNestedError1', () async {
        await expectLater(() async => await returnCustomNestedError1(),
            throwsA(CustomNestedError1.errorNested(CustomNestedError2.customNested2Number(3))));
      });

      test('Throw CustomNestedError1 variant 1', () async {
        await expectLater(
            () async => await returnCustomNestedError1Variant1(), throwsA(CustomNestedError1.customNested1("custom")));
      });

      test('Throw CustomNestedError2', () async {
        await expectLater(
            () async => await returnCustomNestedError2(), throwsA(CustomNestedError2.customNested2("custom")));
      });

      test('Throw CustomError variant 0', () async {
        await expectLater(() async => await returnErrorVariant(variant: 0), throwsA(isA<CustomError>()));
      });

      test('Throw CustomError variant 1', () async {
        await expectLater(() async => await returnErrorVariant(variant: 1), throwsA(isA<CustomError>()));
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomError(), 3);
      });

      test('Throw CustomError static method', () async {
        await expectLater(
            () async => await SomeStruct.staticReturnErrCustomError(bridge: api), throwsA(isA<CustomError>()));
      });

      test('Throw CustomError static method, verifies implements Frb', () async {
        await expectLater(
            () async => await SomeStruct.staticReturnErrCustomError(bridge: api), throwsA(isA<FrbException>()));
      });

      test('Do not throw CustomError static method', () async {
        expect(await SomeStruct.staticReturnOkCustomError(bridge: api), 3);
      });

      test('Do not throw CustomError', () async {
        expect(await returnOkCustomError(), 3);
      });

      test('Throw CustomError non-static method', () async {
        await expectLater(
            () async => await SomeStruct(value: 7).nonStaticReturnErrCustomError(), throwsA(isA<CustomError>()));
        bool didCatch = false;
        try {
          await SomeStruct(value: 7).nonStaticReturnErrCustomError();
        } catch (e) {
          final FrbBacktracedException ex = e as FrbBacktracedException;
          print("backtrace: ${ex.backtrace}");
          assert(ex.backtrace.contains("wire_non_static_return_err_custom_error__method__SomeStruct::"));
          didCatch = true;
        }
        assert(didCatch);
      });

      test('Do not throw CustomError non-static method', () async {
        expect(await SomeStruct(value: 6).nonStaticReturnOkCustomError(), 6);
      });

      test('Throw anyhow error', () async {
        await expectLater(() async => await throwAnyhow(), throwsA(isA<FrbException>()));
        try {
          await throwAnyhow();
        } catch (e) {
          final FrbAnyhowException p = e as FrbAnyhowException;
          print("anyhow error: ${p.anyhow}");
          assert(p.anyhow.contains("anyhow error"));
        }
      });

      test('Function with custom result panics', () async {
        await expectLater(() async => await panicWithCustomResult(), throwsA(isA<FrbException>()));
        try {
          await panicWithCustomResult();
        } catch (e) {
          final PanicException p = e as PanicException;
          print("panic error: ${p.error}");
          assert(p.error.contains("just a panic"));
        }
      });

      test('Stream sink throw anyhow error', () async {
        expect(
          () async {
            await for (final _ in streamSinkThrowAnyhow()) {}
          },
          throwsA(
              isA<FrbAnyhowException>().having((e) => e.toString(), 'toString', 'FrbAnyhowException(anyhow error)')),
        );
      });
    });
  });
}
