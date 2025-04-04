import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

const kRustPackagesAllowWeb = [
  'frb_rust',
  'frb_example/dart_minimal/rust',
  'frb_example/pure_dart/rust',
  'frb_example/pure_dart_pde/rust',
  'frb_example/dart_build_rs/rust',
  'frb_example/deliberate_bad/rust',
  'frb_example/integrate_third_party/rust',
  'frb_example/flutter_via_create/rust',
  'frb_example/flutter_via_integrate/rust',
  'frb_example/flutter_package/rust',
  'frb_example/rust_ui_counter',
  'frb_example/rust_ui_todo_list',
  'frb_example/gallery/rust',
];

const kRustPackagesDisallowWeb = [
  'frb_codegen',
  'frb_macros',
];

const kRustPackages = [
  ...kRustPackagesAllowWeb,
  ...kRustPackagesDisallowWeb,
];

const kDartExampleIntegratePackages = [
  'frb_example/flutter_via_create',
  'frb_example/flutter_via_integrate',
  'frb_example/flutter_package'
];

const kDartExamplePackages = [
  'frb_example/dart_minimal',
  'frb_example/pure_dart',
  'frb_example/pure_dart_pde',
  'frb_example/dart_build_rs',
  'frb_example/deliberate_bad',
  'frb_example/integrate_third_party',
  'frb_example/rust_ui_counter/ui',
  'frb_example/rust_ui_todo_list/ui',
  ...kDartExampleIntegratePackages,
  'frb_example/gallery',
];

const kDartNonExamplePackages = [
  'frb_dart',
  'frb_utils',
  'tools/frb_internal',
];

const kDartPackages = [
  ...kDartNonExamplePackages,
  ...kDartExamplePackages,
];

enum DartMode { dart, flutter }

const kDartModeOfPackage = {
  'frb_dart': DartMode.dart,
  'frb_utils': DartMode.dart,
  'tools/frb_internal': DartMode.dart,
  'frb_example/dart_minimal': DartMode.dart,
  'frb_example/pure_dart': DartMode.dart,
  'frb_example/pure_dart_pde': DartMode.dart,
  'frb_example/dart_build_rs': DartMode.dart,
  'frb_example/deliberate_bad': DartMode.dart,
  'frb_example/integrate_third_party': DartMode.flutter,
  'frb_example/flutter_via_create': DartMode.flutter,
  'frb_example/flutter_via_integrate': DartMode.flutter,
  'frb_example/flutter_package': DartMode.flutter,
  'frb_example/flutter_package/example': DartMode.flutter,
  'frb_example/rust_ui_counter/ui': DartMode.flutter,
  'frb_example/rust_ui_todo_list/ui': DartMode.flutter,
  'frb_example/gallery': DartMode.flutter,
};

const kBuildWebPackageReplacer = {
  'frb_example/flutter_package/example': 'frb_example/flutter_package',
};

final exec = SimpleExecutor(
  env: {
    'CARGO_TERM_COLOR': 'always',
  },
  // Use project root directory
  pwd: Directory.current.parent.parent.uri.toFilePath(),
);

/// Normally, `dart pub get` will be run automatically when executing `dart test` and so on.
/// But there seems to be a bug currently.
/// Temporary workaround before https://github.com/dart-lang/sdk/issues/54160 is fixed.
Future<void> runPubGetIfNotRunYet(String package) async {
  final mode = kDartModeOfPackage[package]!;

  await _runPubGetIfNotRunYetRaw(package, mode);

  for (final extraPackage in [
    '$package/rust_builder/cargokit/build_tool',
    '$package/cargokit/build_tool',
  ]) {
    await _runPubGetIfNotRunYetRaw(extraPackage, mode);
  }
}

Future<void> _runPubGetIfNotRunYetRaw(String package, DartMode mode) async {
  final dirPackage = '${exec.pwd}/$package';
  if ((await Directory(dirPackage).exists()) &&
      (!await Directory('$dirPackage/.dart_tool').exists())) {
    await runPubGet(package, mode);
  }
}

Future<void> runPubGet(String package, DartMode mode) async {
  final cmd = switch (mode) {
    DartMode.dart => 'dart --enable-experiment=native-assets',
    DartMode.flutter => 'flutter',
  };
  await exec('$cmd pub get', relativePwd: package);
}
