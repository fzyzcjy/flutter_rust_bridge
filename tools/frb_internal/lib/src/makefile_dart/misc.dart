import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

const kRustPackages = [
  'frb_rust',
  'frb_codegen',
  'frb_example/dart_minimal/rust',
  'frb_example/pure_dart/rust',
  // TODO `with_flutter` example
];

const kDartExamplePackages = [
  'frb_example/dart_minimal',
  'frb_example/pure_dart',
  // TODO `with_flutter` example
];

const kDartPackages = [
  'frb_dart',
  ...kDartExamplePackages,
  'frb_utils',
  'tools/frb_internal',
];

final exec = SimpleExecutor(
  env: {
    'CARGO_TERM_COLOR': 'always',
  },
  // Use project root directory
  pwd: Directory.current.parent.parent.path,
);
