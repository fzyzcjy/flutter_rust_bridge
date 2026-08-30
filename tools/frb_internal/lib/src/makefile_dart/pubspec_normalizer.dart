import 'dart:io';

import 'package:path/path.dart' as path;

const _kPubHostMirror = 'pub.flutter-io.cn';
const _kPubHostCanonical = 'pub.dev';
const _kPackagesUsingPrimarySdkFloor = {'tools/frb_internal'};

// Keep Flutter 3.47 templates from raising checked-in app SDK floors.
const _kPubspecYamlSdkReplacements = <String, String>{
  '  sdk: ^3.13.0': '  sdk: ^3.11.0',
  '  sdk: ^3.12.0': '  sdk: ^3.11.0',
};

// Keep generated lockfile floors aligned with repo and OHOS package policy.
const _kPubspecLockSdkReplacements = <String, String>{
  '  dart: ">=3.13.0 <4.0.0"': '  dart: ">=3.11.0 <4.0.0"',
  '  dart: ">=3.12.0 <4.0.0"': '  dart: ">=3.11.0 <4.0.0"',
  '  dart: ">=3.11.0-0 <4.0.0"': '  dart: ">=3.9.2 <4.0.0"',
  '  dart: ">=3.10.0-0 <4.0.0"': '  dart: ">=3.9.2 <4.0.0"',
};

void normalizePubspecs({
  required String repoRootPath,
  required Iterable<String> packages,
}) {
  for (final package in packages) {
    normalizePubspecFiles(
      packageRoot: path.join(repoRootPath, package),
      normalizeSdkFloor: !_kPackagesUsingPrimarySdkFloor.contains(package),
    );
  }
}

void normalizePubspecFiles({
  required String packageRoot,
  bool normalizeSdkFloor = true,
}) {
  _normalizeFile(
    path.join(packageRoot, 'pubspec.yaml'),
    normalizeSdkFloor ? normalizePubspecYamlText : _normalizePubspecYamlHost,
  );
  _normalizeFile(
    path.join(packageRoot, 'pubspec.lock'),
    normalizeSdkFloor ? normalizePubspecLockText : _normalizePubspecLockHost,
  );
}

String normalizePubspecYamlText(String text) =>
    _replaceAll(text, _kPubspecYamlSdkReplacements);

String normalizePubspecLockText(String text) =>
    _replaceAll(_normalizePubspecLockHost(text), _kPubspecLockSdkReplacements);

String _normalizePubspecYamlHost(String text) => text;

String _normalizePubspecLockHost(String text) =>
    text.replaceAll(_kPubHostMirror, _kPubHostCanonical);

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
