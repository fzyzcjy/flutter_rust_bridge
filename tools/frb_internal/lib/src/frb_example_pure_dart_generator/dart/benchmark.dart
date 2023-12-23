// NOTE: Currently it still contains a lot of duplicates (because it was
// migrated from manual code). But when adding more tests, we can refactor and avoid it.
import 'dart:convert';

import 'package:recase/recase.dart';

const _kArea = 'PureDart';

String generateBenchmark() {
  final benchmarks = [
    ..._benchmarkMisc(),
    ..._benchmarkVoidFunction(),
    ..._benchmarkBytes(),
    ..._benchmarkBinaryTree(),
    ..._benchmarkBlob(),
  ];

  return '''
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

List<MaybeAsyncBenchmarkBase> createBenchmarks({required ScoreEmitter emitter}) {
  return [
    ${benchmarks.expand(
            (bench) => bench.argValues.isEmpty
                ? ['${bench.className}(emitter: emitter),\n']
                : [
                    for (final argValue in bench.argValues)
                      '${bench.className}(${bench.args.single.name}: $argValue, emitter: emitter),\n'
                  ],
          ).join("")}
  ];
}

${benchmarks.map((e) => '$e\n').join("")}
  ''';
}

class _TypedName {
  final String type;
  final String name;

  const _TypedName(this.type, this.name);
}

enum _Approach {
  frb,
  frbSse,
  frbCstSse,
  raw,
  protobuf,
  json,
  na,
}

enum _Direction {
  input,
  output,
}

class _Benchmark {
  final String task;
  final _Direction? direction;
  final _Approach approach;
  final bool asynchronous;
  final String? setupDataType;
  final List<_TypedName> args;
  final List<String> argValues;
  final String setup;
  final String run;
  final String extra;
  final String Function(String className, String benchmarkName)? raw;

  const _Benchmark({
    required this.task,
    this.direction,
    required this.approach,
    required this.asynchronous,
    this.setupDataType,
    this.args = const [],
    this.argValues = const [],
    this.setup = '',
    required this.run,
    this.extra = '',
    this.raw,
  });

  String get approachName => ReCase(approach.name).pascalCase;

  String? get directionName =>
      direction == null ? null : ReCase(direction!.name).pascalCase;

  String get className =>
      '${task}_$approachName${directionName != null ? "_$directionName" : ""}_${asynchronous ? "Async" : "Sync"}_Benchmark';

  @override
  String toString() {
    assert(args.isNotEmpty == argValues.isNotEmpty);

    final benchName = jsonEncode({
      'area': _kArea,
      'task': task,
      'approach': approachName,
      'direction': directionName,
      'asynchronous': asynchronous,
      if (args.isNotEmpty) 'arg': '\$${args.single.name}',
      'platform': '\$currentPlatformName',
    });

    final String classInside;
    if (raw != null) {
      classInside = raw!(className, benchName);
    } else {
      final functionSetup = asynchronous
          ? 'Future<void> setup() async { $setup }'
          : 'void setup() { $setup }';

      final functionRun = asynchronous
          ? 'Future<void> run() async { $run }'
          : 'void run() { $run }';

      classInside = '''
      ${setupDataType == null ? "" : "late final $setupDataType setupData;"}
      ${args.map((arg) => 'final ${arg.type} ${arg.name};\n').join('')}
      
      $className({
        ${args.map((arg) => 'required this.${arg.name},\n').join('')}
        super.emitter,
      }) : super('$benchName');
      
      @override
      $functionSetup

      @override
      $functionRun
      
      $extra
    ''';
    }

    return '''
class $className extends Enhanced${asynchronous ? "Async" : ""}BenchmarkBase {
  $classInside
}
  ''';
  }
}

List<_Benchmark> _benchmarkMisc() {
  return [
    // For a list of primes: http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
    const _Benchmark(
      task: 'PrimeNumber',
      approach: _Approach.na,
      asynchronous: false,
      args: [_TypedName('int', 'number')],
      argValues: ['90000049', '9000000001', '900000000013'],
      run: '''
        final ans = isPrime(number);
        if (!ans) throw Exception('unexpected');
      ''',
      extra: '''
        static bool isPrime(int n) {
          final sqrtN = sqrt(n);
          for (var i = 2; i <= sqrtN; ++i) {
            if (n % i == 0) return false;
          }
          return true;
        }
      ''',
    ),
    const _Benchmark(
      task: 'IntParse',
      approach: _Approach.na,
      asynchronous: false,
      args: [_TypedName('String', 'number')],
      argValues: ['"0"', '"1000000000"'],
      run: '''
        final ans = int.parse(number);
        dummyValue ^= ans;
      ''',
    ),
    const _Benchmark(
      task: 'Base64Encode',
      approach: _Approach.na,
      asynchronous: false,
      args: [_TypedName('int', 'len')],
      argValues: ['0', '10', '100'],
      setupDataType: 'String',
      setup: "setupData = 'HelloWorld' * (len ~/ 10);",
      run: '''
        final ans = base64Encode(utf8.encode(setupData));
        dummyValue ^= ans.hashCode;
      ''',
    ),
  ];
}

