import 'package:test/scaffolding.dart';

import 'test_utils.dart';

void main() {
  for (final (name, expectSucceed, expectStderrContains) in [
    ('DartCallRust_StackBufferOverflow', false, 'TODO'),
    ('DartCallRust_HeapUseAfterFree', false, 'TODO'),
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
