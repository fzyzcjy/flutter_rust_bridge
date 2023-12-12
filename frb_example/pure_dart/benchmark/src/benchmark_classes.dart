// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:math';
import 'dart:typed_data';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

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

late final RustLibWire _wire = (RustLib.instance.api as RustLibApiImpl).wire;

// For a list of primes: http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
class ComputePrimeBenchmark extends EnhancedBenchmarkBase {
  final int number;

  const ComputePrimeBenchmark(this.number, {super.emitter})
      : super('ComputePrime_Number$number');

  @override
  void run() {
    final ans = isPrime(number);
    if (!ans) throw Exception('unexpected');
  }

  bool isPrime(int n) {
    final sqrtN = sqrt(n);
    for (var i = 2; i <= sqrtN; ++i) {
      if (n % i == 0) return false;
    }
    return true;
  }
}

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
  void run() => _wire.benchmark_raw_void_sync();
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

class InputBytesAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  final Uint8List bytes;

  InputBytesAsyncBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesAsync_Len$len');

  @override
  Future<void> run() async => benchmarkInputBytesTwinNormal(bytes: bytes);
}

class InputBytesSyncBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesSync_Len$len');

  @override
  void run() => benchmarkInputBytesTwinSync(bytes: bytes);
}

class InputBytesSyncRawBenchmark extends EnhancedBenchmarkBase {
  final Uint8List bytes;

  InputBytesSyncRawBenchmark(int len, {super.emitter})
      : bytes = Uint8List(len),
        super('InputBytesSyncRaw_Len$len');

  @override
  void run() {
    final raw = _wire.benchmark_raw_new_list_prim_u_8(bytes.length);
    raw.ptr.asTypedList(raw.len).setAll(0, bytes);
    final ans = _wire.benchmark_raw_input_bytes(raw);
    if (ans != 0) throw Exception();
  }
}

class OutputBytesAsyncBenchmark extends EnhancedAsyncBenchmarkBase {
  final int len;

  OutputBytesAsyncBenchmark(this.len, {super.emitter})
      : super('OutputBytesAsync_Len$len');

  @override
  Future<void> run() async => benchmarkOutputBytesTwinNormal(size: len);
}

class OutputBytesSyncBenchmark extends EnhancedBenchmarkBase {
  final int len;

  OutputBytesSyncBenchmark(this.len, {super.emitter})
      : super('OutputBytesSync_Len$len');

  @override
  void run() => benchmarkOutputBytesTwinSync(size: len);
}

class OutputBytesAsyncRawBenchmark extends EnhancedAsyncBenchmarkBase {
  final receivePort = RawReceivePort();
  late final sendPort = receivePort.sendPort.nativePort;
  final int len;
  final completers = <int, Completer<Uint8List>>{};
  var nextId = 1;

  OutputBytesAsyncRawBenchmark(this.len, {super.emitter})
      : super('OutputBytesAsyncRaw_Len$len') {
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

    _wire.benchmark_raw_output_bytes(sendPort, messageId, len);
    final result = await completer.future;

    // sanity check
    if (result.length != len + 4) throw Exception();
  }
}

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
