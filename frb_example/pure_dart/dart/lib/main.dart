import 'dart:ffi';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_example/generated.dart';
import 'package:test/test.dart';

void main(List<String> args) async {
  test('main test', () async {
    final dylibPath = args[0];

    print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');

    print('construct api');
    final dylib = DynamicLibrary.open(dylibPath);
    final api = FlutterRustBridgeExample(dylib);

    print('call functions');

    print('dart call simpleAdder');
    expect(await api.simpleAdder(a: 42, b: 100), 142);

    print('dart call primitiveTypes');
    expect(
        await api.primitiveTypes(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);

    print('dart call handleString');
    expect(await api.handleString(s: "Hello, world!"), "Hello, world!Hello, world!");

    print('dart call handleVecU8');
    final len = 100000;
    expect(await api.handleVecU8(v: Uint8List.fromList(List.filled(len, 127))),
        Uint8List.fromList(List.filled(len * 2, 127)));

    print('dart call handleZeroCopyResult');
    final n = 100000;
    expect(await api.handleZeroCopyResult(n: n), Uint8List.fromList(List.filled(n, 42)));

    print('dart call handleStruct');
    final structResp =
        await api.handleStruct(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);

    print('dart call handleNewtype');
    final newtypeResp = await api.handleNewtype(arg: NewTypeInt(field0: 42));
    expect(newtypeResp.field0, 84);

    print('dart call handleListOfStruct');
    final listOfStructResp =
        await api.handleListOfStruct(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);

    print('dart call handleComplexStruct');
    final arrLen = 5;
    final complexStructResp = await api.handleComplexStruct(
      s: MyTreeNode(
        valueI32: 100,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
        children: [
          MyTreeNode(
            valueI32: 110,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
            children: [
              MyTreeNode(
                valueI32: 111,
                valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
                children: [],
              ),
            ],
          ),
          MyTreeNode(
            valueI32: 120,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
            children: [],
          ),
        ],
      ),
    );
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));

    print('dart call returnErr');
    try {
      await api.returnErr();
      fail("exception not thrown");
    } catch (e) {
      print('dart catch e: $e');
      expect(e, isA<FfiException>());
    }

    print('dart call returnPanic');
    try {
      await api.returnPanic();
      fail("exception not thrown");
    } catch (e) {
      print('dart catch e: $e');
      expect(e, isA<FfiException>());
    }

    _createGarbage();
    await Future.delayed(Duration(seconds: 1));
    _createGarbage();
    await Future.delayed(Duration(seconds: 1));

    print('loop and call many times');
    var obj = complexStructResp;
    for (var i = 0; i < 500; ++i) {
      obj = await api.handleComplexStruct(s: obj);
    }

    print('flutter_rust_bridge example program end');
  });
}

int _createGarbage() {
  print('dart create garbage (thus make it more possible to GC)');
  var cum = 0;
  for (var i = 0; i < 5000; ++i) {
    final l = List.filled(5000, 42);
    cum += l[42];
  }
  return cum;
}
