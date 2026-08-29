@TestOn('vm')
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:test/test.dart';

void main() {
  test('missing native symbols include external library debug information', () {
    final binding = GeneralizedFrbRustBinding(
      ExternalLibrary.process(
        iKnowHowToUseIt: true,
        debugInfo: 'test process library',
      ),
    );

    expect(
      binding.getRustContentHash,
      throwsA(
        isA<ArgumentError>()
            .having(
              (error) => error.toString(),
              'message',
              contains('Rust library is not loaded correctly'),
            )
            .having(
              (error) => error.toString(),
              'debug information',
              contains('by process()test process library'),
            ),
      ),
    );
  });
}
