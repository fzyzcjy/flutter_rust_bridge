import 'dart:io';

import 'package:ed25519_edwards/ed25519_edwards.dart';
import 'package:http/http.dart';
import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'builder.dart';
import 'crate_hash.dart';
import 'exceptions.dart';
import 'options.dart';
import 'precompile_binaries.dart';
import 'rustup.dart';
import 'target.dart';

class Artifact {
  /// File system location of the artifact.
  final String path;

  /// Actual file name that the artifact should have in destination folder.
  final String finalFileName;

  ArtifactType get type {
    if (finalFileName.endsWith('.dll') ||
        finalFileName.endsWith('.dll.lib') ||
        finalFileName.endsWith('.pdb') ||
        finalFileName.endsWith('.so') ||
        finalFileName.endsWith('.dylib')) {
      return ArtifactType.dylib;
    } else if (finalFileName.endsWith('.lib') || finalFileName.endsWith('.a')) {
      return ArtifactType.staticlib;
    } else {
      throw ArtifactException(
        'Unknown artifact type for "$finalFileName".',
      );
    }
  }

  Artifact({
    required this.path,
    required this.finalFileName,
  });
}

final _log = Logger('artifacts_provider');

class ArtifactMaterializer {
  static Iterable<Artifact> artifactsForType(
    Iterable<Artifact> artifacts, {
    required ArtifactType type,
  }) {
    return artifacts.where((artifact) => artifact.type == type);
  }

  static List<Artifact> flattenForType(
    Map<Target, List<Artifact>> artifacts, {
    required ArtifactType type,
  }) {
    return artifacts.values
        .expand((targetArtifacts) => targetArtifacts)
        .where((artifact) => artifact.type == type)
        .toList(growable: false);
  }

  static List<String> copyDynamicLibraries(
    Iterable<Artifact> artifacts, {
    required String outputDir,
  }) {
    Directory(outputDir).createSync(recursive: true);

    final copied = <String>[];
    for (final lib in artifactsForType(
      artifacts,
      type: ArtifactType.dylib,
    )) {
      final destination = path.join(outputDir, lib.finalFileName);
      File(lib.path).copySync(destination);
      copied.add(destination);
    }
    return copied;
  }
}

class ArtifactProvider {
  ArtifactProvider({
    required this.environment,
    required this.userOptions,
  });

  final BuildEnvironment environment;
  final CargokitUserOptions userOptions;

  Future<Map<Target, List<Artifact>>> getArtifacts(List<Target> targets) async {
    final result = await _getPrecompiledArtifacts(targets);

    final pendingTargets = List.of(targets);
    pendingTargets.removeWhere((element) => result.containsKey(element));

    if (pendingTargets.isEmpty) {
      return result;
    }

    final rustup = Rustup();
    for (final target in pendingTargets) {
      final builder = RustBuilder(target: target, environment: environment);
      builder.prepare(rustup);
      _log.info('Building ${environment.crateInfo.packageName} for $target');
      final targetDir = await builder.build();
      // For local build accept both static and dynamic libraries.
      final artifactNames = <String>{
        ...getArtifactNames(
          target: target,
          libraryName: environment.crateInfo.packageName,
          artifactType: ArtifactType.dylib,
          remote: false,
        ),
        ...getArtifactNames(
          target: target,
          libraryName: environment.crateInfo.packageName,
          artifactType: ArtifactType.staticlib,
          remote: false,
        )
      };
      final artifacts = artifactNames
          .map((artifactName) => Artifact(
                path: path.join(targetDir, artifactName),
                finalFileName: artifactName,
              ))
          .where((element) => File(element.path).existsSync())
          .toList();
      result[target] = artifacts;
    }
    return result;
  }

