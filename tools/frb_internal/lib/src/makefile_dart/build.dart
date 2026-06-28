// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:io/io.dart';

part 'build.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'build-flutter',
      buildFlutter,
      _$populateBuildFlutterConfigParser,
      _$parseBuildFlutterConfigResult,
    ),
  ];
}

// We do not test web, since it is already tested when building the demo on website
enum BuildTarget { windows, macos, linux, androidAab, androidApk, ios, ohos }

@CliOptions()
class BuildFlutterConfig {
  @CliOption(
    defaultsTo: 'frb_example/flutter_via_create',
    convert: convertConfigPackage,
  )
  final String package;
  final BuildTarget target;

  const BuildFlutterConfig({required this.package, required this.target});
}

// ref: https://docs.flutter.dev/deployment
Future<void> buildFlutter(BuildFlutterConfig config) async {
  final outputDir = '${exec.pwd}target/build_flutter_output';
  Directory(outputDir).createSync(recursive: true);
  void copyArtifacts(List<String> paths) {
    for (final path in paths) {
      copyPath('${exec.pwd}${config.package}/$path', outputDir);
    }
  }

  switch (config.target) {
    case BuildTarget.windows:
      // https://docs.flutter.dev/deployment/windows
      // https://docs.flutter.dev/platform-integration/windows/building#compiling-with-visual-studio
      await exec(
        'flutter build windows --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/windows/x64/runner/Release']);

    case BuildTarget.macos:
      // https://docs.flutter.dev/deployment/macos
      await exec('flutter build macos --verbose', relativePwd: config.package);
      copyArtifacts(['build/macos/Build/Products/Release']);

    case BuildTarget.linux:
      // https://docs.flutter.dev/deployment/linux
      // https://stackoverflow.com/questions/73278689/how-to-run-a-standalone-linux-app-built-with-flutter
      await exec('flutter build linux --verbose', relativePwd: config.package);
      copyArtifacts([
        linuxBuildBundlePathForTesting(
          machineArchitecture: currentMachineArchitectureForTesting(),
        ),
      ]);

    case BuildTarget.androidAab:
      // https://docs.flutter.dev/deployment/android
      await exec(
        'flutter build appbundle --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/app/outputs/bundle/release']);

    case BuildTarget.androidApk:
      // https://docs.flutter.dev/deployment/android
      await exec('flutter build apk --verbose', relativePwd: config.package);
      copyArtifacts(['build/app/outputs/apk/release']);

    case BuildTarget.ios:
      // https://docs.flutter.dev/deployment/ios
      await exec(
        'flutter build ipa --no-codesign --verbose',
        relativePwd: config.package,
      );
      copyArtifacts(['build/ios/archive']);

    case BuildTarget.ohos:
      if (_shouldPatchOhosFlutterAutofillForCi()) {
        await exec('flutter pub get --verbose', relativePwd: config.package);
        await exec(
          'flutter precache --ohos --force --verbose',
          relativePwd: config.package,
        );
        await _patchOhosFlutterAutofillForCi(config.package);
        await exec(
          'flutter build hap --no-pub --no-codesign --verbose',
          relativePwd: config.package,
        );
      } else {
        await exec(
          'flutter build hap --no-codesign --verbose',
          relativePwd: config.package,
        );
      }
      copyArtifacts(['build/ohos/hap']);
  }
}

bool _shouldPatchOhosFlutterAutofillForCi() =>
    Platform.environment['FRB_OHOS_PATCH_FLUTTER_AUTOFILL_FOR_CI'] == '1';

Future<void> _patchOhosFlutterAutofillForCi(String package) async {
  final script = Platform.environment['FRB_OHOS_FLUTTER_AUTOFILL_PATCH_SCRIPT'];
  final flutterRoot = Platform.environment['FRB_OHOS_FLUTTER_ROOT'];
  if (script == null || script.isEmpty) {
    throw StateError('FRB_OHOS_FLUTTER_AUTOFILL_PATCH_SCRIPT is not set');
  }
  if (flutterRoot == null || flutterRoot.isEmpty) {
    throw StateError('FRB_OHOS_FLUTTER_ROOT is not set');
  }

  await exec('bash "$script" "$flutterRoot" "${exec.pwd}$package/ohos"');
}

String linuxBuildBundlePathForTesting({required String machineArchitecture}) =>
    'build/linux/${_linuxFlutterArchitecture(machineArchitecture)}/release/bundle';

String currentMachineArchitectureForTesting() {
  try {
    final result = Process.runSync('uname', ['-m']);
    if (result.exitCode != 0) {
      throw StateError(
        'Failed to detect machine architecture: ${result.stderr}',
      );
    }

    return (result.stdout as String).trim();
  } on ProcessException catch (error) {
    throw StateError(
      'Failed to run "uname" to detect machine architecture: ${error.message}',
    );
  }
}

String _linuxFlutterArchitecture(String machineArchitecture) =>
    switch (machineArchitecture) {
      'x86_64' || 'amd64' => 'x64',
      'aarch64' || 'arm64' => 'arm64',
      'riscv64' => 'riscv64',
      _ => throw UnsupportedError(
        'Unsupported Linux machine architecture: $machineArchitecture',
      ),
    };
