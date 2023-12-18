// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:isolate';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import '../benchmark_utils.dart';

class VoidAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  const VoidAsyncBenchmark({super.emitter}) : super('VoidAsync');

  @override
  Future<void> run() async => benchmarkVoidTwinNormal();
}

class VoidSyncBenchmark extends EnhancedBenchmarkBase {
  const VoidSyncBenchmark({super.emitter}) : super('VoidSync');

  @override
  void run() => benchmarkVoidTwinSync();
}

class VoidSyncRawBenchmark extends EnhancedBenchmarkBase {
  VoidSyncRawBenchmark({super.emitter}) : super('VoidSyncRaw');

  @override
  void run() => rawWire.benchmark_raw_void_sync();
}

// For example:
// https://github.com/isar/isar/blob/95e1f02c274bb4bb80f98c1a42ddf33f3690a50c/packages/isar/lib/src/impl/isar_impl.dart#L351
class VoidAsyncRawByIsolateBenchmark extends EnhancedAsyncBenchmarkBase {
  VoidAsyncRawByIsolateBenchmark({super.emitter})
      : super('VoidAsyncRawByIsolate');

  @override
  Future<void> run() async => await Isolate.run(() async {
        // This library loading may not be optimal, just a rough test
        final wire = RustLibWire.fromExternalLibrary(await loadExternalLibrary(
            RustLib.kDefaultExternalLibraryLoaderConfig));
        wire.benchmark_raw_void_sync();
      });
}
