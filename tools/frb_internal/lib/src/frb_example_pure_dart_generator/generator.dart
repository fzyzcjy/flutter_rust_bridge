import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/generator.dart';

Future<void> generate() async {
  final dartRoot = Directory.current.uri.resolve('../../frb_example/pure_dart/');
  if (!Directory(dartRoot.toFilePath()).existsSync()) throw StateError('dartRoot=$dartRoot does not exist');

  await generateRust(rustRoot: dartRoot.resolve('rust/'));
  await generateDart(dartRoot: dartRoot);
}
