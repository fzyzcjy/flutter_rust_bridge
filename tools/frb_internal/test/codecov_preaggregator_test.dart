import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/codecov_preaggregator.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  group('normalizeCodecovFilename', () {
    test('normalizes github actions unix path', () {
      expect(
        normalizeCodecovFilename(
          '/home/runner/work/flutter_rust_bridge/flutter_rust_bridge/frb_codegen/src/binary/commands.rs',
        ),
        'frb_codegen/src/binary/commands.rs',
      );
    });

    test('normalizes github actions windows path', () {
      expect(
        normalizeCodecovFilename(
          r'D:\a\flutter_rust_bridge\flutter_rust_bridge\frb_codegen\src\binary\commands.rs',
        ),
        'frb_codegen/src/binary/commands.rs',
      );
    });
  });

  group('mergeCodecovValues', () {
    test('positive hit wins over miss', () {
      expect(mergeCodecovValues(existingValue: 0, incomingValue: 3), 3);
    });

    test('miss wins over null', () {
      expect(mergeCodecovValues(existingValue: null, incomingValue: 0), 0);
    });

    test('keeps null when there is no executable evidence', () {
      expect(
        mergeCodecovValues(existingValue: null, incomingValue: null),
        null,
      );
    });
  });

  test('preaggregateCodecovReports merges reports and computes coverage', () async {
    final tempDir = await Directory.systemTemp.createTemp();
    addTearDown(() async => tempDir.delete(recursive: true));

    final firstReport = File(path.join(tempDir.path, 'a', 'codecov.json'));
    firstReport.parent.createSync(recursive: true);
    firstReport.writeAsStringSync(
      jsonEncode({
        'coverage': {
          '/home/runner/work/flutter_rust_bridge/flutter_rust_bridge/frb_codegen/src/binary/commands.rs':
              {'12': 0, '13': null},
        },
      }),
    );

    final secondReport = File(path.join(tempDir.path, 'b', 'codecov.json'));
    secondReport.parent.createSync(recursive: true);
    secondReport.writeAsStringSync(
      jsonEncode({
        'coverage': {
          r'D:\a\flutter_rust_bridge\flutter_rust_bridge\frb_codegen\src\binary\commands.rs':
              {'12': 3, '14': 0},
        },
      }),
    );

    final result = await preaggregateCodecovReports(inputDir: tempDir.path);

    expect(result.reportCount, 2);
    expect(result.inputFileCount, 1);
    expect(result.mergedCoverage, {
      'coverage': {
        'frb_codegen/src/binary/commands.rs': {'12': 3, '13': null, '14': 0},
      },
    });
    expect(result.summary.coveredLines, 1);
    expect(result.summary.executableLines, 2);
    expect(result.summary.coveragePercent, 50);
  });
}
