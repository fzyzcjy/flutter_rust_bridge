import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils.dart';

import 'minimal_test.dart' as minimal_test;

void main() {
  dartWebTestEntrypoint(() async {
    await minimal_test.main();
  });
}
