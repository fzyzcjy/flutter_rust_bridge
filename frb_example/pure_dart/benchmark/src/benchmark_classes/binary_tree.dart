// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';

import '../benchmark_utils.dart';

const _kBinaryTreeNodeName = 'HelloWorld';

class BinaryTreeInputSyncBenchmark extends EnhancedBenchmarkBase {
  final BenchmarkBinaryTreeTwinSync tree;

  BinaryTreeInputSyncBenchmark(int depth, {super.emitter})
      : tree = _createTree(depth),
        super('BinaryTreeInputSync_Depth$depth');

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
