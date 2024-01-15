import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/entrypoint.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart_test_entrypoint_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/misc_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/pde_generator.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/entrypoint.dart';

Future<void> generate() async {
  final dirPureDart =
      Directory.current.uri.resolve('../../frb_example/pure_dart/');

  await RustGenerator(
          packageRootDir: dirPureDart.resolve('rust/'), interestDir: 'src/api/')
      .generate();
  await DartGenerator(packageRootDir: dirPureDart, interestDir: 'test/api/')
      .generate();
  await generateDartTestEntrypoints(dartRoot: dirPureDart);
  await generateRustMod(dirPureDart.resolve('rust/src/api/pseudo_manual/'));

  await generatePureDartPde(dirPureDart: dirPureDart);
}
