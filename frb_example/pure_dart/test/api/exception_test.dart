import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/exception.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcReturnErrorTwinNormal', () async {
    await expectLater(
        () async => funcReturnErrorTwinNormal(),
        throwsA(isA<AnyhowException>()
            .having((x) => x.message, 'message', 'return_err() is called, thus deliberately return Err')));
  });

  test('call funcReturnPanicTwinNormal', () async {
    await expectLater(
        () async => funcReturnPanicTwinNormal(),
        throwsA(isA<PanicException>()
            .having((x) => x.message, 'message', 'return_panic() is called, thus deliberately panic')));
  });
 
  TODO;
}
