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
  r'^\s*(#\[derive\(.*\)\]|[)}]\?.*|//.*|\};?)\s*$',
);

@visibleForTesting
bool shouldKeepLine(
  String line, {
  bool isFormatMultilineStringNoise = false,
}) => !isFormatMultilineStringNoise && !_kIgnoreLineRegex.hasMatch(line);

@visibleForTesting
Set<int> computeFormatMultilineStringNoiseLines(List<String> fileLines) {
  final ans = <int>{};

  var pendingFormatMultilineString = false;
  var insideFormatMultilineString = false;

  for (var i = 0; i < fileLines.length; i++) {
    final line = fileLines[i];
    final trimmedLine = line.trim();
    final lineNumber = i + 1;

    if (!insideFormatMultilineString && line.contains('format!(')) {
      pendingFormatMultilineString = true;
      continue;
    }

    if (!insideFormatMultilineString &&
        pendingFormatMultilineString &&
        _startsFormatMultilineString(trimmedLine)) {
      insideFormatMultilineString = true;
    }

    if (insideFormatMultilineString) {
      ans.add(lineNumber);
      if (_hasOddUnescapedDoubleQuotes(line)) {
        insideFormatMultilineString = false;
        pendingFormatMultilineString = false;
      }
      continue;
    }

    if (pendingFormatMultilineString && _endsFormatCall(trimmedLine)) {
      pendingFormatMultilineString = false;
    }
  }

  return ans;
}

@visibleForTesting
Map<String, dynamic> transformCodecovFileCoverageForTest(
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
  final formatMultilineStringNoiseLines = computeFormatMultilineStringNoiseLines(
    fileLines,
  );

  return raw.map((key, value) {
    final lineNumber = int.parse(key);
    final fileLine = fileLines[lineNumber - 1];
    return MapEntry(
      key,
      ((value is int && value > 0) ||
              shouldKeepLine(
                fileLine,
                isFormatMultilineStringNoise:
                    formatMultilineStringNoiseLines.contains(lineNumber),
              ))
          ? value
          : null,
    );
  });
}

bool _startsFormatMultilineString(String trimmedLine) {
  return trimmedLine == '"' ||
      (trimmedLine.endsWith('"') &&
          _hasOddUnescapedDoubleQuotes(trimmedLine) &&
          !trimmedLine.startsWith('//'));
}

bool _hasOddUnescapedDoubleQuotes(String line) {
  var quoteCount = 0;
  var escaped = false;

  for (final rune in line.runes) {
    final char = String.fromCharCode(rune);
    if (escaped) {
      escaped = false;
      continue;
    }

    if (char == r'\') {
      escaped = true;
      continue;
    }

    if (char == '"') {
      quoteCount++;
    }
  }

  return quoteCount.isOdd;
}

bool _endsFormatCall(String trimmedLine) {
  return trimmedLine == ')' ||
      trimmedLine == '),' ||
      trimmedLine.endsWith(');') ||
      trimmedLine.endsWith('),');
}
