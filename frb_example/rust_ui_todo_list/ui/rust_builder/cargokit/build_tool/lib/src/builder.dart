/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';

import 'package:collection/collection.dart';
import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'android_environment.dart';
import 'cargo.dart';
import 'environment.dart';
import 'options.dart';
import 'rustup.dart';
import 'target.dart';
import 'util.dart';

final _log = Logger('builder');

enum BuildConfiguration {
  debug,
  release,
  profile,
}

extension on BuildConfiguration {
  bool get isDebug => this == BuildConfiguration.debug;
  String get rustName => switch (this) {
        BuildConfiguration.debug => 'debug',
        BuildConfiguration.release => 'release',
        BuildConfiguration.profile => 'release',
      };
}

class BuildException implements Exception {
  final String message;

  BuildException(this.message);

  @override
  String toString() {
    return 'BuildException: $message';
  }
}

class BuildEnvironment {
  final BuildConfiguration configuration;
  final CargokitCrateOptions crateOptions;
  final String targetTempDir;
  final String manifestDir;
  final CrateInfo crateInfo;

  final bool isAndroid;
  final String? androidSdkPath;
  final String? androidNdkVersion;
  final int? androidMinSdkVersion;
  final String? javaHome;
  final String? ohosSdkHome;

  final String? glibcVersion;

  BuildEnvironment({
    required this.configuration,
    required this.crateOptions,
    required this.targetTempDir,
    required this.manifestDir,
    required this.crateInfo,
    required this.isAndroid,
    this.androidSdkPath,
    this.androidNdkVersion,
    this.androidMinSdkVersion,
    this.javaHome,
    this.glibcVersion,
    this.ohosSdkHome,
  });

  static BuildConfiguration parseBuildConfiguration(String value) {
    // XCode configuration adds the flavor to configuration name.
    final firstSegment = value.split('-').first;
    final buildConfiguration = BuildConfiguration.values.firstWhereOrNull(
      (e) => e.name == firstSegment,
    );
    if (buildConfiguration == null) {
      _log.warning('Unknown build configuraiton $value, will assume release');
      return BuildConfiguration.release;
    }
    return buildConfiguration;
  }

  static BuildEnvironment fromEnvironment({
    required bool isAndroid,
  }) {
    final buildConfiguration =
        parseBuildConfiguration(Environment.configuration);
    final manifestDir = Environment.manifestDir;
    final crateOptions = CargokitCrateOptions.load(
      manifestDir: manifestDir,
    );
    final crateInfo = CrateInfo.load(manifestDir);
    return BuildEnvironment(
      configuration: buildConfiguration,
      crateOptions: crateOptions,
      targetTempDir: Environment.targetTempDir,
      manifestDir: manifestDir,
      crateInfo: crateInfo,
      isAndroid: isAndroid,
      androidSdkPath: isAndroid ? Environment.sdkPath : null,
      androidNdkVersion: isAndroid ? Environment.ndkVersion : null,
      androidMinSdkVersion:
          isAndroid ? int.parse(Environment.minSdkVersion) : null,
      javaHome: isAndroid ? Environment.javaHome : null,
      ohosSdkHome: Environment.ohosSdkHome,
    );
  }
}

class RustBuilder {
  final Target target;
  final BuildEnvironment environment;
  String? _resolvedToolchain;

  RustBuilder({
    required this.target,
    required this.environment,
  });

  void prepare(
    Rustup rustup,
  ) {
    final toolchain = _toolchain;
    if (rustup.installedTargets(toolchain) == null) {
      rustup.installToolchain(toolchain);
    }
    final resolvedToolchain = rustup.resolveToolchain(toolchain);
    if (toolchain == 'nightly') {
      rustup.installRustSrcForNightly(toolchain: resolvedToolchain);
    }
    if (!rustup.installedTargets(toolchain)!.contains(target.rust)) {
      rustup.installTarget(target.rust, toolchain: resolvedToolchain);
    }
    if (environment.glibcVersion != null) {
      rustup.installZigBuild(resolvedToolchain);
    }
    _resolvedToolchain = resolvedToolchain;
  }

