import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void frb_initialize_rust(
    // ignore: non_constant_identifier_names
    NativePortType dart_opaque_drop_port,
    // ignore: non_constant_identifier_names
    NativePortType dart_fn_invoke_port,
  );
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireConstructor<W> = W Function(ExternalLibrary externalLibrary);
