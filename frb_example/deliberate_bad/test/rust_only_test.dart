import 'package:frb_example_deliberate_bad/src/misc.dart';
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
        'cargo +nightly run ${CargoBuildAsanInfo.kExtraArgs.join(" ")} $name',
        extraEnv: CargoBuildAsanInfo.kExtraEnv,
        relativePwd: 'rust',
        expectSucceed: expectSucceed,
        expectStderrContains: expectStderrContains,
      );
    });
  }
}
