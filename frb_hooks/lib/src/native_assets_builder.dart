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
      final codeConfig = effectiveInput.config.code;
      await native_toolchain_rust.RustBuilder(
        assetName: assetName,
        cratePath: cratePath,
        buildMode: buildMode,
        enableDefaultFeatures: enableDefaultFeatures,
        features: features,
        extraCargoBuildArgs: extraCargoBuildArgs,
        extraCargoEnvironmentVariables: cargoEnvironmentWithAndroidPageSize(
          targetOS: codeConfig.targetOS,
          targetArchitecture: codeConfig.targetArchitecture,
          parentEnvironment: Platform.environment,
          extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
        ),
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
Map<String, String> cargoEnvironmentWithAndroidPageSize({
  required OS targetOS,
  required Architecture targetArchitecture,
  required Map<String, String> parentEnvironment,
  required Map<String, String> extraCargoEnvironmentVariables,
}) {
  final String? targetRustFlagsVariable = switch ((
    targetOS,
    targetArchitecture,
  )) {
    (OS.android, Architecture.arm64) =>
      'CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS',
    (OS.android, Architecture.x64) =>
      'CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS',
    _ => null,
  };
  if (targetRustFlagsVariable == null) {
    return extraCargoEnvironmentVariables;
  }

  final Map<String, String> effectiveEnvironment = {
    ...parentEnvironment,
    ...extraCargoEnvironmentVariables,
  };
  final String rustFlagsVariable;
  final String separator;
  if (effectiveEnvironment.containsKey('CARGO_ENCODED_RUSTFLAGS')) {
    rustFlagsVariable = 'CARGO_ENCODED_RUSTFLAGS';
    separator = '\x1f';
  } else if (effectiveEnvironment.containsKey('RUSTFLAGS')) {
    rustFlagsVariable = 'RUSTFLAGS';
    separator = ' ';
  } else if (effectiveEnvironment.containsKey(targetRustFlagsVariable)) {
    rustFlagsVariable = targetRustFlagsVariable;
    separator = ' ';
  } else if (effectiveEnvironment.containsKey('CARGO_BUILD_RUSTFLAGS')) {
    rustFlagsVariable = 'CARGO_BUILD_RUSTFLAGS';
    separator = ' ';
  } else {
    rustFlagsVariable = targetRustFlagsVariable;
    separator = ' ';
  }
  final existingRustFlags = effectiveEnvironment[rustFlagsVariable] ?? '';
  final List<String> additionalRustFlags = [
    for (final linkerArgument in _androidPageSizeLinkerArguments)
      if (!existingRustFlags.contains(linkerArgument)) ...[
        '-C',
        linkerArgument,
      ],
  ];
  if (additionalRustFlags.isEmpty) {
    return extraCargoEnvironmentVariables;
  }

  return {
    ...extraCargoEnvironmentVariables,
    rustFlagsVariable: [
      if (existingRustFlags.isNotEmpty) existingRustFlags,
      ...additionalRustFlags,
    ].join(separator),
  };
}

const _androidPageSizeLinkerArguments = [
  'link-arg=-Wl,-z,max-page-size=16384',
  'link-arg=-Wl,-z,common-page-size=16384',
];

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
