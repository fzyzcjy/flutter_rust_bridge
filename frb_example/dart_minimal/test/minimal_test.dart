import 'dart:async';
import 'dart:js_interop';

import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:web/web.dart' as web;

// TODO temp
@JS("wasm_bindgen.hello_func")
external void _hello_func(JSAny port);

Future<void> main() async {
  print('minimal_test:main before RustLib.init');
  await RustLib.init();

  final channel = web.MessageChannel();

  final sendPort = channel.port2;
  final receivePort = channel.port1;

  const kMessageEvent = web.EventStreamProvider<web.MessageEvent>('message');
  kMessageEvent.forTarget(receivePort).listen(
        (data) => print('stream recv data=$data'),
        onError: (e, s) => print('stream recv e=$e s=$s'),
        onDone: () => print('stream recv done'),
      );

  print('minimal_test:main before call Rust _hello_func');
  _hello_func(sendPort);

  print('minimal_test:main sleep forever');
  await Future.delayed(const Duration(days: 1));

  // TODO temp
  // print('Action: Init rust (before)');
  // await RustLib.init();
  // print('Action: Init rust (after)');
  //
  // print('Action: Configure tests (before)');
  // test('dart call minimalAdder', () async {
  //   print('Action: Call rust (before)');
  //   expect(await minimalAdder(a: 100, b: 200), 300);
  //   print('Action: Call rust (after)');
  // });
  // print('Action: Configure tests (end)');

  // TODO temp test
  // test('temp', () async {
  //   final input = Uint64List(1)..[0] = BigInt.parse('123456789012345678');
  //   expect(await f(a: input), input);
  // });
}
