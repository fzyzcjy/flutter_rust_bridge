import 'dart:io';

/// {@macro flutter_rust_bridge.internal}
String err(String msg) {
  // return stderr.supportsAnsiEscapes ? Colorize(msg).red().bold().toString() : msg; // #1262
  return msg;
}

/// {@macro flutter_rust_bridge.internal}
void eprint([Object? msg = 'unspecified']) {
  stderr.writeln('${err('error')}: $msg');
}

/// {@macro flutter_rust_bridge.internal}
Never bail([String? message]) {
  eprint(message);
  exit(1);
}
