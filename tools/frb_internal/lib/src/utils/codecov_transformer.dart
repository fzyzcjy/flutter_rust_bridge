// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:meta/meta.dart';

void transformCodecovReport(String path) {
  print('transformCodecovReport act on $path');
  simpleActFile(
    path,
    (raw) => jsonEncode(_transformCodecovReportInner(jsonDecode(raw))),
  );
}

// format: https://docs.codecov.com/docs/codecov-custom-coverage-format
Map<String, dynamic> _transformCodecovReportInner(Map<String, dynamic> raw) {
  return {
    'coverage': (raw['coverage'] as Map<String, dynamic>).map(
      (filename, data) => MapEntry(filename, _transformFile(filename, data)),
    ),
  };
}

Map<String, dynamic> _transformFile(
  String filename,
  Map<String, dynamic> srcData,
) {
  final fileLines = File(filename).readAsStringSync().split('\n');

  var ans = srcData;
  ans = _transformByMimickingLcovInfo(ans);
  ans = _transformByCodeComments(fileLines, ans);
  ans = _transformByPatterns(fileLines, ans);
  return ans;
}

Map<String, dynamic> _transformByMimickingLcovInfo(Map<String, dynamic> raw) {
  return raw.map((key, rawValue) {
    // mimic lcov.info feature (lcov.info is used in Dart side)
    final ansValue = () {
      if (rawValue is! String || !rawValue.contains('/')) return rawValue;
      return int.parse(rawValue.substring(0, rawValue.indexOf('/')));
    }();
    return MapEntry(key, ansValue);
  });
}

Map<String, dynamic> _transformByCodeComments(
  List<String> fileLines,
  Map<String, dynamic> raw,
) {
  final ans = {...raw};

  var ignoring = false;
  // var removeCount = 0;
  for (var i = 0; i < fileLines.length; ++i) {
    final lineContent = fileLines[i];
    final lineNumber = i + 1;

    if (lineContent.contains('frb-coverage:ignore-start')) {
      ignoring = true;
    }

    if (ignoring) {
      ans.remove(lineNumber.toString());
      // if (removed != null) {
      // removeCount++;
      // }
    }

    if (lineContent.contains('frb-coverage:ignore-end')) {
      ignoring = false;
    }
  }

  // if (removeCount > 0) {
  //   print('transformCodecovReport remove $removeCount lines from $filename');
  // }

  return ans;
}

// see the test file for details of this regex
final _kIgnoreLineRegex = RegExp(
  r'^\s*(|#\[derive\(.*\)\]|#\[[^\]]*\]|[\[({]|[)}]\?.*|//.*|[\])}]+(?:\.into\(\))?[,;]?|\$\(|\)\*)\s*$',
);

final _kStructFieldDeclarationRegex = RegExp(
  r'^\s*(pub(?:\([^)]*\))?\s+)?[A-Za-z_][A-Za-z0-9_]*\s*:\s*[^=].*,\s*$',
);

final _kSimpleDestructuredFieldRegex = RegExp(
  r'^\s*[A-Za-z_][A-Za-z0-9_]*,\s*$',
);

final _kNamedDestructuredFieldPrefixRegex = RegExp(
  r'^\s*[A-Za-z_][A-Za-z0-9_]*:\s*$',
);

final _kDotDotPatternRegex = RegExp(r'^\s*\.\.,?\s*$');

final _kLetDestructureStartRegex = RegExp(r'^\s*let\b(?!.*=).*\{\s*$');

final _kVariantDestructureStartRegex = RegExp(
  r'^\s*[A-Za-z_][A-Za-z0-9_:<>]*\([^)]*\{\s*$',
);

final _kDestructureEndRegex = RegExp(r'}\s*(=|if .*=>|=>)');

final _kConstructorStartRegex = RegExp(
  r'^\s*(?:Self|Some|Ok|Err|Acc|vec!|[A-Z][A-Za-z0-9_]*(?:::[A-Z][A-Za-z0-9_]*)*)\s*(?:\(|\{)\s*$',
);

final _kFieldConstructorStartRegex = RegExp(
  r'^\s*[a-z_][A-Za-z0-9_]*:\s*(?:Some|Ok|Err|Self|Acc|[A-Z][A-Za-z0-9_]*(?:::[A-Z][A-Za-z0-9_]*)*)\s*(?:\(|\{)\s*$',
);

final _kWrappedConstructorStartRegex = RegExp(
  r'^\s*(?:Some|Ok|Err)\([^)]*(?:[A-Z][A-Za-z0-9_]*(?:::[A-Z][A-Za-z0-9_]*)*)\s*(?:\(|\{)\s*$',
);

