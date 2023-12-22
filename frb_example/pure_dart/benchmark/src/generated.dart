// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:math';
import 'dart:typed_data';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/benchmark_misc.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import 'benchmark_utils.dart';
import 'protobuf_for_benchmark/protobuf_for_benchmark.pb.dart';

const _kBinaryTreeNodeName = 'HelloWorld';

List<MaybeAsyncBenchmarkBase> createBenchmarks(
    {required ScoreEmitter emitter}) {
  return [
    PrimeNumber_Na_Sync_Benchmark(number: 90000049, emitter: emitter),
    PrimeNumber_Na_Sync_Benchmark(number: 9000000001, emitter: emitter),
    PrimeNumber_Na_Sync_Benchmark(number: 900000000013, emitter: emitter),
    IntParse_Na_Sync_Benchmark(number: "0", emitter: emitter),
    IntParse_Na_Sync_Benchmark(number: "1000000000", emitter: emitter),
    Base64Encode_Na_Sync_Benchmark(len: 0, emitter: emitter),
    Base64Encode_Na_Sync_Benchmark(len: 10, emitter: emitter),
    Base64Encode_Na_Sync_Benchmark(len: 100, emitter: emitter),
    VoidFunction_Frb_Async_Benchmark(emitter: emitter),
    VoidFunction_Frb_Sync_Benchmark(emitter: emitter),
    VoidFunction_FrbSse_Sync_Benchmark(emitter: emitter),
    VoidFunction_FrbCstSse_Sync_Benchmark(emitter: emitter),
    VoidFunction_Raw_Sync_Benchmark(emitter: emitter),
    VoidFunction_Raw_Async_Benchmark(emitter: emitter),
    Bytes_Frb_Input_Async_Benchmark(len: 0, emitter: emitter),
    Bytes_Frb_Input_Async_Benchmark(len: 10000, emitter: emitter),
    Bytes_Frb_Input_Async_Benchmark(len: 1000000, emitter: emitter),
    Bytes_Frb_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Bytes_Frb_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Bytes_Frb_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Bytes_Raw_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Bytes_Raw_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Bytes_Raw_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Bytes_Frb_Output_Async_Benchmark(len: 0, emitter: emitter),
    Bytes_Frb_Output_Async_Benchmark(len: 10000, emitter: emitter),
    Bytes_Frb_Output_Async_Benchmark(len: 1000000, emitter: emitter),
    Bytes_Frb_Output_Sync_Benchmark(len: 0, emitter: emitter),
    Bytes_Frb_Output_Sync_Benchmark(len: 10000, emitter: emitter),
    Bytes_Frb_Output_Sync_Benchmark(len: 1000000, emitter: emitter),
    Bytes_Raw_Output_Async_Benchmark(len: 0, emitter: emitter),
    Bytes_Raw_Output_Async_Benchmark(len: 10000, emitter: emitter),
    Bytes_Raw_Output_Async_Benchmark(len: 1000000, emitter: emitter),
    BinaryTree_Frb_Input_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Frb_Input_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Frb_Input_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_Frb_Output_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Frb_Output_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Frb_Output_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_FrbSse_Input_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_FrbSse_Input_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_FrbSse_Input_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_FrbSse_Output_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_FrbSse_Output_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_FrbSse_Output_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_Protobuf_Input_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Protobuf_Input_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Protobuf_Input_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_Protobuf_Output_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Protobuf_Output_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Protobuf_Output_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_Json_Input_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Json_Input_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Json_Input_Sync_Benchmark(depth: 10, emitter: emitter),
    BinaryTree_Json_Output_Sync_Benchmark(depth: 0, emitter: emitter),
    BinaryTree_Json_Output_Sync_Benchmark(depth: 5, emitter: emitter),
    BinaryTree_Json_Output_Sync_Benchmark(depth: 10, emitter: emitter),
    Blob_Frb_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Frb_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Frb_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_Frb_Output_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Frb_Output_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Frb_Output_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_FrbSse_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_FrbSse_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_FrbSse_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_FrbSse_Output_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_FrbSse_Output_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_FrbSse_Output_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_Protobuf_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Protobuf_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Protobuf_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_Protobuf_Output_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Protobuf_Output_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Protobuf_Output_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_Json_Input_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Json_Input_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Json_Input_Sync_Benchmark(len: 1000000, emitter: emitter),
    Blob_Json_Output_Sync_Benchmark(len: 0, emitter: emitter),
    Blob_Json_Output_Sync_Benchmark(len: 10000, emitter: emitter),
    Blob_Json_Output_Sync_Benchmark(len: 1000000, emitter: emitter),
  ];
}

