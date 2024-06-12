import 'dart:async';
import 'dart:html' as html;
import 'dart:js_interop' as dart_js_interop;

import 'package:js/js.dart' as package_js;
import 'package:web/helpers.dart' as helpers;
import 'package:web/web.dart' as web;

Future<void> run_packageweb() async {
  print('run_packageweb start');

  final messageChannel = web.MessageChannel();

  final _kMessageEvent =
      helpers.EventStreamProvider<web.MessageEvent>('message');
  _kMessageEvent
      .forTarget(messageChannel.port1)
      .listen((event) => print('messageChannel.port1 see event ${event.data}'));

  print('Dart call postMessage');
  messageChannel.port2.postMessage(12345.toJS);

  print('Dart start sleeping');
  await Future.delayed(const Duration(seconds: 1000000));
}

Future<void> run_darthtml() async {
  print('run_darthtml start');

  final messageChannel = html.MessageChannel();

  final _kMessageEvent = html.EventStreamProvider<html.MessageEvent>('message');
  _kMessageEvent
      .forTarget(messageChannel.port1)
      .listen((event) => print('messageChannel.port1 see event ${event.data}'));

  print('Dart call postMessage');
  messageChannel.port2.postMessage(12345);

  print('Dart start sleeping');
  await Future.delayed(const Duration(seconds: 1000000));
}

Future<void> main() async {
  // await run_packageweb();
  await run_darthtml();
}
