import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

// Linux-side raw create/integrate does not preserve the checked-in Apple scaffold.
// In exact remote reproductions it drops the iOS stanza from .metadata, removes
// iOS plugin declarations from flutter_package/pubspec.yaml, and leaves ios/*
// or example/ios/* plus macOS Podfiles absent. Treat the checked-in mac-generated
// Apple scaffold as source-of-truth, and explicitly apply it before diff
// comparison until integrate/create can produce the same result directly.
const _kIntegrateAppleScaffoldSourceOfTruthPaths = <String, List<String>>{
  'frb_example/flutter_via_create': ['.metadata', 'ios', 'macos/Podfile'],
  'frb_example/flutter_via_create_native_assets': [
    '.metadata',
    'ios',
    'macos/Podfile',
  ],
  'frb_example/flutter_via_integrate': ['.metadata', 'ios', 'macos/Podfile'],
  'frb_example/flutter_via_integrate_native_assets': [
    '.metadata',
    'ios',
    'macos/Podfile',
  ],
  'frb_example/flutter_package': [
    '.metadata',
    'pubspec.yaml',
    'example/ios',
    'example/macos/Podfile',
  ],
  'frb_example/flutter_package_native_assets': [
    '.metadata',
    'pubspec.yaml',
    'example/ios',
    'example/macos/Podfile',
  ],
};

const _kPreservedOhosScaffoldPaths = <String, List<String>>{
  'frb_example/flutter_via_create': [
    'ohos',
    'rust_builder/ohos',
    'rust_builder/pubspec.yaml',
  ],
  'frb_example/flutter_via_create_native_assets': ['ohos'],
  'frb_example/flutter_via_integrate': ['ohos'],
};

List<String> integrateAppleScaffoldSourceOfTruthPackages() =>
    List.unmodifiable(_kIntegrateAppleScaffoldSourceOfTruthPaths.keys);

Future<void> applyCheckedInAppleScaffoldSourceOfTruth({
  required String package,
  required String generatedPackageDir,
}) async {
  for (final relativePath in _integrateAppleScaffoldSourceOfTruthPaths(
    package,
  )) {
    _restorePathIfExists(
      source: _integrateAppleScaffoldSourceOfTruthAssetPath(
        package: package,
        relativePath: relativePath,
      ),
      destination: path.join(generatedPackageDir, relativePath),
    );
  }
}

Future<void> preserveCheckedInOhosScaffold({
  required String package,
  required String originalPackageDir,
  required String generatedPackageDir,
}) async {
  for (final relativePath in _preservedOhosScaffoldPaths(package)) {
    _restorePathIfExists(
      source: path.join(originalPackageDir, relativePath),
      destination: path.join(generatedPackageDir, relativePath),
    );
  }
}

Future<void> retainGeneratedOhosScaffold({
  required String package,
  required String originalPackageDir,
  required String generatedPackageDir,
  required String temporaryDirectory,
}) async {
  final relativePaths = _preservedOhosScaffoldPaths(package);
  if (relativePaths.isEmpty) {
    throw StateError('No OHOS scaffold paths configured for $package.');
  }

  final generatedOhosDirectory = path.join(
    temporaryDirectory,
    'generated_ohos',
  );
  for (final relativePath in relativePaths) {
    final source = path.join(generatedPackageDir, relativePath);
    if (path.basename(relativePath) == 'ohos') {
      _deletePathIfExists(path.join(source, 'node_modules'));
    }
    _restorePathIfExists(
      source: source,
      destination: path.join(generatedOhosDirectory, relativePath),
    );
  }

  Directory(generatedPackageDir).deleteSync(recursive: true);
  Directory(originalPackageDir).renameSync(generatedPackageDir);
  for (final relativePath in relativePaths) {
    final destination = path.join(generatedPackageDir, relativePath);
    _deletePathIfExists(destination);
    _restorePathIfExists(
      source: path.join(generatedOhosDirectory, relativePath),
      destination: destination,
    );
  }
}

List<String> _integrateAppleScaffoldSourceOfTruthPaths(String package) =>
    _kIntegrateAppleScaffoldSourceOfTruthPaths[package] ?? const [];

