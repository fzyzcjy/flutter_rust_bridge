import 'dart:async';
import 'dart:html' as html;

import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:js/js.dart';

@JS("wasm_bindgen.my_rust_function")
external void my_rust_function(dynamic message_port);

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Dart before call my_rust_function');
  final messageChannel = html.MessageChannel();
  messageChannel.port1.addEventListener(
      'message', (data) => 'messageChannel.port1 see event $data');
  my_rust_function(messageChannel.port2);
  print('Dart after call my_rust_function');

  print('Dart start sleeping');
  await Future.delayed(const Duration(seconds: 1000000));

  // print('Action: Configure tests (before)');
  // test('dart call minimalAdder', () async {
  //   print('Action: Call rust (before)');
  //   expect(await minimalAdder(a: 100, b: 200), 300);
  //   print('Action: Call rust (after)');
  // });
  // print('Action: Configure tests (end)');
  //
  // test('temp', () async {
  //   final input = Uint64List(1)..[0] = BigInt.parse('123456789012345678');
  //   expect(await f(a: input), input);
  // });
}
