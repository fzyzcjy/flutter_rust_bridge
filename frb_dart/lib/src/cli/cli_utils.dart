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

/// {@macro flutter_rust_bridge.internal}
Future<String> findDartPackageDirectory(String startingDir) async {
  var tentativeDir = Directory(startingDir);
  while (!_isRootDir(tentativeDir)) {
    if (await File(tentativeDir.uri.resolve('pubspec.yaml').toFilePath())
        .exists()) {
      return tentativeDir.absolute.path;
    }
    tentativeDir = tentativeDir.parent;
  }
  throw ArgumentError(
      'Cannot find dart package directory from path $startingDir');
}

bool _isRootDir(Directory d) => d.parent.path == d.path;
