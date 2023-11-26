import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';

Future<void> generateDartWebTestEntrypoint({required Uri dartRoot}) async {
  final dirInterest = dartRoot.resolve('test/api/');
  final files = [for (final file in Glob('${dirInterest.toFilePath()}**.dart').listSync()) file];

  final imports = [for (final file in files) "import '$TODO' as $TODO;\n"].join("");
  final calls = [for (final file in files) "await $TODO.main();\n"].join("");

  final code = '''
import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';

$imports

void main() {
  dartWebTestEntrypoint(() async {
    $calls
  });
}
  ''';

  final pathOutput = dartRoot.resolve('test/dart_web_test_entrypoint.dart').toFilePath();
  File(pathOutput).writeAsStringSync(code);
  executeProcess('dart', ['format', pathOutput]);
}
