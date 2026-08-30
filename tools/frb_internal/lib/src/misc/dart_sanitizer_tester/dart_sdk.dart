// ignore_for_file: avoid_print

import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/sanitizer.dart';
import 'package:path/path.dart' as path;

const kDefaultSanitizedDartReleaseName = 'Build_2026.06.19_13-45-18';
const _kSanitizedDartReleaseNameEnv = 'FRB_SANITIZED_DART_RELEASE_NAME';
const _kMainDartVersionEnv = 'FRB_MAIN_DART_VERSION';
Future<String> getSanitizedDartBinary(TestDartSanitizerConfig config) async {
  if (config.useLocalSanitizedDartBinary) {
    final path =
        '~/dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart';
    await _printSanitizedDartVersion(path);
    return path;
  }

  final releaseName = sanitizedDartReleaseName();
  final pathCacheRoot = path.join(
    Directory.systemTemp.path,
    'frb_sanitized_dart',
    releaseName,
  );
  await Directory(pathCacheRoot).create(recursive: true);
  final lock = await File(
    path.join(pathCacheRoot, '.lock'),
  ).open(mode: FileMode.append);
  await lock.lock(FileLock.exclusive);
  try {
    return await _getCachedSanitizedDartBinary(
      config: config,
      releaseName: releaseName,
      pathCacheRoot: pathCacheRoot,
    );
  } finally {
    await lock.unlock();
    await lock.close();
  }
}

Future<String> _getCachedSanitizedDartBinary({
  required TestDartSanitizerConfig config,
  required String releaseName,
  required String pathCacheRoot,
}) async {
  final baseName = '${config.sanitizer.dartSdkBuildOutDir}_dart-sdk';
  final fileNameTarGz = '$baseName.tar.gz';
  final fileNameChecksum = '$fileNameTarGz.sha256';
  final pathTarGz = path.join(pathCacheRoot, fileNameTarGz);
  final pathChecksum = path.join(pathCacheRoot, fileNameChecksum);
  final pathUnzippedDir = path.join(pathCacheRoot, baseName);
  final pathExtractionComplete = path.join(pathUnzippedDir, '.complete');
  final relativePathCacheRoot = sanitizedDartCacheRelativePathForTesting(
    repoRootPath: Directory.current.parent.parent.path,
    cacheRootPath: pathCacheRoot,
  );
  final pathBin = path.join(
    pathUnzippedDir,
    'dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart',
  );

  if (!await File(pathTarGz).exists()) {
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileName: fileNameTarGz,
      pathDestination: pathTarGz,
    );
  }

  if (!await File(pathChecksum).exists()) {
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileName: fileNameChecksum,
      pathDestination: pathChecksum,
    );
  }

  final checksumResult = await exec(
    'sha256sum --check $fileNameChecksum',
    relativePwd: relativePathCacheRoot,
    checkExitCode: false,
  );
  if (checksumResult.exitCode != 0) {
    await File(pathTarGz).delete();
    await File(pathChecksum).delete();
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileName: fileNameTarGz,
      pathDestination: pathTarGz,
    );
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileName: fileNameChecksum,
      pathDestination: pathChecksum,
    );
    await exec(
      'sha256sum --check $fileNameChecksum',
      relativePwd: relativePathCacheRoot,
    );
  }

  if (!await File(pathExtractionComplete).exists() ||
      !await File(pathBin).exists()) {
    final pathTemporaryDirectory =
        '$pathUnzippedDir.part-${pid}_'
        '${DateTime.now().microsecondsSinceEpoch}';
    await Directory(pathTemporaryDirectory).create(recursive: true);
    try {
      await exec('tar -xvzf $pathTarGz -C $pathTemporaryDirectory');
      final pathTemporaryBin = path.join(
        pathTemporaryDirectory,
        'dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart',
      );
      if (!await File(pathTemporaryBin).exists()) {
        throw Exception('$pathTemporaryBin still not exist');
      }
      await File(path.join(pathTemporaryDirectory, '.complete')).create();

      if (await Directory(pathUnzippedDir).exists()) {
        await Directory(pathUnzippedDir).delete(recursive: true);
      }
      await Directory(pathTemporaryDirectory).rename(pathUnzippedDir);
    } finally {
      if (await Directory(pathTemporaryDirectory).exists()) {
        await Directory(pathTemporaryDirectory).delete(recursive: true);
      }
    }
  }

  if (!await File(pathBin).exists()) {
    throw Exception('$pathBin still not exist');
  }

  await _printSanitizedDartVersion(pathBin);
  return pathBin;
}

