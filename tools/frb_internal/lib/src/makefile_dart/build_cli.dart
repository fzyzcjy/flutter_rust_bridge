import 'dart:io';

import 'package:path/path.dart' as path;

// build_cli 2.3 requires Dart >=3.12, while frb_dart still allows 3.9.2, so
// pub stays on 2.2.13. That release clashes with source_helper 1.4's
// toStringNonNullable extension.
const _kBuildCliDependency =
    '  build_cli: ^2.2.5\n'
    '  source_helper: ">=1.3.10 <1.4.0"';
const _kDisabledBuildCliDependency =
    '  # Temporarily remove before https://github.com/kevmoo/build_cli/issues/168 is fixed\n'
    '  # build_cli: ^2.2.5';
const _kBuildCliPackages = ['frb_dart', 'frb_utils'];

Future<void> withBuildCliEnabled({
  required String repoRootPath,
  required Future<void> Function() action,
}) async {
  final originalContents = <File, String?>{
    for (final package in _kBuildCliPackages)
      File(path.join(repoRootPath, package, 'pubspec.yaml')): null,
  };
  for (final pubspec in originalContents.keys) {
    final contents = pubspec.readAsStringSync();
    if (!contents.contains(_kDisabledBuildCliDependency)) {
      throw StateError(
        'Expected disabled build_cli state not found in ${pubspec.path}.',
      );
    }
    originalContents[pubspec] = contents;
  }

  try {
    for (final entry in originalContents.entries) {
      entry.key.writeAsStringSync(
        entry.value!.replaceFirst(
          _kDisabledBuildCliDependency,
          _kBuildCliDependency,
        ),
      );
    }
    await action();
  } finally {
    for (final entry in originalContents.entries) {
      entry.key.writeAsStringSync(entry.value!);
    }
  }
}
