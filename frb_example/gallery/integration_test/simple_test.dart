import 'package:flutter_test/flutter_test.dart';
import 'package:frb_example_gallery/main.dart';
import 'package:frb_example_gallery/src/ignore_me/mandelbrot_related.dart';
import 'package:frb_example_gallery/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

Future<void> main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());

  testWidgets('can see page', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    await tester.pumpAndSettle();
    expect(find.byType(MandelbrotPageUI), findsOneWidget);
  });
}
