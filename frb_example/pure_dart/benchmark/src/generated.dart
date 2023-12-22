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

class VoidFunction_Std_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  VoidFunction_Std_Async_Benchmark({
    super.emitter,
  }) : super(
            '{"category":"VoidFunction","approach":"Std","direction":null,"asynchronous":true}');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    await benchmarkVoidTwinNormal();
  }
}

class VoidFunction_Void_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_Void_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"category":"VoidFunction","approach":"Void","direction":null,"asynchronous":false}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkVoidTwinSync();
  }
}

class VoidFunction_Raw_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_Raw_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"category":"VoidFunction","approach":"Raw","direction":null,"asynchronous":false}');

  @override
  void setup() {}

  @override
  void run() {
    rawWire.benchmark_raw_void_sync();
  }
}

class VoidFunction_Raw_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  VoidFunction_Raw_Async_Benchmark({
    super.emitter,
  }) : super(
            '{"category":"VoidFunction","approach":"Raw","direction":null,"asynchronous":true}');

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

class Bytes_Frb_Input_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  Bytes_Frb_Input_Async_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Bytes","approach":"Frb","direction":"Input","asynchronous":true,"arg_len":"$len"}');

  @override
  Future<void> setup() async {
    setupData = Uint8List(len);
  }

  @override
  Future<void> run() async {
    await benchmarkInputBytesTwinNormal(bytes: setupData);
  }
}

class Bytes_Frb_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  Bytes_Frb_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Bytes","approach":"Frb","direction":"Input","asynchronous":false,"arg_len":"$len"}');

  @override
  void setup() {
    setupData = Uint8List(len);
  }

  @override
  void run() {
    benchmarkInputBytesTwinSync(bytes: setupData);
  }
}

class Bytes_Raw_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final Uint8List setupData;
  final int len;

  Bytes_Raw_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Bytes","approach":"Raw","direction":"Input","asynchronous":false,"arg_len":"$len"}');

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

class Bytes_Frb_Output_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  final int len;

  Bytes_Frb_Output_Async_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Bytes","approach":"Frb","direction":"Output","asynchronous":true,"arg_len":"$len"}');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    await benchmarkOutputBytesTwinNormal(size: len);
  }
}

class Bytes_Frb_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  Bytes_Frb_Output_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Bytes","approach":"Frb","direction":"Output","asynchronous":false,"arg_len":"$len"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkOutputBytesTwinSync(size: len);
  }
}

class Bytes_Raw_Output_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  final receivePort = RawReceivePort();
  late final sendPort = receivePort.sendPort.nativePort;
  final int len;
  final completers = <int, Completer<Uint8List>>{};
  var nextId = 1;

  Bytes_Raw_Output_Async_Benchmark({required this.len, super.emitter})
      : super(
            '{"category":"Bytes","approach":"Raw","direction":"Output","asynchronous":true,"arg_len":"$len"}') {
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

class BinaryTree_Frb_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSync setupData;
  final int depth;

  BinaryTree_Frb_Input_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Frb","direction":"Input","asynchronous":false,"arg_depth":"$depth"}');

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

class BinaryTree_Frb_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTree_Frb_Output_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Frb","direction":"Output","asynchronous":false,"arg_depth":"$depth"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBinaryTreeOutputTwinSync(depth: depth);
  }
}

class BinaryTree_FrbSse_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSyncSse setupData;
  final int depth;

  BinaryTree_FrbSse_Input_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"FrbSse","direction":"Input","asynchronous":false,"arg_depth":"$depth"}');

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

class BinaryTree_FrbSse_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTree_FrbSse_Output_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"FrbSse","direction":"Output","asynchronous":false,"arg_depth":"$depth"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBinaryTreeOutputTwinSyncSse(depth: depth);
  }
}

class BinaryTree_Protobuf_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BinaryTreeProtobuf setupData;
  final int depth;

  BinaryTree_Protobuf_Input_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Protobuf","direction":"Input","asynchronous":false,"arg_depth":"$depth"}');

  @override
  void setup() {
    setupData = _createTreeProtobuf(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputProtobufTwinSync(raw: setupData.writeToBuffer());
  }
}

class BinaryTree_Protobuf_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTree_Protobuf_Output_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Protobuf","direction":"Output","asynchronous":false,"arg_depth":"$depth"}');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBinaryTreeOutputProtobufTwinSync(depth: depth);
    final proto = BinaryTreeProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class BinaryTree_Json_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSync setupData;
  final int depth;

  BinaryTree_Json_Input_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Json","direction":"Input","asynchronous":false,"arg_depth":"$depth"}');

  @override
  void setup() {
    setupData = BinaryTree_Frb_Input_Sync_Benchmark._createTree(depth);
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

class BinaryTree_Json_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTree_Json_Output_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"category":"BinaryTree","approach":"Json","direction":"Output","asynchronous":false,"arg_depth":"$depth"}');

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

class Blob_Frb_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSync setupData;
  final int len;

  Blob_Frb_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Frb","direction":"Input","asynchronous":false,"arg_len":"$len"}');

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

class Blob_Frb_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  Blob_Frb_Output_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Frb","direction":"Output","asynchronous":false,"arg_len":"$len"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBlobOutputTwinSync(size: len);
  }
}

class Blob_FrbSse_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSyncSse setupData;
  final int len;

  Blob_FrbSse_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"FrbSse","direction":"Input","asynchronous":false,"arg_len":"$len"}');

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

class Blob_FrbSse_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  Blob_FrbSse_Output_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"FrbSse","direction":"Output","asynchronous":false,"arg_len":"$len"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkBlobOutputTwinSyncSse(size: len);
  }
}

class Blob_Protobuf_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BlobProtobuf setupData;
  final int len;

  Blob_Protobuf_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Protobuf","direction":"Input","asynchronous":false,"arg_len":"$len"}');

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

class Blob_Protobuf_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  Blob_Protobuf_Output_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Protobuf","direction":"Output","asynchronous":false,"arg_len":"$len"}');

  @override
  void setup() {}

  @override
  void run() {
    final raw = benchmarkBlobOutputProtobufTwinSync(size: len);
    final proto = BlobProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class Blob_Json_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBlobTwinSyncSse setupData;
  final int len;

  Blob_Json_Input_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Json","direction":"Input","asynchronous":false,"arg_len":"$len"}');

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

class Blob_Json_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int len;

  Blob_Json_Output_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"category":"Blob","approach":"Json","direction":"Output","asynchronous":false,"arg_len":"$len"}');

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
