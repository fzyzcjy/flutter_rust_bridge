import 'package:test/scaffolding.dart';

import 'test_utils.dart';

void main() {
  for (final (name, expectSucceed, expectStderrContains) in [
    ('RustOnly_Good', true, ''),
    (
      'RustOnly_StackBufferOverflow',
      false,
      'ERROR: AddressSanitizer: stack-buffer-overflow',
    ),
    (
      'RustOnly_HeapUseAfterFree',
      false,
      'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ]) {
    test('name=$name', () async {
      await execAndCheck(
        'cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu $name',
        extraEnv: {'RUSTFLAGS': '-Zsanitizer=address'},
        relativePwd: 'rust',
        expectSucceed: expectSucceed,
        expectStderrContains: expectStderrContains,
      );
    });
  }
}
