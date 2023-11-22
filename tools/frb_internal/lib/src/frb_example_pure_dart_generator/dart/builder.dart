class DartFileBuilder {
  final String importName;
  String body = '';

  DartFileBuilder({required this.importName});

  @override
  String toString() {
    return '''
import 'package:frb_example_pure_dart/src/rust/api/$importName.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();
  
  group('$importName', () {
    $body
  });
}
  ''';
  }
}
