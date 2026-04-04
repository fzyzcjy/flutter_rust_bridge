import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_metadata.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/dev_docker_support.dart';
import 'package:meta/meta.dart';

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
    final contextPath = resolveRepoPath(
      rawPath: config.contextPath,
      repoRootPath: repoRootPath,
    );
    final dockerfilePath = resolveRepoPath(
      rawPath: config.dockerfilePath,
      repoRootPath: repoRootPath,
    );

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
      dockerfilePath: resolveRepoPath(
        rawPath: dockerfilePath,
        repoRootPath: repoRootPath,
      ),
    );
    return DevDockerWorkflowMetadata(
      metadata: metadata,
      imageName: imageName,
      shortSha: shortSha,
    );
  }
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
