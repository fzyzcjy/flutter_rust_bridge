import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/generator.dart';

void generate() {
  final dartRoot = Directory.current.uri.resolve('../../frb_example/pure_dart');
  if (!Directory(dartRoot.toFilePath()).existsSync()) throw StateError('dartRoot=$dartRoot does not exist');

  generateRust(rustRoot: dartRoot.resolve('rust'));
  generateDart(dartRoot: dartRoot);
}
