import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/main.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/src/rust/frb_generated.dart';
import 'package:flutter/foundation.dart';
import 'package:integration_test/integration_test.dart';

main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  try {
    setUpAll(() async => await RustLib.init());
  } catch (e, s) {
    debugPrint("While trying to load the rust library: $e\nStackTrace:\n$s");
    exit(1);
  }

  testWidgets('Can call rust function', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
  });
}
