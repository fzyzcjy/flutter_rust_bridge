import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
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

List<String> integrateAppleScaffoldSourceOfTruthPackages() =>
    List.unmodifiable(_kIntegrateAppleScaffoldSourceOfTruthPaths.keys);

Future<void> applyCheckedInAppleScaffoldSourceOfTruth({
  required String package,
  required String generatedPackageDir,
}) async {
  for (final relativePath in integrateAppleScaffoldSourceOfTruthPaths(
    package,
  )) {
    restorePathIfExists(
      source: _integrateAppleScaffoldSourceOfTruthAssetPath(
        package: package,
        relativePath: relativePath,
      ),
      destination: path.join(generatedPackageDir, relativePath),
    );
  }
}

List<String> integrateAppleScaffoldSourceOfTruthPaths(String package) =>
    List.unmodifiable(
      _kIntegrateAppleScaffoldSourceOfTruthPaths[package] ?? const [],
    );

List<String> integrateAppleScaffoldSourceOfTruthAssetPaths({
  required String repoRootPath,
  required String package,
}) => List.unmodifiable(
  integrateAppleScaffoldSourceOfTruthPaths(package).map(
    (relativePath) => _integrateAppleScaffoldSourceOfTruthAssetPathFromRepoRoot(
      repoRootPath: repoRootPath,
      package: package,
      relativePath: relativePath,
    ),
  ),
);

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

void restorePathIfExists({
  required String source,
  required String destination,
}) {
  final sourceEntity = FileSystemEntity.typeSync(source, followLinks: false);
  if (sourceEntity == FileSystemEntityType.notFound) return;

  switch (sourceEntity) {
    case FileSystemEntityType.file:
      deletePathIfExists(destination);
      File(destination).parent.createSync(recursive: true);
      File(source).copySync(destination);
    case FileSystemEntityType.directory:
      deletePathIfExists(destination);
      copyDirectoryRecursive(
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

void deletePathIfExists(String target) {
  switch (FileSystemEntity.typeSync(target, followLinks: false)) {
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

void copyDirectoryRecursive({
  required Directory source,
  required Directory destination,
  Set<String> excludedTopLevelNames = const {},
}) {
  destination.createSync(recursive: true);

  for (final entity in source.listSync(recursive: false, followLinks: false)) {
    final basename = path.basename(entity.path);
    if (excludedTopLevelNames.contains(basename)) continue;
    final destinationPath = path.join(destination.path, basename);

    if (entity is File) {
      entity.copySync(destinationPath);
    } else if (entity is Directory) {
      copyDirectoryRecursive(
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

void requireDirectory(String target, {required String label}) {
  final type = FileSystemEntity.typeSync(target, followLinks: false);
  if (type != FileSystemEntityType.directory) {
    throw StateError('$label must be a directory, got $type: $target');
  }
}

void requirePathAbsent(String target) {
  final type = FileSystemEntity.typeSync(target, followLinks: false);
  if (type != FileSystemEntityType.notFound) {
    throw StateError('Scaffold transaction path already exists: $target');
  }
}

bool renamePathIfExists({required String source, required String destination}) {
  final type = FileSystemEntity.typeSync(source, followLinks: false);
  if (type == FileSystemEntityType.notFound) return false;

  _renamePath(source: source, destination: destination, type: type);
  return true;
}

void renameRequiredPath({required String source, required String destination}) {
  final type = FileSystemEntity.typeSync(source, followLinks: false);
  if (type == FileSystemEntityType.notFound) {
    throw StateError('Required scaffold transaction path is missing: $source');
  }

  _renamePath(source: source, destination: destination, type: type);
}

void _renamePath({
  required String source,
  required String destination,
  required FileSystemEntityType type,
}) {
  Directory(path.dirname(destination)).createSync(recursive: true);
  switch (type) {
    case FileSystemEntityType.file:
      File(source).renameSync(destination);
    case FileSystemEntityType.directory:
      Directory(source).renameSync(destination);
    case FileSystemEntityType.link:
      Link(source).renameSync(destination);
    case FileSystemEntityType.pipe:
    case FileSystemEntityType.unixDomainSock:
      throw StateError(
        'Cannot rename special filesystem entity in scaffold transaction: $source',
      );
    case FileSystemEntityType.notFound:
      throw StateError(
        'Cannot rename missing scaffold transaction path: $source',
      );
  }
}
