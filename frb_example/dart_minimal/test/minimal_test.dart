import 'dart:async';
import 'dart:js_interop';
import 'dart:typed_data';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

@JS("wasm_bindgen.rust_wasm_bindgen_func")
external void rust_wasm_bindgen_func();

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Dart call rust_wasm_bindgen_func before');
  rust_wasm_bindgen_func();
  print('Dart call rust_wasm_bindgen_func after');

  // print('Action: Configure tests (before)');
  // test('dart call minimalAdder', () async {
  //   print('Action: Call rust (before)');
  //   expect(await minimalAdder(a: 100, b: 200), 300);
  //   print('Action: Call rust (after)');
  // });
  // print('Action: Configure tests (end)');
  //
  // for (var i = 0; i < 100; ++i) {
  //   group('group $i', () {
  //     test('loop and call many times', () async {
  //       var obj = _createMyTreeNode(arrLen: 5);
  //       for (var i = 0; i < 500; ++i) {
  //         obj = await handleComplexStructTwinNormal(s: obj);
  //       }
  //     });
  //   });
  // }
}

MyTreeNodeTwinNormal _createMyTreeNode({required int arrLen}) {
  return MyTreeNodeTwinNormal(
    valueI32: 100,
    valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
    valueBoolean: true,
    children: [
      MyTreeNodeTwinNormal(
        valueI32: 110,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
        valueBoolean: true,
        children: [
          MyTreeNodeTwinNormal(
            valueI32: 111,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
            valueBoolean: true,
            children: [],
          ),
        ],
      ),
      MyTreeNodeTwinNormal(
        valueI32: 120,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
        valueBoolean: true,
        children: [],
      ),
    ],
  );
}
