import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_apple_scaffold.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:path/path.dart' as path;

const _kSkipAppleScaffoldSourceOfTruthEnv =
    'FRB_SKIP_APPLE_SCAFFOLD_SOURCE_OF_TRUTH';

Future<void> verifyAppleScaffoldSourceOfTruth() async {
  if (!Platform.isMacOS) {
    throw StateError(
      'verify-apple-scaffold-source-of-truth requires macOS because it compares against Flutter Apple scaffolds generated on macOS.',
    );
  }

  final tempRepoRoot = path.join(
    exec.pwd!,
    'target',
    'VerifyAppleScaffoldSourceOfTruth',
    randomTempDirName(),
  );
  print('Pick temporary repository copy: $tempRepoRoot');
  Directory(tempRepoRoot).createSync(recursive: true);

  var success = false;
  try {
    _copyRepositoryForAppleScaffoldVerification(
      source: Directory(exec.pwd!),
      destination: Directory(tempRepoRoot),
    );

    final tempExec = SimpleExecutor(env: exec.env, pwd: '$tempRepoRoot/');
    for (final package in integrateAppleScaffoldSourceOfTruthPackages()) {
      print('Verifying Apple scaffold source-of-truth for $package');
      await tempExec(
        'bash ./frb_internal generate-run-frb-codegen-command-integrate --package ${_shellQuote(package)}',
        extraEnv: {_kSkipAppleScaffoldSourceOfTruthEnv: '1'},
      );
      _verifyAppleScaffoldPackage(
        repoRootPath: exec.pwd!,
        generatedRepoRootPath: tempRepoRoot,
        package: package,
      );
    }
    success = true;
  } finally {
    if (success) {
      Directory(tempRepoRoot).deleteSync(recursive: true);
    } else {
      print(
        'Apple scaffold verification failed; temporary repository copy preserved at $tempRepoRoot',
      );
    }
  }
}

bool shouldSkipAppleScaffoldSourceOfTruth({
  Map<String, String>? environment,
}) =>
    (environment ?? Platform.environment)[_kSkipAppleScaffoldSourceOfTruthEnv] ==
    '1';

void _copyRepositoryForAppleScaffoldVerification({
  required Directory source,
  required Directory destination,
}) {
  const excludedBasenames = {
    '.git',
    '.dart_tool',
    'build',
    'target',
    '.idea',
    '.vscode',
  };

  destination.createSync(recursive: true);
  for (final entity in source.listSync(recursive: false, followLinks: false)) {
    final basename = path.basename(entity.path);
    if (excludedBasenames.contains(basename)) continue;

    final destinationPath = path.join(destination.path, basename);
    if (entity is File) {
      entity.copySync(destinationPath);
    } else if (entity is Directory) {
      _copyRepositoryForAppleScaffoldVerification(
        source: entity,
        destination: Directory(destinationPath),
      );
    } else if (entity is Link) {
      Link(destinationPath).createSync(entity.targetSync());
    } else {
      throw UnimplementedError(
        'Do not expect special filesystem entity here: ${entity.path}',
      );
    }
  }
}

void _verifyAppleScaffoldPackage({
  required String repoRootPath,
  required String generatedRepoRootPath,
  required String package,
}) {
  for (final relativePath in integrateAppleScaffoldSourceOfTruthPaths(package)) {
    final expectedPath = integrateAppleScaffoldSourceOfTruthAssetPathFromRepoRoot(
      repoRootPath: repoRootPath,
      package: package,
      relativePath: relativePath,
    );
    final actualPath = path.join(generatedRepoRootPath, package, relativePath);
    _verifyAppleScaffoldPath(
      expectedPath: expectedPath,
      actualPath: actualPath,
      package: package,
      relativePath: relativePath,
    );
  }
}

void _verifyAppleScaffoldPath({
  required String expectedPath,
  required String actualPath,
  required String package,
  required String relativePath,
}) {
  final expectedType = FileSystemEntity.typeSync(expectedPath);
  final actualType = FileSystemEntity.typeSync(actualPath);
  if (expectedType != actualType) {
    throw StateError(
      'Apple scaffold source-of-truth mismatch for $package/$relativePath: expected $expectedType at $expectedPath, actual $actualType at $actualPath.',
    );
  }

  switch (expectedType) {
    case FileSystemEntityType.file:
      _verifyAppleScaffoldFile(
        expectedPath: expectedPath,
        actualPath: actualPath,
        package: package,
        relativePath: relativePath,
      );
    case FileSystemEntityType.directory:
      _verifyAppleScaffoldDirectory(
        expectedPath: expectedPath,
        actualPath: actualPath,
        package: package,
        relativePath: relativePath,
      );
    case FileSystemEntityType.link:
      if (Link(expectedPath).targetSync() != Link(actualPath).targetSync()) {
        throw StateError(
          'Apple scaffold source-of-truth symlink mismatch for $package/$relativePath.',
        );
      }
    case FileSystemEntityType.pipe:
    case FileSystemEntityType.unixDomainSock:
      throw UnimplementedError(
        'Do not expect special filesystem entity here: $expectedPath',
      );
    case FileSystemEntityType.notFound:
      throw StateError(
        'Apple scaffold source-of-truth missing expected path for $package/$relativePath: $expectedPath.',
      );
  }
}

void _verifyAppleScaffoldDirectory({
  required String expectedPath,
  required String actualPath,
  required String package,
  required String relativePath,
}) {
  final expectedChildren = _listDirectoryBasenames(expectedPath);
  final actualChildren = _listDirectoryBasenames(actualPath);
  if (!_listEquals(expectedChildren, actualChildren)) {
    throw StateError(
      'Apple scaffold source-of-truth directory mismatch for $package/$relativePath: expected entries $expectedChildren, actual entries $actualChildren.',
    );
  }

  for (final child in expectedChildren) {
    _verifyAppleScaffoldPath(
      expectedPath: path.join(expectedPath, child),
      actualPath: path.join(actualPath, child),
      package: package,
      relativePath: path.join(relativePath, child),
    );
  }
}

void _verifyAppleScaffoldFile({
  required String expectedPath,
  required String actualPath,
  required String package,
  required String relativePath,
}) {
  final expectedBytes = File(expectedPath).readAsBytesSync();
  final actualBytes = File(actualPath).readAsBytesSync();
  if (!_listEquals(expectedBytes, actualBytes)) {
    throw StateError(
      'Apple scaffold source-of-truth file mismatch for $package/$relativePath. Compare $expectedPath and $actualPath.',
    );
  }
}

List<String> _listDirectoryBasenames(String directoryPath) {
  final result = [
    for (final entity in Directory(
      directoryPath,
    ).listSync(recursive: false, followLinks: false))
      path.basename(entity.path),
  ];
  result.sort();
  return result;
}

bool _listEquals<T>(List<T> a, List<T> b) {
  if (a.length != b.length) return false;
  for (var i = 0; i < a.length; i++) {
    if (a[i] != b[i]) return false;
  }
  return true;
}

String _shellQuote(String value) => "'${value.replaceAll("'", r"'\''")}'";
