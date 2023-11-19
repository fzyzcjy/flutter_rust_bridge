import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated_api_1.dart';
import 'bridge_generated_api_2.dart';
import 'package:test/test.dart';

void main(List<String> args) {
  print(args.length);
  print(args);
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final dylib = loadLibForDart(dylibPath);
  final api1 = ApiClass1Impl(dylib);
  test('dart call simpleAdder', () async {
    expect(await api1.simpleAdder1(a: 42, b: 100), 142);
  });

  final api2 = ApiClass2Impl(dylib);
  test('dart call simpleAdder2', () async {
    expect(await api2.simpleAdder2(a: 42, b: 100), 142);
  });

  tearDownAll(() {
    api1.dispose();
    api2.dispose();
  });
}
