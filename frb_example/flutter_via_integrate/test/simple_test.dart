import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_via_integrate/main.dart';

void main() {
  testWidgets('Can call rust function', (WidgetTester tester) async {
    await tester.pumpWidget(MyApp());
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
  });
}
