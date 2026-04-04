import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

typedef DevDockerWorkflowCommandRunner = Future<RunCommandOutput> Function(
  String command, {
  String? relativePwd,
  Map<String, String>? extraEnv,
  bool? checkExitCode,
});

class PublishDevDockerCommand extends Command<void> {
  final DevDockerWorkflowCommandRunner commandRunner;
  final String repoRootPath;

  @override
  final String name = 'publish-dev-docker';

  @override
  final String description =
      'Build, smoke test, and optionally push the dev Docker image';

  PublishDevDockerCommand({
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
        'context-path',
        defaultsTo: '.devcontainer',
        help: 'Docker build context path.',
      )
      ..addOption(
        'image-name',
        defaultsTo: 'fzyzcjy/flutter_rust_bridge_dev',
        help: 'Docker image repository name.',
      )
      ..addOption(
        'short-sha',
        help: 'Short git SHA used to derive the sha tag.',
      )
      ..addOption(
        'platform',
        defaultsTo: 'linux/amd64',
        help: 'Docker platform passed to buildx.',
      )
      ..addFlag(
        'push',
        defaultsTo: false,
        help: 'Push the image tags after the smoke test.',
      )
      ..addOption(
        'github-output-path',
        help: 'Optional GitHub Actions output file path.',
      )
      ..addOption(
        'github-summary-path',
        help: 'Optional GitHub Actions summary file path.',
      );
  }

  @override
  Future<void> run() async {
    final githubOutputPath =
        (argResults!['github-output-path'] as String?) ??
        Platform.environment['GITHUB_OUTPUT'];
    final githubSummaryPath =
        (argResults!['github-summary-path'] as String?) ??
        Platform.environment['GITHUB_STEP_SUMMARY'];

    final result = await PublishDevDockerService(
      commandRunner: commandRunner,
      repoRootPath: repoRootPath,
    ).run(
      config: PublishDevDockerConfig(
        dockerfilePath: argResults!['dockerfile'] as String,
        contextPath: argResults!['context-path'] as String,
        imageName: argResults!['image-name'] as String,
        shortSha: argResults!['short-sha'] as String?,
        platform: argResults!['platform'] as String,
        push: argResults!['push'] as bool,
        githubOutputPath: githubOutputPath,
        githubSummaryPath: githubSummaryPath,
      ),
    );

    stdout.writeln(jsonEncode(result.toJson()));
  }
}

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

class PublishDevDockerConfig {
  final String contextPath;
  final String dockerfilePath;
  final String imageName;
  final String? shortSha;
  final String platform;
  final bool push;
  final String? githubOutputPath;
  final String? githubSummaryPath;

  const PublishDevDockerConfig({
    required this.contextPath,
    required this.dockerfilePath,
    required this.imageName,
    required this.shortSha,
    required this.platform,
    required this.push,
    required this.githubOutputPath,
    required this.githubSummaryPath,
  });
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

class PublishDevDockerResult {
  final DevDockerWorkflowMetadata workflowMetadata;

  const PublishDevDockerResult({required this.workflowMetadata});

  Map<String, Object> toJson() {
    return {
      'image_ref': workflowMetadata.imageRef,
      'local_tag': workflowMetadata.localTag,
      'sha_tag': workflowMetadata.shaTag,
      'tags': workflowMetadata.tags,
      'version_tag': workflowMetadata.metadata.versionTag,
    };
  }
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

class PublishDevDockerService {
  final DevDockerWorkflowCommandRunner commandRunner;
  final String repoRootPath;

  const PublishDevDockerService({
    required this.commandRunner,
    required this.repoRootPath,
  });

  Future<PublishDevDockerResult> run({
    required PublishDevDockerConfig config,
  }) async {
    final shortSha = await _deriveShortSha(config.shortSha);
    final workflowMetadata = _readWorkflowMetadata(
      dockerfilePath: config.dockerfilePath,
      imageName: config.imageName,
      shortSha: shortSha,
    );
    final contextPath = _resolvePath(config.contextPath);
    final dockerfilePath = _resolvePath(config.dockerfilePath);

    await commandRunner(
      'docker buildx build '
      '--file ${shellEscape(dockerfilePath)} '
      '--platform ${shellEscape(config.platform)} '
      '--tag ${shellEscape(workflowMetadata.localTag)} '
      '--load ${shellEscape(contextPath)}',
    );

    await commandRunner(
      'docker run --rm ${shellEscape(workflowMetadata.localTag)} bash -lc '
      '${shellEscape(_publishSmokeTestScript(workflowMetadata))}',
    );

    if (config.push) {
      final tagArgs = workflowMetadata.tags
          .map((tag) => '--tag ${shellEscape(tag)}')
          .join(' ');
      await commandRunner(
        'docker buildx build '
        '--file ${shellEscape(dockerfilePath)} '
        '--platform ${shellEscape(config.platform)} '
        '--push $tagArgs ${shellEscape(contextPath)}',
      );
    }

    if (config.githubOutputPath != null) {
      writeCommandOutput(
        outputText: workflowMetadataToGithubOutput(workflowMetadata),
        outputPath: config.githubOutputPath,
      );
    }

    if (config.githubSummaryPath != null) {
      appendTextToFile(
        outputPath: config.githubSummaryPath!,
        text: buildPublishDevDockerSummary(workflowMetadata),
      );
    }

    return PublishDevDockerResult(workflowMetadata: workflowMetadata);
  }

