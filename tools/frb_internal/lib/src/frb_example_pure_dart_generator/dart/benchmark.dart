// NOTE: Currently it still contains a lot of duplicates (because it was
// migrated from manual code). But when adding more tests, we can refactor and avoid it.
String generateBenchmark() {
  final chunks = [
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
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/benchmark_api_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.io.dart';

import 'benchmark_utils.dart';
import 'protobuf_for_benchmark/protobuf_for_benchmark.pb.dart';

${chunks.join("\n")}
  ''';
}

class _TypedName {
  final String type;
  final String name;

  const _TypedName(this.type, this.name);
}

String _generate({
  required String category,
  String? direction,
  required String approach,
  required bool asynchronous,
  String? setupDataType,
  List<_TypedName> args = const [],
  String setup = '',
  required String run,
  String extra = '',
  String Function(String className, String benchmarkName)? raw,
}) {
  final partialName =
      '${category}_$approach${direction != null ? "_$direction" : ""}_${asynchronous ? "Async" : "Sync"}';
  final className = '${partialName}_Benchmark';
  final benchName =
      '$partialName${args.map((arg) => "_${arg.name}\$${arg.name}").join("")}';

  final String classInside;
  if (raw != null) {
    classInside = raw(className, benchName);
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

List<String> _benchmarkVoidFunction() {
  const category = 'VoidFunction';

  return [
    _generate(
      category: category,
      approach: 'Std',
      asynchronous: true,
      run: 'await benchmarkVoidTwinNormal();',
    ),
    _generate(
      category: category,
      approach: 'Void',
      asynchronous: false,
      run: 'benchmarkVoidTwinSync();',
    ),
    _generate(
      category: category,
      approach: 'Raw',
      asynchronous: false,
      run: 'rawWire.benchmark_raw_void_sync();',
    ),
    // For example:
    // https://github.com/isar/isar/blob/95e1f02c274bb4bb80f98c1a42ddf33f3690a50c/packages/isar/lib/src/impl/isar_impl.dart#L351
    _generate(
      category: category,
      approach: 'Raw',
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

List<String> _benchmarkBytes() {
  const category = 'Bytes';
  const args = [_TypedName('int', 'len')];

  return [
    for (final asynchronous in [true, false])
      _generate(
        category: category,
        approach: 'Frb',
        direction: 'Input',
        asynchronous: asynchronous,
        args: args,
        setupDataType: 'Uint8List',
        setup: 'setupData = Uint8List(len);',
        run:
            'benchmarkInputBytesTwin${asynchronous ? "Normal" : "Sync"}(bytes: setupData);',
      ),
    _generate(
      category: category,
      approach: 'Raw',
      direction: 'Input',
      asynchronous: false,
      args: args,
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
      _generate(
        category: category,
        approach: 'Frb',
        direction: 'Output',
        asynchronous: asynchronous,
        args: args,
        run:
            'benchmarkOutputBytesTwin${asynchronous ? "Normal" : "Sync"}(size: len);',
      ),
    _generate(
      category: category,
      approach: 'Raw',
      direction: 'Output',
      asynchronous: true,
      args: args,
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

List<String> _benchmarkBinaryTree() {
  const category = 'BinaryTree';
  const args = [_TypedName('int', 'depth')];

  return [
    '''
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
    ''',
    for (final sse in [false, true]) ...[
      _generate(
        category: category,
        approach: 'Frb${sse ? "Sse" : ""}',
        direction: 'Input',
        asynchronous: false,
        args: args,
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
      _generate(
        category: category,
        approach: 'Frb${sse ? "Sse" : ""}',
        direction: 'Output',
        asynchronous: false,
        args: args,
        run:
            'benchmarkBinaryTreeOutputTwinSync${sse ? "Sse" : ""}(depth: depth);',
      ),
    ],
    _generate(
      category: category,
      approach: 'Protobuf',
      direction: 'Input',
      asynchronous: false,
      args: args,
      setupDataType: 'BinaryTreeProtobuf',
      setup: 'setupData = _createTreeProtobuf(depth);',
      run:
          'benchmarkBinaryTreeInputProtobufTwinSync(raw: setupData.writeToBuffer());',
    ),
    _generate(
      category: category,
      approach: 'Protobuf',
      direction: 'Output',
      asynchronous: false,
      args: args,
      run: '''
        final raw = benchmarkBinaryTreeOutputProtobufTwinSync(depth: depth);
        final proto = BinaryTreeProtobuf.fromBuffer(raw);
        dummyValue ^= proto.hashCode;
      ''',
    ),
    _generate(
      category: category,
      approach: 'Json',
      direction: 'Input',
      asynchronous: false,
      args: args,
      setupDataType: 'BenchmarkBinaryTreeTwinSync',
      setup: 'setupData = BinaryTreeInput_Sync_Benchmark._createTree(depth);',
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
    _generate(
      category: category,
      approach: 'Json',
      direction: 'Output',
      asynchronous: false,
      args: args,
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

List<String> _benchmarkBlob() {
  const category = 'Blob';
  const args = [_TypedName('int', 'len')];

  String setupDataSimple({required bool sse}) => '''
        setupData = BenchmarkBlobTwinSync${sse ? "Sse" : ""}(
          first: Uint8List(len),
          second: Uint8List(len),
          third: Uint8List(len),
        );
      ''';

  return [
    for (final sse in [false, true]) ...[
      _generate(
        category: category,
        approach: 'Frb${sse ? "Sse" : ""}',
        direction: 'Input',
        asynchronous: false,
        args: args,
        setupDataType: 'BenchmarkBlobTwinSync${sse ? "Sse" : ""}',
        setup: setupDataSimple(sse: sse),
        run: 'benchmarkBlobInputTwinSync${sse ? "Sse" : ""}(blob: setupData);',
      ),
      _generate(
        category: category,
        approach: 'Frb${sse ? "Sse" : ""}',
        direction: 'Output',
        asynchronous: false,
        args: args,
        run: 'benchmarkBlobOutputTwinSync${sse ? "Sse" : ""}(size: len);',
      ),
    ],
    _generate(
      category: category,
      approach: 'Protobuf',
      direction: 'Input',
      asynchronous: false,
      args: args,
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
    _generate(
      category: category,
      approach: 'Protobuf',
      direction: 'Output',
      asynchronous: false,
      args: args,
      run: '''
        final raw = benchmarkBlobOutputProtobufTwinSync(size: len);
        final proto = BlobProtobuf.fromBuffer(raw);
        dummyValue ^= proto.hashCode;
      ''',
    ),
    _generate(
      category: category,
      approach: 'Json',
      direction: 'Input',
      asynchronous: false,
      args: args,
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
    _generate(
      category: category,
      approach: 'Json',
      direction: 'Output',
      asynchronous: false,
      args: args,
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
