import 'dart:ffi';

import 'bridge_generated_api_1.dart';
import 'bridge_generated_api_2.dart';
import 'package:test/test.dart';

void main(List<String> args) {
  String dylibPath1 = args[0];
  String dylibPath2 = args[1];
  print(
      'flutter_rust_bridge example program start (dylibPath=$dylibPath1,$dylibPath2)');
  print('construct api');
  final dylib1 = DynamicLibrary.open(dylibPath1);
  final api1 = ApiClass1Impl(dylib1);
  test('dart call simpleAdder', () async {
    expect(await api1.simpleAdder1(a: 42, b: 100), 142);
  });

  final dylib2 = DynamicLibrary.open(dylibPath2);
  final api2 = ApiClass2Impl(dylib2);
  test('dart call simpleAdder2', () async {
    expect(await api2.simpleAdder2(a: 42, b: 100), 142);
  });
}
