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
bool shouldKeepLine(String line, {bool isFormatCallNoise = false}) =>
    !isFormatCallNoise && !_kIgnoreLineRegex.hasMatch(line);

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

  return raw.map((key, value) {
    final lineNumber = int.parse(key);
    final fileLine = fileLines[lineNumber - 1];
    return MapEntry(
      key,
      ((value is int && value > 0) ||
              shouldKeepLine(
                fileLine,
                isFormatCallNoise: formatCallNoiseLines.contains(lineNumber),
              ))
          ? value
          : null,
    );
  });
}
