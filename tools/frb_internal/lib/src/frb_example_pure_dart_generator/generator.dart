import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/entrypoint.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart_test_entrypoint_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/misc_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/pde_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/entrypoint.dart';

Future<void> generate() async {
  final dirPureDart =
      Directory.current.uri.resolve('../../frb_example/pure_dart/');

  await generateForPackage(dartRoot: dirPureDart);
  await generatePureDartPde(dirPureDart: dirPureDart);
}

Future<void> generateForPackage({required Uri dartRoot}) async {
  await RustGenerator(
          packageRootDir: dartRoot.resolve('rust/'), interestDir: 'src/api/')
      .generate();
  await DartGenerator(packageRootDir: dartRoot, interestDir: 'test/api/')
      .generate();
  await generateDartTestEntrypoints(dartRoot: dartRoot);
  await generateRustMod(dartRoot.resolve('rust/src/api/pseudo_manual/'));
}