List<_Benchmark> _benchmarkVoidFunction() {
  const task = 'VoidFunction';

  return [
    const _Benchmark(
      task: task,
      approach: _Approach.frb,
      asynchronous: true,
      run: 'await benchmarkVoidTwinNormal();',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.frb,
      asynchronous: false,
      run: 'benchmarkVoidTwinSync();',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.frbSse,
      asynchronous: false,
      run: 'benchmarkVoidTwinSyncSse();',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.frbCstSse,
      asynchronous: false,
      run: 'benchmarkVoidSemiSerialize();',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.raw,
      asynchronous: false,
      run: 'rawWire.benchmark_raw_void_sync();',
    ),
    // For example:
    // https://github.com/isar/isar/blob/95e1f02c274bb4bb80f98c1a42ddf33f3690a50c/packages/isar/lib/src/impl/isar_impl.dart#L351
    const _Benchmark(
      task: task,
      approach: _Approach.raw,
      asynchronous: true,
      run: '''
        await Isolate.run(() async {
          // This library loading may not be optimal, just a rough test
          final wire = RustLibWire.fromExternalLibrary(await loadExternalLibrary(
              RustLib.kDefaultExternalLibraryLoaderConfig));
          wire.benchmark_raw_void_sync();
        });
      ''',
    ),
  ];
}

List<_Benchmark> _benchmarkBytes() {
  const task = 'Bytes';
  const args = [_TypedName('int', 'len')];
  const argValues = ['0', '10000', '1000000'];

  return [
    for (final asynchronous in [true, false])
      _Benchmark(
        task: task,
        approach: _Approach.frb,
        direction: _Direction.input,
        asynchronous: asynchronous,
        args: args,
        argValues: argValues,
        setupDataType: 'Uint8List',
        setup: 'setupData = Uint8List(len);',
        run:
            '${asynchronous ? "await" : ""} benchmarkInputBytesTwin${asynchronous ? "Normal" : "Sync"}(bytes: setupData);',
      ),
    const _Benchmark(
      task: task,
      approach: _Approach.raw,
      direction: _Direction.input,
      asynchronous: false,
      args: args,
      argValues: argValues,
      setupDataType: 'Uint8List',
      setup: 'setupData = Uint8List(len);',
      run: '''
        final raw = rawWire.benchmark_raw_new_list_prim_u_8(setupData.length);
        raw.ptr.asTypedList(raw.len).setAll(0, setupData);
        final ans = rawWire.benchmark_raw_input_bytes(raw);
        if (ans != 0) throw Exception();
      ''',
    ),
    for (final asynchronous in [true, false])
      _Benchmark(
        task: task,
        approach: _Approach.frb,
        direction: _Direction.output,
        asynchronous: asynchronous,
        args: args,
        argValues: argValues,
        run:
            '${asynchronous ? "await" : ""} benchmarkOutputBytesTwin${asynchronous ? "Normal" : "Sync"}(size: len);',
      ),
    _Benchmark(
      task: task,
      approach: _Approach.raw,
      direction: _Direction.output,
      asynchronous: true,
      args: args,
      argValues: argValues,
      raw: (className, benchmarkName) => '''
        final receivePort = RawReceivePort();
        late final sendPort = receivePort.sendPort.nativePort;
        final int len;
        final completers = <int, Completer<Uint8List>>{};
        var nextId = 1;

        $className({required this.len, super.emitter})
            : super('$benchmarkName') {
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
      ''',
      run: '',
    ),
  ];
}

