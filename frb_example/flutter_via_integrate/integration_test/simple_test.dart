import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_via_integrate/main.dart';
import 'package:flutter_via_integrate/src/rust/frb_generated.dart';

Future<void> main() async {
  print('hi 1');
  await RustLib.init();
  print('hi 2');
  testWidgets('Can call rust function', (WidgetTester tester) async {
    print('hi 3');
    await tester.pumpWidget(MyApp());
    print('hi 4');
    expect(find.textContaining('Result: `Hello, Tom!`'), findsOneWidget);
    print('hi 5');
  });
}
