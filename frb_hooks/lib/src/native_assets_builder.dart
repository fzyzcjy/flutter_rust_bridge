import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart';
import 'package:logging/logging.dart';
import 'package:meta/meta.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart'
    as native_toolchain_rust;

export 'package:native_toolchain_rust/native_toolchain_rust.dart'
    show RustBuildException;

/// The mode used for `cargo build`.
typedef FlutterRustBridgeBuildMode = native_toolchain_rust.BuildMode;

/// Builds flutter_rust_bridge Rust code through Dart/Flutter Native Assets.
final class FlutterRustBridgeNativeAssetsBuilder implements Builder {
  /// Creates a Native Assets builder for a flutter_rust_bridge Rust crate.
  const FlutterRustBridgeNativeAssetsBuilder({
    this.assetName = 'src/rust/frb_generated.io.dart',
    this.cratePath = 'rust',
    this.buildMode = native_toolchain_rust.BuildMode.release,
    this.enableDefaultFeatures = true,
    this.features = const <String>[],
    this.extraCargoBuildArgs = const <String>[],
    this.extraCargoEnvironmentVariables = const <String, String>{},
    this.assetRouting = const <AssetRouting>[ToAppBundle()],
  });

  /// The Dart library URI, relative to `lib/`, that owns the generated binding.
  final String assetName;

  /// The Rust crate path, relative to the Dart package root.
  final String cratePath;

  /// The Cargo build mode.
  final FlutterRustBridgeBuildMode buildMode;

  /// Whether Cargo should enable default crate features.
  final bool enableDefaultFeatures;

  /// Cargo features to enable.
  final List<String> features;

  /// Extra arguments passed to `cargo build`.
  final List<String> extraCargoBuildArgs;

  /// Extra environment variables passed to `cargo build`.
  final Map<String, String> extraCargoEnvironmentVariables;

  /// How the produced code asset is routed.
  final List<AssetRouting> assetRouting;

  /// Runs the Native Assets build.
  @override
  Future<void> run({
    required BuildInput input,
    required BuildOutputBuilder output,
    Logger? logger,
  }) async {
    await _withOutputDirectoryBuildLock(input.outputDirectory, () async {
      final effectiveInput = await buildInputForHost(
        isWindows: Platform.isWindows,
        input: input,
      );
      final effectiveCargoEnvironment = await cargoEnvironmentForInput(
        input: effectiveInput,
        isWindows: Platform.isWindows,
        extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
      );
      await native_toolchain_rust.RustBuilder(
        assetName: assetName,
        cratePath: cratePath,
        buildMode: buildMode,
        enableDefaultFeatures: enableDefaultFeatures,
        features: features,
        extraCargoBuildArgs: extraCargoBuildArgs,
        extraCargoEnvironmentVariables: effectiveCargoEnvironment,
      ).run(
        input: effectiveInput,
        output: output,
        assetRouting: assetRouting,
        logger: logger,
      );
    });
  }
}

@visibleForTesting
Future<Map<String, String>> cargoEnvironmentForInput({
  required BuildInput input,
  required bool isWindows,
  required Map<String, String> extraCargoEnvironmentVariables,
}) async {
  if (!input.config.buildCodeAssets) {
    return extraCargoEnvironmentVariables;
  }

  final codeConfig = input.config.code;
  return cargoEnvironmentWithAndroidPageSize(
    targetOS: codeConfig.targetOS,
    targetArchitecture: codeConfig.targetArchitecture,
    compiler: codeConfig.cCompiler?.compiler,
    outputDirectory: input.outputDirectory,
    isWindows: isWindows,
    extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
  );
}

