import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_apple_scaffold.dart';
import 'package:path/path.dart' as path;

const _kPreservedOhosScaffoldPaths = <String, List<String>>{
  'frb_example/flutter_via_create': [
    'ohos',
    'rust_builder/ohos',
    'rust_builder/pubspec.yaml',
  ],
  'frb_example/flutter_via_create_native_assets': ['ohos'],
  'frb_example/flutter_via_integrate': ['ohos'],
};

typedef _OhosScaffoldPath = ({String relativePath, FileSystemEntityType type});

const _kGeneratedOhosScaffoldPaths = <String, List<_OhosScaffoldPath>>{
  'frb_example/flutter_via_create': [
    (relativePath: 'ohos', type: FileSystemEntityType.directory),
    (relativePath: 'rust_builder/ohos', type: FileSystemEntityType.directory),
    (
      relativePath: 'rust_builder/pubspec.yaml',
      type: FileSystemEntityType.file,
    ),
  ],
  'frb_example/flutter_via_create_native_assets': [
    (relativePath: 'ohos', type: FileSystemEntityType.directory),
  ],
  'frb_example/flutter_via_integrate': [
    (relativePath: 'ohos', type: FileSystemEntityType.directory),
    (relativePath: 'rust_builder/ohos', type: FileSystemEntityType.directory),
    (
      relativePath: 'rust_builder/pubspec.yaml',
      type: FileSystemEntityType.file,
    ),
  ],
};

Future<void> preserveCheckedInOhosScaffold({
  required String package,
  required String originalPackageDir,
  required String generatedPackageDir,
}) async {
  for (final relativePath in preservedOhosScaffoldPaths(package)) {
    restorePathIfExists(
      source: path.join(originalPackageDir, relativePath),
      destination: path.join(generatedPackageDir, relativePath),
    );
  }
}

Future<void> restoreOriginalPackageWithGeneratedOhosScaffold({
  required String package,
  required String originalPackageDir,
  required String generatedPackageDir,
  required String temporaryDirectory,
}) async {
  final scaffoldPaths = _generatedOhosScaffoldPaths(package);
  if (scaffoldPaths.isEmpty) {
    throw StateError('No OHOS scaffold paths configured for $package.');
  }

  final generatedPackageSnapshot = path.join(
    temporaryDirectory,
    'generated_package',
  );
  final stagedOhosDirectory = path.join(temporaryDirectory, 'staged_ohos');
  final originalOhosBackupDirectory = path.join(
    temporaryDirectory,
    'original_ohos_backup',
  );

  requireDirectory(generatedPackageDir, label: 'Generated package');
  requireDirectory(originalPackageDir, label: 'Original package');
  for (final target in [
    generatedPackageSnapshot,
    stagedOhosDirectory,
    originalOhosBackupDirectory,
  ]) {
    requirePathAbsent(target);
  }

  try {
    _stageGeneratedOhosScaffold(
      scaffoldPaths: scaffoldPaths,
      generatedPackageSnapshot: generatedPackageDir,
      stagedOhosDirectory: stagedOhosDirectory,
    );
    _validateOhosOverlayDestinations(
      scaffoldPaths: scaffoldPaths,
      destinationPackageDir: originalPackageDir,
    );
  } catch (_) {
    deletePathIfExists(stagedOhosDirectory);
    rethrow;
  }

  Directory(generatedPackageDir).renameSync(generatedPackageSnapshot);
  try {
    Directory(originalPackageDir).renameSync(generatedPackageDir);
  } catch (_) {
    Directory(generatedPackageSnapshot).renameSync(generatedPackageDir);
    rethrow;
  }

  _overlayGeneratedOhosScaffold(
    scaffoldPaths: scaffoldPaths,
    stagedOhosDirectory: stagedOhosDirectory,
    destinationPackageDir: generatedPackageDir,
    originalOhosBackupDirectory: originalOhosBackupDirectory,
  );

  for (final target in [
    originalOhosBackupDirectory,
    stagedOhosDirectory,
    generatedPackageSnapshot,
  ]) {
    deletePathIfExists(target);
  }
}

Future<void> restoreOriginalPackageAfterFailedOhosGeneration({
  required String originalPackageDir,
  required String generatedPackageDir,
  required String temporaryDirectory,
}) async {
  final failedGeneratedPackageSnapshot = path.join(
    temporaryDirectory,
    'failed_generated_package',
  );

  requireDirectory(originalPackageDir, label: 'Original package');
  requirePathAbsent(failedGeneratedPackageSnapshot);
  final generatedType = FileSystemEntity.typeSync(
    generatedPackageDir,
    followLinks: false,
  );
  if (generatedType != FileSystemEntityType.directory &&
      generatedType != FileSystemEntityType.notFound) {
    throw StateError(
      'Failed generated package must be a directory or absent, got $generatedType: $generatedPackageDir',
    );
  }

  final hadGeneratedPackage = renamePathIfExists(
    source: generatedPackageDir,
    destination: failedGeneratedPackageSnapshot,
  );
  try {
    Directory(originalPackageDir).renameSync(generatedPackageDir);
  } catch (_) {
    if (hadGeneratedPackage) {
      Directory(failedGeneratedPackageSnapshot).renameSync(generatedPackageDir);
    }
    rethrow;
  }
  deletePathIfExists(failedGeneratedPackageSnapshot);
}

