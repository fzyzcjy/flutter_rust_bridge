// ignore_for_file: unused_import, unused_element

import 'api/array.dart';
import 'api/chrono_type.dart';
import 'api/comment.dart';
import 'api/dart_dynamic.dart';
import 'api/enumeration.dart';
import 'api/exception.dart';
import 'api/inside_macro.dart';
import 'api/misc_example.dart';
import 'api/misc_type.dart';
import 'api/newtype_pattern.dart';
import 'api/pseudo_manual/comment_twin_sync.dart';
import 'api/pseudo_manual/enumeration_twin_sync.dart';
import 'api/pseudo_manual/exception_twin_sync.dart';
import 'api/pseudo_manual/misc_type_twin_sync.dart';
import 'api/pseudo_manual/optional_primitive.dart';
import 'api/pseudo_manual/optional_primitive_twin_sync.dart';
import 'api/pseudo_manual/primitive.dart';
import 'api/pseudo_manual/primitive_list.dart';
import 'api/pseudo_manual/primitive_list_twin_sync.dart';
import 'api/pseudo_manual/primitive_twin_sync.dart';
import 'api/pseudo_manual/simple_twin_sync.dart';
import 'api/pseudo_manual/structure_twin_sync.dart';
import 'api/raw_string.dart';
import 'api/simple.dart';
import 'api/stream.dart';
import 'api/structure.dart';
import 'api/tuple.dart';
import 'api/uuid_type.dart';
import 'auxiliary/sample_types.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.io.dart'
    if (dart.library.html) 'frb_generated.web.dart.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

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
  Future<Blob> boxedBlob({required U8Array1600 blob, dynamic hint});

  Future<TestId> funcTestId({required TestId id, dynamic hint});

  Future<U8Array5> getArray({dynamic hint});

  Future<PointArray2> getComplexArray({dynamic hint});

  Future<double> lastNumber({required F64Array16 array, dynamic hint});

  Future<TestIdArray2> nestedId({required TestIdArray4 id, dynamic hint});

  Future<MessageId> newMsgid({required U8Array32 id, dynamic hint});

  Future<FeedId> returnBoxedFeedId({required U8Array8 id, dynamic hint});

  Future<U8Array8> returnBoxedRawFeedId({required FeedId id, dynamic hint});

  Future<U8Array1600> useBoxedBlob({required Blob blob, dynamic hint});

  Future<U8Array32> useMsgid({required MessageId id, dynamic hint});

  Future<DateTime> datetimeLocal({required DateTime d, dynamic hint});

  Future<DateTime> datetimeUtc({required DateTime d, dynamic hint});

  Future<Duration> duration({required Duration d, dynamic hint});

  Future<List<DateTime>> handleDurations(
      {required List<Duration> durations,
      required DateTime since,
      dynamic hint});

  Future<List<Duration>> handleTimestamps(
      {required List<DateTime> timestamps,
      required DateTime epoch,
      dynamic hint});

  Future<Duration> howLongDoesItTake(
      {required FeatureChrono mine, dynamic hint});

  Future<DateTime> naivedatetime({required DateTime d, dynamic hint});

  Future<DateTime?> optionalEmptyDatetimeUtc({DateTime? d, dynamic hint});

  Future<TestChrono> testChrono({dynamic hint});

  Future<TestChrono> testPreciseChrono({dynamic hint});

  Future<void> structWithCommentsTwinNormalInstanceMethodTwinNormal(
      {required StructWithCommentsTwinNormal that, dynamic hint});

  Future<void> structWithCommentsTwinNormalStaticMethodTwinNormal(
      {dynamic hint});

  Future<void> functionWithCommentsSlashStarStarTwinNormal({dynamic hint});

  Future<void> functionWithCommentsTripleSlashMultiLineTwinNormal(
      {dynamic hint});

  Future<void> functionWithCommentsTripleSlashSingleLineTwinNormal(
      {dynamic hint});

  Future<dynamic> returnDartDynamic({dynamic hint});

  Future<EnumSimpleTwinNormal> funcEnumSimpleTwinNormal(
      {required EnumSimpleTwinNormal arg, dynamic hint});

  Future<EnumWithItemMixedTwinNormal> funcEnumWithItemMixedTwinNormal(
      {required EnumWithItemMixedTwinNormal arg, dynamic hint});

  Future<EnumWithItemStructTwinNormal> funcEnumWithItemStructTwinNormal(
      {required EnumWithItemStructTwinNormal arg, dynamic hint});

  Future<EnumWithItemTupleTwinNormal> funcEnumWithItemTupleTwinNormal(
      {required EnumWithItemTupleTwinNormal arg, dynamic hint});

  Future<Weekdays> handleEnumParameter(
      {required Weekdays weekday, dynamic hint});

  Future<Weekdays?> handleReturnEnum({required String input, dynamic hint});

  Future<Measure?> multiplyByTen({required Measure measure, dynamic hint});

  Future<Uint8List> printNote({required Note note, dynamic hint});

  Future<void> customEnumErrorPanicTwinNormal({dynamic hint});

  Future<int> customEnumErrorReturnErrorTwinNormal({dynamic hint});

  Future<int> customEnumErrorReturnOkTwinNormal(
      {required int arg, dynamic hint});

  Future<void> customNestedErrorReturnErrorTwinNormal(
      {required CustomNestedErrorOuterTwinNormal arg, dynamic hint});

  Future<void> customStructErrorReturnErrorTwinNormal(
      {required CustomStructErrorTwinNormal arg, dynamic hint});

  Future<int> funcReturnErrorTwinNormal({dynamic hint});

  Future<int> funcTypeFalliblePanicTwinNormal({dynamic hint});

  Future<int> funcTypeInfalliblePanicTwinNormal({dynamic hint});

  Future<MacroStruct> funcMacroStruct({required MacroStruct arg, dynamic hint});

  Future<BigBuffers> handleBigBuffers({dynamic hint});

  Future<MyTreeNode> handleComplexStruct({required MyTreeNode s, dynamic hint});

  Future<MyNestedStruct> handleNestedStruct(
      {required MyNestedStruct s, dynamic hint});

  Future<List<Weekdays>> listOfPrimitiveEnums(
      {required List<Weekdays> weekdays, dynamic hint});

  Future<Abc> testAbcEnum({required Abc abc, dynamic hint});

  Future<StructWithEnum> testStructWithEnum(
      {required StructWithEnum se, dynamic hint});

  Future<void> funcReturnUnitTwinNormal({dynamic hint});

  Future<String> funcStringTwinNormal({required String arg, dynamic hint});

  Future<List<MySize>> handleListOfStruct(
      {required List<MySize> l, dynamic hint});

  Future<List<String>> handleStringList(
      {required List<String> names, dynamic hint});

  Future<NewTypeInt> handleNewtype({required NewTypeInt arg, dynamic hint});

  void structWithCommentsTwinSyncInstanceMethodTwinSync(
      {required StructWithCommentsTwinSync that, dynamic hint});

  void structWithCommentsTwinSyncStaticMethodTwinSync({dynamic hint});

  void functionWithCommentsSlashStarStarTwinSync({dynamic hint});

  void functionWithCommentsTripleSlashMultiLineTwinSync({dynamic hint});

  void functionWithCommentsTripleSlashSingleLineTwinSync({dynamic hint});

  EnumSimpleTwinSync funcEnumSimpleTwinSync(
      {required EnumSimpleTwinSync arg, dynamic hint});

  EnumWithItemMixedTwinSync funcEnumWithItemMixedTwinSync(
      {required EnumWithItemMixedTwinSync arg, dynamic hint});

  EnumWithItemStructTwinSync funcEnumWithItemStructTwinSync(
      {required EnumWithItemStructTwinSync arg, dynamic hint});

  EnumWithItemTupleTwinSync funcEnumWithItemTupleTwinSync(
      {required EnumWithItemTupleTwinSync arg, dynamic hint});

  void customEnumErrorPanicTwinSync({dynamic hint});

  int customEnumErrorReturnErrorTwinSync({dynamic hint});

  int customEnumErrorReturnOkTwinSync({required int arg, dynamic hint});

  void customNestedErrorReturnErrorTwinSync(
      {required CustomNestedErrorOuterTwinSync arg, dynamic hint});

  void customStructErrorReturnErrorTwinSync(
      {required CustomStructErrorTwinSync arg, dynamic hint});

  int funcReturnErrorTwinSync({dynamic hint});

  int funcTypeFalliblePanicTwinSync({dynamic hint});

  int funcTypeInfalliblePanicTwinSync({dynamic hint});

  void funcReturnUnitTwinSync({dynamic hint});

  String funcStringTwinSync({required String arg, dynamic hint});

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

  Future<List<bool>> examplePrimitiveListTypeBoolTwinNormal(
      {required List<bool> arg, dynamic hint});

  Future<Float32List> examplePrimitiveListTypeF32TwinNormal(
      {required Float32List arg, dynamic hint});

  Future<Float64List> examplePrimitiveListTypeF64TwinNormal(
      {required Float64List arg, dynamic hint});

  Future<Int16List> examplePrimitiveListTypeI16TwinNormal(
      {required Int16List arg, dynamic hint});

  Future<Int32List> examplePrimitiveListTypeI32TwinNormal(
      {required Int32List arg, dynamic hint});

  Future<Int64List> examplePrimitiveListTypeI64TwinNormal(
      {required Int64List arg, dynamic hint});

  Future<Int8List> examplePrimitiveListTypeI8TwinNormal(
      {required Int8List arg, dynamic hint});

  Future<Uint16List> examplePrimitiveListTypeU16TwinNormal(
      {required Uint16List arg, dynamic hint});

  Future<Uint32List> examplePrimitiveListTypeU32TwinNormal(
      {required Uint32List arg, dynamic hint});

  Future<Uint64List> examplePrimitiveListTypeU64TwinNormal(
      {required Uint64List arg, dynamic hint});

  Future<Uint8List> examplePrimitiveListTypeU8TwinNormal(
      {required Uint8List arg, dynamic hint});

  List<bool> examplePrimitiveListTypeBoolTwinSync(
      {required List<bool> arg, dynamic hint});

  Float32List examplePrimitiveListTypeF32TwinSync(
      {required Float32List arg, dynamic hint});

  Float64List examplePrimitiveListTypeF64TwinSync(
      {required Float64List arg, dynamic hint});

  Int16List examplePrimitiveListTypeI16TwinSync(
      {required Int16List arg, dynamic hint});

  Int32List examplePrimitiveListTypeI32TwinSync(
      {required Int32List arg, dynamic hint});

  Int64List examplePrimitiveListTypeI64TwinSync(
      {required Int64List arg, dynamic hint});

  Int8List examplePrimitiveListTypeI8TwinSync(
      {required Int8List arg, dynamic hint});

  Uint16List examplePrimitiveListTypeU16TwinSync(
      {required Uint16List arg, dynamic hint});

  Uint32List examplePrimitiveListTypeU32TwinSync(
      {required Uint32List arg, dynamic hint});

  Uint64List examplePrimitiveListTypeU64TwinSync(
      {required Uint64List arg, dynamic hint});

  Uint8List examplePrimitiveListTypeU8TwinSync(
      {required Uint8List arg, dynamic hint});

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

  int simpleAdderTwinSync({required int a, required int b, dynamic hint});

  StructWithOneFieldTwinSync funcStructWithOneFieldTwinSync(
      {required StructWithOneFieldTwinSync arg, dynamic hint});

  StructWithTwoFieldTwinSync funcStructWithTwoFieldTwinSync(
      {required StructWithTwoFieldTwinSync arg, dynamic hint});

  StructWithZeroFieldTwinSync funcStructWithZeroFieldTwinSync(
      {required StructWithZeroFieldTwinSync arg, dynamic hint});

  TupleStructWithOneFieldTwinSync funcTupleStructWithOneFieldTwinSync(
      {required TupleStructWithOneFieldTwinSync arg, dynamic hint});

  TupleStructWithTwoFieldTwinSync funcTupleStructWithTwoFieldTwinSync(
      {required TupleStructWithTwoFieldTwinSync arg, dynamic hint});

  Future<MoreThanJustOneRawStringStruct> testMoreThanJustOneRawStringStruct(
      {dynamic hint});

  Future<RawStringItemStruct> testRawStringItemStruct({dynamic hint});

  Future<int> simpleAdderTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<String> funcStreamRealisticTwinNormal(
      {required String arg, dynamic hint});

  Stream<String> funcStreamReturnErrorTwinNormal({dynamic hint});

  Stream<String> funcStreamReturnPanicTwinNormal({dynamic hint});

  Stream<int> funcStreamSinkArgPositionTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<MyStreamEntry> handleStreamOfStruct({dynamic hint});

  Future<StructWithOneFieldTwinNormal> funcStructWithOneFieldTwinNormal(
      {required StructWithOneFieldTwinNormal arg, dynamic hint});

  Future<StructWithTwoFieldTwinNormal> funcStructWithTwoFieldTwinNormal(
      {required StructWithTwoFieldTwinNormal arg, dynamic hint});

  Future<StructWithZeroFieldTwinNormal> funcStructWithZeroFieldTwinNormal(
      {required StructWithZeroFieldTwinNormal arg, dynamic hint});

  Future<TupleStructWithOneFieldTwinNormal>
      funcTupleStructWithOneFieldTwinNormal(
          {required TupleStructWithOneFieldTwinNormal arg, dynamic hint});

  Future<TupleStructWithTwoFieldTwinNormal>
      funcTupleStructWithTwoFieldTwinNormal(
          {required TupleStructWithTwoFieldTwinNormal arg, dynamic hint});

  Future<(String, int)> testTuple({(String, int)? value, dynamic hint});

  Future<void> testTuple2({required List<(String, int)> value, dynamic hint});

  Future<FeatureUuid> handleNestedUuids(
      {required FeatureUuid ids, dynamic hint});

  Future<UuidValue> handleUuid({required UuidValue id, dynamic hint});

  Future<List<UuidValue>> handleUuids(
      {required List<UuidValue> ids, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @override
  Future<Blob> boxedBlob({required U8Array1600 blob, dynamic hint}) {
    var arg0 = api2wire_box_u_8_array_1600(blob);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_boxed_blob(port_, arg0),
      parseSuccessData: _wire2api_blob,
      parseErrorData: null,
      constMeta: kBoxedBlobConstMeta,
      argValues: [blob],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kBoxedBlobConstMeta => const TaskConstMeta(
        debugName: "boxed_blob",
        argNames: ["blob"],
      );

  @override
  Future<TestId> funcTestId({required TestId id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_test_id(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_test_id(port_, arg0),
      parseSuccessData: _wire2api_test_id,
      parseErrorData: null,
      constMeta: kFuncTestIdConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTestIdConstMeta => const TaskConstMeta(
        debugName: "func_test_id",
        argNames: ["id"],
      );

  @override
  Future<U8Array5> getArray({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_array(port_),
      parseSuccessData: _wire2api_u_8_array_5,
      parseErrorData: null,
      constMeta: kGetArrayConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetArrayConstMeta => const TaskConstMeta(
        debugName: "get_array",
        argNames: [],
      );

  @override
  Future<PointArray2> getComplexArray({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_complex_array(port_),
      parseSuccessData: _wire2api_point_array_2,
      parseErrorData: null,
      constMeta: kGetComplexArrayConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetComplexArrayConstMeta => const TaskConstMeta(
        debugName: "get_complex_array",
        argNames: [],
      );

  @override
  Future<double> lastNumber({required F64Array16 array, dynamic hint}) {
    var arg0 = api2wire_f_64_array_16(array);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_last_number(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kLastNumberConstMeta,
      argValues: [array],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLastNumberConstMeta => const TaskConstMeta(
        debugName: "last_number",
        argNames: ["array"],
      );

  @override
  Future<TestIdArray2> nestedId({required TestIdArray4 id, dynamic hint}) {
    var arg0 = api2wire_test_id_array_4(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_nested_id(port_, arg0),
      parseSuccessData: _wire2api_test_id_array_2,
      parseErrorData: null,
      constMeta: kNestedIdConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNestedIdConstMeta => const TaskConstMeta(
        debugName: "nested_id",
        argNames: ["id"],
      );

  @override
  Future<MessageId> newMsgid({required U8Array32 id, dynamic hint}) {
    var arg0 = api2wire_u_8_array_32(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_new_msgid(port_, arg0),
      parseSuccessData: _wire2api_message_id,
      parseErrorData: null,
      constMeta: kNewMsgidConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNewMsgidConstMeta => const TaskConstMeta(
        debugName: "new_msgid",
        argNames: ["id"],
      );

  @override
  Future<FeedId> returnBoxedFeedId({required U8Array8 id, dynamic hint}) {
    var arg0 = api2wire_u_8_array_8(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_boxed_feed_id(port_, arg0),
      parseSuccessData: _wire2api_box_feed_id,
      parseErrorData: null,
      constMeta: kReturnBoxedFeedIdConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnBoxedFeedIdConstMeta => const TaskConstMeta(
        debugName: "return_boxed_feed_id",
        argNames: ["id"],
      );

  @override
  Future<U8Array8> returnBoxedRawFeedId({required FeedId id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feed_id(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_boxed_raw_feed_id(port_, arg0),
      parseSuccessData: _wire2api_box_u_8_array_8,
      parseErrorData: null,
      constMeta: kReturnBoxedRawFeedIdConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnBoxedRawFeedIdConstMeta => const TaskConstMeta(
        debugName: "return_boxed_raw_feed_id",
        argNames: ["id"],
      );

  @override
  Future<U8Array1600> useBoxedBlob({required Blob blob, dynamic hint}) {
    var arg0 = api2wire_box_blob(blob);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_boxed_blob(port_, arg0),
      parseSuccessData: _wire2api_u_8_array_1600,
      parseErrorData: null,
      constMeta: kUseBoxedBlobConstMeta,
      argValues: [blob],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseBoxedBlobConstMeta => const TaskConstMeta(
        debugName: "use_boxed_blob",
        argNames: ["blob"],
      );

  @override
  Future<U8Array32> useMsgid({required MessageId id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_message_id(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_msgid(port_, arg0),
      parseSuccessData: _wire2api_u_8_array_32,
      parseErrorData: null,
      constMeta: kUseMsgidConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseMsgidConstMeta => const TaskConstMeta(
        debugName: "use_msgid",
        argNames: ["id"],
      );

  @override
  Future<DateTime> datetimeLocal({required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Local(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_datetime_local(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Local,
      parseErrorData: null,
      constMeta: kDatetimeLocalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDatetimeLocalConstMeta => const TaskConstMeta(
        debugName: "datetime_local",
        argNames: ["d"],
      );

  @override
  Future<DateTime> datetimeUtc({required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Utc(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_datetime_utc(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Utc,
      parseErrorData: null,
      constMeta: kDatetimeUtcConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDatetimeUtcConstMeta => const TaskConstMeta(
        debugName: "datetime_utc",
        argNames: ["d"],
      );

  @override
  Future<Duration> duration({required Duration d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Duration(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_duration(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Duration,
      parseErrorData: null,
      constMeta: kDurationConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDurationConstMeta => const TaskConstMeta(
        debugName: "duration",
        argNames: ["d"],
      );

  @override
  Future<List<DateTime>> handleDurations(
      {required List<Duration> durations,
      required DateTime since,
      dynamic hint}) {
    var arg0 = api2wire_Chrono_DurationList(durations);
    var arg1 = api2wire_Chrono_Local(since);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_durations(port_, arg0, arg1),
      parseSuccessData: _wire2api_Chrono_LocalList,
      parseErrorData: null,
      constMeta: kHandleDurationsConstMeta,
      argValues: [durations, since],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleDurationsConstMeta => const TaskConstMeta(
        debugName: "handle_durations",
        argNames: ["durations", "since"],
      );

  @override
  Future<List<Duration>> handleTimestamps(
      {required List<DateTime> timestamps,
      required DateTime epoch,
      dynamic hint}) {
    var arg0 = api2wire_Chrono_NaiveList(timestamps);
    var arg1 = api2wire_Chrono_Naive(epoch);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_timestamps(port_, arg0, arg1),
      parseSuccessData: _wire2api_Chrono_DurationList,
      parseErrorData: null,
      constMeta: kHandleTimestampsConstMeta,
      argValues: [timestamps, epoch],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTimestampsConstMeta => const TaskConstMeta(
        debugName: "handle_timestamps",
        argNames: ["timestamps", "epoch"],
      );

  @override
  Future<Duration> howLongDoesItTake(
      {required FeatureChrono mine, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feature_chrono(mine);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_how_long_does_it_take(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Duration,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHowLongDoesItTakeConstMeta,
      argValues: [mine],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHowLongDoesItTakeConstMeta => const TaskConstMeta(
        debugName: "how_long_does_it_take",
        argNames: ["mine"],
      );

  @override
  Future<DateTime> naivedatetime({required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Naive(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_naivedatetime(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Naive,
      parseErrorData: null,
      constMeta: kNaivedatetimeConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNaivedatetimeConstMeta => const TaskConstMeta(
        debugName: "naivedatetime",
        argNames: ["d"],
      );

  @override
  Future<DateTime?> optionalEmptyDatetimeUtc({DateTime? d, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_Chrono_Utc(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_optional_empty_datetime_utc(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_Chrono_Utc,
      parseErrorData: null,
      constMeta: kOptionalEmptyDatetimeUtcConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOptionalEmptyDatetimeUtcConstMeta => const TaskConstMeta(
        debugName: "optional_empty_datetime_utc",
        argNames: ["d"],
      );

  @override
  Future<TestChrono> testChrono({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_chrono(port_),
      parseSuccessData: _wire2api_test_chrono,
      parseErrorData: null,
      constMeta: kTestChronoConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestChronoConstMeta => const TaskConstMeta(
        debugName: "test_chrono",
        argNames: [],
      );

  @override
  Future<TestChrono> testPreciseChrono({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_precise_chrono(port_),
      parseSuccessData: _wire2api_test_chrono,
      parseErrorData: null,
      constMeta: kTestPreciseChronoConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestPreciseChronoConstMeta => const TaskConstMeta(
        debugName: "test_precise_chrono",
        argNames: [],
      );

  @override
  Future<void> structWithCommentsTwinNormalInstanceMethodTwinNormal(
      {required StructWithCommentsTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_comments_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinNormalInstanceMethodTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kStructWithCommentsTwinNormalInstanceMethodTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "StructWithCommentsTwinNormal_instance_method_twin_normal",
            argNames: ["that"],
          );

  @override
  Future<void> structWithCommentsTwinNormalStaticMethodTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kStructWithCommentsTwinNormalStaticMethodTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kStructWithCommentsTwinNormalStaticMethodTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName: "StructWithCommentsTwinNormal_static_method_twin_normal",
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
  Future<dynamic> returnDartDynamic({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_dart_dynamic(port_),
      parseSuccessData: _wire2api_dartabi,
      parseErrorData: null,
      constMeta: kReturnDartDynamicConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnDartDynamicConstMeta => const TaskConstMeta(
        debugName: "return_dart_dynamic",
        argNames: [],
      );

  @override
  Future<EnumSimpleTwinNormal> funcEnumSimpleTwinNormal(
      {required EnumSimpleTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_enum_simple_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_enum_simple_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_enum_simple_twin_normal,
      parseErrorData: null,
      constMeta: kFuncEnumSimpleTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumSimpleTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_enum_simple_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<EnumWithItemMixedTwinNormal> funcEnumWithItemMixedTwinNormal(
      {required EnumWithItemMixedTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_mixed_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_enum_with_item_mixed_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_enum_with_item_mixed_twin_normal,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemMixedTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemMixedTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_mixed_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<EnumWithItemStructTwinNormal> funcEnumWithItemStructTwinNormal(
      {required EnumWithItemStructTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_struct_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_enum_with_item_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_enum_with_item_struct_twin_normal,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemStructTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_struct_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<EnumWithItemTupleTwinNormal> funcEnumWithItemTupleTwinNormal(
      {required EnumWithItemTupleTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_tuple_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_enum_with_item_tuple_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_enum_with_item_tuple_twin_normal,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemTupleTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemTupleTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_tuple_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Weekdays> handleEnumParameter(
      {required Weekdays weekday, dynamic hint}) {
    var arg0 = api2wire_weekdays(weekday);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_enum_parameter(port_, arg0),
      parseSuccessData: _wire2api_weekdays,
      parseErrorData: null,
      constMeta: kHandleEnumParameterConstMeta,
      argValues: [weekday],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleEnumParameterConstMeta => const TaskConstMeta(
        debugName: "handle_enum_parameter",
        argNames: ["weekday"],
      );

  @override
  Future<Weekdays?> handleReturnEnum({required String input, dynamic hint}) {
    var arg0 = api2wire_String(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_return_enum(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_weekdays,
      parseErrorData: null,
      constMeta: kHandleReturnEnumConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleReturnEnumConstMeta => const TaskConstMeta(
        debugName: "handle_return_enum",
        argNames: ["input"],
      );

  @override
  Future<Measure?> multiplyByTen({required Measure measure, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_measure(measure);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_multiply_by_ten(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_measure,
      parseErrorData: null,
      constMeta: kMultiplyByTenConstMeta,
      argValues: [measure],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMultiplyByTenConstMeta => const TaskConstMeta(
        debugName: "multiply_by_ten",
        argNames: ["measure"],
      );

  @override
  Future<Uint8List> printNote({required Note note, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_note(note);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_print_note(port_, arg0),
      parseSuccessData: _wire2api_ZeroCopyBuffer_list_prim_u_8,
      parseErrorData: null,
      constMeta: kPrintNoteConstMeta,
      argValues: [note],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrintNoteConstMeta => const TaskConstMeta(
        debugName: "print_note",
        argNames: ["note"],
      );

  @override
  Future<void> customEnumErrorPanicTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_custom_enum_error_panic_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_enum_error_twin_normal,
      constMeta: kCustomEnumErrorPanicTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorPanicTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_panic_twin_normal",
        argNames: [],
      );

  @override
  Future<int> customEnumErrorReturnErrorTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_custom_enum_error_return_error_twin_normal(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_enum_error_twin_normal,
      constMeta: kCustomEnumErrorReturnErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorReturnErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_return_error_twin_normal",
        argNames: [],
      );

  @override
  Future<int> customEnumErrorReturnOkTwinNormal(
      {required int arg, dynamic hint}) {
    var arg0 = api2wire_u_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_custom_enum_error_return_ok_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_enum_error_twin_normal,
      constMeta: kCustomEnumErrorReturnOkTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorReturnOkTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_return_ok_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<void> customNestedErrorReturnErrorTwinNormal(
      {required CustomNestedErrorOuterTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_nested_error_outer_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_custom_nested_error_return_error_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_outer_twin_normal,
      constMeta: kCustomNestedErrorReturnErrorTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomNestedErrorReturnErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "custom_nested_error_return_error_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<void> customStructErrorReturnErrorTwinNormal(
      {required CustomStructErrorTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct_error_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_custom_struct_error_return_error_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_twin_normal,
      constMeta: kCustomStructErrorReturnErrorTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructErrorReturnErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "custom_struct_error_return_error_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<int> funcReturnErrorTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_return_error_twin_normal(port_),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncReturnErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncReturnErrorTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_return_error_twin_normal",
        argNames: [],
      );

  @override
  Future<int> funcTypeFalliblePanicTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_type_fallible_panic_twin_normal(port_),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncTypeFalliblePanicTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTypeFalliblePanicTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_type_fallible_panic_twin_normal",
        argNames: [],
      );

  @override
  Future<int> funcTypeInfalliblePanicTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_type_infallible_panic_twin_normal(port_),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kFuncTypeInfalliblePanicTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTypeInfalliblePanicTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_type_infallible_panic_twin_normal",
        argNames: [],
      );

  @override
  Future<MacroStruct> funcMacroStruct(
      {required MacroStruct arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_macro_struct(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_macro_struct(port_, arg0),
      parseSuccessData: _wire2api_macro_struct,
      parseErrorData: null,
      constMeta: kFuncMacroStructConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncMacroStructConstMeta => const TaskConstMeta(
        debugName: "func_macro_struct",
        argNames: ["arg"],
      );

  @override
  Future<BigBuffers> handleBigBuffers({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_big_buffers(port_),
      parseSuccessData: _wire2api_big_buffers,
      parseErrorData: null,
      constMeta: kHandleBigBuffersConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleBigBuffersConstMeta => const TaskConstMeta(
        debugName: "handle_big_buffers",
        argNames: [],
      );

  @override
  Future<MyTreeNode> handleComplexStruct(
      {required MyTreeNode s, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_tree_node(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_complex_struct(port_, arg0),
      parseSuccessData: _wire2api_my_tree_node,
      parseErrorData: null,
      constMeta: kHandleComplexStructConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleComplexStructConstMeta => const TaskConstMeta(
        debugName: "handle_complex_struct",
        argNames: ["s"],
      );

  @override
  Future<MyNestedStruct> handleNestedStruct(
      {required MyNestedStruct s, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_nested_struct(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_nested_struct(port_, arg0),
      parseSuccessData: _wire2api_my_nested_struct,
      parseErrorData: null,
      constMeta: kHandleNestedStructConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNestedStructConstMeta => const TaskConstMeta(
        debugName: "handle_nested_struct",
        argNames: ["s"],
      );

  @override
  Future<List<Weekdays>> listOfPrimitiveEnums(
      {required List<Weekdays> weekdays, dynamic hint}) {
    var arg0 = api2wire_list_weekdays(weekdays);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_list_of_primitive_enums(port_, arg0),
      parseSuccessData: _wire2api_list_weekdays,
      parseErrorData: null,
      constMeta: kListOfPrimitiveEnumsConstMeta,
      argValues: [weekdays],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kListOfPrimitiveEnumsConstMeta => const TaskConstMeta(
        debugName: "list_of_primitive_enums",
        argNames: ["weekdays"],
      );

  @override
  Future<Abc> testAbcEnum({required Abc abc, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_abc(abc);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_abc_enum(port_, arg0),
      parseSuccessData: _wire2api_abc,
      parseErrorData: null,
      constMeta: kTestAbcEnumConstMeta,
      argValues: [abc],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestAbcEnumConstMeta => const TaskConstMeta(
        debugName: "test_abc_enum",
        argNames: ["abc"],
      );

  @override
  Future<StructWithEnum> testStructWithEnum(
      {required StructWithEnum se, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_enum(se);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_struct_with_enum(port_, arg0),
      parseSuccessData: _wire2api_struct_with_enum,
      parseErrorData: null,
      constMeta: kTestStructWithEnumConstMeta,
      argValues: [se],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestStructWithEnumConstMeta => const TaskConstMeta(
        debugName: "test_struct_with_enum",
        argNames: ["se"],
      );

  @override
  Future<void> funcReturnUnitTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_return_unit_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFuncReturnUnitTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncReturnUnitTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_return_unit_twin_normal",
        argNames: [],
      );

  @override
  Future<String> funcStringTwinNormal({required String arg, dynamic hint}) {
    var arg0 = api2wire_String(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_string_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kFuncStringTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStringTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_string_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<List<MySize>> handleListOfStruct(
      {required List<MySize> l, dynamic hint}) {
    var arg0 = api2wire_list_my_size(l);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_list_of_struct(port_, arg0),
      parseSuccessData: _wire2api_list_my_size,
      parseErrorData: null,
      constMeta: kHandleListOfStructConstMeta,
      argValues: [l],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleListOfStructConstMeta => const TaskConstMeta(
        debugName: "handle_list_of_struct",
        argNames: ["l"],
      );

  @override
  Future<List<String>> handleStringList(
      {required List<String> names, dynamic hint}) {
    var arg0 = api2wire_StringList(names);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_string_list(port_, arg0),
      parseSuccessData: _wire2api_StringList,
      parseErrorData: null,
      constMeta: kHandleStringListConstMeta,
      argValues: [names],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStringListConstMeta => const TaskConstMeta(
        debugName: "handle_string_list",
        argNames: ["names"],
      );

  @override
  Future<NewTypeInt> handleNewtype({required NewTypeInt arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_new_type_int(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_newtype(port_, arg0),
      parseSuccessData: _wire2api_new_type_int,
      parseErrorData: null,
      constMeta: kHandleNewtypeConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNewtypeConstMeta => const TaskConstMeta(
        debugName: "handle_newtype",
        argNames: ["arg"],
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
  EnumSimpleTwinSync funcEnumSimpleTwinSync(
      {required EnumSimpleTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_enum_simple_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_enum_simple_twin_sync(arg0),
      parseSuccessData: _wire2api_enum_simple_twin_sync,
      parseErrorData: null,
      constMeta: kFuncEnumSimpleTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumSimpleTwinSyncConstMeta => const TaskConstMeta(
        debugName: "func_enum_simple_twin_sync",
        argNames: ["arg"],
      );

  @override
  EnumWithItemMixedTwinSync funcEnumWithItemMixedTwinSync(
      {required EnumWithItemMixedTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_mixed_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_enum_with_item_mixed_twin_sync(arg0),
      parseSuccessData: _wire2api_enum_with_item_mixed_twin_sync,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemMixedTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemMixedTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_mixed_twin_sync",
        argNames: ["arg"],
      );

  @override
  EnumWithItemStructTwinSync funcEnumWithItemStructTwinSync(
      {required EnumWithItemStructTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_struct_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_enum_with_item_struct_twin_sync(arg0),
      parseSuccessData: _wire2api_enum_with_item_struct_twin_sync,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemStructTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemStructTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_struct_twin_sync",
        argNames: ["arg"],
      );

  @override
  EnumWithItemTupleTwinSync funcEnumWithItemTupleTwinSync(
      {required EnumWithItemTupleTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_with_item_tuple_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_enum_with_item_tuple_twin_sync(arg0),
      parseSuccessData: _wire2api_enum_with_item_tuple_twin_sync,
      parseErrorData: null,
      constMeta: kFuncEnumWithItemTupleTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncEnumWithItemTupleTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_enum_with_item_tuple_twin_sync",
        argNames: ["arg"],
      );

  @override
  void customEnumErrorPanicTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_custom_enum_error_panic_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_enum_error_twin_sync,
      constMeta: kCustomEnumErrorPanicTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorPanicTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_panic_twin_sync",
        argNames: [],
      );

  @override
  int customEnumErrorReturnErrorTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_custom_enum_error_return_error_twin_sync(),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_enum_error_twin_sync,
      constMeta: kCustomEnumErrorReturnErrorTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorReturnErrorTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_return_error_twin_sync",
        argNames: [],
      );

  @override
  int customEnumErrorReturnOkTwinSync({required int arg, dynamic hint}) {
    var arg0 = api2wire_u_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_custom_enum_error_return_ok_twin_sync(arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_enum_error_twin_sync,
      constMeta: kCustomEnumErrorReturnOkTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomEnumErrorReturnOkTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "custom_enum_error_return_ok_twin_sync",
        argNames: ["arg"],
      );

  @override
  void customNestedErrorReturnErrorTwinSync(
      {required CustomNestedErrorOuterTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_nested_error_outer_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_custom_nested_error_return_error_twin_sync(arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_outer_twin_sync,
      constMeta: kCustomNestedErrorReturnErrorTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomNestedErrorReturnErrorTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "custom_nested_error_return_error_twin_sync",
        argNames: ["arg"],
      );

  @override
  void customStructErrorReturnErrorTwinSync(
      {required CustomStructErrorTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct_error_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_custom_struct_error_return_error_twin_sync(arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_twin_sync,
      constMeta: kCustomStructErrorReturnErrorTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructErrorReturnErrorTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "custom_struct_error_return_error_twin_sync",
        argNames: ["arg"],
      );

  @override
  int funcReturnErrorTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_return_error_twin_sync(),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncReturnErrorTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncReturnErrorTwinSyncConstMeta => const TaskConstMeta(
        debugName: "func_return_error_twin_sync",
        argNames: [],
      );

  @override
  int funcTypeFalliblePanicTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_type_fallible_panic_twin_sync(),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncTypeFalliblePanicTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTypeFalliblePanicTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_type_fallible_panic_twin_sync",
        argNames: [],
      );

  @override
  int funcTypeInfalliblePanicTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_type_infallible_panic_twin_sync(),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kFuncTypeInfalliblePanicTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTypeInfalliblePanicTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_type_infallible_panic_twin_sync",
        argNames: [],
      );

  @override
  void funcReturnUnitTwinSync({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_return_unit_twin_sync(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kFuncReturnUnitTwinSyncConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncReturnUnitTwinSyncConstMeta => const TaskConstMeta(
        debugName: "func_return_unit_twin_sync",
        argNames: [],
      );

  @override
  String funcStringTwinSync({required String arg, dynamic hint}) {
    var arg0 = api2wire_String(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_string_twin_sync(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kFuncStringTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStringTwinSyncConstMeta => const TaskConstMeta(
        debugName: "func_string_twin_sync",
        argNames: ["arg"],
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
  Future<List<bool>> examplePrimitiveListTypeBoolTwinNormal(
      {required List<bool> arg, dynamic hint}) {
    var arg0 = api2wire_list_bool(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_bool_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_bool,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeBoolTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeBoolTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_bool_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Float32List> examplePrimitiveListTypeF32TwinNormal(
      {required Float32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_f_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_f32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_f_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeF32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeF32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_f32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Float64List> examplePrimitiveListTypeF64TwinNormal(
      {required Float64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_f_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_f64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_f_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeF64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeF64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_f64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Int16List> examplePrimitiveListTypeI16TwinNormal(
      {required Int16List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_i16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_i_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Int32List> examplePrimitiveListTypeI32TwinNormal(
      {required Int32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_i32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_i_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Int64List> examplePrimitiveListTypeI64TwinNormal(
      {required Int64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_i64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_i_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Int8List> examplePrimitiveListTypeI8TwinNormal(
      {required Int8List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_i8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_i_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i8_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Uint16List> examplePrimitiveListTypeU16TwinNormal(
      {required Uint16List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_16(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_u16_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU16TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU16TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u16_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Uint32List> examplePrimitiveListTypeU32TwinNormal(
      {required Uint32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_32(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_u32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU32TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU32TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u32_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Uint64List> examplePrimitiveListTypeU64TwinNormal(
      {required Uint64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_64(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_u64_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU64TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU64TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u64_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<Uint8List> examplePrimitiveListTypeU8TwinNormal(
      {required Uint8List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_8(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_example_primitive_list_type_u8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU8TwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU8TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u8_twin_normal",
        argNames: ["arg"],
      );

  @override
  List<bool> examplePrimitiveListTypeBoolTwinSync(
      {required List<bool> arg, dynamic hint}) {
    var arg0 = api2wire_list_bool(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_bool_twin_sync(arg0),
      parseSuccessData: _wire2api_list_bool,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeBoolTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeBoolTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_bool_twin_sync",
        argNames: ["arg"],
      );

  @override
  Float32List examplePrimitiveListTypeF32TwinSync(
      {required Float32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_f_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_f32_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_f_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeF32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeF32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_f32_twin_sync",
        argNames: ["arg"],
      );

  @override
  Float64List examplePrimitiveListTypeF64TwinSync(
      {required Float64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_f_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_f64_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_f_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeF64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeF64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_f64_twin_sync",
        argNames: ["arg"],
      );

  @override
  Int16List examplePrimitiveListTypeI16TwinSync(
      {required Int16List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_i16_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_i_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i16_twin_sync",
        argNames: ["arg"],
      );

  @override
  Int32List examplePrimitiveListTypeI32TwinSync(
      {required Int32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_i32_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_i_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i32_twin_sync",
        argNames: ["arg"],
      );

  @override
  Int64List examplePrimitiveListTypeI64TwinSync(
      {required Int64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_i64_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_i_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i64_twin_sync",
        argNames: ["arg"],
      );

  @override
  Int8List examplePrimitiveListTypeI8TwinSync(
      {required Int8List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_i_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_i8_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_i_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeI8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeI8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_i8_twin_sync",
        argNames: ["arg"],
      );

  @override
  Uint16List examplePrimitiveListTypeU16TwinSync(
      {required Uint16List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_16(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_u16_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_u_16,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU16TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU16TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u16_twin_sync",
        argNames: ["arg"],
      );

  @override
  Uint32List examplePrimitiveListTypeU32TwinSync(
      {required Uint32List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_32(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_u32_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_u_32,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU32TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU32TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u32_twin_sync",
        argNames: ["arg"],
      );

  @override
  Uint64List examplePrimitiveListTypeU64TwinSync(
      {required Uint64List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_64(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_u64_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_u_64,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU64TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU64TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u64_twin_sync",
        argNames: ["arg"],
      );

  @override
  Uint8List examplePrimitiveListTypeU8TwinSync(
      {required Uint8List arg, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_8(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_example_primitive_list_type_u8_twin_sync(arg0),
      parseSuccessData: _wire2api_list_prim_u_8,
      parseErrorData: null,
      constMeta: kExamplePrimitiveListTypeU8TwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kExamplePrimitiveListTypeU8TwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "example_primitive_list_type_u8_twin_sync",
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

  @override
  StructWithOneFieldTwinSync funcStructWithOneFieldTwinSync(
      {required StructWithOneFieldTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_one_field_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_struct_with_one_field_twin_sync(arg0),
      parseSuccessData: _wire2api_struct_with_one_field_twin_sync,
      parseErrorData: null,
      constMeta: kFuncStructWithOneFieldTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithOneFieldTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_one_field_twin_sync",
        argNames: ["arg"],
      );

  @override
  StructWithTwoFieldTwinSync funcStructWithTwoFieldTwinSync(
      {required StructWithTwoFieldTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_two_field_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_struct_with_two_field_twin_sync(arg0),
      parseSuccessData: _wire2api_struct_with_two_field_twin_sync,
      parseErrorData: null,
      constMeta: kFuncStructWithTwoFieldTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithTwoFieldTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_two_field_twin_sync",
        argNames: ["arg"],
      );

  @override
  StructWithZeroFieldTwinSync funcStructWithZeroFieldTwinSync(
      {required StructWithZeroFieldTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_zero_field_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_struct_with_zero_field_twin_sync(arg0),
      parseSuccessData: _wire2api_struct_with_zero_field_twin_sync,
      parseErrorData: null,
      constMeta: kFuncStructWithZeroFieldTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithZeroFieldTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_zero_field_twin_sync",
        argNames: ["arg"],
      );

  @override
  TupleStructWithOneFieldTwinSync funcTupleStructWithOneFieldTwinSync(
      {required TupleStructWithOneFieldTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_tuple_struct_with_one_field_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_tuple_struct_with_one_field_twin_sync(arg0),
      parseSuccessData: _wire2api_tuple_struct_with_one_field_twin_sync,
      parseErrorData: null,
      constMeta: kFuncTupleStructWithOneFieldTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTupleStructWithOneFieldTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_tuple_struct_with_one_field_twin_sync",
        argNames: ["arg"],
      );

  @override
  TupleStructWithTwoFieldTwinSync funcTupleStructWithTwoFieldTwinSync(
      {required TupleStructWithTwoFieldTwinSync arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_tuple_struct_with_two_field_twin_sync(arg);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_func_tuple_struct_with_two_field_twin_sync(arg0),
      parseSuccessData: _wire2api_tuple_struct_with_two_field_twin_sync,
      parseErrorData: null,
      constMeta: kFuncTupleStructWithTwoFieldTwinSyncConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTupleStructWithTwoFieldTwinSyncConstMeta =>
      const TaskConstMeta(
        debugName: "func_tuple_struct_with_two_field_twin_sync",
        argNames: ["arg"],
      );

  @override
  Future<MoreThanJustOneRawStringStruct> testMoreThanJustOneRawStringStruct(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_more_than_just_one_raw_string_struct(port_),
      parseSuccessData: _wire2api_more_than_just_one_raw_string_struct,
      parseErrorData: null,
      constMeta: kTestMoreThanJustOneRawStringStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestMoreThanJustOneRawStringStructConstMeta =>
      const TaskConstMeta(
        debugName: "test_more_than_just_one_raw_string_struct",
        argNames: [],
      );

  @override
  Future<RawStringItemStruct> testRawStringItemStruct({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_raw_string_item_struct(port_),
      parseSuccessData: _wire2api_raw_string_item_struct,
      parseErrorData: null,
      constMeta: kTestRawStringItemStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringItemStructConstMeta => const TaskConstMeta(
        debugName: "test_raw_string_item_struct",
        argNames: [],
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
  Stream<String> funcStreamRealisticTwinNormal(
      {required String arg, dynamic hint}) {
    var arg0 = api2wire_String(arg);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_func_stream_realistic_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kFuncStreamRealisticTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStreamRealisticTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_stream_realistic_twin_normal",
        argNames: ["arg"],
      );

  @override
  Stream<String> funcStreamReturnErrorTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_func_stream_return_error_twin_normal(port_),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncStreamReturnErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStreamReturnErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_stream_return_error_twin_normal",
        argNames: [],
      );

  @override
  Stream<String> funcStreamReturnPanicTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_func_stream_return_panic_twin_normal(port_),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kFuncStreamReturnPanicTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStreamReturnPanicTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_stream_return_panic_twin_normal",
        argNames: [],
      );

  @override
  Stream<int> funcStreamSinkArgPositionTwinNormal(
      {required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_u_32(a);
    var arg1 = api2wire_u_32(b);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_func_stream_sink_arg_position_twin_normal(
          port_, arg0, arg1),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kFuncStreamSinkArgPositionTwinNormalConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStreamSinkArgPositionTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_stream_sink_arg_position_twin_normal",
        argNames: ["a", "b"],
      );

  @override
  Stream<MyStreamEntry> handleStreamOfStruct({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_handle_stream_of_struct(port_),
      parseSuccessData: _wire2api_my_stream_entry,
      parseErrorData: null,
      constMeta: kHandleStreamOfStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamOfStructConstMeta => const TaskConstMeta(
        debugName: "handle_stream_of_struct",
        argNames: [],
      );

  @override
  Future<StructWithOneFieldTwinNormal> funcStructWithOneFieldTwinNormal(
      {required StructWithOneFieldTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_one_field_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_struct_with_one_field_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_struct_with_one_field_twin_normal,
      parseErrorData: null,
      constMeta: kFuncStructWithOneFieldTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithOneFieldTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_one_field_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<StructWithTwoFieldTwinNormal> funcStructWithTwoFieldTwinNormal(
      {required StructWithTwoFieldTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_two_field_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_struct_with_two_field_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_struct_with_two_field_twin_normal,
      parseErrorData: null,
      constMeta: kFuncStructWithTwoFieldTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithTwoFieldTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_two_field_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<StructWithZeroFieldTwinNormal> funcStructWithZeroFieldTwinNormal(
      {required StructWithZeroFieldTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_zero_field_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_struct_with_zero_field_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_struct_with_zero_field_twin_normal,
      parseErrorData: null,
      constMeta: kFuncStructWithZeroFieldTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncStructWithZeroFieldTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_struct_with_zero_field_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<TupleStructWithOneFieldTwinNormal>
      funcTupleStructWithOneFieldTwinNormal(
          {required TupleStructWithOneFieldTwinNormal arg, dynamic hint}) {
    var arg0 =
        api2wire_box_autoadd_tuple_struct_with_one_field_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_tuple_struct_with_one_field_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_tuple_struct_with_one_field_twin_normal,
      parseErrorData: null,
      constMeta: kFuncTupleStructWithOneFieldTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTupleStructWithOneFieldTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_tuple_struct_with_one_field_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<TupleStructWithTwoFieldTwinNormal>
      funcTupleStructWithTwoFieldTwinNormal(
          {required TupleStructWithTwoFieldTwinNormal arg, dynamic hint}) {
    var arg0 =
        api2wire_box_autoadd_tuple_struct_with_two_field_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_func_tuple_struct_with_two_field_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_tuple_struct_with_two_field_twin_normal,
      parseErrorData: null,
      constMeta: kFuncTupleStructWithTwoFieldTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTupleStructWithTwoFieldTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "func_tuple_struct_with_two_field_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<(String, int)> testTuple({(String, int)? value, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_record_string_i_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_tuple(port_, arg0),
      parseSuccessData: _wire2api_record_string_i_32,
      parseErrorData: null,
      constMeta: kTestTupleConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestTupleConstMeta => const TaskConstMeta(
        debugName: "test_tuple",
        argNames: ["value"],
      );

  @override
  Future<void> testTuple2({required List<(String, int)> value, dynamic hint}) {
    var arg0 = api2wire_list_record_string_i_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_tuple_2(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kTestTuple2ConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestTuple2ConstMeta => const TaskConstMeta(
        debugName: "test_tuple_2",
        argNames: ["value"],
      );

  @override
  Future<FeatureUuid> handleNestedUuids(
      {required FeatureUuid ids, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feature_uuid(ids);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_nested_uuids(port_, arg0),
      parseSuccessData: _wire2api_feature_uuid,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleNestedUuidsConstMeta,
      argValues: [ids],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNestedUuidsConstMeta => const TaskConstMeta(
        debugName: "handle_nested_uuids",
        argNames: ["ids"],
      );

  @override
  Future<UuidValue> handleUuid({required UuidValue id, dynamic hint}) {
    var arg0 = api2wire_Uuid(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_uuid(port_, arg0),
      parseSuccessData: _wire2api_Uuid,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleUuidConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleUuidConstMeta => const TaskConstMeta(
        debugName: "handle_uuid",
        argNames: ["id"],
      );

  @override
  Future<List<UuidValue>> handleUuids(
      {required List<UuidValue> ids, dynamic hint}) {
    var arg0 = api2wire_Uuids(ids);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_uuids(port_, arg0),
      parseSuccessData: _wire2api_Uuids,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleUuidsConstMeta,
      argValues: [ids],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleUuidsConstMeta => const TaskConstMeta(
        debugName: "handle_uuids",
        argNames: ["ids"],
      );
}

// Section: impl_wire2api

AnyhowException _wire2api_AnyhowException(dynamic raw) {
  return AnyhowException(raw as String);
}

Duration _wire2api_Chrono_Duration(dynamic raw) {
  return wire2apiDuration(_wire2api_i_64(raw));
}

List<Duration> _wire2api_Chrono_DurationList(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_Chrono_Duration).toList();
}

DateTime _wire2api_Chrono_Local(dynamic raw) {
  return wire2apiTimestamp(ts: _wire2api_i_64(raw), isUtc: false);
}

List<DateTime> _wire2api_Chrono_LocalList(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_Chrono_Local).toList();
}

DateTime _wire2api_Chrono_Naive(dynamic raw) {
  return wire2apiTimestamp(ts: _wire2api_i_64(raw), isUtc: true);
}

DateTime _wire2api_Chrono_Utc(dynamic raw) {
  return wire2apiTimestamp(ts: _wire2api_i_64(raw), isUtc: true);
}

String _wire2api_String(dynamic raw) {
  return raw as String;
}

List<String> _wire2api_StringList(dynamic raw) {
  return (raw as List<dynamic>).cast<String>();
}

UuidValue _wire2api_Uuid(dynamic raw) {
  return UuidValue.fromByteList(_wire2api_list_prim_u_8(raw));
}

List<UuidValue> _wire2api_Uuids(dynamic raw) {
  final bytes = _wire2api_list_prim_u_8(raw);
  return wire2apiUuids(bytes);
}

Uint8List _wire2api_ZeroCopyBuffer_list_prim_u_8(dynamic raw) {
  return raw as Uint8List;
}

A _wire2api_a(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return A(
    a: _wire2api_String(arr[0]),
  );
}

Abc _wire2api_abc(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return Abc_A(
        _wire2api_box_autoadd_a(raw[1]),
      );
    case 1:
      return Abc_B(
        _wire2api_box_autoadd_b(raw[1]),
      );
    case 2:
      return Abc_C(
        _wire2api_box_autoadd_c(raw[1]),
      );
    case 3:
      return Abc_JustInt(
        _wire2api_i_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

B _wire2api_b(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return B(
    b: _wire2api_i_32(arr[0]),
  );
}

BigBuffers _wire2api_big_buffers(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return BigBuffers(
    int64: _wire2api_list_prim_i_64(arr[0]),
    uint64: _wire2api_list_prim_u_64(arr[1]),
  );
}

Blob _wire2api_blob(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return Blob(
    field0: _wire2api_u_8_array_1600(arr[0]),
  );
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

Duration _wire2api_box_autoadd_Chrono_Duration(dynamic raw) {
  return _wire2api_Chrono_Duration(raw);
}

DateTime _wire2api_box_autoadd_Chrono_Naive(dynamic raw) {
  return _wire2api_Chrono_Naive(raw);
}

DateTime _wire2api_box_autoadd_Chrono_Utc(dynamic raw) {
  return _wire2api_Chrono_Utc(raw);
}

A _wire2api_box_autoadd_a(dynamic raw) {
  return _wire2api_a(raw);
}

B _wire2api_box_autoadd_b(dynamic raw) {
  return _wire2api_b(raw);
}

bool _wire2api_box_autoadd_bool(dynamic raw) {
  return raw as bool;
}

C _wire2api_box_autoadd_c(dynamic raw) {
  return _wire2api_c(raw);
}

CustomNestedErrorInnerTwinNormal
    _wire2api_box_autoadd_custom_nested_error_inner_twin_normal(dynamic raw) {
  return _wire2api_custom_nested_error_inner_twin_normal(raw);
}

CustomNestedErrorInnerTwinSync
    _wire2api_box_autoadd_custom_nested_error_inner_twin_sync(dynamic raw) {
  return _wire2api_custom_nested_error_inner_twin_sync(raw);
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

Measure _wire2api_box_autoadd_measure(dynamic raw) {
  return _wire2api_measure(raw);
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

Weekdays _wire2api_box_autoadd_weekdays(dynamic raw) {
  return _wire2api_weekdays(raw);
}

Distance _wire2api_box_distance(dynamic raw) {
  return _wire2api_distance(raw);
}

FeedId _wire2api_box_feed_id(dynamic raw) {
  return _wire2api_feed_id(raw);
}

Speed _wire2api_box_speed(dynamic raw) {
  return _wire2api_speed(raw);
}

U8Array8 _wire2api_box_u_8_array_8(dynamic raw) {
  return _wire2api_u_8_array_8(raw);
}

C _wire2api_c(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return C(
    c: _wire2api_bool(arr[0]),
  );
}

CustomEnumErrorTwinNormal _wire2api_custom_enum_error_twin_normal(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomEnumErrorTwinNormal_One(
        message: _wire2api_String(raw[1]),
        backtrace: _wire2api_String(raw[2]),
      );
    case 1:
      return CustomEnumErrorTwinNormal_Two(
        message: _wire2api_u_32(raw[1]),
        backtrace: _wire2api_String(raw[2]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomEnumErrorTwinSync _wire2api_custom_enum_error_twin_sync(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomEnumErrorTwinSync_One(
        message: _wire2api_String(raw[1]),
        backtrace: _wire2api_String(raw[2]),
      );
    case 1:
      return CustomEnumErrorTwinSync_Two(
        message: _wire2api_u_32(raw[1]),
        backtrace: _wire2api_String(raw[2]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomNestedErrorInnerTwinNormal
    _wire2api_custom_nested_error_inner_twin_normal(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomNestedErrorInnerTwinNormal_Three(
        _wire2api_String(raw[1]),
      );
    case 1:
      return CustomNestedErrorInnerTwinNormal_Four(
        _wire2api_u_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomNestedErrorInnerTwinSync _wire2api_custom_nested_error_inner_twin_sync(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomNestedErrorInnerTwinSync_Three(
        _wire2api_String(raw[1]),
      );
    case 1:
      return CustomNestedErrorInnerTwinSync_Four(
        _wire2api_u_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomNestedErrorOuterTwinNormal
    _wire2api_custom_nested_error_outer_twin_normal(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomNestedErrorOuterTwinNormal_One(
        _wire2api_String(raw[1]),
      );
    case 1:
      return CustomNestedErrorOuterTwinNormal_Two(
        _wire2api_box_autoadd_custom_nested_error_inner_twin_normal(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomNestedErrorOuterTwinSync _wire2api_custom_nested_error_outer_twin_sync(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return CustomNestedErrorOuterTwinSync_One(
        _wire2api_String(raw[1]),
      );
    case 1:
      return CustomNestedErrorOuterTwinSync_Two(
        _wire2api_box_autoadd_custom_nested_error_inner_twin_sync(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

CustomStructErrorTwinNormal _wire2api_custom_struct_error_twin_normal(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return CustomStructErrorTwinNormal(
    a: _wire2api_String(arr[0]),
  );
}

CustomStructErrorTwinSync _wire2api_custom_struct_error_twin_sync(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return CustomStructErrorTwinSync(
    a: _wire2api_String(arr[0]),
  );
}

dynamic _wire2api_dartabi(dynamic raw) {
  return raw;
}

Distance _wire2api_distance(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return Distance_Unknown();
    case 1:
      return Distance_Map(
        _wire2api_f_64(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumSimpleTwinNormal _wire2api_enum_simple_twin_normal(dynamic raw) {
  return EnumSimpleTwinNormal.values[raw as int];
}

EnumSimpleTwinSync _wire2api_enum_simple_twin_sync(dynamic raw) {
  return EnumSimpleTwinSync.values[raw as int];
}

EnumWithItemMixedTwinNormal _wire2api_enum_with_item_mixed_twin_normal(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemMixedTwinNormal_A();
    case 1:
      return EnumWithItemMixedTwinNormal_B(
        _wire2api_list_prim_u_8(raw[1]),
      );
    case 2:
      return EnumWithItemMixedTwinNormal_C(
        cField: _wire2api_String(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumWithItemMixedTwinSync _wire2api_enum_with_item_mixed_twin_sync(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemMixedTwinSync_A();
    case 1:
      return EnumWithItemMixedTwinSync_B(
        _wire2api_list_prim_u_8(raw[1]),
      );
    case 2:
      return EnumWithItemMixedTwinSync_C(
        cField: _wire2api_String(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumWithItemStructTwinNormal _wire2api_enum_with_item_struct_twin_normal(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemStructTwinNormal_A(
        aField: _wire2api_list_prim_u_8(raw[1]),
      );
    case 1:
      return EnumWithItemStructTwinNormal_B(
        bField: _wire2api_list_prim_i_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumWithItemStructTwinSync _wire2api_enum_with_item_struct_twin_sync(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemStructTwinSync_A(
        aField: _wire2api_list_prim_u_8(raw[1]),
      );
    case 1:
      return EnumWithItemStructTwinSync_B(
        bField: _wire2api_list_prim_i_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumWithItemTupleTwinNormal _wire2api_enum_with_item_tuple_twin_normal(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemTupleTwinNormal_A(
        _wire2api_list_prim_u_8(raw[1]),
      );
    case 1:
      return EnumWithItemTupleTwinNormal_B(
        _wire2api_list_prim_i_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

EnumWithItemTupleTwinSync _wire2api_enum_with_item_tuple_twin_sync(
    dynamic raw) {
  switch (raw[0]) {
    case 0:
      return EnumWithItemTupleTwinSync_A(
        _wire2api_list_prim_u_8(raw[1]),
      );
    case 1:
      return EnumWithItemTupleTwinSync_B(
        _wire2api_list_prim_i_32(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

double _wire2api_f_32(dynamic raw) {
  return raw as double;
}

double _wire2api_f_64(dynamic raw) {
  return raw as double;
}

FeatureUuid _wire2api_feature_uuid(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return FeatureUuid(
    one: _wire2api_Uuid(arr[0]),
    many: _wire2api_Uuids(arr[1]),
  );
}

FeedId _wire2api_feed_id(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return FeedId(
    field0: _wire2api_u_8_array_8(arr[0]),
  );
}

int _wire2api_i_16(dynamic raw) {
  return raw as int;
}

int _wire2api_i_32(dynamic raw) {
  return raw as int;
}

I32Array2 _wire2api_i_32_array_2(dynamic raw) {
  return I32Array2(_wire2api_list_prim_i_32(raw));
}

BigInt _wire2api_i_64(dynamic raw) {
  return wire2apiI64OrU64(raw);
}

int _wire2api_i_8(dynamic raw) {
  return raw as int;
}

List<bool> _wire2api_list_bool(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_bool).toList();
}

List<MySize> _wire2api_list_my_size(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_my_size).toList();
}

List<MyTreeNode> _wire2api_list_my_tree_node(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_my_tree_node).toList();
}

List<Point> _wire2api_list_point(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_point).toList();
}

Float32List _wire2api_list_prim_f_32(dynamic raw) {
  return raw as Float32List;
}

Float64List _wire2api_list_prim_f_64(dynamic raw) {
  return raw as Float64List;
}

Int16List _wire2api_list_prim_i_16(dynamic raw) {
  return raw as Int16List;
}

Int32List _wire2api_list_prim_i_32(dynamic raw) {
  return raw as Int32List;
}

Int64List _wire2api_list_prim_i_64(dynamic raw) {
  return Int64List.from(raw);
}

Int8List _wire2api_list_prim_i_8(dynamic raw) {
  return raw as Int8List;
}

Uint16List _wire2api_list_prim_u_16(dynamic raw) {
  return raw as Uint16List;
}

Uint32List _wire2api_list_prim_u_32(dynamic raw) {
  return raw as Uint32List;
}

Uint64List _wire2api_list_prim_u_64(dynamic raw) {
  return Uint64List.from(raw);
}

Uint8List _wire2api_list_prim_u_8(dynamic raw) {
  return raw as Uint8List;
}

List<TestId> _wire2api_list_test_id(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_test_id).toList();
}

List<Weekdays> _wire2api_list_weekdays(dynamic raw) {
  return (raw as List<dynamic>).map(_wire2api_weekdays).toList();
}

MacroStruct _wire2api_macro_struct(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return MacroStruct(
    data: _wire2api_i_32(arr[0]),
  );
}

Measure _wire2api_measure(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return Measure_Speed(
        _wire2api_box_speed(raw[1]),
      );
    case 1:
      return Measure_Distance(
        _wire2api_box_distance(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

MessageId _wire2api_message_id(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return MessageId(
    field0: _wire2api_u_8_array_32(arr[0]),
  );
}

MoreThanJustOneRawStringStruct _wire2api_more_than_just_one_raw_string_struct(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return MoreThanJustOneRawStringStruct(
    regular: _wire2api_String(arr[0]),
    type: _wire2api_String(arr[1]),
    async: _wire2api_bool(arr[2]),
    another: _wire2api_String(arr[3]),
  );
}

MyNestedStruct _wire2api_my_nested_struct(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return MyNestedStruct(
    treeNode: _wire2api_my_tree_node(arr[0]),
    weekday: _wire2api_weekdays(arr[1]),
  );
}

MySize _wire2api_my_size(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return MySize(
    width: _wire2api_i_32(arr[0]),
    height: _wire2api_i_32(arr[1]),
  );
}

MyStreamEntry _wire2api_my_stream_entry(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return MyStreamEntry(
    hello: _wire2api_String(arr[0]),
  );
}

MyTreeNode _wire2api_my_tree_node(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 4)
    throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
  return MyTreeNode(
    valueI32: _wire2api_i_32(arr[0]),
    valueVecU8: _wire2api_list_prim_u_8(arr[1]),
    valueBoolean: _wire2api_bool(arr[2]),
    children: _wire2api_list_my_tree_node(arr[3]),
  );
}

NewTypeInt _wire2api_new_type_int(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return NewTypeInt(
    field0: _wire2api_i_64(arr[0]),
  );
}

Duration? _wire2api_opt_box_autoadd_Chrono_Duration(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_Chrono_Duration(raw);
}

DateTime? _wire2api_opt_box_autoadd_Chrono_Naive(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_Chrono_Naive(raw);
}

DateTime? _wire2api_opt_box_autoadd_Chrono_Utc(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_Chrono_Utc(raw);
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

Measure? _wire2api_opt_box_autoadd_measure(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_measure(raw);
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

Weekdays? _wire2api_opt_box_autoadd_weekdays(dynamic raw) {
  return raw == null ? null : _wire2api_box_autoadd_weekdays(raw);
}

Point _wire2api_point(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return Point(
    x: _wire2api_f_32(arr[0]),
    y: _wire2api_f_32(arr[1]),
  );
}

PointArray2 _wire2api_point_array_2(dynamic raw) {
  return PointArray2((raw as List<dynamic>).map(_wire2api_point).toList());
}

RawStringItemStruct _wire2api_raw_string_item_struct(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return RawStringItemStruct(
    type: _wire2api_String(arr[0]),
  );
}

(String, int) _wire2api_record_string_i_32(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2) {
    throw Exception('Expected 2 elements, got ${arr.length}');
  }
  return (
    _wire2api_String(arr[0]),
    _wire2api_i_32(arr[1]),
  );
}

Speed _wire2api_speed(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return Speed_Unknown();
    case 1:
      return Speed_GPS(
        _wire2api_f_64(raw[1]),
      );
    default:
      throw Exception("unreachable");
  }
}

StructWithEnum _wire2api_struct_with_enum(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return StructWithEnum(
    abc1: _wire2api_abc(arr[0]),
    abc2: _wire2api_abc(arr[1]),
  );
}

StructWithOneFieldTwinNormal _wire2api_struct_with_one_field_twin_normal(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return StructWithOneFieldTwinNormal(
    a: _wire2api_i_32(arr[0]),
  );
}

StructWithOneFieldTwinSync _wire2api_struct_with_one_field_twin_sync(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return StructWithOneFieldTwinSync(
    a: _wire2api_i_32(arr[0]),
  );
}

StructWithTwoFieldTwinNormal _wire2api_struct_with_two_field_twin_normal(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return StructWithTwoFieldTwinNormal(
    a: _wire2api_i_32(arr[0]),
    b: _wire2api_i_32(arr[1]),
  );
}

StructWithTwoFieldTwinSync _wire2api_struct_with_two_field_twin_sync(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return StructWithTwoFieldTwinSync(
    a: _wire2api_i_32(arr[0]),
    b: _wire2api_i_32(arr[1]),
  );
}

StructWithZeroFieldTwinNormal _wire2api_struct_with_zero_field_twin_normal(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 0)
    throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
  return StructWithZeroFieldTwinNormal();
}

StructWithZeroFieldTwinSync _wire2api_struct_with_zero_field_twin_sync(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 0)
    throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
  return StructWithZeroFieldTwinSync();
}

TestChrono _wire2api_test_chrono(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 3)
    throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
  return TestChrono(
    dt: _wire2api_opt_box_autoadd_Chrono_Utc(arr[0]),
    dt2: _wire2api_opt_box_autoadd_Chrono_Naive(arr[1]),
    du: _wire2api_opt_box_autoadd_Chrono_Duration(arr[2]),
  );
}

TestId _wire2api_test_id(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return TestId(
    field0: _wire2api_i_32_array_2(arr[0]),
  );
}

TestIdArray2 _wire2api_test_id_array_2(dynamic raw) {
  return TestIdArray2((raw as List<dynamic>).map(_wire2api_test_id).toList());
}

TupleStructWithOneFieldTwinNormal
    _wire2api_tuple_struct_with_one_field_twin_normal(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return TupleStructWithOneFieldTwinNormal(
    field0: _wire2api_i_32(arr[0]),
  );
}

TupleStructWithOneFieldTwinSync _wire2api_tuple_struct_with_one_field_twin_sync(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 1)
    throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
  return TupleStructWithOneFieldTwinSync(
    field0: _wire2api_i_32(arr[0]),
  );
}

TupleStructWithTwoFieldTwinNormal
    _wire2api_tuple_struct_with_two_field_twin_normal(dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return TupleStructWithTwoFieldTwinNormal(
    field0: _wire2api_i_32(arr[0]),
    field1: _wire2api_i_32(arr[1]),
  );
}

TupleStructWithTwoFieldTwinSync _wire2api_tuple_struct_with_two_field_twin_sync(
    dynamic raw) {
  final arr = raw as List<dynamic>;
  if (arr.length != 2)
    throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
  return TupleStructWithTwoFieldTwinSync(
    field0: _wire2api_i_32(arr[0]),
    field1: _wire2api_i_32(arr[1]),
  );
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

U8Array1600 _wire2api_u_8_array_1600(dynamic raw) {
  return U8Array1600(_wire2api_list_prim_u_8(raw));
}

U8Array32 _wire2api_u_8_array_32(dynamic raw) {
  return U8Array32(_wire2api_list_prim_u_8(raw));
}

U8Array5 _wire2api_u_8_array_5(dynamic raw) {
  return U8Array5(_wire2api_list_prim_u_8(raw));
}

U8Array8 _wire2api_u_8_array_8(dynamic raw) {
  return U8Array8(_wire2api_list_prim_u_8(raw));
}

void _wire2api_unit(dynamic raw) {
  return;
}

Weekdays _wire2api_weekdays(dynamic raw) {
  return Weekdays.values[raw as int];
}

// Section: api2wire_funcs

bool api2wire_bool(bool raw) {
  return raw;
}

int api2wire_enum_simple_twin_normal(EnumSimpleTwinNormal raw) {
  return api2wire_i_32(raw.index);
}

int api2wire_enum_simple_twin_sync(EnumSimpleTwinSync raw) {
  return api2wire_i_32(raw.index);
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

int api2wire_weekdays(Weekdays raw) {
  return api2wire_i_32(raw.index);
}