final _kChainContinuationRegex = RegExp(
  r'^\s*\.[A-Za-z_][A-Za-z0-9_]*(?:\([^)]*\))?\s*$',
);

final _kMacroStartRegex = RegExp(r'^\s*[A-Za-z_][A-Za-z0-9_:]*!\s*\{\s*$');

final _kStaticRefStartRegex = RegExp(
  r'^\s*(?:pub\([^)]*\)\s+)?static ref [A-Za-z_][A-Za-z0-9_]*: .*(?:=\s*$|;\s*$)',
);

final _kStringArgumentRegex = RegExp(r'^\s*(?:[rb]|br)?#*".*"#*\s*,\s*$');

final _kBoolOrNumberArgumentRegex = RegExp(
  r'^\s*(?:true|false|-?\d+(?:\.\d+)?)\s*,\s*$',
);

@visibleForTesting
bool shouldKeepLine(String line, {bool isFormatCallNoise = false}) =>
    !isFormatCallNoise &&
    !_kIgnoreLineRegex.hasMatch(line) &&
    !_kStructFieldDeclarationRegex.hasMatch(line);

@visibleForTesting
Set<int> computeFormatCallNoiseLines(List<String> fileLines) {
  final ans = <int>{};

  var pendingFormatCall = false;
  var insideFormatCall = false;
  var formatCallDepth = 0;
  var insideString = false;
  var escaping = false;

  for (var i = 0; i < fileLines.length; i++) {
    final line = fileLines[i];
    final lineNumber = i + 1;
    var lineContainsFormatCallBody = false;
    var index = 0;

    while (index < line.length) {
      final char = line[index];

      if (insideFormatCall) {
        lineContainsFormatCallBody = true;
      }

      if (escaping) {
        escaping = false;
        index++;
        continue;
      }

      if (insideString) {
        if (char == r'\') {
          escaping = true;
        } else if (char == '"') {
          insideString = false;
        }
        index++;
        continue;
      }

      if (char == '/' && index + 1 < line.length && line[index + 1] == '/') {
        break;
      }

      if (!insideFormatCall &&
          !pendingFormatCall &&
          line.startsWith('format!', index)) {
        pendingFormatCall = true;
        index += 'format!'.length;
        continue;
      }

      if (char == '"') {
        insideString = true;
        index++;
        continue;
      }

      if (pendingFormatCall && char == '(') {
        pendingFormatCall = false;
        insideFormatCall = true;
        formatCallDepth = 1;
        lineContainsFormatCallBody = true;
        index++;
        continue;
      }
      if (insideFormatCall) {
        if (char == '(') {
          formatCallDepth++;
        } else if (char == ')') {
          formatCallDepth--;
          if (formatCallDepth == 0) {
            insideFormatCall = false;
          }
        }
      }

      index++;
    }

    if (lineContainsFormatCallBody) {
      ans.add(lineNumber);
    }
  }

  return ans;
}

@visibleForTesting
Map<String, dynamic> transformCodecovFileCoverageForTest(
  List<String> fileLines,
  Map<String, dynamic> raw,
) {
  return postprocessCodecovFileCoverage(fileLines, raw);
}

Map<String, dynamic> postprocessCodecovFileCoverage(
  List<String> fileLines,
  Map<String, dynamic> raw,
) {
  var ans = raw;
  ans = _transformByCodeComments(fileLines, ans);
  ans = _transformByPatterns(fileLines, ans);
  return ans;
}

Map<String, dynamic> _transformByPatterns(
  List<String> fileLines,
  Map<String, dynamic> raw,
) {
  final formatCallNoiseLines = computeFormatCallNoiseLines(fileLines);
  final structuralNoiseLines = computeStructuralNoiseLines(fileLines);

  return raw.map((key, value) {
    final lineNumber = int.parse(key);
    final fileLine = fileLines[lineNumber - 1];
    return MapEntry(
      key,
      ((value is int && value > 0) ||
              !structuralNoiseLines.contains(lineNumber) &&
              shouldKeepLine(
                fileLine,
                isFormatCallNoise: formatCallNoiseLines.contains(lineNumber),
              ))
          ? value
          : null,
    );
  });
}

@visibleForTesting
Set<int> computeStructuralNoiseLines(List<String> fileLines) {
  final ans = <int>{};
  ans.addAll(_computeMultilineStructFieldLines(fileLines));
  ans.addAll(_computeMultilineDestructureLines(fileLines));
  ans.addAll(_computeMultilineDeriveLines(fileLines));
  ans.addAll(_computeMultilineStaticRefLines(fileLines));
  ans.addAll(_computeInlineStructuralNoiseLines(fileLines));
  return ans;
}