  CargoBuildOptions? get _buildOptions =>
      environment.crateOptions.cargo[environment.configuration];

  String get _toolchain => _buildOptions?.toolchain.name ?? 'stable';
  String get _effectiveToolchain => _resolvedToolchain ?? _toolchain;

  /// Returns the path of directory containing build artifacts.
  Future<String> build() async {
    final extraArgs = _buildOptions?.flags ?? [];
    final manifestPath = path.join(environment.manifestDir, 'Cargo.toml');
    runCommand(
      'rustup',
      [
        'run',
        _effectiveToolchain,
        'cargo',
        (target.android == null && environment.glibcVersion != null)
            ? 'zigbuild'
            : 'build',
        ...extraArgs,
        '--manifest-path',
        manifestPath,
        '-p',
        environment.crateInfo.packageName,
        if (!environment.configuration.isDebug) '--release',
        '--target',
        target.rust +
            ((target.android == null && environment.glibcVersion != null)
                ? '.${environment.glibcVersion!}'
                : ""),
        '--target-dir',
        environment.targetTempDir,
      ],
      environment: await _buildEnvironment(),
    );
    return path.join(
      environment.targetTempDir,
      target.rust,
      environment.configuration.rustName,
    );
  }

  Future<Map<String, String>> _buildEnvironment() async {
    if (target.ohos != null) {
      return _buildOhosEnv();
    }
    if (target.android != null) {
      return _buildAndroidEnv();
    }
    return {};
  }

  Future<Map<String, String>> _buildAndroidEnv() {
    final sdkPath = environment.androidSdkPath;
    final ndkVersion = environment.androidNdkVersion;
    final minSdkVersion = environment.androidMinSdkVersion;
    if (sdkPath == null) {
      throw BuildException('androidSdkPath is not set');
    }
    if (ndkVersion == null) {
      throw BuildException('androidNdkVersion is not set');
    }
    if (minSdkVersion == null) {
      throw BuildException('androidMinSdkVersion is not set');
    }
    final env = AndroidEnvironment(
      sdkPath: sdkPath,
      ndkVersion: ndkVersion,
      minSdkVersion: minSdkVersion,
      targetTempDir: environment.targetTempDir,
      target: target,
    );
    if (!env.ndkIsInstalled() && environment.javaHome != null) {
      env.installNdk(javaHome: environment.javaHome!);
    }
    return env.buildEnvironment();
  }

  Map<String, String> _buildOhosEnv() {
    final sdkPath = environment.ohosSdkHome;
    if (sdkPath == null) {
      throw BuildException('OHOS SDK native path is not set');
    }
    final exe = Platform.isWindows ? '.exe' : '';
    final clangPath = path.join(sdkPath, 'llvm', 'bin', 'clang$exe');
    final sysroot = path.join(sdkPath, 'sysroot');
    String clangTarget;
    switch (target.ohos) {
      case 'arm64-v8a':
        clangTarget = 'aarch64-linux-ohos';
        break;
      case 'armeabi-v7a':
        clangTarget = 'arm-linux-ohos';
        break;
      case 'x86_64':
        clangTarget = 'x86_64-linux-ohos';
        break;
      default:
        clangTarget = 'aarch64-linux-ohos';
    }
    final targetEnvName = target.rust.toUpperCase().replaceAll('-', '_');
    final linkerEnvVar = 'CARGO_TARGET_${targetEnvName}_LINKER';
    final rustFlagsEnvVar = 'CARGO_TARGET_${targetEnvName}_RUSTFLAGS';
    final rustFlags = '-C link-arg=--target=$clangTarget '
        '-C link-arg=-fuse-ld=lld '
        '-C link-arg=--sysroot=$sysroot '
        '-C link-arg=-D__MUSL__';
    return {
      rustFlagsEnvVar: rustFlags,
      linkerEnvVar: clangPath,
      'CC_${target.rust}': clangPath,
      'AR_${target.rust}': path.join(sdkPath, 'llvm', 'bin', 'llvm-ar$exe'),
    };
  }
}
