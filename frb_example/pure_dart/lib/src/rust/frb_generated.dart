// ignore_for_file: unused_import, unused_element

import 'api/comment.dart';
import 'api/comment_twin_sync.dart';
import 'api/optional_primitive.dart';
import 'api/optional_primitive_twin_sync.dart';
import 'api/primitive.dart';
import 'api/primitive_twin_sync.dart';
import 'api/simple.dart';
import 'api/simple_twin_sync.dart';
import 'frb_generated.io.dart'
    if (dart.library.html) 'frb_generated.web.dart.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart'
    if (dart.library.html) 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
  }) async {
    await instance.initImpl(api: api, handler: handler);
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor => RustLibWire.new;

  @override
  String get defaultExternalLibraryStem => 'frb_example_pure_dart';

  @override
  String get defaultExternalLibraryRelativeDirectory => 'rust/target/release/';
}

abstract class RustLibApi extends BaseApi {
  Future<void> structWithCommentsTwinNormalInstanceMethod(
      {required StructWithCommentsTwinNormal that, dynamic hint});

  Future<void> structWithCommentsTwinNormalStaticMethod({dynamic hint});

  Future<void> functionWithCommentsSlashStarStarTwinNormal({dynamic hint});

  Future<void> functionWithCommentsTripleSlashMultiLineTwinNormal(
      {dynamic hint});

  Future<void> functionWithCommentsTripleSlashSingleLineTwinNormal(
      {dynamic hint});

  void structWithCommentsTwinSyncInstanceMethodTwinSync(
      {required StructWithCommentsTwinSync that, dynamic hint});

  void structWithCommentsTwinSyncStaticMethodTwinSync({dynamic hint});

  void functionWithCommentsSlashStarStarTwinSync({dynamic hint});

  void functionWithCommentsTripleSlashMultiLineTwinSync({dynamic hint});

  void functionWithCommentsTripleSlashSingleLineTwinSync({dynamic hint});

  Future<bool?> exampleOptionalPrimitiveTypeBoolTwinNormal(
      {bool? arg, dynamic hint});

  Future<double?> exampleOptionalPrimitiveTypeF32TwinNormal(
      {double? arg, dynamic hint});

