import 'package:test/scaffolding.dart';

import 'test_utils.dart';

void main() {
  // TODO do not hardcode
  const sanitizedDart =
      '/home/cs144/dart-sdk/sdk/out/ReleaseASANX64/dart-sdk/bin/dart';

  for (final (name, expectSucceed, expectStderrContains) in [
    ('DartOnly_Good', true, ''),
    (
      'DartOnly_HeapUseAfterFree',
      false,
      'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ]) {
    test('name=$name', () async {
      await execAndCheck(
        '$sanitizedDart --enable-experiment=native-assets run '
        'frb_example_deliberate_bad $name',
        relativePwd: '.',
        expectSucceed: expectSucceed,
        expectStderrContains: expectStderrContains,
      );
    });
  }
}
