import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_via_integrate/main.dart';
import 'package:flutter_via_integrate/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  testWidgets('Can call rust function', (WidgetTester tester) async {
    await tester.pumpWidget(MyApp());
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
  });
}
