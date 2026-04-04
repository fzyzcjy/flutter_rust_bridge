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
    argParser.addOption(
      'dockerfile',
      defaultsTo: defaultDockerfilePath,
      help: 'Path to the Dockerfile to parse.',
    );
  }

  @override
  Future<void> run() async {
    final dockerfilePath = argResults!['dockerfile'] as String;
    final metadata = readDevDockerMetadataFile(dockerfilePath: dockerfilePath);

    print(metadata.toJsonString());
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
