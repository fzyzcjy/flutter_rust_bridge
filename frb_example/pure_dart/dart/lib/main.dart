import 'dart:ffi';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_example/bridge_generated.dart';

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

    print('dart call handle_stream');
    {
      final stream = api.handleStream(arg: 'hello');
      var cnt = 0;
      await for (final value in stream) {
        print("output from handle_stream's stream: $value");
        cnt++;
      }
      expect(cnt, 10);
    }

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

    print('dart call handleOptionalReturn');
    final optional1 = await api.handleOptionalReturn(left: 1, right: 1);
    expect(optional1!, 1);
    final optional2 = await api.handleOptionalReturn(left: 2, right: 0);
    expect(optional2, null);

    final optional3 = await api.handleOptionalStruct();
    expect(optional3, null);

    final message = 'Hello there.';
    final optional4 = await api.handleOptionalStruct(document: message);
    if (optional4 == null) return fail('handleOptionalStruct returned null for non-null document');
    expect(optional4.tag, 'div');
    expect(optional4.text, null);
    expect(optional4.attributes?[0].key, 'id');
    expect(optional4.attributes?[0].value, 'root');

    expect(optional4.children?[0].tag, 'p');
    expect(optional4.children?[0].text, null);
    expect(optional4.children?[0].attributes, null);
    expect(optional4.children?[0].children?[0].text, message);

    print('dart call increment');
    final optional5 = await api.increment();
    expect(optional5, null);

    var optional6 = await api.increment(
      opt: ExoticOptionals(
        attributesNullable: [],
      ),
    );
    if (optional6 == null) return fail('increment returned null for non-null params');
    final loopFor = 500;
    for (var i = 1; i < loopFor; i++) {
      optional6 = await api.increment(opt: optional6);
    }
    if (optional6 == null) return fail('optional6 nulled after loop');
    expect(optional6.int32, loopFor);
    expect(optional6.int32, loopFor);
    expect(optional6.float64, loopFor);
    expect(optional6.boolean, false);
    expect(optional6.zerocopy?.length, loopFor);
    expect(optional6.int8List?.length, loopFor - 1);
    expect(optional6.uint8List?.length, loopFor - 1);
    // expect(optional6.float64List?.length, loopFor);
    expect(optional6.attributes?.length, loopFor - 1);
    expect(optional6.newtypeint?.field0, loopFor - 1);

    print('dart call handleBoxedOptional');
    final optional7 = await api.handleBoxedOptional();
    expect(optional7, 42);
    var optional8 = 0.0;
    for (var i = 0; i < loopFor; i++) {
      optional8 = await api.handleBoxedOptional(opt: optional8);
    }
    expect(optional8, loopFor);

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
