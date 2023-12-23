import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('when load dylib fails, should have nice message', () async {
    await expectLater(
      () async => await RustLib.init(
        // deliberately bad external library
        externalLibrary: ExternalLibrary.process(),
      ),
      throwsA(isA<ArgumentError>().having(
          (x) => x.message, 'message', contains('This is often because'))),
    );
  });
}
