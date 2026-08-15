import 'dart:convert';
import 'dart:io';

import 'package:path/path.dart' as path;

Future<bool> hasOhosArchiveLister() async =>
    await _findOhosArchiveLister() != null;

Future<List<String>> listOhosHapEntries(String hapPath) async {
  final lister = await _findOhosArchiveLister();
  if (lister == null) {
    throw StateError(
      'Cannot inspect OHOS HAP $hapPath because neither `jar` nor `unzip` '
      'is available. Install a full JDK 17 or add `unzip` to PATH.',
    );
  }
  final arguments = switch (lister.executable) {
    'jar' => ['tf', hapPath],
    'unzip' => ['-Z1', hapPath],
    _ => throw StateError(
      'Unsupported OHOS HAP archive lister: ${lister.executable}',
    ),
  };
  final result = await Process.run(lister.executable, arguments);
  if (result.exitCode != 0) {
    throw StateError(
      'Failed to inspect OHOS HAP $hapPath with ${lister.executable}: '
      '${result.stderr}',
    );
  }
  return (result.stdout as String).split(RegExp(r'\r?\n'));
}

Future<String> readOhosHapBundleName(String hapPath) async {
  const entry = 'pack.info';
  final lister = await _findOhosArchiveLister();
  if (lister == null) {
    throw StateError(
      'Cannot inspect OHOS HAP $hapPath because neither `jar` nor `unzip` '
      'is available. Install a full JDK 17 or add `unzip` to PATH.',
    );
  }

  String packInfo;
  if (lister.executable == 'unzip') {
    final result = await Process.run('unzip', ['-p', hapPath, entry]);
    if (result.exitCode != 0) {
      throw StateError(
        'Failed to read $entry from OHOS HAP $hapPath with unzip: '
        '${result.stderr}',
      );
    }
    packInfo = result.stdout as String;
  } else if (lister.executable == 'jar') {
    final temporaryDirectory = Directory.systemTemp.createTempSync(
      'frb_ohos_hap_metadata_',
    );
    try {
      final result = await Process.run('jar', [
        'xf',
        hapPath,
        entry,
      ], workingDirectory: temporaryDirectory.path);
      final extracted = File(path.join(temporaryDirectory.path, entry));
      if (result.exitCode != 0 || !extracted.existsSync()) {
        throw StateError(
          'Failed to read $entry from OHOS HAP $hapPath with jar: '
          '${result.stderr}',
        );
      }
      packInfo = extracted.readAsStringSync();
    } finally {
      temporaryDirectory.deleteSync(recursive: true);
    }
  } else {
    throw StateError(
      'Unsupported OHOS HAP archive lister: ${lister.executable}',
    );
  }

  try {
    return ohosHapBundleNameForTesting(packInfo);
  } on FormatException catch (error) {
    throw StateError('Invalid $entry in OHOS HAP $hapPath: $error');
  }
}

Future<({String executable, List<String> arguments})?>
_findOhosArchiveLister() async {
  for (final candidate in ohosArchiveListerCandidatesForTesting()) {
    try {
      final result = await Process.run(
        candidate.executable,
        candidate.arguments,
      );
      if (result.exitCode == 0) return candidate;
    } on ProcessException {
      // Try the next supported archive lister.
    }
  }
  return null;
}

String ohosHapBundleNameForTesting(String packInfo) {
  final decoded = jsonDecode(packInfo);
  if (decoded is! Map<String, dynamic>) {
    throw const FormatException('pack.info root must be a JSON object');
  }
  final summary = decoded['summary'];
  final app = summary is Map<String, dynamic> ? summary['app'] : null;
  final bundleName = app is Map<String, dynamic> ? app['bundleName'] : null;
  if (bundleName is! String || bundleName.trim().isEmpty) {
    throw const FormatException(
      'pack.info does not contain summary.app.bundleName',
    );
  }
  return bundleName;
}

List<({String executable, List<String> arguments})>
ohosArchiveListerCandidatesForTesting() => const [
  (executable: 'jar', arguments: ['--version']),
  (executable: 'unzip', arguments: ['-v']),
];

bool ohosHapContainsRustLibraryForTesting(
  Iterable<String> entries, {
  required String expectedLibrary,
}) => entries.any(
  (entry) =>
      entry.trim().replaceAll('\\', '/') == 'libs/arm64-v8a/$expectedLibrary',
);
