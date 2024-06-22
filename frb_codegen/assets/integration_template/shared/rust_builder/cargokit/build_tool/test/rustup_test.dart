import 'package:build_tool/src/rustup.dart';
import 'package:build_tool/src/util.dart';
import 'package:test/test.dart';

void main() {
  test('rustup with no toolchains', () {
    bool didListToolchains = false;
    bool didInstallStable = false;
    bool didListTargets = false;
    testRunCommandOverride = (args) {
      expect(args.executable, 'rustup');
      switch (args.arguments) {
        case ['toolchain', 'list']:
          didListToolchains = true;
          return TestRunCommandResult(stdout: 'no installed toolchains\n');
        case ['toolchain', 'install', 'stable']:
          didInstallStable = true;
          return TestRunCommandResult();
        case ['target', 'list', '--toolchain', 'stable', '--installed']:
          didListTargets = true;
          return TestRunCommandResult(
              stdout: 'x86_64-unknown-linux-gnu\nx86_64-apple-darwin\n');
        default:
          throw Exception('Unexpected call: ${args.arguments}');
      }
    };
    final rustup = Rustup();
    rustup.installToolchain('stable');
    expect(didInstallStable, true);
    expect(didListToolchains, true);
    expect(didListTargets, true);
    expect(rustup.installedTargets('stable'), [
      'x86_64-unknown-linux-gnu',
      'x86_64-apple-darwin',
    ]);
    testRunCommandOverride = null;
  });

  test('rustup with esp toolchain', () {
    final targetsQueried = <String>[];
    testRunCommandOverride = (args) {
      expect(args.executable, 'rustup');
      switch (args.arguments) {
        case ['toolchain', 'list']:
          return TestRunCommandResult(
              stdout: 'stable-aarch64-apple-darwin (default)\n'
                  'nightly-aarch64-apple-darwin\n'
                  'esp\n');
        case ['target', 'list', '--toolchain', String toolchain, '--installed']:
          targetsQueried.add(toolchain);
          return TestRunCommandResult(stdout: '$toolchain:target\n');
        default:
          throw Exception('Unexpected call: ${args.arguments}');
      }
    };
    final rustup = Rustup();
    expect(targetsQueried, [
      'stable-aarch64-apple-darwin',
      'nightly-aarch64-apple-darwin',
    ]);
    expect(rustup.installedTargets('stable'),
        ['stable-aarch64-apple-darwin:target']);
    expect(rustup.installedTargets('nightly'),
        ['nightly-aarch64-apple-darwin:target']);
  });
}
