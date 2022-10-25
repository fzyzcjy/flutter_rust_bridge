// ignore_for_file: implementation_imports

import 'dart:convert';
import 'dart:html';

import 'strings.dart' as io;
import 'package:js/js.dart';

@JS()
external void close();

void main() async {
  final socket = WebSocket(Uri.base.replace(scheme: 'ws').toString());
  socket.onOpen.first.then((_) async {
    return Future.sync(() => io.main(['stub']));
  }).then((value) {
    socket.send(jsonEncode({'__result__': true}));
    close();
  });
}
