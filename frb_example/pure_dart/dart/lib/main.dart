import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'bridge_definitions.dart';

const isWeb = bool.fromEnvironment('dart.library.html');

String? skipWeb([String reason = 'unspecified']) => isWeb ? 'Skipped on web (reason: $reason)' : null;

void main(List<String> args) async {
  String dylibPath = args[0];
  var releaseMode = true;
  assert(() {
    releaseMode = false;
    return true;
  }());
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);

  for (var i =0 ; i<100; ++i) {
    var magic = await api.magic();
    magic.dispose();
  }
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // print(await api.handleMagic(magic: magic));
  // var magic2 =  magic.clone();
  // print(magic.isStale());
  // print(magic2.isStale());
  // magic.dispose();
  // print(magic.isStale());
  // print(magic2.isStale());
  // print(await api.handleMagic(magic: magic2));
  
  // var wtffi = await api.handleOpaqueAaa();
  // print('Create');
  // var strWtffi = await api.handleOpaqueBbb(value: wtffi);
  // print('MY FFI ${strWtffi}');
  // wtffi.magic.dispose();
  // var strWtffi2 = await api.handleOpaqueBbb(value: wtffi);
  // print('MY FFI2 ${strWtffi2}');
  //   var strWtffi3 = await api.handleOpaqueBbb(value: wtffi);
  // print('MY FFI3 ${strWtffi3}');
  //   var strWtffi4 = await api.handleOpaqueBbb(value: wtffi);
  // print('MY FFI4 ${strWtffi4}');
  //   var strWtffi5 = await api.handleOpaqueBbb(value: wtffi);
  // print('MY FFI5 ${strWtffi5}');
  
  // OpaqueBag? bag;
  // print('RAZ');
  // var a = await api.handleOpaque(value: null);
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);
  // print('DWA ${a.primitive}');
  // var b = await api.handleOpaqueRepr(value: a.primitive);
  // print('TREE $b');

  // a.array.dispose();
  // a.traitObj.dispose();
  // a.lifetime.dispose();
  // a.primitive.dispose();
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);
  // await api.handleOpaque(value: a);

  // b = await api.handleOpaqueRepr(value: a.primitive);
  // print('TREE2 $b');

  // print('TREE $a');
  // await api.handleOpaque(value: a);
  // print('CH ${await api.handleOpaqueRepr(value: a.primitive)}');
}

int _createGarbage() {
  print('dart create garbage (thus make it more possible to GC)');
  var cum = 0;
  for (var i = 0; i < 1000; ++i) {
    final l = List.filled(5000, 42);
    cum += l[42];
  }
  return cum;
}

// MyTreeNode _createMyTreeNode({required int arrLen}) {
//   return MyTreeNode(
//     valueI32: 100,
//     valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
//     valueBoolean: true,
//     children: [
//       MyTreeNode(
//         valueI32: 110,
//         valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
//         valueBoolean: true,
//         children: [
//           MyTreeNode(
//             valueI32: 111,
//             valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
//             valueBoolean: true,
//             children: [],
//           ),
//         ],
//       ),
//       MyTreeNode(
//         valueI32: 120,
//         valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
//         valueBoolean: true,
//         children: [],
//       ),
//     ],
//   );
// }

class MatchBigInt extends CustomMatcher {
  MatchBigInt(matcher) : super("is a numeric", "value", _featureValueOf(matcher));
  @override
  Object? featureValueOf(actual) => _featureValueOf(actual);

  static Object? _featureValueOf(actual) {
    if (actual is Iterable) return actual.map(_featureValueOf);
    if (actual is int) return BigInt.from(actual);
    return actual;
  }
}

// vim:expandtab:ts=2:sw=2
