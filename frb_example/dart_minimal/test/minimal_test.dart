import 'dart:async';
import 'dart:html' as html;
import 'dart:js_interop' as dart_js_interop;

import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:js/js.dart' as package_js;
import 'package:web/helpers.dart' as helpers;
import 'package:web/web.dart' as web;

@dart_js_interop.JS("wasm_bindgen.my_rust_function")
external void my_rust_function_packageweb(web.EventTarget message_port);

@package_js.JS("wasm_bindgen.my_rust_function")
external void my_rust_function_darthtml(dynamic message_port);

Future<void> run_packageweb() async {
  print('run_packageweb start');

  final messageChannel = web.MessageChannel();

  final _kMessageEvent =
      helpers.EventStreamProvider<web.MessageEvent>('message');
  _kMessageEvent
      .forTarget(messageChannel.port1)
      .listen((event) => print('messageChannel.port1 see event $event'));

  print('Dart before call my_rust_function');
  my_rust_function_packageweb(messageChannel.port2);
  print('Dart after call my_rust_function');

  print('Dart start sleeping');
  await Future.delayed(const Duration(seconds: 1000000));
}

Future<void> run_darthtml() async {
  print('run_darthtml start');

  final messageChannel = html.MessageChannel();

  final _kMessageEvent = html.EventStreamProvider<html.MessageEvent>('message');
  _kMessageEvent
      .forTarget(messageChannel.port1)
      .listen((event) => print('messageChannel.port1 see event $event'));

  print('Dart before call my_rust_function');
  my_rust_function_darthtml(messageChannel.port2);
  print('Dart after call my_rust_function');

  print('Dart start sleeping');
  await Future.delayed(const Duration(seconds: 1000000));
}

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  await run_packageweb();
  // await run_darthtml();

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
