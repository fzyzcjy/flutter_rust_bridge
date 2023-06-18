import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_definitions.dart';

import 'bridge_generated_api_block_1.dart';
import 'bridge_generated_api_block_2.dart';
import 'bridge_generated_api_block_3.dart';
import 'package:test/test.dart';

import 'test_api_block_1.dart';
import 'test_api_block_2.dart';
import 'test_api_block_3.dart';
import 'test_type_methods.dart';

void main(List<String> args) {
  print(args.length);
  print(args);
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');

  // essential initialization
  final dylib = loadLibForDart(dylibPath);

  final api1 = ApiBlock1ClassImpl(dylib);
  final api2 = ApiBlock2ClassImpl(dylib);
  final api3 = ApiBlock3ClassImpl(dylib);
  final apiShared = BridgeGeneratedSharesImpl(dylib);

  final enumList = [
    EnumType.enums(Weekdays.Friday),
    EnumType.nested(EnumType.enums(Weekdays.Friday)),
    EnumType.empty(),
    EnumType.primitives(int32: 1, float64: 2.0, boolean: true),
    EnumType.optional(null, Uint8List.fromList([1, 2])),
    EnumType.buffer(Float32List.fromList([1.1, 2.2])),
    EnumType.bytesArray(U8Array3(Uint8List.fromList([1, 2, 3])))
  ];
  testTypeMethods(api1, apiShared);
  testApiBlock1(api1, apiShared, enumList);
  testApiBlock2(api2, apiShared, enumList);
  testApiBlock3(api3, apiShared);

  tearDownAll(() {
    api1.dispose();
    api2.dispose();
  });
}
