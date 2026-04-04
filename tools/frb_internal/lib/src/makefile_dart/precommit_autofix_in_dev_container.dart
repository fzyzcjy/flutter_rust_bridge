import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_support.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

class PrecommitAutofixInDevContainerCommand extends Command<void> {
  final DevDockerWorkflowCommandRunner commandRunner;
  final String repoRootPath;

  @override
  final String name = 'precommit-autofix-in-dev-container';

  @override
  final String description =
      'Run precommit-autofix inside the published dev Docker image';

  PrecommitAutofixInDevContainerCommand({
    required this.commandRunner,
    required this.repoRootPath,
  }) {
    argParser
      ..addOption(
        'dockerfile',
        defaultsTo: '.devcontainer/Dockerfile',
        help: 'Path to the dev Dockerfile.',
      )
      ..addOption(
        'image-name',
        defaultsTo: 'fzyzcjy/flutter_rust_bridge_dev',
        help: 'Docker image repository name.',
      )
      ..addOption(
        'mode',
        allowed: ['fast', 'slow'],
        defaultsTo: 'slow',
        help: 'Precommit mode to execute inside the dev image.',
      )
      ..addOption(
        'output',
        help: 'Host-side path to write the final patch file.',
      )
      ..addOption(
        'artifact-name',
        defaultsTo: 'precommit-autofix-diff',
        help: 'Artifact name shown in GitHub output and summary.',
      )
      ..addOption(
        'github-run-id',
        help: 'GitHub run id used in the suggested download command.',
      )
      ..addOption(
        'github-output-path',
        help: 'Optional GitHub Actions output file path.',
      )
      ..addOption(
        'github-summary-path',
        help: 'Optional GitHub Actions summary file path.',
      )
      ..addFlag(
        'github-notice',
        defaultsTo: false,
        help: 'Emit a ::notice:: line when a patch is produced.',
      );
  }

  @override
  Future<void> run() async {
    final githubWorkspace = Platform.environment['GITHUB_WORKSPACE'];
    final outputPath =
        (argResults!['output'] as String?) ??
        (githubWorkspace == null
            ? null
            : path.join(githubWorkspace, 'precommit-autofix.diff'));
    if (outputPath == null) {
      throw ArgumentError(
        'Either --output must be provided or GITHUB_WORKSPACE must be set.',
      );
    }

    final githubRunId =
        (argResults!['github-run-id'] as String?) ??
        Platform.environment['GITHUB_RUN_ID'];
    final githubOutputPath =
        (argResults!['github-output-path'] as String?) ??
        Platform.environment['GITHUB_OUTPUT'];
    final githubSummaryPath =
        (argResults!['github-summary-path'] as String?) ??
        Platform.environment['GITHUB_STEP_SUMMARY'];
    final githubNotice =
        (argResults!['github-notice'] as bool) ||
        Platform.environment['GITHUB_ACTIONS'] == 'true';

    final result = await PrecommitAutofixInDevContainerService(
      commandRunner: commandRunner,
      repoRootPath: repoRootPath,
    ).run(
      config: PrecommitAutofixInDevContainerConfig(
        dockerfilePath: argResults!['dockerfile'] as String,
        imageName: argResults!['image-name'] as String,
        mode: argResults!['mode'] as String,
        outputPath: outputPath,
        artifactName: argResults!['artifact-name'] as String,
        githubRunId: githubRunId,
        githubOutputPath: githubOutputPath,
        githubSummaryPath: githubSummaryPath,
        githubNotice: githubNotice,
      ),
    );

    stdout.writeln(jsonEncode(result.toJson()));
  }
}

class PrecommitAutofixInDevContainerConfig {
  final String artifactName;
  final String dockerfilePath;
  final String? githubRunId;
  final String? githubOutputPath;
  final String? githubSummaryPath;
  final bool githubNotice;
  final String imageName;
  final String mode;
  final String outputPath;

  const PrecommitAutofixInDevContainerConfig({
    required this.artifactName,
    required this.dockerfilePath,
    required this.githubRunId,
    required this.githubOutputPath,
    required this.githubSummaryPath,
    required this.githubNotice,
    required this.imageName,
    required this.mode,
    required this.outputPath,
  });
}

class PrecommitAutofixInDevContainerResult {
  final String applyCommand;
  final String artifactName;
  final bool hasPatch;
  final String imageRef;
  final String outputPath;

  const PrecommitAutofixInDevContainerResult({
    required this.applyCommand,
    required this.artifactName,
    required this.hasPatch,
    required this.imageRef,
    required this.outputPath,
  });

  Map<String, Object> toJson() {
    return {
      'apply_command': applyCommand,
      'artifact_name': artifactName,
      'has_patch': hasPatch,
      'image_ref': imageRef,
      'output_path': outputPath,
    };
  }
}

class PrecommitAutofixInDevContainerService {
  final DevDockerWorkflowCommandRunner commandRunner;
  final String repoRootPath;

  const PrecommitAutofixInDevContainerService({
    required this.commandRunner,
    required this.repoRootPath,
  });

