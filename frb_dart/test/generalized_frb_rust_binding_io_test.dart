@TestOn('vm')
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:test/test.dart';

void main() {
  test('package prefix is applied to content hash lookup', () {
    final binding = GeneralizedFrbRustBinding(
      ExternalLibrary.process(iKnowHowToUseIt: true),
      nativeSymbolPrefix: 'package_a_',
    );
    expect(
      binding.getRustContentHash,
      throwsA(
        isA<ArgumentError>().having(
          (error) => error.toString(),
          'symbol',
          contains('package_a_frb_get_rust_content_hash'),
        ),
      ),
    );
  });

  test('shared runtime lookup does not use the package prefix', () {
    final binding = GeneralizedFrbRustBinding(
      ExternalLibrary.process(iKnowHowToUseIt: true),
      nativeSymbolPrefix: 'package_a_',
    );
    expect(
      binding.storeDartPostCObject,
      throwsA(
        isA<ArgumentError>().having(
          (error) => error.toString(),
          'symbol',
          allOf(
            contains('store_dart_post_cobject'),
            isNot(contains('package_a_store_dart_post_cobject')),
          ),
        ),
      ),
    );
  });

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
