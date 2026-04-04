import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_diff_exclusions.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;

typedef PrecommitAutofixCommandRunner =
    Future<RunCommandOutput> Function(
      String command, {
      String? relativePwd,
      Map<String, String>? extraEnv,
      bool? checkExitCode,
    });

typedef PrecommitAutofixRunner =
    Future<void> Function(PrecommitAutofixMode mode);

enum PrecommitAutofixMode { fast, slow }

enum PrecommitAutofixStatus { clean, patched }

class PrecommitAutofixSummary {
  final PrecommitAutofixStatus status;
  final String artifactName;
  final String outputPath;
  final String patchFileName;
  final String applyCommand;
  final String message;

  const PrecommitAutofixSummary({
    required this.status,
    required this.artifactName,
    required this.outputPath,
    required this.patchFileName,
    required this.applyCommand,
    required this.message,
  });

  Map<String, Object> toJson() {
    return {
      'status': status.name,
      'artifact_name': artifactName,
      'output_path': outputPath,
      'patch_file_name': patchFileName,
      'apply_command': applyCommand,
      'message': message,
    };
  }

  String toJsonString() => jsonEncode(toJson());
}

class PrecommitAutofixRunResult {
  final PrecommitAutofixSummary summary;
  final String? patchText;

  const PrecommitAutofixRunResult({
    required this.summary,
    required this.patchText,
  });

  bool get hasPatch => patchText?.trim().isNotEmpty ?? false;
}

class PrecommitAutofixCommand extends Command<void> {
  final PrecommitAutofixCommandRunner commandRunner;
  final PrecommitAutofixRunner precommitRunner;
  final String repoRootPath;

  @override
  final String name = 'precommit-autofix';

  @override
  final String description =
      'Run precommit and export a patch when it changes files';

  PrecommitAutofixCommand({
    required this.commandRunner,
    required this.precommitRunner,
    required this.repoRootPath,
  }) {
    argParser
      ..addOption(
        'mode',
        allowed: ['fast', 'slow'],
        defaultsTo: 'slow',
        help: 'Run precommit in fast or slow mode.',
      )
      ..addOption(
        'output',
        mandatory: true,
        help: 'Path to write the generated .diff file.',
      );
  }

  @override
  Future<void> run() async {
    final mode = parsePrecommitAutofixMode(argResults!['mode'] as String);
    final outputPath = argResults!['output'] as String;

    final result = await PrecommitAutofixService(
      commandRunner: commandRunner,
      precommitRunner: precommitRunner,
      repoRootPath: repoRootPath,
    ).run(mode: mode, outputPath: outputPath);

    stdout.writeln(result.summary.toJsonString());
  }
}

class PrecommitAutofixService {
  final PrecommitAutofixCommandRunner commandRunner;
  final PrecommitAutofixRunner precommitRunner;
  final String repoRootPath;

  const PrecommitAutofixService({
    required this.commandRunner,
    required this.precommitRunner,
    required this.repoRootPath,
  });

  Future<PrecommitAutofixRunResult> run({
    required PrecommitAutofixMode mode,
    required String outputPath,
  }) async {
    await _ensureRepoCleanBeforeRun();

    await precommitRunner(mode);

    final resolvedOutputPath = _resolveOutputPath(outputPath);
    final patchText = await _generatePatchText().whenComplete(_restoreIndex);

    if (!hasPrecommitAutofixPatch(patchText)) {
      await _removeFileIfExists(resolvedOutputPath);
      return PrecommitAutofixRunResult(
        summary: buildPrecommitAutofixSummary(
          status: PrecommitAutofixStatus.clean,
          outputPath: resolvedOutputPath,
        ),
        patchText: null,
      );
    }

    final outputFile = File(resolvedOutputPath);
    outputFile.parent.createSync(recursive: true);
    await outputFile.writeAsString(patchText);

    return PrecommitAutofixRunResult(
      summary: buildPrecommitAutofixSummary(
        status: PrecommitAutofixStatus.patched,
        outputPath: resolvedOutputPath,
      ),
      patchText: patchText,
    );
  }

  Future<void> _ensureRepoCleanBeforeRun() async {
    final status = await _gitStatusPorcelain();
    if (status.trim().isEmpty) {
      return;
    }

    throw StateError(
      'Repository must be clean before running precommit-autofix.\n'
      'Commit, stash, or remove the existing changes and try again.\n'
      'Current status:\n$status',
    );
  }

  Future<String> _generatePatchText() async {
    await commandRunner('git add -A .');
    final diff = await commandRunner(
      'git diff --cached --binary --full-index --no-color -- . '
      '${gitExcludePathspecArgs(kIntegrateDiffExcludedPaths)}',
    );
    return diff.stdout;
  }

  Future<String> _gitStatusPorcelain() async {
    final output = await commandRunner(
      'git status --porcelain=v1 --untracked-files=all',
    );
    return output.stdout;
  }

  Future<void> _restoreIndex() async {
    await commandRunner('git reset --mixed');
  }

  Future<void> _removeFileIfExists(String outputPath) async {
    final file = File(outputPath);
    if (await file.exists()) {
      await file.delete();
    }
  }

  String _resolveOutputPath(String outputPath) {
    if (path.isAbsolute(outputPath)) {
      return path.normalize(outputPath);
    }

    return path.normalize(path.join(repoRootPath, outputPath));
  }
}

@visibleForTesting
PrecommitAutofixMode parsePrecommitAutofixMode(String raw) {
  return switch (raw) {
    'fast' => PrecommitAutofixMode.fast,
    'slow' => PrecommitAutofixMode.slow,
    _ => throw ArgumentError.value(raw, 'mode'),
  };
}

@visibleForTesting
PrecommitAutofixSummary buildPrecommitAutofixSummary({
  required PrecommitAutofixStatus status,
  required String outputPath,
}) {
  return PrecommitAutofixSummary(
    status: status,
    artifactName: 'precommit-autofix-diff',
    outputPath: outputPath,
    patchFileName: path.basename(outputPath),
    applyCommand: 'git apply ${path.basename(outputPath)}',
    message: switch (status) {
      PrecommitAutofixStatus.clean => 'No changes were produced.',
      PrecommitAutofixStatus.patched =>
        'Patch written to ${path.basename(outputPath)}.',
    },
  );
}

@visibleForTesting
bool hasPrecommitAutofixPatch(String patchText) => patchText.trim().isNotEmpty;

@visibleForTesting
String renderPrecommitAutofixSummary({
  required PrecommitAutofixStatus status,
  required String outputPath,
}) {
  return buildPrecommitAutofixSummary(
    status: status,
    outputPath: outputPath,
  ).toJsonString();
}
