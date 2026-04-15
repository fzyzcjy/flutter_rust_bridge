import 'dart:convert';
import 'dart:io';

import 'package:glob/glob.dart';
import 'package:flutter_rust_bridge_internal/src/utils/codecov_transformer.dart';
import 'package:meta/meta.dart';
import 'package:path/path.dart' as path;
import 'package:yaml/yaml.dart';

class CodecovCoverageSummary {
  final int coveredLines;
  final int executableLines;

  const CodecovCoverageSummary({
    required this.coveredLines,
    required this.executableLines,
  });

  double get coveragePercent =>
      executableLines == 0 ? 100 : coveredLines * 100 / executableLines;
}

class CodecovPreaggregateResult {
  final Map<String, dynamic> mergedCoverage;
  final int reportCount;
  final int inputFileCount;
  final CodecovCoverageSummary summary;

  const CodecovPreaggregateResult({
    required this.mergedCoverage,
    required this.reportCount,
    required this.inputFileCount,
    required this.summary,
  });
}

Future<CodecovPreaggregateResult> preaggregateCodecovReports({
  required String inputDir,
  String? outputPath,
  String? ignoreConfigPath,
}) async {
  final effectiveIgnoreConfigPath =
      ignoreConfigPath ?? findNearestCodecovYamlPath();
  final reportFiles =
      Directory(inputDir)
          .listSync(recursive: true)
          .whereType<File>()
          .where((file) => path.basename(file.path) == 'codecov.json')
          .toList()
        ..sort((a, b) => a.path.compareTo(b.path));
  final ignoreMatcher = loadCodecovIgnoreMatcher(ignoreConfigPath);
  final repoRoot = effectiveIgnoreConfigPath == null
      ? Directory.current.path
      : path.dirname(effectiveIgnoreConfigPath);

  final mergedCoverage = <String, Map<String, dynamic>>{};

  for (final reportFile in reportFiles) {
    final raw =
        jsonDecode(await reportFile.readAsString()) as Map<String, dynamic>;
    final coverage = raw['coverage'] as Map<String, dynamic>;
    coverage.forEach((filename, fileCoverageRaw) {
      final normalizedFilename = normalizeCodecovFilename(filename);
      if (ignoreMatcher.matches(normalizedFilename)) return;
      final fileCoverage = (fileCoverageRaw as Map<String, dynamic>)
          .cast<String, dynamic>();
      final mergedFileCoverage = mergedCoverage.putIfAbsent(
        normalizedFilename,
        () => {},
      );

      fileCoverage.forEach((lineNumber, rawValue) {
        mergedFileCoverage[lineNumber] = mergeCodecovValues(
          existingValue: mergedFileCoverage[lineNumber],
          incomingValue: rawValue,
        );
      });
    });
  }

  final postprocessedMergedCoverage = mergedCoverage.map((
    filename,
    fileCoverage,
  ) {
    final file = File(path.join(repoRoot, filename));
    final transformedCoverage = file.existsSync()
        ? postprocessCodecovFileCoverage(
            file.readAsStringSync().split('\n'),
            fileCoverage,
          )
        : fileCoverage;
    return MapEntry(filename, transformedCoverage);
  });

  final sortedMergedCoverage = _sortMergedCoverage(postprocessedMergedCoverage);
  final result = CodecovPreaggregateResult(
    mergedCoverage: {'coverage': sortedMergedCoverage},
    reportCount: reportFiles.length,
    inputFileCount: sortedMergedCoverage.length,
    summary: computeCodecovCoverageSummary(sortedMergedCoverage),
  );

  if (outputPath != null) {
    final outputFile = File(outputPath);
    outputFile.parent.createSync(recursive: true);
    await outputFile.writeAsString(
      const JsonEncoder.withIndent('  ').convert(result.mergedCoverage),
    );
  }

  return result;
}

CodecovIgnoreMatcher loadCodecovIgnoreMatcher(String? configPath) {
  final effectiveConfigPath = configPath ?? findNearestCodecovYamlPath();
  if (effectiveConfigPath == null) {
    return const CodecovIgnoreMatcher(patterns: []);
  }

  final raw = loadYaml(File(effectiveConfigPath).readAsStringSync()) as YamlMap;
  final patterns = (raw['ignore'] as YamlList? ?? const [])
      .map((item) => item.toString())
      .toList();
  return CodecovIgnoreMatcher(patterns: patterns);
}

