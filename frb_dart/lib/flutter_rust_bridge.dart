import 'package:flutter_rust_bridge/src/basic.dart';

export 'src/flutter_rust_bridge_io.dart'
    if (dart.library.html) 'flutter_rust_bridge_web.dart';
export 'src/platform_independent.dart';

class Anyhow implements FrbException {
  String anyhow;

  Anyhow(this.anyhow);
}
