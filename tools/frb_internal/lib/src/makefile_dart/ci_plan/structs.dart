import 'package:json_annotation/json_annotation.dart';

part 'structs.g.dart';

class CiPlan {
  final Set<String> enabledJobs;
  final Map<String, Map<String, Object?>> matrixByJob;

  const CiPlan({required this.enabledJobs, required this.matrixByJob});
}

typedef CiPlanOutput = Map<String, CiPlanJobOutput>;

@JsonSerializable(createFactory: false, includeIfNull: false)
class CiPlanJobOutput {
  final bool enable;
  final Map<String, Object?>? matrix;

  const CiPlanJobOutput({required this.enable, this.matrix});

  Map<String, Object?> toJson() => _$CiPlanJobOutputToJson(this);
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
    if (body.trim().isEmpty) {
      throw FormatException(
        'CI filter `$original` does not specify matrix filters.',
      );
    }
    final filters = <String, Set<String>>{};
    for (final rawCondition in splitTopLevel(body, ',')) {
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

List<String> splitTopLevel(String text, String separator) {
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
