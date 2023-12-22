// ignore_for_file: avoid_print

import 'dart:io';

// format: https://docs.codecov.com/docs/codecov-custom-coverage-format
Future<Map<String, dynamic>> transformCodecovReport(Map<String, dynamic> raw) async {
  return {
    'coverage': (raw['coverage'] as Map<String, dynamic>)
        .map((filename, data) => MapEntry(filename, _transformFile(filename, data))),
  };
}

Map<String, dynamic> _transformFile(String filename, Map<String, dynamic> srcData) {
  final ansData = {...srcData};
  final fileLines = File(filename).readAsStringSync().split('\n');

  var ignoring = false;
  var removeCount = 0;
  for (var i = 0; i < fileLines.length; ++i) {
    final lineContent = fileLines[i];
    final lineNumber = i + 1;

    if (lineContent.contains('frb-coverage:ignore-start')) {
      ignoring = true;
    }
    if (lineContent.contains('frb-coverage:ignore-end')) {
      ignoring = false;
    }

    if (ignoring) {
      final removed = ansData.remove(lineNumber.toString());
      if (removed != null) {
        removeCount++;
      }
    }
  }

  if (removeCount > 0) {
    print('transformCodecovReport remove $removeCount lines from $filename');
  }

  return ansData;
}
