import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
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

class DevDockerMetadataCommand extends Command<void> {
  @override
  final String name = 'dev-docker-metadata';

  @override
  final String description =
      'Print dev Docker image metadata derived from .devcontainer/Dockerfile';

  final String defaultDockerfilePath;

  DevDockerMetadataCommand({
    this.defaultDockerfilePath = '.devcontainer/Dockerfile',
  }) {
    argParser
      ..addOption(
        'dockerfile',
        defaultsTo: defaultDockerfilePath,
        help: 'Path to the Dockerfile to parse.',
      )
      ..addFlag(
        'github-output',
        defaultsTo: false,
        help: 'Print GitHub Actions output format instead of JSON.',
      )
      ..addOption(
        'image-name',
        defaultsTo: 'fzyzcjy/flutter_rust_bridge_dev',
        help: 'Docker image repository name for GitHub Actions output.',
      )
      ..addOption(
        'short-sha',
        help: 'Short git SHA used to derive GitHub Actions tags.',
      );
  }

  @override
  Future<void> run() async {
    final dockerfilePath = argResults!['dockerfile'] as String;
    final metadata = readDevDockerMetadataFile(dockerfilePath: dockerfilePath);
    final githubOutput = argResults!['github-output'] as bool;

    if (!githubOutput) {
      print(metadata.toJsonString());
      return;
    }

    final imageName = argResults!['image-name'] as String;
    final shortSha = argResults!['short-sha'] as String? ?? '';
    final workflowMetadata = DevDockerWorkflowMetadata(
      metadata: metadata,
      imageName: imageName,
      shortSha: shortSha,
    );

    stdout.write(workflowMetadataToGithubOutput(workflowMetadata));
  }
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
