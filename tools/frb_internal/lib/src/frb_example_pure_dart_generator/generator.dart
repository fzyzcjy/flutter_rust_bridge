import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/entrypoint.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart_test_entrypoint_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/entrypoint.dart';

Future<void> generate() async {
  final dartRoot =
      Directory.current.uri.resolve('../../frb_example/pure_dart/');
  if (!Directory(dartRoot.toFilePath()).existsSync()) {
    throw StateError('dartRoot=$dartRoot does not exist');
  }

  await RustGenerator(
          packageRootDir: dartRoot.resolve('rust/'), interestDir: 'src/api/')
      .generate();
  await DartGenerator(packageRootDir: dartRoot, interestDir: 'test/api/')
      .generate();
  await generateDartTestEntrypoints(dartRoot: dartRoot);
}
