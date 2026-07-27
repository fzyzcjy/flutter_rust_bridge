import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/main.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

Future<void> main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async => await RustLib.init());
  testWidgets('Can call rust function', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
  });

  await verifyLibraryLinked();
}

Future<void> verifyLibraryLinked() async {
  if (!Platform.isLinux) return;

  final binaryPath = Platform.resolvedExecutable;
  final processResult = await Process.run("ldd", [binaryPath]);

  if (processResult.exitCode != 0) {
    throw Exception("While trying to run ldd: ${processResult.stderr}");
  }

  final output = processResult.stdout.toString();

  if (!output.contains('librust_lib')) {
    throw Exception(
      'The library from flutter_rust_bridge is not linked (by default with a `librust_lib` prefix)\nLDD Output:\n$output',
    );
  }

  if (output.toLowerCase().contains('not found')) {
    throw Exception(
      'Unresolved dynamic dependencies found\nLDD Output:\n$output',
    );
  }
}