  Future<String> _deriveShortSha(String? shortSha) async {
    if (shortSha != null && shortSha.isNotEmpty) {
      return shortSha;
    }

    final output = await commandRunner(
      'git rev-parse --short=7 HEAD',
      checkExitCode: true,
    );
    return output.stdout.trim();
  }

  DevDockerWorkflowMetadata _readWorkflowMetadata({
    required String dockerfilePath,
    required String imageName,
    required String shortSha,
  }) {
    final metadata = readDevDockerMetadataFile(
      dockerfilePath: _resolvePath(dockerfilePath),
    );
    return DevDockerWorkflowMetadata(
      metadata: metadata,
      imageName: imageName,
      shortSha: shortSha,
    );
  }

  String _resolvePath(String rawPath) {
    if (path.isAbsolute(rawPath)) {
      return path.normalize(rawPath);
    }

    return path.normalize(path.join(repoRootPath, rawPath));
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
    final outputPath = _resolvePath(config.outputPath);
    final dockerfilePath = _resolvePath(config.dockerfilePath);
    final metadata = readDevDockerMetadataFile(dockerfilePath: dockerfilePath);
    final imageRef = '${config.imageName}:${metadata.versionTag}';
    final artifactDir = Directory(path.join(randomTempDir(), 'artifacts'));
    artifactDir.createSync(recursive: true);
    final containerPatchPath = '/artifacts/${path.basename(outputPath)}';
    final hostPatchPath = path.join(artifactDir.path, path.basename(outputPath));

    await commandRunner('docker pull ${shellEscape(imageRef)}');

    await commandRunner(
      'docker run --rm '
      '--volume ${shellEscape('$repoRootPath:/workspace')} '
      '--volume ${shellEscape('${artifactDir.path}:/artifacts')} '
      '--workdir /workspace '
      '${shellEscape(imageRef)} bash -lc '
      '${shellEscape(
        'git config --global --add safe.directory /workspace && '
        './frb_internal precommit-autofix --mode ${config.mode} --output $containerPatchPath',
      )}',
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

  String _resolvePath(String rawPath) {
    if (path.isAbsolute(rawPath)) {
      return path.normalize(rawPath);
    }

    return path.normalize(path.join(repoRootPath, rawPath));
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

@visibleForTesting
String buildPublishDevDockerSummary(
  DevDockerWorkflowMetadata workflowMetadata,
) {
  return [
    '## Dev Docker Image Published',
    '',
    '- Repository: `${workflowMetadata.imageName}`',
    '- Flutter: `${workflowMetadata.metadata.flutterVersion}`',
    '- Rust: `${workflowMetadata.metadata.rustVersion}`',
    '- Rust nightly: `${workflowMetadata.metadata.rustNightlyVersion}`',
    '- Image ref: `${workflowMetadata.imageRef}`',
    '- Tags:',
    '  - `latest`',
    '  - `${workflowMetadata.metadata.versionTag}`',
    '  - `${workflowMetadata.shaTag}`',
    '',
  ].join('\n');
}

@visibleForTesting
String shellEscape(String value) {
  return "'${value.replaceAll("'", r"'\''")}'";
}

String _publishSmokeTestScript(DevDockerWorkflowMetadata workflowMetadata) {
  return '''
set -euo pipefail
flutter --version
dart --version
cargo --version
rustup show
rustup target list --toolchain nightly-${workflowMetadata.metadata.rustNightlyVersion} --installed
wasm-pack --version
"\${CHROME_BIN}" --version
rustup target list --toolchain nightly-${workflowMetadata.metadata.rustNightlyVersion} --installed | grep wasm32-unknown-unknown
rustup show | grep "nightly-${workflowMetadata.metadata.rustNightlyVersion}"
''';
}

void appendTextToFile({
  required String outputPath,
  required String text,
}) {
  final outputFile = File(outputPath);
  outputFile.parent.createSync(recursive: true);
  outputFile.writeAsStringSync(text, mode: FileMode.append);
}
