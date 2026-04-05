import 'package:flutter_rust_bridge_internal/src/makefile_dart/precommit_autofix_in_dev_container.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:test/test.dart';

void main() {
  test('validatePrecommitAutofixPatch runs git apply check in repo root', () async {
    String? command;
    String? capturedRelativePwd;

    await validatePrecommitAutofixPatch(
      commandRunner: (
        cmd, {
        String? relativePwd,
        Map<String, String>? extraEnv,
        bool? checkExitCode,
      }) async {
        command = cmd;
        capturedRelativePwd = relativePwd;
        return const RunCommandOutput(stdout: '', stderr: '', exitCode: 0);
      },
      outputPath: '/tmp/precommit-autofix.diff',
      repoRootPath: '/repo',
    );

    expect(command, "git apply --check '/tmp/precommit-autofix.diff'");
    expect(capturedRelativePwd, '/repo');
  });

  test(
    'buildPrecommitAutofixApplyCommand uses concrete run id when provided',
    () {
      expect(
        buildPrecommitAutofixApplyCommand(
          artifactName: 'precommit-autofix-diff',
          githubRunId: '12345',
          patchFileName: 'precommit-autofix.diff',
        ),
        'artifact_dir="\$(mktemp -d)" && '
        'gh run download 12345 -n precommit-autofix-diff -D "\$artifact_dir" && '
        'git apply "\$artifact_dir/precommit-autofix.diff" && '
        'rm -rf "\$artifact_dir" && '
        'git add -A && '
        'git commit -m "Apply precommit autofix" && '
        'git push',
      );
    },
  );

  test('buildPrecommitAutofixApplyCommand falls back to placeholder run id', () {
    expect(
      buildPrecommitAutofixApplyCommand(
        artifactName: 'precommit-autofix-diff',
        githubRunId: null,
        patchFileName: 'precommit-autofix.diff',
      ),
      contains(
        'gh run download <run-id> -n precommit-autofix-diff -D "\$artifact_dir"',
      ),
    );
  });

  test('buildPrecommitAutofixGithubOutput emits expected keys', () {
    final output = buildPrecommitAutofixGithubOutput(
      applyCommand: 'git apply precommit-autofix.diff',
      artifactName: 'precommit-autofix-diff',
      hasPatch: true,
      imageRef:
          'fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01',
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

  test('buildPrecommitAutofixContainerCommand runs in temporary workspace', () {
    final command = buildPrecommitAutofixContainerCommand(
      mode: 'slow',
      outputPath: '/artifacts/precommit-autofix.diff',
    );

    expect(command, contains(r'cp -a /source/. "${temp_workspace}/"'));
    expect(command, contains(r'cd "${temp_workspace}"'));
    expect(command, contains(r'log_path="/artifacts/precommit-autofix.log"'));
    expect(
      command,
      contains(
        './frb_internal precommit-autofix --mode slow --output /artifacts/precommit-autofix.diff',
      ),
    );
    expect(command, contains(r'tail -n 200 "${log_path}" >&2 || true'));
    expect(command, contains(r'tail -n 40 "${log_path}" || true'));
    expect(command, isNot(contains('Validate precommit autofix patch')));
  });
}
