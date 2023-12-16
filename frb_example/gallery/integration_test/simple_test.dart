import 'package:flutter_test/flutter_test.dart';
import 'package:frb_example_gallery/main.dart';
import 'package:frb_example_gallery/src/examples/mandelbrot.dart';
import 'package:frb_example_gallery/src/examples/polars.dart';
import 'package:frb_example_gallery/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

Future<void> main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  for (final (pageName, pageFinder) in [
    ('Mandelbrot', find.byType(MandelbrotPageBody)),
    ('Polars', find.byType(PolarsPageBody)),
  ]) {
    testWidgets('$pageName page', (WidgetTester tester) async {
      await tester.pumpWidget(const MyApp());
      await tester.tap(find.text(pageName));
      await tester.pumpAndSettle();
      expect(pageFinder, findsOneWidget);
    });
  }
}
