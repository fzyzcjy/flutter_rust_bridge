// ignore_for_file: avoid_print

import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/sanitizer.dart';
import 'package:path/path.dart' as path;

Future<String> getSanitizedDartBinary(TestDartSanitizerConfig config) async {
  if (config.useLocalSanitizedDartBinary) {
    return '~/dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart';
  }

  // const releaseName = 'Build_2023.12.01_09-42-01';
  const releaseName = 'Build_2025.02.09_04-28-46';
  final baseName = '${config.sanitizer.dartSdkBuildOutDir}_dart-sdk';
  final fileNameTarGz = '$baseName.tar.gz';

  final pathTarGz = path.join(Directory.systemTemp.path, fileNameTarGz);
  final pathUnzippedDir = path.join(Directory.systemTemp.path, baseName);
  final pathBin = path.join(
    pathUnzippedDir,
    'dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart',
  );

  if (!await File(pathTarGz).exists()) {
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileNameTarGz: fileNameTarGz,
      pathTarGz: pathTarGz,
    );
  }

  if (!await File(pathBin).exists()) {
    await exec('mkdir $pathUnzippedDir');
    await exec('tar -xvzf $pathTarGz -C $pathUnzippedDir');
  }

  if (!await File(pathBin).exists()) {
    throw Exception('$pathBin still not exist');
  }

  return pathBin;
}

Future<void> _downloadSanitizedDartBinaryArtifact({
  required String releaseName,
  required String fileNameTarGz,
  required String pathTarGz,
}) async {
  final publicUrl =
      'https://github.com/fzyzcjy/dart_lang_ci/releases/download/$releaseName/$fileNameTarGz';
  print('Download artifact from $publicUrl to $pathTarGz...');

  try {
    await Dio().download(publicUrl, pathTarGz);
    return;
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
      fileNameTarGz: fileNameTarGz,
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
    await File(pathTarGz).writeAsBytes(response.data!);
  }
}

Future<int> _findGitHubReleaseAssetId({
  required String releaseName,
  required String fileNameTarGz,
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
    (element) => element['name'] == fileNameTarGz,
    orElse: () => throw Exception(
      'Cannot find GitHub release asset `$fileNameTarGz` in `$releaseName`',
    ),
  );
  return asset['id'] as int;
}