List<_Benchmark> _benchmarkBinaryTree() {
  const task = 'BinaryTree';
  const args = [_TypedName('int', 'depth')];
  const argValues = ['0', '5', '10'];

  return [
    for (final sse in [false, true]) ...[
      _Benchmark(
        task: task,
        approach: sse ? _Approach.frbSse : _Approach.frb,
        direction: _Direction.input,
        asynchronous: false,
        args: args,
        argValues: argValues,
        setupDataType: 'BenchmarkBinaryTreeTwinSync${sse ? "Sse" : ""}',
        setup: 'setupData = _createTree(depth);',
        run:
            'benchmarkBinaryTreeInputTwinSync${sse ? "Sse" : ""}(tree: setupData);',
        extra: '''
          static BenchmarkBinaryTreeTwinSync${sse ? "Sse" : ""} _createTree(int depth) {
            if (depth == 0) {
              return BenchmarkBinaryTreeTwinSync${sse ? "Sse" : ""}(
                name: _kBinaryTreeNodeName,
                left: null,
                right: null,
              );
            }
            return BenchmarkBinaryTreeTwinSync${sse ? "Sse" : ""}(
              name: _kBinaryTreeNodeName,
              left: _createTree(depth - 1),
              right: _createTree(depth - 1),
            );
          }
        ''',
      ),
      _Benchmark(
        task: task,
        approach: sse ? _Approach.frbSse : _Approach.frb,
        direction: _Direction.output,
        asynchronous: false,
        args: args,
        argValues: argValues,
        run:
            'benchmarkBinaryTreeOutputTwinSync${sse ? "Sse" : ""}(depth: depth);',
      ),
    ],
    const _Benchmark(
      task: task,
      approach: _Approach.protobuf,
      direction: _Direction.input,
      asynchronous: false,
      args: args,
      argValues: argValues,
      setupDataType: 'BinaryTreeProtobuf',
      setup: 'setupData = _createTreeProtobuf(depth);',
      run:
          'benchmarkBinaryTreeInputProtobufTwinSync(raw: setupData.writeToBuffer());',
      extra: '''
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
      ''',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.protobuf,
      direction: _Direction.output,
      asynchronous: false,
      args: args,
      argValues: argValues,
      run: '''
        final raw = benchmarkBinaryTreeOutputProtobufTwinSync(depth: depth);
        final proto = BinaryTreeProtobuf.fromBuffer(raw);
        dummyValue ^= proto.hashCode;
      ''',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.json,
      direction: _Direction.input,
      asynchronous: false,
      args: args,
      argValues: argValues,
      setupDataType: 'BenchmarkBinaryTreeTwinSync',
      setup:
          'setupData = BinaryTree_Frb_Input_Sync_Benchmark._createTree(depth);',
      run:
          'benchmarkBinaryTreeInputJsonTwinSync(raw: jsonEncode(setupData, toEncodable: _toJson));',
      extra: '''
        // Normally use `json_serializable`, but we only use for benchmark so manually write
        static Map<String, dynamic> _toJson(dynamic tree) => {
              'name': tree.name,
              'left': tree.left,
              'right': tree.right,
            };
      ''',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.json,
      direction: _Direction.output,
      asynchronous: false,
      args: args,
      argValues: argValues,
      run: '''
    final raw = benchmarkBinaryTreeOutputJsonTwinSync(depth: depth);
    // TODO: Should use json_serialize to further generate Dart objects
    // Otherwise this comparison is unfair (JSON does fewer amount of work)
    final json = jsonDecode(raw);
    dummyValue ^= json.hashCode;
      ''',
    ),
  ];
}

List<_Benchmark> _benchmarkBlob() {
  const task = 'Blob';
  const args = [_TypedName('int', 'len')];
  const argValues = ['0', '10000', '1000000'];

  String setupDataSimple({required bool sse}) => '''
        setupData = BenchmarkBlobTwinSync${sse ? "Sse" : ""}(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        );
      ''';

  return [
    for (final sse in [false, true]) ...[
      _Benchmark(
        task: task,
        approach: sse ? _Approach.frbSse : _Approach.frb,
        direction: _Direction.input,
        asynchronous: false,
        args: args,
        argValues: argValues,
        setupDataType: 'BenchmarkBlobTwinSync${sse ? "Sse" : ""}',
        setup: setupDataSimple(sse: sse),
        run: 'benchmarkBlobInputTwinSync${sse ? "Sse" : ""}(blob: setupData);',
      ),
      _Benchmark(
        task: task,
        approach: sse ? _Approach.frbSse : _Approach.frb,
        direction: _Direction.output,
        asynchronous: false,
        args: args,
        argValues: argValues,
        run: 'benchmarkBlobOutputTwinSync${sse ? "Sse" : ""}(size: len);',
      ),
    ],
    const _Benchmark(
      task: task,
      approach: _Approach.protobuf,
      direction: _Direction.input,
      asynchronous: false,
      args: args,
      argValues: argValues,
      setupDataType: 'BlobProtobuf',
      setup: '''
        setupData = BlobProtobuf(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        );
      ''',
      run:
          'benchmarkBlobInputProtobufTwinSync(raw: setupData.writeToBuffer());',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.protobuf,
      direction: _Direction.output,
      asynchronous: false,
      args: args,
      argValues: argValues,
      run: '''
        final raw = benchmarkBlobOutputProtobufTwinSync(size: len);
        final proto = BlobProtobuf.fromBuffer(raw);
        dummyValue ^= proto.hashCode;
      ''',
    ),
    _Benchmark(
      task: task,
      approach: _Approach.json,
      direction: _Direction.input,
      asynchronous: false,
      args: args,
      argValues: argValues,
      setupDataType: 'BenchmarkBlobTwinSyncSse',
      setup: setupDataSimple(sse: true),
      run:
          'benchmarkBlobInputJsonTwinSync(raw: jsonEncode(setupData, toEncodable: _toJson));',
      extra: '''
        // Normally use `json_serializable`, but we only use for benchmark so manually write
        static Map<String, dynamic> _toJson(dynamic blob) => {
              'first': blob.first,
              'second': blob.second,
              'third': blob.third,
            };
      ''',
    ),
    const _Benchmark(
      task: task,
      approach: _Approach.json,
      direction: _Direction.output,
      asynchronous: false,
      args: args,
      argValues: argValues,
      run: '''
        final raw = benchmarkBlobOutputJsonTwinSync(size: len);
        // TODO: Should use json_serialize to further generate Dart objects
        // Otherwise this comparison is unfair (JSON does fewer amount of work)
        final json = jsonDecode(raw);
        dummyValue ^= json.hashCode;
      ''',
    ),
  ];
}
