// ignore_for_file: unused_import

import 'api/simple.dart';
import 'api/comment.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'frb_generated.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_comments(
      StructWithComments raw) {
    return api2wire_struct_with_comments(raw);
  }

  @protected
  List<dynamic> api2wire_struct_with_comments(StructWithComments raw) {
    return [api2wire_i_32(raw.fieldWithComments)];
  }
}

// Section: boilerplate

class RustLibWire extends BaseWire {
  // TODO
}