  Future<double?> exampleOptionalPrimitiveTypeF64TwinNormal(
      {double? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeI16TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeI32TwinNormal(
      {int? arg, dynamic hint});

  Future<BigInt?> exampleOptionalPrimitiveTypeI64TwinNormal(
      {BigInt? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeI8TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU16TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU32TwinNormal(
      {int? arg, dynamic hint});

  Future<BigInt?> exampleOptionalPrimitiveTypeU64TwinNormal(
      {BigInt? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU8TwinNormal(
      {int? arg, dynamic hint});

  bool? exampleOptionalPrimitiveTypeBoolTwinSync({bool? arg, dynamic hint});

  double? exampleOptionalPrimitiveTypeF32TwinSync({double? arg, dynamic hint});

  double? exampleOptionalPrimitiveTypeF64TwinSync({double? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI16TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI32TwinSync({int? arg, dynamic hint});

  BigInt? exampleOptionalPrimitiveTypeI64TwinSync({BigInt? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI8TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU16TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU32TwinSync({int? arg, dynamic hint});

  BigInt? exampleOptionalPrimitiveTypeU64TwinSync({BigInt? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU8TwinSync({int? arg, dynamic hint});

  Future<bool> examplePrimitiveTypeBoolTwinNormal(
      {required bool arg, dynamic hint});

  Future<double> examplePrimitiveTypeF32TwinNormal(
      {required double arg, dynamic hint});

  Future<double> examplePrimitiveTypeF64TwinNormal(
      {required double arg, dynamic hint});

  Future<int> examplePrimitiveTypeI16TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeI32TwinNormal(
      {required int arg, dynamic hint});

  Future<BigInt> examplePrimitiveTypeI64TwinNormal(
      {required BigInt arg, dynamic hint});

  Future<int> examplePrimitiveTypeI8TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU16TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU32TwinNormal(
      {required int arg, dynamic hint});

  Future<BigInt> examplePrimitiveTypeU64TwinNormal(
      {required BigInt arg, dynamic hint});

  Future<int> examplePrimitiveTypeU8TwinNormal(
      {required int arg, dynamic hint});

  bool examplePrimitiveTypeBoolTwinSync({required bool arg, dynamic hint});

  double examplePrimitiveTypeF32TwinSync({required double arg, dynamic hint});

  double examplePrimitiveTypeF64TwinSync({required double arg, dynamic hint});

  int examplePrimitiveTypeI16TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeI32TwinSync({required int arg, dynamic hint});

  BigInt examplePrimitiveTypeI64TwinSync({required BigInt arg, dynamic hint});

  int examplePrimitiveTypeI8TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeU16TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeU32TwinSync({required int arg, dynamic hint});

  BigInt examplePrimitiveTypeU64TwinSync({required BigInt arg, dynamic hint});

  int examplePrimitiveTypeU8TwinSync({required int arg, dynamic hint});

  Future<int> simpleAdderTwinNormal(
      {required int a, required int b, dynamic hint});

  int simpleAdderTwinSync({required int a, required int b, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @override
  Future<void> structWithCommentsTwinNormalInstanceMethod(
      {required StructWithCommentsTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_comments_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_StructWithCommentsTwinNormal_instance_method(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinNormalInstanceMethodConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStructWithCommentsTwinNormalInstanceMethodConstMeta =>
      const TaskConstMeta(
        debugName: "StructWithCommentsTwinNormal_instance_method",
        argNames: ["that"],
      );

  @override
  Future<void> structWithCommentsTwinNormalStaticMethod({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_StructWithCommentsTwinNormal_static_method(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinNormalStaticMethodConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStructWithCommentsTwinNormalStaticMethodConstMeta =>
      const TaskConstMeta(
        debugName: "StructWithCommentsTwinNormal_static_method",
        argNames: [],
      );

  @override
  Future<void> functionWithCommentsSlashStarStarTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_slash_star_star_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsSlashStarStarTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFunctionWithCommentsSlashStarStarTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "function_with_comments_slash_star_star_twin_normal",
        argNames: [],
      );

  @override
  Future<void> functionWithCommentsTripleSlashMultiLineTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_triple_slash_multi_line_twin_normal(
              port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashMultiLineTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kFunctionWithCommentsTripleSlashMultiLineTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "function_with_comments_triple_slash_multi_line_twin_normal",
            argNames: [],
          );

  @override
  Future<void> functionWithCommentsTripleSlashSingleLineTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_triple_slash_single_line_twin_normal(
              port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashSingleLineTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kFunctionWithCommentsTripleSlashSingleLineTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "function_with_comments_triple_slash_single_line_twin_normal",
            argNames: [],
          );

  @override
  void structWithCommentsTwinSyncInstanceMethodTwinSync(
      {required StructWithCommentsTwinSync that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_comments_twin_sync(that);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_StructWithCommentsTwinSync_instance_method_twin_sync(arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinSyncInstanceMethodTwinSyncConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kStructWithCommentsTwinSyncInstanceMethodTwinSyncConstMeta =>
          const TaskConstMeta(
            debugName: "StructWithCommentsTwinSync_instance_method_twin_sync",
            argNames: ["that"],
          );

  @override
  void structWithCommentsTwinSyncStaticMethodTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_StructWithCommentsTwinSync_static_method_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinSyncStaticMethodTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStructWithCommentsTwinSyncStaticMethodTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "StructWithCommentsTwinSync_static_method_twin_sync",
        argNames: [],
      );

  @override
  void functionWithCommentsSlashStarStarTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_function_with_comments_slash_star_star_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsSlashStarStarTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFunctionWithCommentsSlashStarStarTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "function_with_comments_slash_star_star_twin_sync",
        argNames: [],
      );

  @override
  void functionWithCommentsTripleSlashMultiLineTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_function_with_comments_triple_slash_multi_line_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashMultiLineTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kFunctionWithCommentsTripleSlashMultiLineTwinSyncConstMeta =>
          const TaskConstMeta(
            debugName:
                "function_with_comments_triple_slash_multi_line_twin_sync",
            argNames: [],
          );

  @override
  void functionWithCommentsTripleSlashSingleLineTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_function_with_comments_triple_slash_single_line_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashSingleLineTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kFunctionWithCommentsTripleSlashSingleLineTwinSyncConstMeta =>
          const TaskConstMeta(
            debugName:
                "function_with_comments_triple_slash_single_line_twin_sync",
            argNames: [],
          );

  @override
  Future<bool?> exampleOptionalPrimitiveTypeBoolTwinNormal(
      {bool? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_bool(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_bool_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_bool,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeBoolTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeBoolTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_bool_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<double?> exampleOptionalPrimitiveTypeF32TwinNormal(
      {double? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_f_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_f32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_f_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeF32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeF32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_f32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<double?> exampleOptionalPrimitiveTypeF64TwinNormal(
      {double? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_f_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_f64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_f_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeF64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeF64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_f64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeI16TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_i16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_16,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeI32TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_i32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<BigInt?> exampleOptionalPrimitiveTypeI64TwinNormal(
      {BigInt? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_i64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeI8TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_optional_primitive_type_i8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_8,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i8_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeU16TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_u16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_16,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeU32TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_u32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<BigInt?> exampleOptionalPrimitiveTypeU64TwinNormal(
      {BigInt? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_example_optional_primitive_type_u64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int?> exampleOptionalPrimitiveTypeU8TwinNormal(
      {int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_optional_primitive_type_u8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_8,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u8_twin_normal",
        argNames: ["arg"],
      );

  @override
  bool? exampleOptionalPrimitiveTypeBoolTwinSync({bool? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_bool(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_bool_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_bool,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeBoolTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeBoolTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_bool_twin_sync",
        argNames: ["arg"],
      );

  @override
  double? exampleOptionalPrimitiveTypeF32TwinSync({double? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_f_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_f32_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_f_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeF32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeF32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_f32_twin_sync",
        argNames: ["arg"],
      );

  @override
  double? exampleOptionalPrimitiveTypeF64TwinSync({double? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_f_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_f64_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_f_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeF64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeF64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_f64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeI16TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_i16_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_16,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i16_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeI32TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_i32_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i32_twin_sync",
        argNames: ["arg"],
      );

  @override
  BigInt? exampleOptionalPrimitiveTypeI64TwinSync({BigInt? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_i64_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeI8TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_i8_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_8,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeI8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeI8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_i8_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeU16TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_u16_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_16,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u16_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeU32TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_u32_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_32,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u32_twin_sync",
        argNames: ["arg"],
      );

  @override
  BigInt? exampleOptionalPrimitiveTypeU64TwinSync({BigInt? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_u64_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_64,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int? exampleOptionalPrimitiveTypeU8TwinSync({int? arg, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_u_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_example_optional_primitive_type_u8_twin_sync(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_u_8,
      parseErrorData: null,
      constMeta: kExampleOptionalPrimitiveTypeU8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExampleOptionalPrimitiveTypeU8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_optional_primitive_type_u8_twin_sync",
        argNames: ["arg"],
      );

  @override
  Future<bool> examplePrimitiveTypeBoolTwinNormal(
      {required bool arg, dynamic hint}) {
    var arg0 = api2wire_bool(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_bool_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeBoolTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeBoolTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_bool_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<double> examplePrimitiveTypeF32TwinNormal(
      {required double arg, dynamic hint}) {
    var arg0 = api2wire_f_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_f32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_f_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_f32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<double> examplePrimitiveTypeF64TwinNormal(
      {required double arg, dynamic hint}) {
    var arg0 = api2wire_f_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_f64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_f64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI16TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_i_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_i16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_i_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI32TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_i_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_i32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<BigInt> examplePrimitiveTypeI64TwinNormal(
      {required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_i_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_i64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_i_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI8TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_i_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_i8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_i_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i8_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU16TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_u_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_u16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU32TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_u_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_u32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<BigInt> examplePrimitiveTypeU64TwinNormal(
      {required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_u_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_u64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU8TwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_u_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_type_u8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u8_twin_normal",
        argNames: ["arg"],
      );

  @override
  bool examplePrimitiveTypeBoolTwinSync({required bool arg, dynamic hint}) {
    var arg0 = api2wire_bool(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_bool_twin_sync(arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeBoolTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeBoolTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_bool_twin_sync",
        argNames: ["arg"],
      );

  @override
  double examplePrimitiveTypeF32TwinSync({required double arg, dynamic hint}) {
    var arg0 = api2wire_f_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_f32_twin_sync(arg0),
      parseSuccessData: _wire2api_f_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_f32_twin_sync",
        argNames: ["arg"],
      );

  @override
  double examplePrimitiveTypeF64TwinSync({required double arg, dynamic hint}) {
    var arg0 = api2wire_f_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_f64_twin_sync(arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_f64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeI16TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_i16_twin_sync(arg0),
      parseSuccessData: _wire2api_i_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i16_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeI32TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_i32_twin_sync(arg0),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i32_twin_sync",
        argNames: ["arg"],
      );

  @override
  BigInt examplePrimitiveTypeI64TwinSync({required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_i_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_i64_twin_sync(arg0),
      parseSuccessData: _wire2api_i_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeI8TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_i8_twin_sync(arg0),
      parseSuccessData: _wire2api_i_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_i8_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeU16TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_u16_twin_sync(arg0),
      parseSuccessData: _wire2api_u_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u16_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeU32TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_u32_twin_sync(arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u32_twin_sync",
        argNames: ["arg"],
      );

  @override
  BigInt examplePrimitiveTypeU64TwinSync({required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_u_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_u64_twin_sync(arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u64_twin_sync",
        argNames: ["arg"],
      );

  @override
  int examplePrimitiveTypeU8TwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_type_u8_twin_sync(arg0),
      parseSuccessData: _wire2api_u_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_type_u8_twin_sync",
        argNames: ["arg"],
      );

  @override
  Future<int> simpleAdderTwinNormal(
      {required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_simple_adder_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderTwinNormalConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderTwinNormalConstMeta => const TaskConstMeta(
        debugName: "simple_adder_twin_normal",
        argNames: ["a", "b"],
      );

  @override
  int simpleAdderTwinSync({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_simple_adder_twin_sync(arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderTwinSyncConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderTwinSyncConstMeta => const TaskConstMeta(
        debugName: "simple_adder_twin_sync",
        argNames: ["a", "b"],
      );
}

// Section: impl_wire2api

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

bool _wire2api_box_autoadd_bool(dynamic raw) {
  return raw as bool;
}

double _wire2api_box_autoadd_f_32(dynamic raw) {
  return raw as double;
}

double _wire2api_box_autoadd_f_64(dynamic raw) {
  return raw as double;
}

int _wire2api_box_autoadd_i_16(dynamic raw) {
  return raw as int;
}

int _wire2api_box_autoadd_i_32(dynamic raw) {
  return raw as int;
}

BigInt _wire2api_box_autoadd_i_64(dynamic raw) {
  return _wire2api_i_64(raw);
}

int _wire2api_box_autoadd_i_8(dynamic raw) {
  return raw as int;
}

int _wire2api_box_autoadd_u_16(dynamic raw) {
  return raw as int;
}

int _wire2api_box_autoadd_u_32(dynamic raw) {
  return raw as int;
}

BigInt _wire2api_box_autoadd_u_64(dynamic raw) {
  return _wire2api_u_64(raw);
}

int _wire2api_box_autoadd_u_8(dynamic raw) {
  return raw as int;
}

double _wire2api_f_32(dynamic raw) {
  return raw as double;
}

double _wire2api_f_64(dynamic raw) {
  return raw as double;
}

int _wire2api_i_16(dynamic raw) {
  return raw as int;
}

int _wire2api_i_32(dynamic raw) {
  return raw as int;
}

BigInt _wire2api_i_64(dynamic raw) {
  return wire2apiI64OrU64(raw);
}

int _wire2api_i_8(dynamic raw) {
  return raw as int;
}

bool? _wire2api_opt_box_autoadd_bool(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_bool(raw);
}

double? _wire2api_opt_box_autoadd_f_32(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_f_32(raw);
}

double? _wire2api_opt_box_autoadd_f_64(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_f_64(raw);
}

int? _wire2api_opt_box_autoadd_i_16(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_i_16(raw);
}

int? _wire2api_opt_box_autoadd_i_32(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_i_32(raw);
}

BigInt? _wire2api_opt_box_autoadd_i_64(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_i_64(raw);
}

int? _wire2api_opt_box_autoadd_i_8(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_i_8(raw);
}

int? _wire2api_opt_box_autoadd_u_16(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u_16(raw);
}

int? _wire2api_opt_box_autoadd_u_32(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u_32(raw);
}

BigInt? _wire2api_opt_box_autoadd_u_64(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u_64(raw);
}

int? _wire2api_opt_box_autoadd_u_8(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_u_8(raw);
}

int _wire2api_u_16(dynamic raw) {
  return raw as int;
}

int _wire2api_u_32(dynamic raw) {
  return raw as int;
}

BigInt _wire2api_u_64(dynamic raw) {
  return wire2apiI64OrU64(raw);
}

int _wire2api_u_8(dynamic raw) {
  return raw as int;
}

void _wire2api_unit(dynamic raw) {
  return;
}

// Section: api2wire_funcs

bool api2wire_bool(bool raw) {
  return raw;
}

double api2wire_f_32(double raw) {
  return raw;
}

double api2wire_f_64(double raw) {
  return raw;
}

int api2wire_i_16(int raw) {
  return raw;
}

int api2wire_i_32(int raw) {
  return raw;
}

int api2wire_i_8(int raw) {
  return raw;
}

int api2wire_u_16(int raw) {
  return raw;
}

int api2wire_u_32(int raw) {
  return raw;
}

int api2wire_u_8(int raw) {
  return raw;
}
