// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';

import '../benchmark_utils.dart';

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
    final raw = rawWire.benchmark_raw_new_list_prim_u_8(bytes.length);
    raw.ptr.asTypedList(raw.len).setAll(0, bytes);
    final ans = rawWire.benchmark_raw_input_bytes(raw);
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

    rawWire.benchmark_raw_output_bytes(sendPort, messageId, len);
    final result = await completer.future;

    // sanity check
    if (result.length != len + 4) throw Exception();
  }
}
