import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_via_create/main.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  testWidgets('Can call rust function', (WidgetTester tester) async {
    final result = greet(name: 'Tom');
    final smokeStatus = await runFrbSmokeSuite();
    await tester.pumpWidget(MyApp(result: result, smokeStatus: smokeStatus));
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
    expect(find.textContaining('Extended smoke: `PASS`'), findsOneWidget);
  });
}
