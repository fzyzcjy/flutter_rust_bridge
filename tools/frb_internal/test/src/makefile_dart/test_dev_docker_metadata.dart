import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  test('parseDevDockerMetadataFromText parses the current Dockerfile', () {
    final metadata = parseDevDockerMetadataFromText(
      _readCurrentDockerfileText(),
    );

    expect(metadata.flutterVersion, '3.44.0');
    expect(metadata.rustVersion, '1.93.1');
    expect(metadata.rustNightlyVersion, '2025-02-01');
    expect(
      metadata.versionTag,
      'flutter-3.44.0-rust-1.93.1-nightly-2025-02-01',
    );
    expect(
      metadata.imageRef(imageName: 'fzyzcjy/flutter_rust_bridge_dev'),
      'fzyzcjy/flutter_rust_bridge_dev:flutter-3.44.0-rust-1.93.1-nightly-2025-02-01',
    );
    expect(
      metadata.versionCodeTag(shortSha: 'abcdef0'),
      'flutter-3.44.0-rust-1.93.1-nightly-2025-02-01-code-abcdef0',
    );
    expect(
      metadata.imageRefForRevision(
        imageName: 'fzyzcjy/flutter_rust_bridge_dev',
        shortSha: 'abcdef0',
      ),
      'fzyzcjy/flutter_rust_bridge_dev:flutter-3.44.0-rust-1.93.1-nightly-2025-02-01-code-abcdef0',
    );
  });

  test('metadata imageRef respects custom image name', () {
    const metadata = DevDockerMetadata(
      flutterVersion: '3.44.0',
      rustVersion: '1.93.1',
      rustNightlyVersion: '2025-02-01',
    );

    expect(
      metadata.imageRef(imageName: 'example/custom-dev-image'),
      'example/custom-dev-image:flutter-3.44.0-rust-1.93.1-nightly-2025-02-01',
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