class PrimeNumber_Na_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int number;

  PrimeNumber_Na_Sync_Benchmark({
    required this.number,
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"PrimeNumber","approach":"Na","direction":null,"asynchronous":false,"arg":"$number","platform":"$currentPlatformName"}');

  @override
  void setup() {}

  @override
  void run() {
    final ans = isPrime(number);
    if (!ans) throw Exception('unexpected');
  }

  static bool isPrime(int n) {
    final sqrtN = sqrt(n);
    for (var i = 2; i <= sqrtN; ++i) {
      if (n % i == 0) return false;
    }
    return true;
  }
}

class IntParse_Na_Sync_Benchmark extends EnhancedBenchmarkBase {
  final String number;

  IntParse_Na_Sync_Benchmark({
    required this.number,
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"IntParse","approach":"Na","direction":null,"asynchronous":false,"arg":"$number","platform":"$currentPlatformName"}');

  @override
  void setup() {}

  @override
  void run() {
    final ans = int.parse(number);
    dummyValue ^= ans;
  }
}

class Base64Encode_Na_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final String setupData;
  final int len;

  Base64Encode_Na_Sync_Benchmark({
    required this.len,
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"Base64Encode","approach":"Na","direction":null,"asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

  @override
  void setup() {
    setupData = 'HelloWorld' * (len ~/ 10);
  }

  @override
  void run() {
    final ans = base64Encode(utf8.encode(setupData));
    dummyValue ^= ans.hashCode;
  }
}

class VoidFunction_Frb_Async_Benchmark extends EnhancedAsyncBenchmarkBase {
  VoidFunction_Frb_Async_Benchmark({
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"VoidFunction","approach":"Frb","direction":null,"asynchronous":true,"platform":"$currentPlatformName"}');

  @override
  Future<void> setup() async {}

  @override
  Future<void> run() async {
    await benchmarkVoidTwinNormal();
  }
}

class VoidFunction_Frb_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_Frb_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"VoidFunction","approach":"Frb","direction":null,"asynchronous":false,"platform":"$currentPlatformName"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkVoidTwinSync();
  }
}

class VoidFunction_FrbSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_FrbSse_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"VoidFunction","approach":"FrbSse","direction":null,"asynchronous":false,"platform":"$currentPlatformName"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkVoidTwinSyncSse();
  }
}

class VoidFunction_FrbCstSse_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_FrbCstSse_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"VoidFunction","approach":"FrbCstSse","direction":null,"asynchronous":false,"platform":"$currentPlatformName"}');

  @override
  void setup() {}

  @override
  void run() {
    benchmarkVoidSemiSerialize();
  }
}

class VoidFunction_Raw_Sync_Benchmark extends EnhancedBenchmarkBase {
  VoidFunction_Raw_Sync_Benchmark({
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"VoidFunction","approach":"Raw","direction":null,"asynchronous":false,"platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"VoidFunction","approach":"Raw","direction":null,"asynchronous":true,"platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Frb","direction":"Input","asynchronous":true,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Frb","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Raw","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Frb","direction":"Output","asynchronous":true,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Frb","direction":"Output","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Bytes","approach":"Raw","direction":"Output","asynchronous":true,"arg":"$len","platform":"$currentPlatformName"}') {
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

class BinaryTree_Frb_Input_Sync_Benchmark extends EnhancedBenchmarkBase {
  late final BenchmarkBinaryTreeTwinSync setupData;
  final int depth;

  BinaryTree_Frb_Input_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"BinaryTree","approach":"Frb","direction":"Input","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"Frb","direction":"Output","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"FrbSse","direction":"Input","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"FrbSse","direction":"Output","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"Protobuf","direction":"Input","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

  @override
  void setup() {
    setupData = _createTreeProtobuf(depth);
  }

  @override
  void run() {
    benchmarkBinaryTreeInputProtobufTwinSync(raw: setupData.writeToBuffer());
  }

  static BinaryTreeProtobuf _createTreeProtobuf(int depth) {
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
}

class BinaryTree_Protobuf_Output_Sync_Benchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTree_Protobuf_Output_Sync_Benchmark({
    required this.depth,
    super.emitter,
  }) : super(
            '{"area":"PureDart","task":"BinaryTree","approach":"Protobuf","direction":"Output","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"Json","direction":"Input","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"BinaryTree","approach":"Json","direction":"Output","asynchronous":false,"arg":"$depth","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Frb","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Frb","direction":"Output","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"FrbSse","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"FrbSse","direction":"Output","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Protobuf","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Protobuf","direction":"Output","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Json","direction":"Input","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
            '{"area":"PureDart","task":"Blob","approach":"Json","direction":"Output","asynchronous":false,"arg":"$len","platform":"$currentPlatformName"}');

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
