import 'package:flutter_package_native_assets/flutter_package_native_assets.dart';

Future<void> main() async {
  await RustLib.init();

  final result = greet(name: 'Tom');
  if (result != 'Hello, Tom!') {
    throw StateError('Unexpected greeting: $result');
  }
}