List<String> _preservedOhosScaffoldPaths(String package) =>
    _kPreservedOhosScaffoldPaths[package] ?? const [];

String _integrateAppleScaffoldSourceOfTruthAssetPath({
  required String package,
  required String relativePath,
}) {
  return _integrateAppleScaffoldSourceOfTruthAssetPathFromRepoRoot(
    repoRootPath: exec.pwd!,
    package: package,
    relativePath: relativePath,
  );
}

String _integrateAppleScaffoldSourceOfTruthAssetPathFromRepoRoot({
  required String repoRootPath,
  required String package,
  required String relativePath,
}) {
  return path.join(
    repoRootPath,
    'tools',
    'frb_internal',
    'assets',
    'apple_scaffold',
    package,
    relativePath,
  );
}

@visibleForTesting
List<String> integrateAppleScaffoldSourceOfTruthPathsForTesting(
  String package,
) => List.unmodifiable(_integrateAppleScaffoldSourceOfTruthPaths(package));

@visibleForTesting
List<String> preservedOhosScaffoldPathsForTesting(String package) =>
    List.unmodifiable(_preservedOhosScaffoldPaths(package));

@visibleForTesting
List<String> integrateAppleScaffoldSourceOfTruthAssetPathsForTesting({
  required String repoRootPath,
  required String package,
}) => List.unmodifiable(
  _integrateAppleScaffoldSourceOfTruthPaths(package).map(
    (relativePath) => _integrateAppleScaffoldSourceOfTruthAssetPathFromRepoRoot(
      repoRootPath: repoRootPath,
      package: package,
      relativePath: relativePath,
    ),
  ),
);

void _restorePathIfExists({
  required String source,
  required String destination,
}) {
  final sourceEntity = FileSystemEntity.typeSync(source);
  if (sourceEntity == FileSystemEntityType.notFound) return;

  _deletePathIfExists(destination);

  switch (sourceEntity) {
    case FileSystemEntityType.file:
      File(destination).parent.createSync(recursive: true);
      File(source).copySync(destination);
    case FileSystemEntityType.directory:
      _copyDirectoryRecursive(
        source: Directory(source),
        destination: Directory(destination),
      );
    case FileSystemEntityType.link:
      throw UnimplementedError('Do not expect symlink here: $source');
    case FileSystemEntityType.pipe:
    case FileSystemEntityType.unixDomainSock:
      throw UnimplementedError(
        'Do not expect special filesystem entity here: $source',
      );
    case FileSystemEntityType.notFound:
      break;
  }
}

void _deletePathIfExists(String target) {
  switch (FileSystemEntity.typeSync(target)) {
    case FileSystemEntityType.file:
      File(target).deleteSync();
    case FileSystemEntityType.directory:
      Directory(target).deleteSync(recursive: true);
    case FileSystemEntityType.link:
      Link(target).deleteSync();
    case FileSystemEntityType.pipe:
    case FileSystemEntityType.unixDomainSock:
      throw UnimplementedError(
        'Do not expect special filesystem entity here: $target',
      );
    case FileSystemEntityType.notFound:
      break;
  }
}

void _copyDirectoryRecursive({
  required Directory source,
  required Directory destination,
}) {
  destination.createSync(recursive: true);

  for (final entity in source.listSync(recursive: false, followLinks: false)) {
    final basename = path.basename(entity.path);
    final destinationPath = path.join(destination.path, basename);

    if (entity is File) {
      entity.copySync(destinationPath);
    } else if (entity is Directory) {
      _copyDirectoryRecursive(
        source: entity,
        destination: Directory(destinationPath),
      );
    } else if (entity is Link) {
      throw UnimplementedError('Do not expect symlink here: ${entity.path}');
    } else {
      throw UnimplementedError(
        'Do not expect special filesystem entity here: ${entity.path}',
      );
    }
  }
}

@visibleForTesting
void copyDirectoryRecursiveForTesting({
  required Directory source,
  required Directory destination,
}) {
  _copyDirectoryRecursive(source: source, destination: destination);
}
