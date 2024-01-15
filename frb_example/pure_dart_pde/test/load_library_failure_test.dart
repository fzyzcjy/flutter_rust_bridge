import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('when load dylib fails, should have nice message',
      skip: !Platform.isLinux, () async {
    await expectLater(
      () async => await RustLib.init(
        // deliberately bad external library, on macos/linux
        externalLibrary: ExternalLibrary.process(iKnowHowToUseIt: true),
      ),
      throwsA(isA<ArgumentError>().having(
          (x) => x.message, 'message', contains('This is often because'))),
    );
  });
}
