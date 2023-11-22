import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });
}

// Section: boilerplate

class RustLibWire extends BaseWire {
  // TODO
}

// Section: api2wire_funcs

@protected
List<dynamic> api2wire_box_autoadd_struct_with_comments(
    StructWithComments raw) {
  return api2wire_struct_with_comments(raw);
}

@protected
List<dynamic> api2wire_struct_with_comments(StructWithComments raw) {
  return [api2wire_i_32(raw.fieldWithComments)];
}
