import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';

const kCiManualDispatchLabel = 'ci-manual-dispatch';

class CiPlanCommand extends Command<void> {
  @override
  final String name = 'plan-ci';

  @override
  final String description =
      'Plan GitHub Actions CI jobs and matrices from a compact filter';

  CiPlanCommand() {
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

    final outputLines = ['plan=${jsonEncode(plan.toJson())}'];

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
    return CiPlan.empty();
  }

  final normalizedFilter = (filter ?? '').trim();
  if (normalizedFilter.isEmpty ||
      normalizedFilter == 'full' ||
      normalizedFilter == '*') {
    return CiPlan.full();
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
    matrixByJob[job.id] = {'include': entries};
  }

  return CiPlan(enabledJobs: enabledJobs, matrixByJob: matrixByJob);
}

class CiPlan {
  final Set<String> enabledJobs;
  final Map<String, Map<String, Object?>> matrixByJob;

  const CiPlan({required this.enabledJobs, required this.matrixByJob});

  factory CiPlan.empty() =>
      CiPlan(enabledJobs: {}, matrixByJob: _emptyMatrixByJob());

  factory CiPlan.full() => CiPlan(
    enabledJobs: kCiJobs.map((job) => job.id).toSet(),
    matrixByJob: {
      for (final job in kCiJobs)
        if (job.matrix != null) job.id: {'include': job.matrix!.entries},
    },
  );

  Map<String, Object?> toJson() => {
    'jobs': {for (final job in kCiJobs) job.id: enabledJobs.contains(job.id)},
    'matrices': matrixByJob,
  };
}

class CiJob {
  final String id;
  final CiMatrix? matrix;

  const CiJob(this.id, {this.matrix});
}

class CiMatrix {
  final List<Map<String, Object?>> entries;

  const CiMatrix(this.entries);
}

