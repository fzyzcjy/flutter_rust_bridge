// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import 'benchmark_utils.dart';
import 'protobuf_for_benchmark/protobuf_for_benchmark.pb.dart';

class Void_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  Void_Async_Benchmark({
    super.emitter,
  }) : super('Void_Async');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    await benchmarkVoidTwinNormal();
  }
}

class Void_Sync_Benchmark extends EnhancedBenchmarkBase {
  Void_Sync_Benchmark({
    super.emitter,
  }) : super('Void_Sync');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkVoidTwinSync();
  }
}

class VoidRaw_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidRaw_Sync_Benchmark({
    super.emitter,
  }) : super('VoidRaw_Sync');

  @override
  void setup() {}

  @override
  void run() {
    rawWire.benchmark_raw_void_sync();
  }
}

class VoidRawByIsolate_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  VoidRawByIsolate_Async_Benchmark({
    super.emitter,
  }) : super('VoidRawByIsolate_Async');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    await Isolate.run(() async {
      // This library loading may not be optimal, just a rough test
      final wire = RustLibWire.fromExternalLibrary(await loadExternalLibrary(
          RustLib.kDefaultExternalLibraryLoaderConfig));
      wire.benchmark_raw_void_sync();
    });
  }
}

class InputBytes_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  InputBytes_Async_Benchmark({
    required this.len,
    super.emitter,
  }) : super('InputBytes_Async_len$len');

  @override
  Future<void> setup() async {
    setupData = Uint8List(len);
  }

  @override
  Future<void> run() async {
    benchmarkInputBytesTwinNormal(bytes: setupData);
  }
}

class InputBytes_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  InputBytes_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('InputBytes_Sync_len$len');

  @override
  void setup() {
    setupData = Uint8List(len);
  }

  @override
  void run() {
    benchmarkInputBytesTwinSync(bytes: setupData);
  }
}

class InputBytesRaw_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  InputBytesRaw_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('InputBytesRaw_Sync_len$len');

  @override
  void setup() {
    setupData = Uint8List(len);
  }

  @override
  void run() {
    final raw = rawWire.benchmark_raw_new_list_prim_u_8(setupData.length);
    raw.ptr.asTypedList(raw.len).setAll(0, setupData);
    final ans = rawWire.benchmark_raw_input_bytes(raw);
    if (ans != 0) throw Exception();
  }
}

class OutputBytes_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  final int len;

  OutputBytes_Async_Benchmark({
    required this.len,
    super.emitter,
  }) : super('OutputBytes_Async_len$len');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    benchmarkOutputBytesTwinNormal(size: len);
  }
}

class OutputBytes_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  OutputBytes_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('OutputBytes_Sync_len$len');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkOutputBytesTwinSync(size: len);
  }
}

class OutputBytesRaw_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  final receivePort = RawReceivePort();
  late final sendPort = receivePort.sendPort.nativePort;
  final int len;
  final completers = <int, Completer<Uint8List>>{};
  var nextId = 1;

  OutputBytesRaw_Async_Benchmark({required this.len, super.emitter})
      : super('OutputBytesRaw_Async_len$len') {
    receivePort.handler = (dynamic response) {
      final bytes = response as Uint8List;
      final messageId = ByteData.view(bytes.buffer).getInt32(0, Endian.big);
      // indeed a sublist view of the bytes
      completers.remove(messageId)!.complete(bytes);
    };
  }

  @override
  Future<void> teardown() async {
    receivePort.close();
  }

  @override
  Future<void> run() async {
    final messageId = nextId++;
    final completer = Completer<Uint8List>();
    completers[messageId] = completer;

    rawWire.benchmark_raw_output_bytes(sendPort, messageId, len);
    final result = await completer.future;

    // sanity check
    if (result.length != len + 4) throw Exception();
  }
}

const _kBinaryTreeNodeName = 'HelloWorld';

BinaryTreeProtobuf _createTreeProtobuf(int depth) {
  if (depth == 0) {
    return BinaryTreeProtobuf(
      name: _kBinaryTreeNodeName,
      left: null,
      right: null,
    );
  }
  return BinaryTreeProtobuf(
    name: _kBinaryTreeNodeName,
    left: _createTreeProtobuf(depth - 1),
    right: _createTreeProtobuf(depth - 1),
  );
}

class BinaryTreeInput_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSync setupData;
  final int depth;

  BinaryTreeInput_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeInput_Sync_depth$depth');

  @override
  void setup() {
    setupData = _createTree(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputTwinSync(tree: setupData);
  }

  static BenchmarkBinaryTreeTwinSync _createTree(int depth) {
    if (depth == 0) {
      return BenchmarkBinaryTreeTwinSync(
        name: _kBinaryTreeNodeName,
        left: null,
        right: null,
      );
    }
    return BenchmarkBinaryTreeTwinSync(
      name: _kBinaryTreeNodeName,
      left: _createTree(depth - 1),
      right: _createTree(depth - 1),
    );
  }
}

class BinaryTreeOutput_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutput_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeOutput_Sync_depth$depth');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBinaryTreeOutputTwinSync(depth: depth);
  }
}

class BinaryTreeInputSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSyncSse setupData;
  final int depth;

  BinaryTreeInputSse_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeInputSse_Sync_depth$depth');

  @override
  void setup() {
    setupData = _createTree(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputTwinSyncSse(tree: setupData);
  }

  static BenchmarkBinaryTreeTwinSyncSse _createTree(int depth) {
    if (depth == 0) {
      return BenchmarkBinaryTreeTwinSyncSse(
        name: _kBinaryTreeNodeName,
        left: null,
        right: null,
      );
    }
    return BenchmarkBinaryTreeTwinSyncSse(
      name: _kBinaryTreeNodeName,
      left: _createTree(depth - 1),
      right: _createTree(depth - 1),
    );
  }
}

class BinaryTreeOutputSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputSse_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeOutputSse_Sync_depth$depth');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBinaryTreeOutputTwinSyncSse(depth: depth);
  }
}

class BinaryTreeInputProtobuf_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BinaryTreeProtobuf setupData;
  final int depth;

  BinaryTreeInputProtobuf_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeInputProtobuf_Sync_depth$depth');

  @override
  void setup() {
    setupData = _createTreeProtobuf(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputProtobufTwinSync(raw: setupData.writeToBuffer());
  }
}

class BinaryTreeOutputProtobuf_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputProtobuf_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeOutputProtobuf_Sync_depth$depth');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBinaryTreeOutputProtobufTwinSync(depth: depth);
    final proto = BinaryTreeProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class BinaryTreeInputJson_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSync setupData;
  final int depth;

  BinaryTreeInputJson_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeInputJson_Sync_depth$depth');

  @override
  void setup() {
    setupData = BinaryTreeInput_Sync_Benchmark._createTree(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputJsonTwinSync(
        raw: jsonEncode(setupData, toEncodable: _toJson));
  }

  // Normally use `json_serializable`, but we only use for benchmark so manually write
  static Map<String, dynamic> _toJson(dynamic tree) => {
        'name': tree.name,
        'left': tree.left,
        'right': tree.right,
      };
}

class BinaryTreeOutputJson_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputJson_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super('BinaryTreeOutputJson_Sync_depth$depth');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBinaryTreeOutputJsonTwinSync(depth: depth);
    // TODO: Should use json_serialize to further generate Dart objects
    // Otherwise this comparison is unfair (JSON does fewer amount of work)
    final json = jsonDecode(raw);
    dummyValue ^= json.hashCode;
  }
}

class BlobInput_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSync setupData;
  final int len;

  BlobInput_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobInput_Sync_len$len');

  @override
  void setup() {
    setupData = BenchmarkBlobTwinSync(
      first: Uint8List(len),
      second: Uint8List(len),
      third: Uint8List(len),
    );
  }

  @override
  void run() {
    benchmarkBlobInputTwinSync(blob: setupData);
  }
}

class BlobOutput_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutput_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobOutput_Sync_len$len');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBlobOutputTwinSync(size: len);
  }
}

class BlobInputSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSyncSse setupData;
  final int len;

  BlobInputSse_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobInputSse_Sync_len$len');

  @override
  void setup() {
    setupData = BenchmarkBlobTwinSyncSse(
      first: Uint8List(len),
      second: Uint8List(len),
      third: Uint8List(len),
    );
  }

  @override
  void run() {
    benchmarkBlobInputTwinSyncSse(blob: setupData);
  }
}

class BlobOutputSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputSse_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobOutputSse_Sync_len$len');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBlobOutputTwinSyncSse(size: len);
  }
}

class BlobInputProtobuf_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BlobProtobuf setupData;
  final int len;

  BlobInputProtobuf_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobInputProtobuf_Sync_len$len');

  @override
  void setup() {
    setupData = BlobProtobuf(
      first: Uint8List(len),
      second: Uint8List(len),
      third: Uint8List(len),
    );
  }

  @override
  void run() {
    benchmarkBlobInputProtobufTwinSync(raw: setupData.writeToBuffer());
  }
}

class BlobOutputProtobuf_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputProtobuf_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobOutputProtobuf_Sync_len$len');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBlobOutputProtobufTwinSync(size: len);
    final proto = BlobProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class BlobInputJson_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSyncSse setupData;
  final int len;

  BlobInputJson_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobInputJson_Sync_len$len');

  @override
  void setup() {
    setupData = BenchmarkBlobTwinSyncSse(
      first: Uint8List(len),
      second: Uint8List(len),
      third: Uint8List(len),
    );
  }

  @override
  void run() {
    benchmarkBlobInputJsonTwinSync(
        raw: jsonEncode(setupData, toEncodable: _toJson));
  }

  // Normally use `json_serializable`, but we only use for benchmark so manually write
  static Map<String, dynamic> _toJson(dynamic blob) => {
        'first': blob.first,
        'second': blob.second,
        'third': blob.third,
      };
}

class BlobOutputJson_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  BlobOutputJson_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super('BlobOutputJson_Sync_len$len');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBlobOutputJsonTwinSync(size: len);
    // TODO: Should use json_serialize to further generate Dart objects
    // Otherwise this comparison is unfair (JSON does fewer amount of work)
    final json = jsonDecode(raw);
    dummyValue ^= json.hashCode;
  }
}