@visibleForTesting
Future<Map<String, String>> cargoEnvironmentWithAndroidPageSize({
  required OS targetOS,
  required Architecture targetArchitecture,
  required Uri? compiler,
  required Uri outputDirectory,
  required bool isWindows,
  required Map<String, String> extraCargoEnvironmentVariables,
}) async {
  final String? targetTriple = switch ((targetOS, targetArchitecture)) {
    (OS.android, Architecture.arm64) => 'aarch64-linux-android',
    (OS.android, Architecture.x64) => 'x86_64-linux-android',
    _ => null,
  };
  if (targetTriple == null) {
    return extraCargoEnvironmentVariables;
  }
  if (compiler == null) {
    throw UnsupportedError(
      'CCompilerConfig is required for Android target $targetTriple',
    );
  }

  final String linkerEnvironmentVariable =
      'CARGO_TARGET_${targetTriple.replaceAll('-', '_').toUpperCase()}_LINKER';
  final String defaultLinker = File.fromUri(compiler).parent.uri
      .resolve('${targetTriple}35-clang${isWindows ? '.cmd' : ''}')
      .toFilePath();
  final String linker =
      extraCargoEnvironmentVariables[linkerEnvironmentVariable] ??
      defaultLinker;
  final File wrapper = File.fromUri(
    Directory.fromUri(outputDirectory).uri.resolve(
      'flutter_rust_bridge_android_linker_${targetArchitecture.name}'
      '${isWindows ? '.cmd' : '.sh'}',
    ),
  );
  await wrapper.writeAsString(
    isWindows ? _windowsLinkerWrapper(linker) : _posixLinkerWrapper(linker),
  );
  if (!isWindows) {
    final ProcessResult chmod = await Process.run('chmod', [
      '+x',
      wrapper.path,
    ]);
    if (chmod.exitCode != 0) {
      throw FileSystemException(
        'Failed to make Android linker wrapper executable: ${chmod.stderr}',
        wrapper.path,
      );
    }
  }

  return {
    ...extraCargoEnvironmentVariables,
    linkerEnvironmentVariable: wrapper.path,
  };
}

const _androidPageSizeLinkerArguments = [
  '-z',
  'max-page-size=16384',
  '-z',
  'common-page-size=16384',
];

String _posixLinkerWrapper(String linker) =>
    '#!/bin/sh\n'
    'exec ${_posixShellQuote(linker)} "\$@" '
    '${_androidPageSizeLinkerArguments.join(' ')}\n';

String _windowsLinkerWrapper(String linker) =>
    '@echo off\r\n'
    '"$linker" %* ${_androidPageSizeLinkerArguments.join(' ')}\r\n';

String _posixShellQuote(String value) =>
    "'${value.replaceAll("'", "'\"'\"'")}'";

/// Returns a build input adjusted for host-specific Native Assets behavior.
@visibleForTesting
Future<BuildInput> buildInputForHost({
  required bool isWindows,
  required BuildInput input,
}) async {
  if (!isWindows) {
    return input;
  }

  final windowsOutputDirectoryShared =
      await _prepareShortWindowsOutputDirectoryShared(
        input.outputDirectoryShared,
      );
  // Keep Windows Native Assets output paths short. native_toolchain_rust places
  // Cargo artifacts under input.outputDirectory/target, and Flutter hook output
  // roots can otherwise make those paths exceed Windows toolchain limits.
  return BuildInput({
    ...input.json,
    'out_dir_shared': Directory.fromUri(windowsOutputDirectoryShared).path,
  });
}

Future<Uri> _prepareShortWindowsOutputDirectoryShared(
  Uri outputDirectoryShared,
) async {
  final shortOutputDirectoryShared = Directory(
    '${Directory.systemTemp.path}${Platform.pathSeparator}'
    'frb_native_assets_${_stablePathHash(outputDirectoryShared.toString())}',
  );
  await shortOutputDirectoryShared.create(recursive: true);
  return shortOutputDirectoryShared.uri;
}

Future<T> _withOutputDirectoryBuildLock<T>(
  Uri outputDirectory,
  Future<T> Function() action,
) async {
  final lockFile = File(
    '${Directory.fromUri(outputDirectory).path}'
    '${Platform.pathSeparator}.flutter_rust_bridge_native_assets_build.lock',
  );
  await lockFile.create(recursive: true);
  final lock = await lockFile.open(mode: FileMode.write);

  try {
    await lock.lock(FileLock.exclusive);
    return await action();
  } finally {
    await lock.unlock();
    await lock.close();
  }
}

String _stablePathHash(String value) {
  var hash = 0x811c9dc5;
  for (final codeUnit in value.codeUnits) {
    hash ^= codeUnit;
    hash = (hash * 0x01000193) & 0xffffffff;
  }
  return hash.toRadixString(16).padLeft(8, '0');
}