final kCiJobs = [
  CiJob('deploy_website'),
  CiJob('lint_rust_primary'),
  CiJob('lint_dart_primary'),
  CiJob('lint_rust_feature_flag'),
  CiJob(
    'generate_run_frb_codegen_command_generate',
    matrix: CiMatrix([
      for (final image in ['windows-2025', 'macos-15-intel', 'ubuntu-24.04'])
        for (final package in [
          'frb_example--dart_minimal',
          'frb_example--pure_dart',
          'frb_example--pure_dart_pde',
          'frb_example--dart_build_rs',
          'frb_example--deliberate_bad',
          'frb_example--integrate_third_party',
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_integrate',
          'frb_example--flutter_package',
          'frb_example--rust_ui_counter--ui',
          'frb_example--rust_ui_todo_list--ui',
        ])
          if (!_isExcludedGenerateCommandGenerate(
            image: image,
            package: package,
          ))
            {'image': image, 'package': package},
    ]),
  ),
  CiJob(
    'generate_run_frb_codegen_command_integrate',
    matrix: CiMatrix([
      for (final image in ['macos-15-intel', 'windows-2025', 'ubuntu-24.04'])
        for (final package in [
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_integrate',
          'frb_example--flutter_package',
        ])
          {'image': image, 'package': package, 'platforms': 'default'},
      {
        'image': 'ubuntu-24.04',
        'package': 'frb_example--flutter_via_create',
        'platforms': 'ohos',
      },
    ]),
  ),
  CiJob('generate_internal'),
  CiJob(
    'bench_dart_native',
    matrix: CiMatrix([
      for (final image in ['windows-2025', 'macos-15-intel', 'ubuntu-24.04'])
        {'image': image},
    ]),
  ),
  CiJob('bench_upload'),
  CiJob(
    'build_flutter',
    matrix: CiMatrix([
      for (final info in [
        {'image': 'windows-2025', 'target': 'windows'},
        {'image': 'windows-11-arm', 'target': 'windows'},
        {'image': 'macos-15-intel', 'target': 'macos'},
        {'image': 'ubuntu-latest', 'target': 'linux'},
        {'image': 'ubuntu-latest', 'target': 'android-aab'},
        {'image': 'ubuntu-latest', 'target': 'android-apk'},
        {'image': 'macos-15-intel', 'target': 'ios'},
        {'image': 'ubuntu-latest', 'target': 'ohos'},
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_mimic_quickstart',
    matrix: CiMatrix([
      for (final image in ['windows-2025', 'macos-15-intel', 'ubuntu-latest'])
        {'image': image},
    ]),
  ),
  CiJob(
    'test_rust',
    matrix: CiMatrix([
      for (final info in [
        {'image': 'macos-15-intel', 'version': ''},
        {'image': 'windows-2025', 'version': ''},
        {'image': 'ubuntu-latest', 'version': ''},
        {'image': 'ubuntu-latest', 'version': 'nightly'},
        {'image': 'ubuntu-latest', 'version': '1.85.0'},
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_dart_native',
    matrix: CiMatrix([
      for (final image in ['windows-2025', 'macos-15-intel', 'ubuntu-24.04'])
        for (final package in [
          'frb_dart',
          'frb_utils',
          'tools--frb_internal',
          'frb_example--dart_minimal',
          'frb_example--pure_dart',
          'frb_example--pure_dart_pde',
          'frb_example--dart_build_rs',
        ])
          if (!_isExcludedTestDartNative(image: image, package: package))
            {'image': image, 'package': package},
    ]),
  ),
  CiJob(
    'test_dart_web',
    matrix: CiMatrix([
      for (final package in [
        'frb_dart',
        'frb_example--dart_minimal',
        'frb_example--pure_dart',
        'frb_example--pure_dart_pde',
      ])
        {'package': package},
    ]),
  ),
  CiJob(
    'test_dart_valgrind',
    matrix: CiMatrix([
      for (final package in [
        'frb_example--dart_minimal',
        'frb_example--pure_dart',
        'frb_example--pure_dart_pde',
      ])
        {'package': package},
    ]),
  ),
  CiJob(
    'test_dart_sanitizer',
    matrix: CiMatrix([
      for (final sanitizer in ['asan', 'lsan'])
        for (final package in [
          'frb_example--dart_minimal',
          'frb_example--pure_dart',
          'frb_example--pure_dart_pde',
        ])
          {'sanitizer': sanitizer, 'package': package},
    ]),
  ),
  CiJob(
    'test_flutter_native_android',
    matrix: CiMatrix([
      for (final package in [
        'frb_example--flutter_via_create',
        'frb_example--flutter_package--example',
        'frb_example--rust_ui_counter--ui',
        'frb_example--rust_ui_todo_list--ui',
      ])
        for (final device in ['pixel', 'Nexus 6'])
          {'package': package, 'device': device, 'api-level': 35},
    ]),
  ),
  CiJob(
    'test_flutter_native_ios',
    matrix: CiMatrix([
      for (final package in [
        'frb_example--flutter_via_create',
        'frb_example--flutter_package--example',
        'frb_example--rust_ui_counter--ui',
        'frb_example--rust_ui_todo_list--ui',
      ])
        for (final device in [
          'iPad (10th generation) Simulator (18.6)',
          'iPhone 16 Pro Max Simulator (18.6)',
        ])
          {'package': package, 'device': device},
    ]),
  ),
  CiJob(
    'test_flutter_native_desktop',
    matrix: CiMatrix([
      for (final info in [
        {
          'image': 'windows-2025',
          'platform': 'windows',
          'package': 'frb_example--flutter_via_create',
        },
        {
          'image': 'macos-15-intel',
          'platform': 'macos',
          'package': 'frb_example--flutter_via_create',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--flutter_via_create',
        },
        {
          'image': 'windows-2025',
          'platform': 'windows',
          'package': 'frb_example--flutter_package--example',
        },
        {
          'image': 'macos-15-intel',
          'platform': 'macos',
          'package': 'frb_example--flutter_package--example',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--flutter_package--example',
        },
        {
          'image': 'windows-2025',
          'platform': 'windows',
          'package': 'frb_example--rust_ui_counter--ui',
        },
        {
          'image': 'macos-15-intel',
          'platform': 'macos',
          'package': 'frb_example--rust_ui_counter--ui',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--rust_ui_counter--ui',
        },
        {
          'image': 'windows-2025',
          'platform': 'windows',
          'package': 'frb_example--rust_ui_todo_list--ui',
        },
        {
          'image': 'macos-15-intel',
          'platform': 'macos',
          'package': 'frb_example--rust_ui_todo_list--ui',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--rust_ui_todo_list--ui',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--flutter_via_integrate',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--gallery',
        },
        {
          'image': 'ubuntu-latest',
          'platform': 'linux',
          'package': 'frb_example--integrate_third_party',
        },
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_flutter_web',
    matrix: CiMatrix([
      for (final package in [
        'frb_example--flutter_via_create',
        'frb_example--gallery',
      ])
        {'package': package},
    ]),
  ),
  CiJob('misc_codecov'),
];

final _jobById = {for (final job in kCiJobs) job.id: job};

List<CiFilterSpec> _parseFilter(String filter) {
  final parts = _splitTopLevel(filter, ',');
  if (parts.isEmpty) {
    throw const FormatException('CI filter is empty.');
  }
  return [for (final part in parts) CiFilterSpec.parse(part)];
}

class CiFilterSpec {
  final String original;
  final String jobId;
  final Map<String, Set<String>> filters;

  const CiFilterSpec({
    required this.original,
    required this.jobId,
    required this.filters,
  });

  factory CiFilterSpec.parse(String raw) {
    final original = raw.trim();
    if (original.isEmpty) {
      throw const FormatException('CI filter contains an empty item.');
    }

    final bracketStart = original.indexOf('[');
    if (bracketStart == -1) {
      if (original.contains(']')) {
        throw FormatException('Unexpected `]` in CI filter `$original`.');
      }
      return CiFilterSpec(original: original, jobId: original, filters: {});
    }

    if (!original.endsWith(']')) {
      throw FormatException('CI filter `$original` must end with `]`.');
    }
    final jobId = original.substring(0, bracketStart).trim();
    if (jobId.isEmpty) {
      throw FormatException('CI filter `$original` does not specify a job.');
    }

    final body = original.substring(bracketStart + 1, original.length - 1);
    final filters = <String, Set<String>>{};
    for (final rawCondition in _splitTopLevel(body, ',')) {
      final condition = rawCondition.trim();
      final equalIndex = condition.indexOf('=');
      if (equalIndex <= 0) {
        throw FormatException(
          'CI filter condition `$condition` must use dimension=value.',
        );
      }
      final dimension = condition.substring(0, equalIndex).trim();
      final values = condition
          .substring(equalIndex + 1)
          .split('|')
          .map((value) => value.trim())
          .where((value) => value.isNotEmpty)
          .toSet();
      if (values.isEmpty) {
        throw FormatException(
          'CI filter condition `$condition` does not specify a value.',
        );
      }
      filters[dimension] = {...?filters[dimension], ...values};
    }

    return CiFilterSpec(original: original, jobId: jobId, filters: filters);
  }
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

List<String> _splitTopLevel(String text, String separator) {
  final result = <String>[];
  var bracketDepth = 0;
  var start = 0;
  for (var index = 0; index < text.length; index++) {
    final char = text[index];
    if (char == '[') {
      bracketDepth++;
    } else if (char == ']') {
      bracketDepth--;
      if (bracketDepth < 0) {
        throw FormatException('Unexpected `]` in CI filter `$text`.');
      }
    } else if (char == separator && bracketDepth == 0) {
      result.add(text.substring(start, index));
      start = index + 1;
    }
  }
  if (bracketDepth != 0) {
    throw FormatException('Unclosed `[` in CI filter `$text`.');
  }
  result.add(text.substring(start));
  return result.where((part) => part.trim().isNotEmpty).toList();
}

Map<String, Map<String, Object?>> _emptyMatrixByJob() => {
  for (final job in kCiJobs)
    if (job.matrix != null) job.id: {'include': <Map<String, Object?>>[]},
};

bool _isExcludedGenerateCommandGenerate({
  required String image,
  required String package,
}) =>
    (image == 'windows-2025' || image == 'macos-15-intel') &&
    {
      'frb_example--deliberate_bad',
      'frb_example--integrate_third_party',
      'frb_example--flutter_via_integrate',
    }.contains(package);

bool _isExcludedTestDartNative({
  required String image,
  required String package,
}) =>
    (image == 'windows-2025' || image == 'macos-15-intel') &&
    {'frb_utils', 'tools--frb_internal'}.contains(package);
