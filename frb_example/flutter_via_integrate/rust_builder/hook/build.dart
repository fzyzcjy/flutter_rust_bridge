import 'dart:convert';
import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

const _cratePath = '../rust';
const _temporaryToolchainMarker =
    '# Generated temporarily by flutter_rust_bridge Apple build hook.';

void main(List<String> args) async {
  await build(args, (input, output) async {
    if (!input.config.buildCodeAssets) {
      return;
    }

    final codeConfig = input.config.code;
    if (!_usesSwiftPackageManagerForApplePlatform(
      input.outputDirectoryShared,
      input.packageName,
      codeConfig.targetOS,
      output.dependencies.add,
    )) {
      return;
    }

    await _withTemporaryRustToolchain(
      packageRoot: input.packageRoot,
      outputDirectoryShared: input.outputDirectoryShared,
      cratePath: _cratePath,
      targetTriple: _targetTriple(codeConfig),
      addDependency: output.dependencies.add,
      action: () => FlutterRustBridgeNativeAssetsBuilder(
        assetName: 'rust_lib_flutter_via_integrate.dylib',
        cratePath: _cratePath,
        extraCargoEnvironmentVariables: {
          if (codeConfig.targetOS == OS.iOS)
            'IPHONEOS_DEPLOYMENT_TARGET': '${codeConfig.iOS.targetVersion}.0',
        },
      ).run(input: input, output: output),
    );
  });
}

String _targetTriple(CodeConfig config) {
  return switch ((config.targetOS, config.targetArchitecture)) {
    (OS.macOS, Architecture.arm64) => 'aarch64-apple-darwin',
    (OS.macOS, Architecture.x64) => 'x86_64-apple-darwin',
    (OS.iOS, Architecture.arm64)
        when config.iOS.targetSdk == IOSSdk.iPhoneSimulator =>
      'aarch64-apple-ios-sim',
    (OS.iOS, Architecture.arm64) => 'aarch64-apple-ios',
    (OS.iOS, Architecture.x64) => 'x86_64-apple-ios',
    _ => throw UnsupportedError(
        'Unsupported Apple target: ${config.targetOS} '
        'on ${config.targetArchitecture}',
      ),
  };
}

bool _usesSwiftPackageManagerForApplePlatform(
  Uri outputDirectory,
  String packageName,
  OS targetOS,
  void Function(Uri) addDependency,
) {
  final platformDirectory = switch (targetOS) {
    OS.iOS => 'ios',
    OS.macOS => 'macos',
    _ => null,
  };
  if (platformDirectory == null) {
    return false;
  }

  final manifest = _findFlutterSwiftPackageManifest(
    outputDirectory,
    platformDirectory,
    addDependency,
  );
  return manifest != null &&
      manifest.readAsStringSync().contains('name: "$packageName"');
}

File? _findFlutterSwiftPackageManifest(
  Uri outputDirectory,
  String platformDirectory,
  void Function(Uri) addDependency,
) {
  var directory = Directory.fromUri(outputDirectory);
  while (directory.parent.path != directory.path) {
    final pluginDependencies = File.fromUri(
      directory.uri.resolve('.flutter-plugins-dependencies'),
    );
    if (pluginDependencies.existsSync()) {
      addDependency(pluginDependencies.uri);
    }
    final candidate = File.fromUri(
      directory.uri.resolve(
        '$platformDirectory/Flutter/ephemeral/Packages/'
        'FlutterGeneratedPluginSwiftPackage/Package.swift',
      ),
    );
    if (candidate.existsSync()) {
      addDependency(candidate.uri);
      if (_swiftPackageManagerEnabled(pluginDependencies, platformDirectory) ==
          false) {
        return null;
      }
      return candidate;
    }
    directory = directory.parent;
  }
  return null;
}

bool? _swiftPackageManagerEnabled(File stateFile, String platformDirectory) {
  if (!stateFile.existsSync()) {
    return null;
  }
  try {
    final state = jsonDecode(stateFile.readAsStringSync());
    if (state
        case {
          'swift_package_manager_enabled': final Map<String, dynamic> platforms,
        }) {
      return platforms[platformDirectory] as bool?;
    }
  } on FormatException {
    return null;
  }
  return null;
}

