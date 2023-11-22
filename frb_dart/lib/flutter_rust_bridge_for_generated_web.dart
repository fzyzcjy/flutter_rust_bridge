/// {@macro flutter_rust_bridge.only_for_generated_code}
library;

export 'flutter_rust_bridge_for_generated.dart';

// We do export things via `if (dart.library.html)`, but dart analyzer does not know it.
// Thus, we make extra (duplicated) export here to be clear.
export 'src/utils/web_utils.dart';
