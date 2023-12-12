// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:convert';
import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';

import '../benchmark_utils.dart';
import '../protobuf_for_benchmark/protobuf_for_benchmark.pb.dart';

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

BenchmarkBlobTwinSyncSse _createBlob(int len) => BenchmarkBlobTwinSyncSse(
      first: Uint8List(len),
      second: Uint8List(len),
      third: Uint8List(len),
    );

class BlobInputSyncSseBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBlobTwinSyncSse blob;

  BlobInputSyncSseBenchmark(int len, {super.emitter})
      : blob = _createBlob(len),
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

class BlobInputSyncProtobufBenchmark extends EnhancedBenchmarkBase {
  final BlobProtobuf blob;

  BlobInputSyncProtobufBenchmark(int len, {super.emitter})
      : blob = BlobProtobuf(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        ),
        super('BlobInputSyncProtobufBenchmark_Len$len');

  @override
  void run() => benchmarkBlobInputProtobufTwinSync(raw: blob.writeToBuffer());
}

class BlobOutputSyncProtobufBenchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputSyncProtobufBenchmark(this.len, {super.emitter})
      : super('BlobOutputSyncProtobuf_Len$len');

  @override
  void run() {
    final raw = benchmarkBlobOutputProtobufTwinSync(size: len);
    final proto = BlobProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class BlobInputSyncJsonBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBlobTwinSyncSse blob;

  BlobInputSyncJsonBenchmark(int len, {super.emitter})
      : blob = _createBlob(len),
        super('BlobInputSyncJsonBenchmark_Len$len');

  @override
  void run() => benchmarkBlobInputJsonTwinSync(raw: jsonEncode(blob));
}

class BlobOutputSyncJsonBenchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputSyncJsonBenchmark(this.len, {super.emitter})
      : super('BlobOutputSyncJson_Len$len');

  @override
  void run() {
    final raw = benchmarkBlobOutputJsonTwinSync(size: len);
    // TODO: Should use json_serialize to further generate Dart objects
    // Otherwise this comparison is unfair (JSON does fewer amount of work)
    final json = jsonDecode(raw);
    dummyValue ^= json.hashCode;
  }
}
