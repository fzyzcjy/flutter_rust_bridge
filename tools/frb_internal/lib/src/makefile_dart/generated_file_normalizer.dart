import 'dart:io';

import 'package:path/path.dart' as path;

void normalizeGeneratedFiles({required String repoRootPath}) {
  _normalizeFile(
    path.join(
      repoRootPath,
      'frb_example/flutter_via_create/ohos/entry/oh-package.json5',
    ),
    normalizeSingleTrailingNewlineText,
  );
}

String normalizeSingleTrailingNewlineText(String text) {
  var output = text;
  while (output.endsWith('\n\n')) {
    output = output.substring(0, output.length - 1);
  }
  return output.endsWith('\n') ? output : '$output\n';
}

void _normalizeFile(String filePath, String Function(String) normalize) {
  final file = File(filePath);
  if (!file.existsSync()) return;

  final original = file.readAsStringSync();
  final modified = normalize(original);
  if (modified == original) return;

  file.writeAsStringSync(modified);
}