  Future<Map<Target, List<Artifact>>> _getPrecompiledArtifacts(
      List<Target> targets) async {
    if (userOptions.usePrecompiledBinaries == false) {
      _log.info('Precompiled binaries are disabled');
      return {};
    }
    if (environment.crateOptions.precompiledBinaries == null) {
      _log.fine('Precompiled binaries not enabled for this crate');
      return {};
    }

    final start = Stopwatch()..start();
    final crateHash = CrateHash.compute(environment.manifestDir,
        tempStorage: environment.targetTempDir);
    _log.fine(
        'Computed crate hash $crateHash in ${start.elapsedMilliseconds}ms');

    final downloadedArtifactsDir =
        path.join(environment.targetTempDir, 'precompiled', crateHash);
    Directory(downloadedArtifactsDir).createSync(recursive: true);

    final res = <Target, List<Artifact>>{};

    for (final target in targets) {
      final requiredArtifacts = getArtifactNames(
        target: target,
        libraryName: environment.crateInfo.packageName,
        remote: true,
      );
      final artifactsForTarget = <Artifact>[];

      for (final artifact in requiredArtifacts) {
        final fileName = PrecompileBinaries.fileName(target, artifact);
        final downloadedPath = path.join(downloadedArtifactsDir, fileName);
        if (!File(downloadedPath).existsSync()) {
          final signatureFileName =
              PrecompileBinaries.signatureFileName(target, artifact);
          await _tryDownloadArtifacts(
            crateHash: crateHash,
            fileName: fileName,
            signatureFileName: signatureFileName,
            finalPath: downloadedPath,
          );
        }
        if (File(downloadedPath).existsSync()) {
          artifactsForTarget.add(Artifact(
            path: downloadedPath,
            finalFileName: artifact,
          ));
        } else {
          break;
        }
      }

      // Only provide complete set of artifacts.
      if (artifactsForTarget.length == requiredArtifacts.length) {
        _log.fine('Found precompiled artifacts for $target');
        res[target] = artifactsForTarget;
      }
    }

    return res;
  }

  static Future<Response> _get(Uri url, {Map<String, String>? headers}) async {
    int attempt = 0;
    const maxAttempts = 10;
    while (true) {
      try {
        return await get(url, headers: headers);
      } on SocketException catch (e) {
        // Try to detect reset by peer error and retry.
        if (attempt++ < maxAttempts &&
            (e.osError?.errorCode == 54 || e.osError?.errorCode == 10054)) {
          _log.severe(
              'Failed to download $url: $e, attempt $attempt of $maxAttempts, will retry...');
          await Future.delayed(Duration(seconds: 1));
          continue;
        } else {
          rethrow;
        }
      }
    }
  }

  Future<void> _tryDownloadArtifacts({
    required String crateHash,
    required String fileName,
    required String signatureFileName,
    required String finalPath,
  }) async {
    final precompiledBinaries = environment.crateOptions.precompiledBinaries!;
    final prefix = precompiledBinaries.uriPrefix;
    final url = Uri.parse('$prefix$crateHash/$fileName');
    final signatureUrl = Uri.parse('$prefix$crateHash/$signatureFileName');
    _log.fine('Downloading signature from $signatureUrl');
    final signature = await _get(signatureUrl);
    if (signature.statusCode == 404) {
      _log.warning(
          'Precompiled binaries not available for crate hash $crateHash ($fileName)');
      return;
    }
    if (signature.statusCode != 200) {
      _log.severe(
          'Failed to download signature $signatureUrl: status ${signature.statusCode}');
      return;
    }
    _log.fine('Downloading binary from $url');
    final res = await _get(url);
    if (res.statusCode != 200) {
      _log.severe('Failed to download binary $url: status ${res.statusCode}');
      return;
    }
    if (verify(
        precompiledBinaries.publicKey, res.bodyBytes, signature.bodyBytes)) {
      File(finalPath).writeAsBytesSync(res.bodyBytes);
    } else {
      _log.shout('Signature verification failed! Ignoring binary.');
    }
  }
}

enum ArtifactType {
  staticlib,
  dylib,
}

ArtifactType artifactTypeForTarget(Target target) {
  if (target.darwinPlatform != null) {
    return ArtifactType.staticlib;
  } else {
    return ArtifactType.dylib;
  }
}

List<String> getArtifactNames({
  required Target target,
  required String libraryName,
  required bool remote,
  ArtifactType? artifactType,
}) {
  artifactType ??= artifactTypeForTarget(target);
  if (target.darwinArch != null) {
    if (artifactType == ArtifactType.staticlib) {
      return ['lib$libraryName.a'];
    } else {
      return ['lib$libraryName.dylib'];
    }
  } else if (target.rust.contains('-windows-')) {
    if (artifactType == ArtifactType.staticlib) {
      return ['$libraryName.lib'];
    } else {
      return [
        '$libraryName.dll',
        '$libraryName.dll.lib',
        if (!remote) '$libraryName.pdb'
      ];
    }
  } else if (target.rust.contains('-linux-')) {
    if (artifactType == ArtifactType.staticlib) {
      return ['lib$libraryName.a'];
    } else {
      return ['lib$libraryName.so'];
    }
  } else {
    throw UnsupportedPlatformException(
      'Artifact naming is not implemented for Rust target "${target.rust}".',
    );
  }
}
