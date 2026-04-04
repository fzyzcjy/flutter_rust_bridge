import 'dart:convert';
import 'dart:io';

import 'package:meta/meta.dart';

class DevDockerMetadata {
  final String flutterVersion;
  final String rustVersion;
  final String rustNightlyVersion;

  const DevDockerMetadata({
    required this.flutterVersion,
    required this.rustVersion,
    required this.rustNightlyVersion,
  });

  String get versionTag =>
      'flutter-$flutterVersion-rust-$rustVersion-nightly-$rustNightlyVersion';

  String imageRef({required String imageName}) => '$imageName:$versionTag';

  Map<String, Object> toJson() {
    return {
      'flutter_version': flutterVersion,
      'rust_version': rustVersion,
      'rust_nightly_version': rustNightlyVersion,
      'version_tag': versionTag,
      'image_ref': imageRef(imageName: 'fzyzcjy/flutter_rust_bridge_dev'),
    };
  }

  String toJsonString() => jsonEncode(toJson());
}

class DevDockerWorkflowMetadata {
  final DevDockerMetadata metadata;
  final String imageName;
  final String shortSha;

  const DevDockerWorkflowMetadata({
    required this.metadata,
    required this.imageName,
    required this.shortSha,
  });

  String get imageRef => metadata.imageRef(imageName: imageName);

  String get shaTag => 'sha-$shortSha';

  String get localTag => 'frb-dev-image-smoke:$shortSha';

  List<String> get tags => [
    '$imageName:latest',
    imageRef,
    '$imageName:$shaTag',
  ];
}

@visibleForTesting
DevDockerMetadata parseDevDockerMetadataFromText(String dockerfileText) {
  return DevDockerMetadata(
    flutterVersion: _parseDockerfileArgument(dockerfileText, 'FLUTTER_VERSION'),
    rustVersion: _parseDockerfileArgument(dockerfileText, 'RUST_VERSION'),
    rustNightlyVersion: _parseDockerfileArgument(
      dockerfileText,
      'RUST_NIGHTLY_VERSION',
    ),
  );
}

DevDockerMetadata readDevDockerMetadataFile({required String dockerfilePath}) {
  return parseDevDockerMetadataFromText(
    File(dockerfilePath).readAsStringSync(),
  );
}

void writeCommandOutput({
  required String outputText,
  required String? outputPath,
}) {
  if (outputPath == null) {
    stdout.write(outputText);
    return;
  }

  final outputFile = File(outputPath);
  outputFile.parent.createSync(recursive: true);
  outputFile.writeAsStringSync(outputText);
}

String _parseDockerfileArgument(String dockerfileText, String argumentName) {
  final match = RegExp(
    '^ARG\\s+$argumentName\\s*=\\s*(.+)\$',
    multiLine: true,
  ).firstMatch(dockerfileText);
  if (match == null) {
    throw StateError('Missing ARG $argumentName in .devcontainer/Dockerfile.');
  }

  return match.group(1)!.trim();
}
