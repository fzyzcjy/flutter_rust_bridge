import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';

import 'minimal_test.dart' as minimal_test;

Future<void> main() async {
  await dartWebTestEntrypoint(() async {
    await minimal_test.main();
  });
}
