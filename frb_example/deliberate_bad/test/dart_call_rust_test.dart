import 'package:test/scaffolding.dart';

import 'test_utils.dart';

void main() {
  for (final (name, expectSucceed, expectStderrContains) in [
    // NOTE It should fail, but ASAN did not realize this case...
    ('DartCallRust_StackBufferOverflow', true, ''),
    // ASAN successfully understand this case
    (
      'DartCallRust_HeapUseAfterFree',
      false,
      'ERROR: AddressSanitizer: heap-use-after-free'
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