Set<int> _computeMultilineStructFieldLines(List<String> fileLines) {
  final ans = <int>{};

  var insideStruct = false;
  var braceDepth = 0;
  for (var i = 0; i < fileLines.length; i++) {
    final line = fileLines[i];
    final trimmed = line.trim();
    final lineNumber = i + 1;

    if (!insideStruct &&
        RegExp(r'^\s*(pub(?:\([^)]*\))?\s+)?struct\b.*\{\s*$').hasMatch(line)) {
      insideStruct = true;
      braceDepth = _computeBraceDelta(line);
      continue;
    }

    if (!insideStruct) continue;

    if (_kStructFieldDeclarationRegex.hasMatch(line)) {
      ans.add(lineNumber);
    }

    braceDepth += _computeBraceDelta(line);
    if (braceDepth <= 0 || trimmed == '}') {
      insideStruct = false;
      braceDepth = 0;
    }
  }

  return ans;
}

Set<int> _computeMultilineDestructureLines(List<String> fileLines) {
  final ans = <int>{};

  var insideDestructure = false;
  for (var i = 0; i < fileLines.length; i++) {
    final line = fileLines[i];
    final trimmed = line.trim();
    final lineNumber = i + 1;

    if (!insideDestructure &&
        (_kLetDestructureStartRegex.hasMatch(line) ||
            _kVariantDestructureStartRegex.hasMatch(line))) {
      insideDestructure = true;
      ans.add(lineNumber);
      continue;
    }

    if (!insideDestructure) continue;

    if (_kSimpleDestructuredFieldRegex.hasMatch(line) ||
        _kNamedDestructuredFieldPrefixRegex.hasMatch(line) ||
        _kDotDotPatternRegex.hasMatch(line) ||
        trimmed == '),' ||
        trimmed == '})' ||
        trimmed == '}),' ||
        trimmed == '}' ||
        trimmed == '},' ||
        trimmed.endsWith('{')) {
      ans.add(lineNumber);
    }

    if (_kDestructureEndRegex.hasMatch(trimmed)) {
      ans.add(lineNumber);
      insideDestructure = false;
    }
  }

  return ans;
}

Set<int> _computeMultilineDeriveLines(List<String> fileLines) {
  final ans = <int>{};

  var insideDerive = false;
  for (var i = 0; i < fileLines.length; i++) {
    final trimmed = fileLines[i].trim();
    final lineNumber = i + 1;

    if (!insideDerive && trimmed.startsWith('#[derive(') && !trimmed.endsWith(')]')) {
      insideDerive = true;
      ans.add(lineNumber);
      continue;
    }

    if (!insideDerive) continue;

    ans.add(lineNumber);
    if (trimmed.endsWith(')]')) {
      insideDerive = false;
    }
  }

  return ans;
}

Set<int> _computeMultilineStaticRefLines(List<String> fileLines) {
  final ans = <int>{};

  var insideStaticRef = false;
  for (var i = 0; i < fileLines.length; i++) {
    final trimmed = fileLines[i].trim();
    final lineNumber = i + 1;

    if (!insideStaticRef &&
        RegExp(r'^\s*(?:pub\([^)]*\)\s+)?static ref [A-Za-z_][A-Za-z0-9_]*: .*=+\s*$')
            .hasMatch(fileLines[i])) {
      insideStaticRef = true;
      continue;
    }

    if (!insideStaticRef) continue;

    ans.add(lineNumber);
    if (trimmed.endsWith(';')) {
      insideStaticRef = false;
    }
  }

  return ans;
}

Set<int> _computeInlineStructuralNoiseLines(List<String> fileLines) {
  final ans = <int>{};

  for (var i = 0; i < fileLines.length; i++) {
    final line = fileLines[i];
    final lineNumber = i + 1;

    if (_kConstructorStartRegex.hasMatch(line) ||
        _kFieldConstructorStartRegex.hasMatch(line) ||
        _kWrappedConstructorStartRegex.hasMatch(line) ||
        _kChainContinuationRegex.hasMatch(line) ||
        _kMacroStartRegex.hasMatch(line) ||
        _kStaticRefStartRegex.hasMatch(line) ||
        _kStringArgumentRegex.hasMatch(line) ||
        _kBoolOrNumberArgumentRegex.hasMatch(line) ||
        _kNamedDestructuredFieldPrefixRegex.hasMatch(line)) {
      ans.add(lineNumber);
    }
  }

  return ans;
}

int _computeBraceDelta(String line) {
  var delta = 0;
  for (final char in line.split('')) {
    if (char == '{') delta++;
    if (char == '}') delta--;
  }
  return delta;
}
