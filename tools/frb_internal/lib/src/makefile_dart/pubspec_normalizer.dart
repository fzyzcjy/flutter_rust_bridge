import 'dart:io';

import 'package:path/path.dart' as path;

const _kPubHostMirror = 'pub.flutter-io.cn';
const _kPubHostCanonical = 'pub.dev';

// Keep Flutter 3.44 templates from raising checked-in app SDK floors.
const _kPubspecYamlSdkReplacements = <String, String>{
  '  sdk: ^3.12.0': '  sdk: ^3.11.0',
};

// Keep generated lockfile floors aligned with repo and OHOS package policy.
const _kPubspecLockSdkReplacements = <String, String>{
  '  dart: ">=3.12.0 <4.0.0"': '  dart: ">=3.11.0 <4.0.0"',
  '  dart: ">=3.10.0-0 <4.0.0"': '  dart: ">=3.9.2 <4.0.0"',
};

void normalizePubspecs({
  required String repoRootPath,
  required Iterable<String> packages,
}) {
  for (final package in packages) {
    normalizePubspecFiles(packageRoot: path.join(repoRootPath, package));
  }
}

void normalizePubspecFiles({required String packageRoot}) {
  _normalizeFile(
    path.join(packageRoot, 'pubspec.yaml'),
    normalizePubspecYamlText,
  );
  _normalizeFile(
    path.join(packageRoot, 'pubspec.lock'),
    normalizePubspecLockText,
  );
}

String normalizePubspecYamlText(String text) =>
    _replaceAll(text, _kPubspecYamlSdkReplacements);

String normalizePubspecLockText(String text) => _replaceAll(
  text.replaceAll(_kPubHostMirror, _kPubHostCanonical),
  _kPubspecLockSdkReplacements,
);

String _replaceAll(String text, Map<String, String> replacements) {
  var output = text;
  for (final entry in replacements.entries) {
    output = output.replaceAll(entry.key, entry.value);
  }
  return output;
}

void _normalizeFile(String filePath, String Function(String) normalize) {
  final file = File(filePath);
  if (!file.existsSync()) return;

  final original = file.readAsStringSync();
  final modified = normalize(original);
  if (modified == original) return;

  file.writeAsStringSync(modified);
}