Future<void> _withTemporaryRustToolchain({
  required Uri packageRoot,
  required Uri outputDirectoryShared,
  required String cratePath,
  required String targetTriple,
  required void Function(Uri) addDependency,
  required Future<void> Function() action,
}) async {
  final crateDirectory = Directory.fromUri(packageRoot.resolve('$cratePath/'));
  final toolchainFile = File.fromUri(
    crateDirectory.uri.resolve('rust-toolchain.toml'),
  );
  final lockFile = File.fromUri(
    outputDirectoryShared.resolve('frb_rust_toolchain.lock'),
  );
  await lockFile.create(recursive: true);
  final lock = await lockFile.open(mode: FileMode.write);

  try {
    await lock.lock(FileLock.exclusive);
    if (await toolchainFile.exists()) {
      final existing = await toolchainFile.readAsString();
      if (!existing.startsWith(_temporaryToolchainMarker)) {
        await _addBuildDependencies(crateDirectory, addDependency);
        await action();
        return;
      }
      await toolchainFile.delete();
    }

    await _addBuildDependencies(crateDirectory, addDependency);
    final channel = await _currentRustToolchainChannel(crateDirectory);
    final generated = '$_temporaryToolchainMarker\n'
        '[toolchain]\n'
        'channel = "$channel"\n'
        'targets = ["$targetTriple"]\n';
    await toolchainFile.writeAsString(generated);

    try {
      await action();
    } finally {
      if (await toolchainFile.exists() &&
          await toolchainFile.readAsString() == generated) {
        await toolchainFile.delete();
      }
    }
  } finally {
    await lock.unlock();
    await lock.close();
  }
}

Future<void> _addBuildDependencies(
  Directory crateDirectory,
  void Function(Uri) addDependency,
) async {
  for (final name in const ['Cargo.toml', 'Cargo.lock', 'build.rs']) {
    final file = File.fromUri(crateDirectory.uri.resolve(name));
    if (await file.exists()) {
      addDependency(file.uri);
    }
  }

  final configuredToolchain = _findRustToolchain(crateDirectory);
  if (configuredToolchain != null) {
    addDependency(configuredToolchain.uri);
  }

  final home = Platform.environment['HOME'];
  if (home != null) {
    final rustupSettings = File('$home/.rustup/settings.toml');
    if (await rustupSettings.exists()) {
      addDependency(rustupSettings.uri);
    }
  }

  try {
    final result = await Process.run(
        'rustup',
        [
          'which',
          'rustc',
        ],
        workingDirectory: crateDirectory.path);
    if (result.exitCode == 0) {
      final rustc = File(result.stdout.toString().trim());
      if (await rustc.exists()) {
        addDependency(rustc.uri);
      }
    }
  } on ProcessException {
    // rustup is optional when Rust is installed through another toolchain.
  }
}

File? _findRustToolchain(Directory start) {
  var directory = start;
  while (true) {
    for (final name in const ['rust-toolchain.toml', 'rust-toolchain']) {
      final candidate = File.fromUri(directory.uri.resolve(name));
      if (candidate.existsSync()) {
        return candidate;
      }
    }
    if (directory.parent.path == directory.path) {
      return null;
    }
    directory = directory.parent;
  }
}

Future<String> _currentRustToolchainChannel(Directory crateDirectory) async {
  final result = await Process.run(
      'rustc',
      [
        '--version',
        '--verbose',
      ],
      workingDirectory: crateDirectory.path);
  if (result.exitCode != 0) {
    throw ProcessException(
      'rustc',
      const ['--version', '--verbose'],
      result.stderr.toString(),
      result.exitCode,
    );
  }

  final output = result.stdout.toString();
  final release = _rustcVersionField(output, 'release');
  if (release.contains('-nightly')) {
    return 'nightly-${_rustcVersionField(output, 'commit-date')}';
  }
  if (release.contains('-beta')) {
    return 'beta-${_rustcVersionField(output, 'commit-date')}';
  }
  if (!RegExp(r'^\d+\.\d+\.\d+$').hasMatch(release)) {
    throw StateError('Unsupported rustc release: $release');
  }
  return release;
}

String _rustcVersionField(String output, String field) {
  final prefix = '$field: ';
  for (final line in output.split('\n')) {
    if (line.startsWith(prefix)) {
      return line.substring(prefix.length).trim();
    }
  }
  throw StateError('Missing `$field` in `rustc --version --verbose` output');
}