String? findNearestCodecovYamlPath() {
  var current = Directory.current.absolute;
  while (true) {
    for (final candidate in [
      path.join(current.path, 'codecov.yml'),
      path.join(current.path, '.codecov.yml'),
      path.join(current.path, 'codecov.yaml'),
      path.join(current.path, '.codecov.yaml'),
    ]) {
      if (File(candidate).existsSync()) return candidate;
    }

    final parent = current.parent;
    if (parent.path == current.path) return null;
    current = parent;
  }
}

CodecovCoverageSummary computeCodecovCoverageSummary(
  Map<String, dynamic> mergedCoverage,
) {
  var coveredLines = 0;
  var executableLines = 0;

  for (final fileCoverageRaw in mergedCoverage.values) {
    final fileCoverage = fileCoverageRaw as Map<String, dynamic>;
    for (final value in fileCoverage.values) {
      final normalizedValue = normalizeCodecovValue(value);
      if (normalizedValue == null) continue;
      executableLines++;
      if (normalizedValue > 0) coveredLines++;
    }
  }

  return CodecovCoverageSummary(
    coveredLines: coveredLines,
    executableLines: executableLines,
  );
}

Map<String, dynamic> _sortMergedCoverage(
  Map<String, Map<String, dynamic>> mergedCoverage,
) {
  final sortedFileKeys = mergedCoverage.keys.toList()..sort();
  return {
    for (final fileKey in sortedFileKeys)
      fileKey: {
        for (final lineKey
            in (mergedCoverage[fileKey]!.keys.toList()
              ..sort((a, b) => int.parse(a).compareTo(int.parse(b)))))
          lineKey: mergedCoverage[fileKey]![lineKey],
      },
  };
}

dynamic mergeCodecovValues({
  required dynamic existingValue,
  required dynamic incomingValue,
}) {
  final existingHitCount = normalizeCodecovValue(existingValue);
  final incomingHitCount = normalizeCodecovValue(incomingValue);

  if (_isCoveredHitCount(existingHitCount) ||
      _isCoveredHitCount(incomingHitCount)) {
    return [
      ?existingHitCount,
      ?incomingHitCount,
    ].reduce((a, b) => a > b ? a : b);
  }

  if (_isMissedHitCount(existingHitCount) ||
      _isMissedHitCount(incomingHitCount)) {
    return 0;
  }

  if (existingValue != null) return existingValue;
  return incomingValue;
}

bool _isCoveredHitCount(int? hitCount) => hitCount != null && hitCount > 0;

bool _isMissedHitCount(int? hitCount) => hitCount == 0;

int? normalizeCodecovValue(dynamic value) {
  if (value == null) return null;
  if (value is int) return value;
  if (value is num) return value.toInt();
  if (value is String && value.contains('/')) {
    return int.parse(value.substring(0, value.indexOf('/')));
  }
  throw ArgumentError('Unsupported Codecov value: $value');
}

@visibleForTesting
String normalizeCodecovFilename(String filename) {
  final normalized = filename.replaceAll(r'\', '/');

  final repoAnchors = [
    'frb_codegen/',
    'frb_dart/',
    'frb_example/',
    'frb_macros/',
    'frb_rust/',
    'frb_utils/',
    'tools/',
    'website/',
    'book/',
    'scripts/',
  ];

  for (final anchor in repoAnchors) {
    final index = normalized.indexOf('/$anchor');
    if (index >= 0) return normalized.substring(index + 1);
    if (normalized.startsWith(anchor)) return normalized;
  }

  final cwd = path.normalize(Directory.current.path).replaceAll(r'\', '/');
  final normalizedCwd = cwd.endsWith('/') ? cwd : '$cwd/';
  if (normalized.startsWith(normalizedCwd)) {
    return normalized.substring(normalizedCwd.length);
  }

  return normalized;
}

class CodecovIgnoreMatcher {
  final List<String> patterns;

  const CodecovIgnoreMatcher({required this.patterns});

  bool matches(String filename) {
    final normalizedFilename = filename.replaceAll(r'\', '/');
    return patterns.any(
      (pattern) =>
          Glob(_normalizeIgnorePattern(pattern)).matches(normalizedFilename),
    );
  }
}

String _normalizeIgnorePattern(String pattern) {
  if (pattern.endsWith('/')) return '$pattern**';
  return pattern;
}
