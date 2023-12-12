// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';

import '../benchmark_utils.dart';

class BlobInputSyncBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBlobTwinSync blob;

  BlobInputSyncBenchmark(int len, {super.emitter})
      : blob = BenchmarkBlobTwinSync(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        ),
        super('BlobInputSyncBenchmark_Len$len');

  @override
  void run() => benchmarkBlobInputTwinSync(blob: blob);
}

class BlobOutputSyncBenchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputSyncBenchmark(this.len, {super.emitter})
      : super('BlobOutputSync_Len$len');

  @override
  void run() => benchmarkBlobOutputTwinSync(size: len);
}

class BlobInputSyncSseBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBlobTwinSyncSse blob;

  BlobInputSyncSseBenchmark(int len, {super.emitter})
      : blob = BenchmarkBlobTwinSyncSse(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        ),
        super('BlobInputSyncSseBenchmark_Len$len');

  @override
  void run() => benchmarkBlobInputTwinSyncSse(blob: blob);
}

class BlobOutputSyncSseBenchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputSyncSseBenchmark(this.len, {super.emitter})
      : super('BlobOutputSyncSse_Len$len');

  @override
  void run() => benchmarkBlobOutputTwinSyncSse(size: len);
}
