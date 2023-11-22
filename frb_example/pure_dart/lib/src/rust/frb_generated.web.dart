// ignore_for_file: unused_import

import 'api/simple.dart';
import 'api/sync.dart';
import 'api/comment.dart';
import 'api/primitive.dart';

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
  Object api2wire_i_64(BigInt raw) {
    return castNativeBigInt(raw);
  }

  @protected
  List<dynamic> api2wire_struct_with_comments(StructWithComments raw) {
    return [api2wire_i_32(raw.fieldWithComments)];
  }

  @protected
  Object api2wire_u_64(BigInt raw) {
    return castNativeBigInt(raw);
  }
}

// Section: boilerplate

class RustLibWire extends BaseWire {
  // TODO
}