String sanitizedDartReleaseName({Map<String, String>? environment}) {
  final value =
      (environment ?? Platform.environment)[_kSanitizedDartReleaseNameEnv];
  if (value == null || value.trim().isEmpty) {
    return kDefaultSanitizedDartReleaseName;
  }
  return value.trim();
}

String sanitizedDartCacheRelativePathForTesting({
  required String repoRootPath,
  required String cacheRootPath,
}) => path.relative(cacheRootPath, from: repoRootPath);

Future<void> _printSanitizedDartVersion(String sanitizedDart) async {
  final output = await exec('$sanitizedDart --version', checkExitCode: false);
  final versionOutput = '${output.stdout}\n${output.stderr}';
  print(
    'sanitized Dart version: '
    'stdout=${output.stdout.trim()} stderr=${output.stderr.trim()}',
  );
  checkSanitizedDartVersionForTesting(
    versionOutput: versionOutput,
    environment: Platform.environment,
  );
}

void checkSanitizedDartVersionForTesting({
  required String versionOutput,
  required Map<String, String> environment,
}) {
  final expectedVersion = environment[_kMainDartVersionEnv]?.trim();
  if (expectedVersion == null || expectedVersion.isEmpty) return;

  final match = RegExp(
    r'Dart SDK version:\s*([0-9]+\.[0-9]+\.[0-9]+)',
  ).firstMatch(versionOutput);
  if (match == null) {
    throw Exception(
      'Cannot parse sanitized Dart version from output: $versionOutput',
    );
  }

  final actualVersion = match.group(1);
  if (actualVersion != expectedVersion) {
    throw Exception(
      'Sanitized Dart version $actualVersion does not match '
      '$_kMainDartVersionEnv=$expectedVersion. Build a new sanitized Dart '
      'artifact instead of lowering pubspec SDK constraints.',
    );
  }
}

Future<void> _downloadSanitizedDartBinaryArtifact({
  required String releaseName,
  required String fileName,
  required String pathDestination,
}) async {
  final publicUrl =
      'https://github.com/fzyzcjy/dart_lang_ci/releases/download/$releaseName/$fileName';
  final pathTemporary =
      '$pathDestination.part-${pid}_'
      '${DateTime.now().microsecondsSinceEpoch}';
  print('Download artifact from $publicUrl to $pathDestination...');

  try {
    try {
      await Dio().download(publicUrl, pathTemporary);
    } on DioException {
      final token =
          Platform.environment['GITHUB_TOKEN'] ??
          Platform.environment['GH_TOKEN'];
      if (token == null || token.isEmpty) rethrow;

      print(
        'Public artifact download failed; retry via GitHub API asset download',
      );

      final assetId = await _findGitHubReleaseAssetId(
        releaseName: releaseName,
        fileName: fileName,
        token: token,
      );
      final response = await Dio().get<List<int>>(
        'https://api.github.com/repos/fzyzcjy/dart_lang_ci/releases/assets/$assetId',
        options: Options(
          responseType: ResponseType.bytes,
          headers: {
            HttpHeaders.authorizationHeader: 'Bearer $token',
            HttpHeaders.acceptHeader: 'application/octet-stream',
            HttpHeaders.userAgentHeader: 'flutter-rust-bridge-ci',
          },
        ),
      );
      await File(pathTemporary).writeAsBytes(response.data!);
    }

    if (await File(pathDestination).exists()) {
      await File(pathTemporary).delete();
    } else {
      await File(pathTemporary).rename(pathDestination);
    }
  } finally {
    if (await File(pathTemporary).exists()) {
      await File(pathTemporary).delete();
    }
  }
}

Future<int> _findGitHubReleaseAssetId({
  required String releaseName,
  required String fileName,
  required String token,
}) async {
  final response = await Dio().get<Map<String, dynamic>>(
    'https://api.github.com/repos/fzyzcjy/dart_lang_ci/releases/tags/$releaseName',
    options: Options(
      headers: {
        HttpHeaders.authorizationHeader: 'Bearer $token',
        HttpHeaders.acceptHeader: 'application/vnd.github+json',
        HttpHeaders.userAgentHeader: 'flutter-rust-bridge-ci',
      },
    ),
  );

  final assets = response.data!['assets'] as List<dynamic>;
  final asset = assets.cast<Map<String, dynamic>>().firstWhere(
    (element) => element['name'] == fileName,
    orElse: () => throw Exception(
      'Cannot find GitHub release asset `$fileName` in `$releaseName`',
    ),
  );
  return asset['id'] as int;
}
