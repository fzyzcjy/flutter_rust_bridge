import 'dart:io';

import 'package:hooks/hooks.dart';
import 'package:logging/logging.dart';
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
    this.buildMode,
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

  /// The Cargo build mode. When omitted, flutter_rust_bridge chooses from the
  /// hook input config.
  final FlutterRustBridgeBuildMode? buildMode;

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
      final effectiveInput = buildInputForHost(
        isWindows: Platform.isWindows,
        input: input,
        windowsOutputDirectoryShared: Platform.isWindows
            ? await _prepareShortWindowsOutputDirectoryShared(
                input.outputDirectoryShared,
              )
            : null,
      );
      final effectiveBuildMode =
          buildMode ??
          buildModeFromLinkingEnabled(
            linkingEnabled: input.config.linkingEnabled,
          );
      final builder = native_toolchain_rust.RustBuilder(
        assetName: assetName,
        cratePath: cratePath,
        buildMode: effectiveBuildMode,
        enableDefaultFeatures: enableDefaultFeatures,
        features: features,
        extraCargoBuildArgs: extraCargoBuildArgs,
        extraCargoEnvironmentVariables: defaultCargoEnvironmentVariablesForHost(
          isWindows: Platform.isWindows,
          userEnvironmentVariables: extraCargoEnvironmentVariables,
        ),
      );
      await builder.run(
        input: effectiveInput,
        output: output,
        assetRouting: assetRouting,
        logger: logger,
      );
    });
  }
}

/// Returns the default Cargo build mode for a Dart/Flutter hook invocation.
FlutterRustBridgeBuildMode buildModeFromLinkingEnabled({
  required bool linkingEnabled,
}) => linkingEnabled
    ? native_toolchain_rust.BuildMode.release
    : native_toolchain_rust.BuildMode.debug;

/// Returns Cargo environment variables that make FRB native-assets builds predictable.
Map<String, String> defaultCargoEnvironmentVariablesForHost({
  required bool isWindows,
  required Map<String, String> userEnvironmentVariables,
}) => {if (isWindows) 'CARGO_BUILD_JOBS': '1', ...userEnvironmentVariables};

/// Returns a build input adjusted for host-specific Native Assets behavior.
BuildInput buildInputForHost({
  required bool isWindows,
  required BuildInput input,
  required Uri? windowsOutputDirectoryShared,
}) {
  if (!isWindows || windowsOutputDirectoryShared == null) {
    return input;
  }
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
