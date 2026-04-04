import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/publish_dev_docker.dart';
import 'package:test/test.dart';

void main() {
  test('buildPublishDevDockerSummary includes image metadata', () {
    final summary = buildPublishDevDockerSummary(
      const DevDockerWorkflowMetadata(
        metadata: DevDockerMetadata(
          flutterVersion: '3.41.2',
          rustVersion: '1.93.1',
          rustNightlyVersion: '2025-02-01',
        ),
        imageName: 'fzyzcjy/flutter_rust_bridge_dev',
        shortSha: 'abc1234',
      ),
    );

    expect(summary, contains('## Dev Docker Image Published'));
    expect(summary, contains('- Repository: `fzyzcjy/flutter_rust_bridge_dev`'));
    expect(
      summary,
      contains(
        '- Image ref: `fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01`',
      ),
    );
    expect(summary, contains('  - `sha-abc1234`'));
  });
}
