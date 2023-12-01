import 'package:test/expect.dart';
import 'package:test/scaffolding.dart';

import 'test_utils.dart';

void main() {
  for (final (name, expectSucceed, expectOutputContains) in [
    ('Good', true, 'This is good code'),
    (
      'StackBufferOverflow',
      false,
      'ERROR: AddressSanitizer: stack-buffer-overflow',
    ),
  ]) {
    test('name=$name', () async {
      await _execAndCheck(
        'cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu $name',
        extraEnv: {'RUSTFLAGS': '-Zsanitizer=address'},
        relativePwd: 'rust',
        expectSucceed: expectSucceed,
        expectOutputContains: expectOutputContains,
      );
    });
  }
}

Future<void> _execAndCheck(
  String cmd, {
  String? relativePwd,
  Map<String, String>? extraEnv,
  required bool expectSucceed,
  required String expectOutputContains,
}) async {
  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: extraEnv,
    checkExitCode: false,
  );

  expect(output.exitCode, expectSucceed ? 0 : isNot(0));
  expect(output.stdout, contains(expectOutputContains));
}
