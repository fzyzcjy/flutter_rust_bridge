// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:convert';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';

import '../benchmark_utils.dart';
import '../protobuf_for_benchmark/protobuf_for_benchmark.pb.dart';

const _kBinaryTreeNodeName = 'HelloWorld';

BenchmarkBinaryTreeTwinSync _createTree(int depth) {
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

class BinaryTreeInputSyncBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBinaryTreeTwinSync tree;

  BinaryTreeInputSyncBenchmark(int depth, {super.emitter})
      : tree = _createTree(depth),
        super('BinaryTreeInputSync_Depth$depth');

  @override
  void run() => benchmarkBinaryTreeInputTwinSync(tree: tree);
}

class BinaryTreeOutputSyncBenchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputSyncBenchmark(this.depth, {super.emitter})
      : super('BinaryTreeOutputSync_Depth$depth');

  @override
  void run() => benchmarkBinaryTreeOutputTwinSync(depth: depth);
}

class BinaryTreeInputSyncSseBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBinaryTreeTwinSyncSse tree;

  BinaryTreeInputSyncSseBenchmark(int depth, {super.emitter})
      : tree = _createTree(depth),
        super('BinaryTreeInputSyncSse_Depth$depth');

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

  @override
  void run() => benchmarkBinaryTreeInputTwinSyncSse(tree: tree);
}

class BinaryTreeOutputSyncSseBenchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputSyncSseBenchmark(this.depth, {super.emitter})
      : super('BinaryTreeOutputSyncSse_Depth$depth');

  @override
  void run() => benchmarkBinaryTreeOutputTwinSyncSse(depth: depth);
}

class BinaryTreeInputSyncProtobufBenchmark extends EnhancedBenchmarkBase {
  final BinaryTreeProtobuf tree;

  BinaryTreeInputSyncProtobufBenchmark(int depth, {super.emitter})
      : tree = _createTreeProtobuf(depth),
        super('BinaryTreeInputSyncProtobuf_Depth$depth');

  @override
  void run() => benchmarkBinaryTreeInputTwinSyncProtobuf(tree: tree);
}

class BinaryTreeOutputSyncProtobufBenchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputSyncProtobufBenchmark(this.depth, {super.emitter})
      : super('BinaryTreeOutputSyncProtobuf_Depth$depth');

  @override
  void run() {
    final raw = benchmarkBinaryTreeOutputTwinSyncProtobuf(depth: depth);
    final proto = BinaryTreeProtobuf.fromBuffer(raw);
    dummyValue ^= proto.hashCode;
  }
}

class BinaryTreeInputSyncJsonBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBinaryTreeTwinSync tree;

  BinaryTreeInputSyncJsonBenchmark(int depth, {super.emitter})
      : tree = _createTree(depth),
        super('BinaryTreeInputSyncJson_Depth$depth');

  @override
  void run() => benchmarkBinaryTreeInputTwinSyncJson(tree: tree);
}

class BinaryTreeOutputSyncJsonBenchmark extends EnhancedBenchmarkBase {
  final int depth;

  BinaryTreeOutputSyncJsonBenchmark(this.depth, {super.emitter})
      : super('BinaryTreeOutputSyncJson_Depth$depth');

  @override
  void run() {
    final raw = benchmarkBinaryTreeOutputTwinSyncJson(depth: depth);
    // TODO: Should use json_serialize to further generate Dart objects
    // Otherwise this comparison is unfair (JSON does fewer amount of work)
    final json = jsonDecode(raw);
    dummyValue ^= json.hashCode;
  }
}
