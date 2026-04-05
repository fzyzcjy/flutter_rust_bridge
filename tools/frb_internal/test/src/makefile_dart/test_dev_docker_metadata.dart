import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  test('parseDevDockerMetadataFromText parses the current Dockerfile', () {
    final metadata = parseDevDockerMetadataFromText(
      _readCurrentDockerfileText(),
    );

    expect(metadata.flutterVersion, '3.41.2');
    expect(metadata.rustVersion, '1.93.1');
    expect(metadata.rustNightlyVersion, '2025-02-01');
    expect(
      metadata.versionTag,
      'flutter-3.41.2-rust-1.93.1-nightly-2025-02-01',
    );
    expect(
      metadata.imageRef(imageName: 'fzyzcjy/flutter_rust_bridge_dev'),
      'fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01',
    );
  });

  test('workflow metadata imageRef respects custom image name', () {
    const metadata = DevDockerMetadata(
      flutterVersion: '3.41.2',
      rustVersion: '1.93.1',
      rustNightlyVersion: '2025-02-01',
    );
    const workflowMetadata = DevDockerWorkflowMetadata(
      metadata: metadata,
      imageName: 'example/custom-dev-image',
      shortSha: 'abc1234',
    );

    expect(
      workflowMetadata.imageRef,
      'example/custom-dev-image:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01',
    );
  });
}

String _readCurrentDockerfileText() {
  var currentDirectory = Directory.current.absolute;

  while (true) {
    final candidate = File(
      path.join(currentDirectory.path, '.devcontainer', 'Dockerfile'),
    );
    if (candidate.existsSync()) {
      return candidate.readAsStringSync();
    }

    final parentDirectory = currentDirectory.parent;
    if (parentDirectory.path == currentDirectory.path) {
      break;
    }
    currentDirectory = parentDirectory;
  }

  throw StateError(
    'Could not find .devcontainer/Dockerfile from ${Directory.current.path}.',
  );
}