List<String> preservedOhosScaffoldPaths(String package) =>
    List.unmodifiable(_kPreservedOhosScaffoldPaths[package] ?? const []);

List<_OhosScaffoldPath> _generatedOhosScaffoldPaths(String package) =>
    _kGeneratedOhosScaffoldPaths[package] ?? const [];

void _stageGeneratedOhosScaffold({
  required List<_OhosScaffoldPath> scaffoldPaths,
  required String generatedPackageSnapshot,
  required String stagedOhosDirectory,
}) {
  for (final scaffoldPath in scaffoldPaths) {
    final source = path.join(
      generatedPackageSnapshot,
      scaffoldPath.relativePath,
    );
    final actualType = FileSystemEntity.typeSync(source, followLinks: false);
    if (actualType != scaffoldPath.type) {
      throw StateError(
        'Required generated OHOS scaffold path has type $actualType instead of ${scaffoldPath.type}: $source',
      );
    }

    final destination = path.join(
      stagedOhosDirectory,
      scaffoldPath.relativePath,
    );
    switch (actualType) {
      case FileSystemEntityType.file:
        File(destination).parent.createSync(recursive: true);
        File(source).copySync(destination);
      case FileSystemEntityType.directory:
        copyDirectoryRecursive(
          source: Directory(source),
          destination: Directory(destination),
          excludedTopLevelNames:
              path.basename(scaffoldPath.relativePath) == 'ohos'
              ? const {'node_modules'}
              : const {},
        );
      case FileSystemEntityType.link:
      case FileSystemEntityType.pipe:
      case FileSystemEntityType.unixDomainSock:
      case FileSystemEntityType.notFound:
        throw StateError(
          'Unexpected generated OHOS scaffold path type $actualType: $source',
        );
    }
  }
}

void _validateOhosOverlayDestinations({
  required List<_OhosScaffoldPath> scaffoldPaths,
  required String destinationPackageDir,
}) {
  for (final scaffoldPath in scaffoldPaths) {
    final destination = path.join(
      destinationPackageDir,
      scaffoldPath.relativePath,
    );
    final parentType = FileSystemEntity.typeSync(
      path.dirname(destination),
      followLinks: false,
    );
    if (parentType != FileSystemEntityType.directory) {
      throw StateError(
        'OHOS scaffold destination parent is not a directory: ${path.dirname(destination)}',
      );
    }

    switch (FileSystemEntity.typeSync(destination, followLinks: false)) {
      case FileSystemEntityType.file:
      case FileSystemEntityType.directory:
      case FileSystemEntityType.notFound:
        break;
      case FileSystemEntityType.link:
        throw StateError(
          'OHOS scaffold destination must not be symlink: $destination',
        );
      case FileSystemEntityType.pipe:
      case FileSystemEntityType.unixDomainSock:
        throw StateError(
          'OHOS scaffold destination must not be special filesystem entity: $destination',
        );
    }
  }
}

void _overlayGeneratedOhosScaffold({
  required List<_OhosScaffoldPath> scaffoldPaths,
  required String stagedOhosDirectory,
  required String destinationPackageDir,
  required String originalOhosBackupDirectory,
}) {
  final installedRelativePaths = <String>[];
  final backedUpRelativePaths = <String>{};
  try {
    for (final scaffoldPath in scaffoldPaths) {
      final relativePath = scaffoldPath.relativePath;
      final staged = path.join(stagedOhosDirectory, relativePath);
      final destination = path.join(destinationPackageDir, relativePath);
      final backup = path.join(originalOhosBackupDirectory, relativePath);
      final hadDestination = renamePathIfExists(
        source: destination,
        destination: backup,
      );
      if (hadDestination) backedUpRelativePaths.add(relativePath);
      try {
        renameRequiredPath(source: staged, destination: destination);
      } catch (_) {
        if (hadDestination) {
          renameRequiredPath(source: backup, destination: destination);
          backedUpRelativePaths.remove(relativePath);
        }
        rethrow;
      }
      installedRelativePaths.add(relativePath);
    }
  } catch (_) {
    for (final relativePath in installedRelativePaths.reversed) {
      final destination = path.join(destinationPackageDir, relativePath);
      deletePathIfExists(destination);
      if (backedUpRelativePaths.contains(relativePath)) {
        renameRequiredPath(
          source: path.join(originalOhosBackupDirectory, relativePath),
          destination: destination,
        );
      }
    }
    rethrow;
  }
}
