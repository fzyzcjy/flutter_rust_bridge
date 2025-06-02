import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';

import 'no_rust_async_test.dart' as no_rust_async_test;

Future<void> main() async {
  await dartWebTestEntrypoint(() async {
    await no_rust_async_test.main();
  });
}
