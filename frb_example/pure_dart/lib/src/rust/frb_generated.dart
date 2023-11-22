// ignore_for_file: unused_import, unused_element

import 'api/simple.dart';
import 'frb_generated.io.dart'
    if (dart.library.html) 'frb_generated.web.dart.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart'
    if (dart.library.html) 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'api/comment.dart';
import 'api/primitive.dart';
import 'api/sync.dart';

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
  Future<void> structWithCommentsInstanceMethod(
      {required StructWithComments that, dynamic hint});

  Future<void> structWithCommentsStaticMethod({dynamic hint});

  Future<void> functionWithCommentsSlashStarStar({dynamic hint});

  Future<void> functionWithCommentsTripleSlashMultiLine({dynamic hint});

  Future<void> functionWithCommentsTripleSlashSingleLine({dynamic hint});

  Future<bool> examplePrimitiveTypeBool({required bool arg, dynamic hint});

  Future<double> examplePrimitiveTypeF32({required double arg, dynamic hint});

  Future<double> examplePrimitiveTypeF64({required double arg, dynamic hint});

  Future<int> examplePrimitiveTypeI16({required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeI32({required int arg, dynamic hint});

  Future<BigInt> examplePrimitiveTypeI64({required BigInt arg, dynamic hint});

  Future<int> examplePrimitiveTypeI8({required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU16({required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU32({required int arg, dynamic hint});

  Future<BigInt> examplePrimitiveTypeU64({required BigInt arg, dynamic hint});

  Future<int> examplePrimitiveTypeU8({required int arg, dynamic hint});

  Future<int> simpleAdder({required int a, required int b, dynamic hint});

  int simpleAdderSync({required int a, required int b, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @override
  Future<void> structWithCommentsInstanceMethod(
      {required StructWithComments that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_comments(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_StructWithComments_instance_method(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsInstanceMethodConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStructWithCommentsInstanceMethodConstMeta =>
      const TaskConstMeta(
        debugName: "StructWithComments_instance_method",
        argNames: ["that"],
      );

  @override
  Future<void> structWithCommentsStaticMethod({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_StructWithComments_static_method(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsStaticMethodConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStructWithCommentsStaticMethodConstMeta =>
      const TaskConstMeta(
        debugName: "StructWithComments_static_method",
        argNames: [],
      );

  @override
  Future<void> functionWithCommentsSlashStarStar({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_slash_star_star(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsSlashStarStarConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFunctionWithCommentsSlashStarStarConstMeta =>
      const TaskConstMeta(
        debugName: "function_with_comments_slash_star_star",
        argNames: [],
      );

  @override
  Future<void> functionWithCommentsTripleSlashMultiLine({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_triple_slash_multi_line(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashMultiLineConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFunctionWithCommentsTripleSlashMultiLineConstMeta =>
      const TaskConstMeta(
        debugName: "function_with_comments_triple_slash_multi_line",
        argNames: [],
      );

  @override
  Future<void> functionWithCommentsTripleSlashSingleLine({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_function_with_comments_triple_slash_single_line(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFunctionWithCommentsTripleSlashSingleLineConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFunctionWithCommentsTripleSlashSingleLineConstMeta =>
      const TaskConstMeta(
        debugName: "function_with_comments_triple_slash_single_line",
        argNames: [],
      );

  @override
  Future<bool> examplePrimitiveTypeBool({required bool arg, dynamic hint}) {
    var arg0 = api2wire_bool(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_bool(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeBoolConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeBoolConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_bool",
        argNames: ["arg"],
      );

  @override
  Future<double> examplePrimitiveTypeF32({required double arg, dynamic hint}) {
    var arg0 = api2wire_f_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_f32(port_, arg0),
      parseSuccessData: _wire2api_f_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF32ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF32ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_f32",
        argNames: ["arg"],
      );

  @override
  Future<double> examplePrimitiveTypeF64({required double arg, dynamic hint}) {
    var arg0 = api2wire_f_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_f64(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeF64ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeF64ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_f64",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI16({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_i16(port_, arg0),
      parseSuccessData: _wire2api_i_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI16ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI16ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_i16",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI32({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_i32(port_, arg0),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI32ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI32ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_i32",
        argNames: ["arg"],
      );

  @override
  Future<BigInt> examplePrimitiveTypeI64({required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_i_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_i64(port_, arg0),
      parseSuccessData: _wire2api_i_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI64ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI64ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_i64",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeI8({required int arg, dynamic hint}) {
    var arg0 = api2wire_i_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_i8(port_, arg0),
      parseSuccessData: _wire2api_i_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeI8ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeI8ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_i8",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU16({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_u16(port_, arg0),
      parseSuccessData: _wire2api_u_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU16ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU16ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_u16",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU32({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_u32(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU32ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU32ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_u32",
        argNames: ["arg"],
      );

  @override
  Future<BigInt> examplePrimitiveTypeU64({required BigInt arg, dynamic hint}) {
    var arg0 = api2wire_u_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_u64(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU64ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU64ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_u64",
        argNames: ["arg"],
      );

  @override
  Future<int> examplePrimitiveTypeU8({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_example_primitive_type_u8(port_, arg0),
      parseSuccessData: _wire2api_u_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveTypeU8ConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveTypeU8ConstMeta => const TaskConstMeta(
        debugName: "example_primitive_type_u8",
        argNames: ["arg"],
      );

  @override
  Future<int> simpleAdder({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_simple_adder(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderConstMeta => const TaskConstMeta(
        debugName: "simple_adder",
        argNames: ["a", "b"],
      );

  @override
  int simpleAdderSync({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_simple_adder_sync(arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderSyncConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderSyncConstMeta => const TaskConstMeta(
        debugName: "simple_adder_sync",
        argNames: ["a", "b"],
      );
}

// Section: impl_wire2api

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
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
  return castInt(raw);
}

int _wire2api_i_8(dynamic raw) {
  return raw as int;
}

int _wire2api_u_16(dynamic raw) {
  return raw as int;
}

int _wire2api_u_32(dynamic raw) {
  return raw as int;
}

BigInt _wire2api_u_64(dynamic raw) {
  return castInt(raw);
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
