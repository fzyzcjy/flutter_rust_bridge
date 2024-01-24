// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'post_release.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
        'post-release-mimic-quickstart',
        postReleaseMimicQuickstart,
        _$populatePostReleaseConfigParser,
        _$parsePostReleaseConfigResult),
  ];
}

@CliOptions()
class PostReleaseConfig {
  final CodegenInstallMode codegenInstallMode;

  const PostReleaseConfig({required this.codegenInstallMode});
}

Future<void> postReleaseMimicQuickstart(PostReleaseConfig config) async {
  await quickstartStepInstall(config.codegenInstallMode,
      versionConstraint: '^2.0.0-dev.0');
  await const MimicQuickstartTester(postRelease: true).test();
}

enum CodegenInstallMode {
  cargoInstall,
  cargoBinstall,
  scoop,
  homebrew,
}

Future<void> quickstartStepInstall(CodegenInstallMode mode,
    {required String versionConstraint}) async {
  switch (mode) {
    case CodegenInstallMode.cargoInstall:
      await exec(
          "cargo install 'flutter_rust_bridge_codegen@$versionConstraint'");
    case CodegenInstallMode.cargoBinstall:
      await exec(
          "cargo binstall -y 'flutter_rust_bridge_codegen@$versionConstraint'");
    case CodegenInstallMode.scoop:
      await exec(
          'scoop bucket add frb https://github.com/Desdaemon/scoop-repo');
      await exec('scoop install flutter_rust_bridge_codegen');
    case CodegenInstallMode.homebrew:
      await exec('brew install desdaemon/repo/flutter_rust_bridge_codegen');
  }
}
