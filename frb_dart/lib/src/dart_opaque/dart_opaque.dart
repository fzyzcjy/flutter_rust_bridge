import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';

// ignore: always_use_package_imports
import '_io.dart' if (dart.library.html) '_web.dart';

export '_common.dart';
export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object decodeDartOpaque(
    dynamic raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  return generalizedFrbRustBinding
      .dartOpaqueRust2DartDecode(decoderDartOpaqueInner(raw));
}