  Future<PrecommitAutofixInDevContainerResult> run({
    required PrecommitAutofixInDevContainerConfig config,
  }) async {
    final outputPath = resolveRepoPath(
      rawPath: config.outputPath,
      repoRootPath: repoRootPath,
    );
    final dockerfilePath = resolveRepoPath(
      rawPath: config.dockerfilePath,
      repoRootPath: repoRootPath,
    );
    final metadata = readDevDockerMetadataFile(dockerfilePath: dockerfilePath);
    final imageRef = '${config.imageName}:${metadata.versionTag}';
    final artifactDir = Directory(path.join(randomTempDir(), 'artifacts'));
    artifactDir.createSync(recursive: true);
    final containerPatchPath = '/artifacts/${path.basename(outputPath)}';
    final hostPatchPath = path.join(
      artifactDir.path,
      path.basename(outputPath),
    );
    final containerCommand = buildPrecommitAutofixContainerCommand(
      mode: config.mode,
      outputPath: containerPatchPath,
    );

    await commandRunner('docker pull ${shellEscape(imageRef)}');

    await commandRunner(
      'docker run --rm '
      '--volume ${shellEscape('$repoRootPath:/source:ro')} '
      '--volume ${shellEscape('${artifactDir.path}:/artifacts')} '
      '${shellEscape(imageRef)} bash -lc '
      '${shellEscape(containerCommand)}',
    );

    final outputFile = File(outputPath);
    final hostPatchFile = File(hostPatchPath);
    final hasPatch = hostPatchFile.existsSync() && hostPatchFile.lengthSync() > 0;

    if (hasPatch) {
      outputFile.parent.createSync(recursive: true);
      hostPatchFile.copySync(outputPath);
    } else if (outputFile.existsSync()) {
      outputFile.deleteSync();
    }

    final applyCommand = buildPrecommitAutofixApplyCommand(
      artifactName: config.artifactName,
      githubRunId: config.githubRunId,
      patchFileName: path.basename(outputPath),
    );

    if (config.githubOutputPath != null) {
      writeCommandOutput(
        outputText: buildPrecommitAutofixGithubOutput(
          applyCommand: applyCommand,
          artifactName: config.artifactName,
          hasPatch: hasPatch,
          imageRef: imageRef,
          outputPath: outputPath,
        ),
        outputPath: config.githubOutputPath,
      );
    }

    if (config.githubSummaryPath != null) {
      appendTextToFile(
        outputPath: config.githubSummaryPath!,
        text: buildPrecommitAutofixSummaryMarkdown(
          applyCommand: applyCommand,
          artifactName: config.artifactName,
          hasPatch: hasPatch,
        ),
      );
    }

    if (config.githubNotice && hasPatch) {
      stdout.writeln('::notice::$applyCommand');
    }

    return PrecommitAutofixInDevContainerResult(
      applyCommand: applyCommand,
      artifactName: config.artifactName,
      hasPatch: hasPatch,
      imageRef: imageRef,
      outputPath: outputPath,
    );
  }
}

@visibleForTesting
String buildPrecommitAutofixApplyCommand({
  required String artifactName,
  required String? githubRunId,
  required String patchFileName,
}) {
  final effectiveGithubRunId = githubRunId ?? '<run-id>';
  return 'gh run download $effectiveGithubRunId -n $artifactName && '
      'git apply $patchFileName && '
      'git add -A && '
      'git commit -m "Apply precommit autofix" && '
      'git push';
}

@visibleForTesting
String buildPrecommitAutofixContainerCommand({
  required String mode,
  required String outputPath,
}) {
  return [
    'set -euo pipefail',
    'temp_workspace="\$(mktemp -d /tmp/frb-precommit-workspace.XXXXXX)"',
    'cp -a /source/. "\${temp_workspace}/"',
    'cd "\${temp_workspace}"',
    'git config --global --add safe.directory "\${temp_workspace}"',
    'rustup target add wasm32-unknown-unknown',
    '(cargo expand --version >/dev/null 2>&1 || '
        'cargo install cargo-expand || '
        'cargo install cargo-expand --version 1.0.112 --locked)',
    './frb_internal precommit-autofix --mode $mode --output $outputPath',
    'if [ -s $outputPath ]; then',
    '  git apply $outputPath',
    '  git add -A',
    '  git config user.email precommit-autofix@local',
    '  git config user.name "Precommit Autofix"',
    '  git commit -m "Validate precommit autofix patch" >/dev/null',
    '  ./frb_internal precommit-autofix --mode $mode '
        '--output /tmp/precommit-autofix-validate.diff '
        '>/tmp/precommit-autofix-validate.json',
    '  if [ -s /tmp/precommit-autofix-validate.diff ]; then',
    '    echo "Generated precommit autofix patch is not idempotent." >&2',
    '    cat /tmp/precommit-autofix-validate.json >&2 || true',
    '    exit 1',
    '  fi',
    'fi',
  ].join('\n');
}

@visibleForTesting
String buildPrecommitAutofixGithubOutput({
  required String applyCommand,
  required String artifactName,
  required bool hasPatch,
  required String imageRef,
  required String outputPath,
}) {
  final buffer = StringBuffer()
    ..writeln('has_patch=${hasPatch.toString()}')
    ..writeln('artifact_name=$artifactName')
    ..writeln('apply_command=$applyCommand')
    ..writeln('image_ref=$imageRef')
    ..writeln('output_path=$outputPath');
  return buffer.toString();
}

@visibleForTesting
String buildPrecommitAutofixSummaryMarkdown({
  required String applyCommand,
  required String artifactName,
  required bool hasPatch,
}) {
  if (!hasPatch) {
    return 'No autofix patch was produced; the branch is already clean after precommit.\n';
  }

  return [
    '## Precommit Autofix',
    '',
    'A precommit autofix patch was produced in the standardized dev image.',
    '',
    'Download the `$artifactName` artifact and apply:',
    '',
    '```shell',
    applyCommand,
    '```',
    '',
  ].join('\n');
}
