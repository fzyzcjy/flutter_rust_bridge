import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_workflow.dart';
import 'package:test/test.dart';

void main() {
  test('buildPrecommitAutofixApplyCommand uses concrete run id when provided', () {
    expect(
      buildPrecommitAutofixApplyCommand(
        artifactName: 'precommit-autofix-diff',
        githubRunId: '12345',
        patchFileName: 'precommit-autofix.diff',
      ),
      'gh run download 12345 -n precommit-autofix-diff && '
          'git apply precommit-autofix.diff && '
          'git add -A && '
          'git commit -m "Apply precommit autofix" && '
          'git push',
    );
  });

  test('buildPrecommitAutofixApplyCommand falls back to placeholder run id', () {
    expect(
      buildPrecommitAutofixApplyCommand(
        artifactName: 'precommit-autofix-diff',
        githubRunId: null,
        patchFileName: 'precommit-autofix.diff',
      ),
      contains('gh run download <run-id> -n precommit-autofix-diff'),
    );
  });

  test('buildPrecommitAutofixGithubOutput emits expected keys', () {
    final output = buildPrecommitAutofixGithubOutput(
      applyCommand: 'git apply precommit-autofix.diff',
      artifactName: 'precommit-autofix-diff',
      hasPatch: true,
      imageRef: 'fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01',
      outputPath: '/tmp/precommit-autofix.diff',
    );

    expect(output, contains('has_patch=true'));
    expect(output, contains('artifact_name=precommit-autofix-diff'));
    expect(output, contains('apply_command=git apply precommit-autofix.diff'));
    expect(output, contains('output_path=/tmp/precommit-autofix.diff'));
  });

  test('buildPrecommitAutofixSummaryMarkdown reports clean branch', () {
    expect(
      buildPrecommitAutofixSummaryMarkdown(
        applyCommand: 'unused',
        artifactName: 'precommit-autofix-diff',
        hasPatch: false,
      ),
      'No autofix patch was produced; the branch is already clean after precommit.\n',
    );
  });

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
    expect(summary, contains('- Image ref: `fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01`'));
    expect(summary, contains('  - `sha-abc1234`'));
  });
}
