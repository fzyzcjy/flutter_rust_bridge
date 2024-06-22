import 'package:flutter_test/flutter_test.dart';
import 'package:frb_example_rust_ui_todo_list/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  testWidgets('todo', (WidgetTester tester) async {
    // TODO
  });
}
