import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';

import 'full_jobs.dart';
import 'structs.dart';

const kCiManualDispatchLabel = 'ci-manual-dispatch';

class PlanCiCommand extends Command<void> {
  @override
  final String name = 'plan-ci';

  @override
  final String description =
      'Plan GitHub Actions CI jobs and matrices from a compact filter';

  PlanCiCommand() {
    argParser
      ..addOption(
        'filter',
        defaultsTo: 'full',
        help:
            'Filter such as test_dart_web[package=frb_example--pure_dart_pde], or full.',
      )
      ..addOption(
        'automatic-ci-disabled',
        defaultsTo: 'false',
        help:
            'Disable normal automatic CI jobs, used for PRs labeled $kCiManualDispatchLabel.',
      )
      ..addOption(
        'github-output',
        help: 'Path to GitHub Actions GITHUB_OUTPUT.',
      );
  }

  @override
  Future<void> run() async {
    final plan = buildCiPlan(
      filter: argResults!['filter'] as String?,
      automaticCiDisabled:
          (argResults!['automatic-ci-disabled'] as String).toLowerCase() ==
          'true',
    );
    final githubOutputPath = argResults!['github-output'] as String?;

    final outputLines = ['plan=${jsonEncode(buildCiPlanOutput(plan))}'];

    if (githubOutputPath != null) {
      File(githubOutputPath).writeAsStringSync('${outputLines.join('\n')}\n');
    } else {
      stdout.write(outputLines.join('\n'));
      stdout.write('\n');
    }
  }
}

CiPlan buildCiPlan({
  required String? filter,
  required bool automaticCiDisabled,
}) {
  if (automaticCiDisabled) {
    return _emptyCiPlan();
  }

  final normalizedFilter = (filter ?? '').trim();
  if (normalizedFilter.isEmpty ||
      normalizedFilter == 'full' ||
      normalizedFilter == '*') {
    return _fullCiPlan();
  }

  final specs = _parseFilter(normalizedFilter);
  final enabledJobs = <String>{};
  final matrixByJob = _emptyMatrixByJob();
  for (final spec in specs) {
    final job = _jobById[spec.jobId];
    if (job == null) {
      throw FormatException('Unknown CI job `${spec.jobId}`.');
    }

    enabledJobs.add(job.id);
    final matrix = job.matrix;
    if (matrix == null) {
      if (spec.filters.isNotEmpty) {
        throw FormatException('CI job `${job.id}` does not have a matrix.');
      }
      continue;
    }

    final entries = spec.filters.isEmpty
        ? matrix.entries
        : _filterMatrixEntries(job: job, filters: spec.filters);
    if (entries.isEmpty) {
      throw FormatException(
        'CI filter `${spec.original}` did not match any matrix entries.',
      );
    }
    matrixByJob[job.id] = {
      'include': _mergeMatrixEntries(
        matrix: matrix,
        current: matrixByJob[job.id]!['include'] as List<Map<String, Object?>>,
        additions: entries,
      ),
    };
  }

  return CiPlan(enabledJobs: enabledJobs, matrixByJob: matrixByJob);
}

CiPlanOutput buildCiPlanOutput(CiPlan plan) => {
  for (final job in kCiJobs)
    job.id: CiPlanJobOutput(
      enable: plan.enabledJobs.contains(job.id),
      matrix: plan.matrixByJob[job.id],
    ),
};

final _jobById = {for (final job in kCiJobs) job.id: job};

CiPlan _emptyCiPlan() =>
    CiPlan(enabledJobs: {}, matrixByJob: _emptyMatrixByJob());

CiPlan _fullCiPlan() => CiPlan(
  enabledJobs: kCiJobs.map((job) => job.id).toSet(),
  matrixByJob: {
    for (final job in kCiJobs)
      if (job.matrix != null) job.id: {'include': job.matrix!.entries},
  },
);

List<CiFilterSpec> _parseFilter(String filter) {
  final parts = splitTopLevel(filter, ',');
  if (parts.isEmpty) {
    throw const FormatException('CI filter is empty.');
  }
  return [for (final part in parts) CiFilterSpec.parse(part)];
}

List<Map<String, Object?>> _filterMatrixEntries({
  required CiJob job,
  required Map<String, Set<String>> filters,
}) {
  for (final dimension in filters.keys) {
    final hasDimension = job.matrix!.entries.any(
      (entry) => _matrixValue(entry: entry, dimension: dimension) != null,
    );
    if (!hasDimension) {
      throw FormatException(
        'CI job `${job.id}` does not have matrix dimension `$dimension`.',
      );
    }
  }

  return [
    for (final entry in job.matrix!.entries)
      if (_entryMatchesFilters(entry: entry, filters: filters)) entry,
  ];
}

List<Map<String, Object?>> _mergeMatrixEntries({
  required CiMatrix matrix,
  required List<Map<String, Object?>> current,
  required List<Map<String, Object?>> additions,
}) {
  final selected = {...current, ...additions};
  return [
    for (final entry in matrix.entries)
      if (selected.contains(entry)) entry,
  ];
}

bool _entryMatchesFilters({
  required Map<String, Object?> entry,
  required Map<String, Set<String>> filters,
}) {
  for (final MapEntry(key: dimension, value: allowedValues)
      in filters.entries) {
    final value = _matrixValue(entry: entry, dimension: dimension);
    if (value == null || !allowedValues.contains(value)) {
      return false;
    }
  }
  return true;
}

String? _matrixValue({
  required Map<String, Object?> entry,
  required String dimension,
}) {
  final value = entry[dimension];
  if (value != null) {
    return value.toString();
  }

  final info = entry['info'];
  if (info is Map<String, Object?>) {
    return info[dimension]?.toString();
  }
  return null;
}

Map<String, Map<String, Object?>> _emptyMatrixByJob() => {
  for (final job in kCiJobs)
    if (job.matrix != null) job.id: {'include': <Map<String, Object?>>[]},
};
