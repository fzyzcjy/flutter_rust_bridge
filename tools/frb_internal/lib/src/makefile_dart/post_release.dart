// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('post-release-mimic-quickstart', postReleaseMimicQuickstart),
  ];
}

Future<void> postReleaseMimicQuickstart() async {
  await _stepInstall(config.installMode);
  await _stepCreate();
  await _stepRun();
  await _stepModifyAndGenerate();
}

const _kPackageName = 'my_app';

enum InstallMode {
  cargoInstall,
  cargoBinstall,
  scoop,
  homebrew,
}

Future<void> _stepInstall(InstallMode mode) async {
  switch (mode) {
    case InstallMode.cargoInstall:
      await exec('cargo install flutter_rust_bridge_codegen');
    case InstallMode.cargoBinstall:
      await exec('cargo binstall flutter_rust_bridge_codegen');
    case InstallMode.scoop:
      await exec(
          'scoop bucket add frb https://github.com/Desdaemon/scoop-repo');
      await exec('scoop install flutter_rust_bridge_codegen');
    case InstallMode.homebrew:
      await exec('brew install desdaemon/repo/flutter_rust_bridge_codegen');
  }
}

Future<void> _stepCreate() async {
  await exec('flutter_rust_bridge_codegen create $_kPackageName');
}

Future<void> _stepRun() async {
  throw UnimplementedError();
}

Future<void> _stepModifyAndGenerate() async {
  throw UnimplementedError();
}
