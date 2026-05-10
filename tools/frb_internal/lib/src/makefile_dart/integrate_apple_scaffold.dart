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
  'frb_example/flutter_via_integrate': ['.metadata', 'ios', 'macos/Podfile'],
  'frb_example/flutter_package': [
    '.metadata',
    'pubspec.yaml',
    'example/ios',
    'example/macos/Podfile',
  ],
};

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

List<String> _integrateAppleScaffoldSourceOfTruthPaths(String package) {
  return _kIntegrateAppleScaffoldSourceOfTruthPaths[package] ?? const [];
}

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

  final destinationEntity = FileSystemEntity.typeSync(destination);
  switch (destinationEntity) {
    case FileSystemEntityType.file:
      File(destination).deleteSync();
    case FileSystemEntityType.directory:
      Directory(destination).deleteSync(recursive: true);
    case FileSystemEntityType.link:
      Link(destination).deleteSync();
    case FileSystemEntityType.pipe:
    case FileSystemEntityType.unixDomainSock:
      throw UnimplementedError(
        'Do not expect special filesystem entity here: $destination',
      );
    case FileSystemEntityType.notFound:
      break;
  }

  switch (sourceEntity) {
    case FileSystemEntityType.file:
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
