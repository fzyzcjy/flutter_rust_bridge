// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member


import 'package:benchmark_harness/benchmark_harness.dart';

import 'benchmark_classes/binary_tree.dart';
import 'benchmark_classes/blob.dart';
import 'benchmark_classes/bytes.dart';
import 'benchmark_classes/compute_prime.dart';
import 'benchmark_classes/void_function.dart';
import 'benchmark_utils.dart';

List<MaybeAsyncBenchmarkBase> createBenchmarks(
    {required ScoreEmitter emitter}) {
  return [
    // compute prime
    ComputePrimeBenchmark(90000049, emitter: emitter),
    ComputePrimeBenchmark(9000000001, emitter: emitter),
    ComputePrimeBenchmark(900000000013, emitter: emitter),

    // void
    VoidAsyncBenchmark(emitter: emitter),
    VoidSyncBenchmark(emitter: emitter),
    VoidSyncRawBenchmark(emitter: emitter),
    VoidAsyncRawByIsolateBenchmark(emitter: emitter),

    //
    for (final len in [0, 10000, 1000000]) ...[
      // input bytes
      InputBytesAsyncBenchmark(len, emitter: emitter),
      InputBytesSyncBenchmark(len, emitter: emitter),
      InputBytesSyncRawBenchmark(len, emitter: emitter),

      // output bytes
      OutputBytesAsyncBenchmark(len, emitter: emitter),
      OutputBytesSyncBenchmark(len, emitter: emitter),
      OutputBytesAsyncRawBenchmark(len, emitter: emitter),

      // input blob
      BlobInputSyncBenchmark(len, emitter: emitter),
      BlobInputSyncSseBenchmark(len, emitter: emitter),
      BlobInputSyncProtobufBenchmark(len, emitter: emitter),
      BlobInputSyncJsonBenchmark(len, emitter: emitter),

      // output blob
      BlobOutputSyncBenchmark(len, emitter: emitter),
      BlobOutputSyncSseBenchmark(len, emitter: emitter),
      BlobOutputSyncProtobufBenchmark(len, emitter: emitter),
      BlobOutputSyncJsonBenchmark(len, emitter: emitter),
    ],

    for (final depth in [0, 5, 10]) ...[
      // input binary tree
      BinaryTreeInputSyncBenchmark(depth, emitter: emitter),
      BinaryTreeInputSyncSseBenchmark(depth, emitter: emitter),
      BinaryTreeInputSyncProtobufBenchmark(depth, emitter: emitter),
      BinaryTreeInputSyncJsonBenchmark(depth, emitter: emitter),

      // output binary tree
      BinaryTreeOutputSyncBenchmark(depth, emitter: emitter),
      BinaryTreeOutputSyncSseBenchmark(depth, emitter: emitter),
      BinaryTreeOutputSyncProtobufBenchmark(depth, emitter: emitter),
      BinaryTreeOutputSyncJsonBenchmark(depth, emitter: emitter),
    ],
  ];
}
