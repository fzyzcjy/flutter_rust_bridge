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

  String get imageRef => 'fzyzcjy/flutter_rust_bridge_dev:$versionTag';

  Map<String, Object> toJson() {
    return {
      'flutter_version': flutterVersion,
      'rust_version': rustVersion,
      'rust_nightly_version': rustNightlyVersion,
      'version_tag': versionTag,
      'image_ref': imageRef,
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

  String get imageRef => metadata.imageRef;

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
    flutterVersion: _parseDockerfileArgument(
      dockerfileText,
      'FLUTTER_VERSION',
    ),
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

@visibleForTesting
String workflowMetadataToGithubOutput(
  DevDockerWorkflowMetadata workflowMetadata,
) {
  final buffer = StringBuffer()
    ..writeln('flutter_version=${workflowMetadata.metadata.flutterVersion}')
    ..writeln('rust_version=${workflowMetadata.metadata.rustVersion}')
    ..writeln(
      'rust_nightly_version=${workflowMetadata.metadata.rustNightlyVersion}',
    )
    ..writeln('version_tag=${workflowMetadata.metadata.versionTag}')
    ..writeln('image_ref=${workflowMetadata.imageRef}')
    ..writeln('sha_tag=${workflowMetadata.shaTag}')
    ..writeln('local_tag=${workflowMetadata.localTag}')
    ..writeln('tags<<EOF');

  for (final tag in workflowMetadata.tags) {
    buffer.writeln(tag);
  }

  buffer.writeln('EOF');
  return buffer.toString();
}

@visibleForTesting
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
    throw StateError(
      'Missing ARG $argumentName in .devcontainer/Dockerfile.',
    );
  }

  return match.group(1)!.trim();
}
