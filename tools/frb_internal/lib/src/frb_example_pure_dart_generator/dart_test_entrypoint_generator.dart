import 'dart:io';

import 'package:collection/collection.dart';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:path/path.dart' as path;

Future<void> generateDartTestEntrypoints(
  Package package, {
  required Uri dartRoot,
}) async {
  await _generateDartSanitizerRuntimeShutdown(package, dartRoot: dartRoot);
  await _generateDartValgrindTestEntrypoint(package, dartRoot: dartRoot);
  await _generateDartWebTestEntrypoint(package, dartRoot: dartRoot);
}

Future<void> _generateDartSanitizerRuntimeShutdown(
  Package package, {
  required Uri dartRoot,
}) async {
  final ioCode =
      '''
$_kPrelude

import 'dart:ffi' as ffi;
import 'dart:io';

Future<void> shutdownSanitizerRuntime() async {
  final nativeLibraryDir =
      Platform.environment['FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR']!;
  final dylib = ffi.DynamicLibrary.open(
    '\$nativeLibraryDir/lib${package.dartPackageName}.so',
  );
  final shutdown = dylib
      .lookupFunction<ffi.Void Function(), void Function()>(
        'frb_shutdown_sanitizer_runtime',
      );
  final pendingDropCount = dylib
      .lookupFunction<ffi.UintPtr Function(), int Function()>(
        'frb_dart_opaque_pending_drop_count',
      );
  final drainFailedDrops = dylib
      .lookupFunction<ffi.Void Function(), void Function()>(
        'frb_dart_opaque_drain_failed_drops',
      );
  shutdown();

  final deadline = DateTime.now().add(const Duration(seconds: 10));
  while (true) {
    drainFailedDrops();
    if (pendingDropCount() == 0) {
      await Future<void>.delayed(const Duration(milliseconds: 10));
      drainFailedDrops();
      if (pendingDropCount() == 0) {
        break;
      }
    }
    if (DateTime.now().isAfter(deadline)) {
      throw StateError('Timed out draining pending DartOpaque drops');
    }
    await Future<void>.delayed(Duration.zero);
  }
}
  ''';
  const stubCode =
      '''
$_kPrelude

Future<void> shutdownSanitizerRuntime() async {}
  ''';

  await _writeToFile(
    dartRoot,
    'test/dart_sanitizer_runtime_shutdown_io.dart',
    ioCode,
  );
  await _writeToFile(
    dartRoot,
    'test/dart_sanitizer_runtime_shutdown_stub.dart',
    stubCode,
  );
}

Future<void> _generateDartWebTestEntrypoint(
  Package package, {
  required Uri dartRoot,
}) async {
  final code =
      '''
$_kPrelude

import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';
import 'package:${package.dartPackageName}/src/rust/frb_generated.dart';

import 'dart_valgrind_test_entrypoint.dart' as dart_valgrind_test_entrypoint;

Future<void> main() async {
  await dartWebTestEntrypoint(() async {
    await RustLib.init();

    await dart_valgrind_test_entrypoint.callFileEntrypoints();
  });
}
  ''';

  await _writeToFile(dartRoot, 'test/dart_web_test_entrypoint.dart', code);
}

Future<void> _generateDartValgrindTestEntrypoint(
  Package package, {
  required Uri dartRoot,
}) async {
  final dirTest = dartRoot.resolve('test/');
  final dirInterest = dirTest.resolve('api/');
  final files = [
    for (final file in Directory(
      dirInterest.toFilePath(),
    ).listSync(recursive: true))
      if (file is File && path.extension(file.path) == '.dart') file.path,
  ].sorted();

  final imports = [
    for (final file in files) //
      "import '${path.relative(file, from: dirTest.toFilePath()).replaceAll(r'\', '/')}' as ${path.basenameWithoutExtension(file)};\n",
  ];
  final entrypoints = [
    for (final file in files)
      if (_shouldSkipDisposedRustAutoOpaqueArgumentTest(
        package: package,
        fileStem: path.basenameWithoutExtension(file),
      ))
        '''({bool skipRustLibInit = false}) => ${path.basenameWithoutExtension(file)}.main(
          skipRustLibInit: skipRustLibInit,
          skipDisposedRustAutoOpaqueArgumentTest: skipDisposedRustAutoOpaqueArgumentTest,
        ),\n'''
      else
        '${path.basenameWithoutExtension(file)}.main,\n',
  ];

  final code =
      '''
$_kPrelude

import 'dart:io';

import 'package:${package.dartPackageName}/src/rust/frb_generated.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';
import 'dart_sanitizer_runtime_shutdown_stub.dart'
    if (dart.library.io) 'dart_sanitizer_runtime_shutdown_io.dart';
import 'utils/test_flutter_memory_leak_utility.dart';

${imports.join("")}

Future<void> main() async {
  await RustLib.init();

  final success = await directRunTests(
    () async {
      await callFileEntrypoints(skipDisposedRustAutoOpaqueArgumentTest: true);
    },
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      PrintSink(),
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );
  print('FRB_DART_TEST_RESULT: \${success ? 'success' : 'failure'}');

  if (Platform.environment['FRB_SANITIZER_SHUTDOWN_RUNTIME'] == '1') {
    final vmService = await VmServiceUtil.create();
    if (vmService == null) {
      throw StateError('Sanitizer test requires the Dart VM service');
    }
    await vmService.gc();
    await Future<void>.delayed(const Duration(milliseconds: 10));
    await shutdownSanitizerRuntime();
    await vmService.gc();
    await Future<void>.delayed(const Duration(milliseconds: 10));
    vmService.dispose();
  }
  RustLib.dispose();
  exitCode = success ? 0 : 1;
}

Future<void> callFileEntrypoints({
  bool skipDisposedRustAutoOpaqueArgumentTest = false,
}) async {
  final entrypoints = <Future<void> Function({bool skipRustLibInit})>[
    ${entrypoints.join("")}
  ];

  for (final entrypoint in entrypoints) {
    await entrypoint(skipRustLibInit: true);
  }
}
  ''';

  await _writeToFile(dartRoot, 'test/dart_valgrind_test_entrypoint.dart', code);
}

bool _shouldSkipDisposedRustAutoOpaqueArgumentTest({
  required Package package,
  required String fileStem,
}) => switch (package) {
  Package.pureDart => {
    'rust_auto_opaque_twin_rust_async_sse_test',
    'rust_auto_opaque_twin_sync_sse_test',
  }.contains(fileStem),
  Package.pureDartPde => {
    'rust_auto_opaque_twin_rust_async_test',
    'rust_auto_opaque_twin_sync_test',
  }.contains(fileStem),
};

Future<void> _writeToFile(
  Uri dartRoot,
  String relativePath,
  String code,
) async {
  final pathOutput = dartRoot.resolve(relativePath).toFilePath();
  File(pathOutput).writeAsStringSync(code);
  await runCommand('dart', ['format', pathOutput]);
}

const _kPrelude =
    '/// NOTE: This file is auto-generated by frb_internal. Please do not manually modify it.';
