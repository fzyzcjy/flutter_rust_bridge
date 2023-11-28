// ignore_for_file: unused_import, unused_element, duplicate_ignore

import 'api/array.dart';
import 'api/attribute.dart';
import 'api/chrono_type.dart';
import 'api/comment.dart';
import 'api/dart_dynamic.dart';
import 'api/dart_opaque.dart';
import 'api/dart_opaque_sync.dart';
import 'api/enumeration.dart';
import 'api/event_listener.dart';
import 'api/exception.dart';
import 'api/external_type_in_crate.dart';
import 'api/inside_macro.dart';
import 'api/method.dart';
import 'api/mirror.dart';
import 'api/misc_example.dart';
import 'api/misc_type.dart';
import 'api/newtype_pattern.dart';
import 'api/optional.dart';
import 'api/optional_primitive_misc.dart';
import 'api/primitive_list_misc.dart';
import 'api/primitive_list_sync_misc.dart';
import 'api/primitive_misc.dart';
import 'api/raw_string.dart';
import 'api/rust_opaque.dart';
import 'api/rust_opaque_sync.dart';
import 'api/simple.dart';
import 'api/stream.dart';
import 'api/structure.dart';
import 'api/tuple.dart';
import 'api/type_alias.dart';
import 'api/uuid_type.dart';
import 'auxiliary/new_module_system/sub_module.dart';
import 'auxiliary/old_module_system/sub_module.dart';
import 'auxiliary/sample_types.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:meta/meta.dart' as meta;
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
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      const ExternalLibraryLoaderConfig(
        stem: 'frb_example_pure_dart',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
      );
}

abstract class RustLibApi extends BaseApi {
  Future<BlobTwinNormal> boxedBlobTwinNormal(
      {required U8Array1600 blob, dynamic hint});

  Future<TestIdTwinNormal> funcTestIdTwinNormal(
      {required TestIdTwinNormal id, dynamic hint});

  Future<U8Array5> getArrayTwinNormal({dynamic hint});

  Future<PointTwinNormalArray2> getComplexArrayTwinNormal({dynamic hint});

  Future<double> lastNumberTwinNormal(
      {required F64Array16 array, dynamic hint});

  Future<TestIdTwinNormalArray2> nestedIdTwinNormal(
      {required TestIdTwinNormalArray4 id, dynamic hint});

  Future<MessageIdTwinNormal> newMsgidTwinNormal(
      {required U8Array32 id, dynamic hint});

  Future<FeedIdTwinNormal> returnBoxedFeedIdTwinNormal(
      {required U8Array8 id, dynamic hint});

  Future<U8Array8> returnBoxedRawFeedIdTwinNormal(
      {required FeedIdTwinNormal id, dynamic hint});

  Future<U8Array1600> useBoxedBlobTwinNormal(
      {required BlobTwinNormal blob, dynamic hint});

  Future<U8Array32> useMsgidTwinNormal(
      {required MessageIdTwinNormal id, dynamic hint});

  Future<void> handleCustomizedStructTwinNormal(
      {required CustomizedTwinNormal val, dynamic hint});

  Future<UserIdTwinNormal> nextUserIdTwinNormal(
      {required UserIdTwinNormal userId, dynamic hint});

  Future<DateTime> datetimeLocalTwinNormal({required DateTime d, dynamic hint});

  Future<DateTime> datetimeUtcTwinNormal({required DateTime d, dynamic hint});

  Future<Duration> durationTwinNormal({required Duration d, dynamic hint});

  Future<List<DateTime>> handleDurationsTwinNormal(
      {required List<Duration> durations,
      required DateTime since,
      dynamic hint});

  Future<List<Duration>> handleTimestampsTwinNormal(
      {required List<DateTime> timestamps,
      required DateTime epoch,
      dynamic hint});

  Future<Duration> howLongDoesItTakeTwinNormal(
      {required FeatureChronoTwinNormal mine, dynamic hint});

  Future<DateTime> naivedatetimeTwinNormal({required DateTime d, dynamic hint});

  Future<DateTime?> optionalEmptyDatetimeUtcTwinNormal(
      {DateTime? d, dynamic hint});

  Future<TestChronoTwinNormal> testChronoTwinNormal({dynamic hint});

  Future<TestChronoTwinNormal> testPreciseChronoTwinNormal({dynamic hint});

  Future<void> structWithCommentsTwinNormalInstanceMethodTwinNormal(
      {required StructWithCommentsTwinNormal that, dynamic hint});

  Future<void> structWithCommentsTwinNormalStaticMethodTwinNormal(
      {dynamic hint});

  Future<void> functionWithCommentsSlashStarStarTwinNormal({dynamic hint});

  Future<void> functionWithCommentsTripleSlashMultiLineTwinNormal(
      {dynamic hint});

  Future<void> functionWithCommentsTripleSlashSingleLineTwinNormal(
      {dynamic hint});

  Future<dynamic> returnDartDynamicTwinNormal({dynamic hint});

  Future<String> asyncAcceptDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  Future<EnumDartOpaqueTwinNormal> createEnumDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  Future<DartOpaqueNestedTwinNormal> createNestedDartOpaqueTwinNormal(
      {required Object opaque1, required Object opaque2, dynamic hint});

  Future<void> dropStaticDartOpaqueTwinNormal({dynamic hint});

  Future<void> getEnumDartOpaqueTwinNormal(
      {required EnumDartOpaqueTwinNormal opaque, dynamic hint});

  Future<void> getNestedDartOpaqueTwinNormal(
      {required DartOpaqueNestedTwinNormal opaque, dynamic hint});

  Future<void> loopBackArrayGetTwinNormal(
      {required ObjectArray1 opaque, dynamic hint});

  Future<ObjectArray1> loopBackArrayTwinNormal(
      {required Object opaque, dynamic hint});

  Future<void> loopBackOptionGetTwinNormal({Object? opaque, dynamic hint});

  Future<Object?> loopBackOptionTwinNormal(
      {required Object opaque, dynamic hint});

  Future<Object> loopBackTwinNormal({required Object opaque, dynamic hint});

  Future<void> loopBackVecGetTwinNormal(
      {required List<Object> opaque, dynamic hint});

  Future<List<Object>> loopBackVecTwinNormal(
      {required Object opaque, dynamic hint});

  Future<void> panicUnwrapDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  Future<void> setStaticDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  Object returnNonDroppableDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  String syncAcceptDartOpaqueTwinNormal({required Object opaque, dynamic hint});

  Object syncLoopbackTwinNormal({required Object opaque, dynamic hint});

  Object? syncOptionDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint});

  Object? syncOptionLoopbackTwinNormal({Object? opaque, dynamic hint});

  String unwrapDartOpaqueTwinNormal({required Object opaque, dynamic hint});

  Future<EnumSimpleTwinNormal> funcEnumSimpleTwinNormal(
      {required EnumSimpleTwinNormal arg, dynamic hint});

  Future<EnumWithItemMixedTwinNormal> funcEnumWithItemMixedTwinNormal(
      {required EnumWithItemMixedTwinNormal arg, dynamic hint});

  Future<EnumWithItemStructTwinNormal> funcEnumWithItemStructTwinNormal(
      {required EnumWithItemStructTwinNormal arg, dynamic hint});

  Future<EnumWithItemTupleTwinNormal> funcEnumWithItemTupleTwinNormal(
      {required EnumWithItemTupleTwinNormal arg, dynamic hint});

  Future<WeekdaysTwinNormal> handleEnumParameterTwinNormal(
      {required WeekdaysTwinNormal weekday, dynamic hint});

  Future<KitchenSinkTwinNormal> handleEnumStructTwinNormal(
      {required KitchenSinkTwinNormal val, dynamic hint});

  Future<WeekdaysTwinNormal?> handleReturnEnumTwinNormal(
      {required String input, dynamic hint});

  Future<MeasureTwinNormal?> multiplyByTenTwinNormal(
      {required MeasureTwinNormal measure, dynamic hint});

  Future<Uint8List> printNoteTwinNormal(
      {required NoteTwinNormal note, dynamic hint});

  Future<String> eventTwinNormalAsStringTwinNormal(
      {required EventTwinNormal that, dynamic hint});

  Future<void> closeEventListenerTwinNormal({dynamic hint});

  Future<void> createEventTwinNormal(
      {required String address, required String payload, dynamic hint});

  Stream<EventTwinNormal> registerEventListenerTwinNormal({dynamic hint});

  Future<CustomStructTwinNormal> customStructTwinNormalNewTwinNormal(
      {required String message, dynamic hint});

  Future<void> customStructTwinNormalNonstaticReturnCustomStructErrorTwinNormal(
      {required CustomStructTwinNormal that, dynamic hint});

  Future<int> customStructTwinNormalNonstaticReturnCustomStructOkTwinNormal(
      {required CustomStructTwinNormal that, dynamic hint});

  Future<void> customStructTwinNormalStaticReturnCustomStructErrorTwinNormal(
      {dynamic hint});

  Future<int> customStructTwinNormalStaticReturnCustomStructOkTwinNormal(
      {dynamic hint});

  Future<SomeStructTwinNormal> someStructTwinNormalNewTwinNormal(
      {required int value, dynamic hint});

  Future<int> someStructTwinNormalNonStaticReturnErrCustomErrorTwinNormal(
      {required SomeStructTwinNormal that, dynamic hint});

  Future<int> someStructTwinNormalNonStaticReturnOkCustomErrorTwinNormal(
      {required SomeStructTwinNormal that, dynamic hint});

  Future<int> someStructTwinNormalStaticReturnErrCustomErrorTwinNormal(
      {dynamic hint});

  Future<int> someStructTwinNormalStaticReturnOkCustomErrorTwinNormal(
      {dynamic hint});

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

  Future<void> panicWithCustomResultTwinNormal({dynamic hint});

  Future<void> returnCustomNestedError1TwinNormal({dynamic hint});

  Future<void> returnCustomNestedError1Variant1TwinNormal({dynamic hint});

  Future<void> returnCustomNestedError2TwinNormal({dynamic hint});

  Future<void> returnCustomStructErrorTwinNormal({dynamic hint});

  Future<int> returnCustomStructOkTwinNormal({dynamic hint});

  Future<int> returnErrCustomErrorTwinNormal({dynamic hint});

  Future<int> returnErrorVariantTwinNormal(
      {required int variant, dynamic hint});

  Future<int> returnOkCustomErrorTwinNormal({dynamic hint});

  Stream<String> streamSinkThrowAnyhowTwinNormal({dynamic hint});

  void syncReturnCustomStructErrorTwinNormal({dynamic hint});

  Future<void> throwAnyhowTwinNormal({dynamic hint});

  Future<NewSimpleStruct> callNewModuleSystemTwinNormal({dynamic hint});

  Future<OldSimpleStruct> callOldModuleSystemTwinNormal({dynamic hint});

  Future<bool> useImportedEnumTwinNormal(
      {required MyEnum myEnum, dynamic hint});

  Future<bool> useImportedStructTwinNormal(
      {required MyStruct myStruct, dynamic hint});

  Future<AnotherMacroStructTwinNormal> anotherMacroStructTwinNormal(
      {dynamic hint});

  Future<MacroStruct> funcMacroStructTwinNormal(
      {required MacroStruct arg, dynamic hint});

  Future<String> concatenateWithTwinNormalConcatenateStaticTwinNormal(
      {required String a, required String b, dynamic hint});

  Future<String> concatenateWithTwinNormalConcatenateTwinNormal(
      {required ConcatenateWithTwinNormal that,
      required String b,
      dynamic hint});

  Stream<int>
      concatenateWithTwinNormalHandleSomeStaticStreamSinkSingleArgTwinNormal(
          {dynamic hint});

  Stream<Log2TwinNormal>
      concatenateWithTwinNormalHandleSomeStaticStreamSinkTwinNormal(
          {required int key, required int max, dynamic hint});

  Stream<int> concatenateWithTwinNormalHandleSomeStreamSinkAt1TwinNormal(
      {required ConcatenateWithTwinNormal that, dynamic hint});

  Stream<Log2TwinNormal>
      concatenateWithTwinNormalHandleSomeStreamSinkTwinNormal(
          {required ConcatenateWithTwinNormal that,
          required int key,
          required int max,
          dynamic hint});

  Future<ConcatenateWithTwinNormal> concatenateWithTwinNormalNewTwinNormal(
      {required String a, dynamic hint});

  Future<int> sumWithTwinNormalSumTwinNormal(
      {required SumWithTwinNormal that,
      required int y,
      required int z,
      dynamic hint});

  Future<SumWithTwinNormalArray3> getSumArrayTwinNormal(
      {required int a, required int b, required int c, dynamic hint});

  Future<SumWithTwinNormal> getSumStructTwinNormal({dynamic hint});

  Stream<ApplicationSettings> appSettingsStreamTwinNormal({dynamic hint});

  Stream<List<ApplicationSettings>> appSettingsVecStreamTwinNormal(
      {dynamic hint});

  Future<int?> firstNumberTwinNormal({required Numbers nums, dynamic hint});

  Future<int?> firstSequenceTwinNormal({required Sequences seqs, dynamic hint});

  Future<ApplicationSettings> getAppSettingsTwinNormal({dynamic hint});

  Future<ApplicationSettings> getFallibleAppSettingsTwinNormal({dynamic hint});

  Future<ApplicationMessage> getMessageTwinNormal({dynamic hint});

  Future<bool> isAppEmbeddedTwinNormal(
      {required ApplicationSettings appSettings, dynamic hint});

  Stream<MirrorStructTwinNormal> mirrorStructStreamTwinNormal({dynamic hint});

  Stream<(ApplicationSettings, RawStringEnumMirrored)>
      mirrorTupleStreamTwinNormal({dynamic hint});

  Future<Numbers> repeatNumberTwinNormal(
      {required int num, required int times, dynamic hint});

  Future<Sequences> repeatSequenceTwinNormal(
      {required int seq, required int times, dynamic hint});

  Future<ContainsMirroredSubStructTwinNormal>
      testContainsMirroredSubStructTwinNormal({dynamic hint});

  Future<List<RawStringMirrored>> testFallibleOfRawStringMirroredTwinNormal(
      {dynamic hint});

  Future<List<RawStringEnumMirrored>> testListOfNestedEnumsMirroredTwinNormal(
      {dynamic hint});

  Future<ListOfNestedRawStringMirrored>
      testListOfRawNestedStringMirroredTwinNormal({dynamic hint});

  Future<NestedRawStringMirrored> testNestedRawStringMirroredTwinNormal(
      {dynamic hint});

  Future<RawStringEnumMirrored> testRawStringEnumMirroredTwinNormal(
      {required bool nested, dynamic hint});

  Future<RawStringMirrored> testRawStringMirroredTwinNormal({dynamic hint});

  Future<BigBuffersTwinNormal> handleBigBuffersTwinNormal({dynamic hint});

  Future<MyTreeNodeTwinNormal> handleComplexStructTwinNormal(
      {required MyTreeNodeTwinNormal s, dynamic hint});

  Future<MyNestedStructTwinNormal> handleNestedStructTwinNormal(
      {required MyNestedStructTwinNormal s, dynamic hint});

  Future<String> handleStringTwinNormal({required String s, dynamic hint});

  MySizeFreezedTwinNormal handleStructSyncFreezedTwinNormal(
      {required MySizeFreezedTwinNormal arg,
      required MySizeFreezedTwinNormal boxed,
      dynamic hint});

  Future<MySize> handleStructTwinNormal(
      {required MySize arg, required MySize boxed, dynamic hint});

  Future<Uint8List> handleVecU8TwinNormal({required Uint8List v, dynamic hint});

  Future<List<WeekdaysTwinNormal>> listOfPrimitiveEnumsTwinNormal(
      {required List<WeekdaysTwinNormal> weekdays, dynamic hint});

  Future<AbcTwinNormal> testAbcEnumTwinNormal(
      {required AbcTwinNormal abc, dynamic hint});

  Future<StructWithEnumTwinNormal> testStructWithEnumTwinNormal(
      {required StructWithEnumTwinNormal se, dynamic hint});

  Future<EmptyTwinNormal> emptyStructTwinNormal(
      {required EmptyTwinNormal empty, dynamic hint});

  Future<void> funcReturnUnitTwinNormal({dynamic hint});

  Future<String> funcStringTwinNormal({required String arg, dynamic hint});

  Future<List<MySize>> handleListOfStructTwinNormal(
      {required List<MySize> l, dynamic hint});

  Future<List<String>> handleStringListTwinNormal(
      {required List<String> names, dynamic hint});

  Future<NewTypeIntTwinNormal> handleNewtypeTwinNormal(
      {required NewTypeIntTwinNormal arg, dynamic hint});

  Future<double> handleIncrementBoxedOptionalTwinNormal(
      {double? opt, dynamic hint});

  Future<String> handleOptionBoxArgumentsTwinNormal(
      {int? i8Box,
      int? u8Box,
      int? i32Box,
      int? i64Box,
      double? f64Box,
      bool? boolbox,
      ExoticOptionalsTwinNormal? structbox,
      dynamic hint});

  Future<ExoticOptionalsTwinNormal?> handleOptionalIncrementTwinNormal(
      {ExoticOptionalsTwinNormal? opt, dynamic hint});

  Future<double?> handleOptionalReturnTwinNormal(
      {required double left, required double right, dynamic hint});

  Future<ElementTwinNormal?> handleOptionalStructTwinNormal(
      {String? document, dynamic hint});

  Future<OptVecsTwinNormal> handleVecOfOptsTwinNormal(
      {required OptVecsTwinNormal opt, dynamic hint});

  String? syncOptionNullTwinNormal({dynamic hint});

  String? syncOptionTwinNormal({dynamic hint});

  Future<int?> primitiveOptionalTypesTwinNormal(
      {int? myI32, int? myI64, double? myF64, bool? myBool, dynamic hint});

  Future<VecOfPrimitivePackTwinNormal> handleVecOfPrimitiveTwinNormal(
      {required int n, dynamic hint});

  Future<ZeroCopyVecOfPrimitivePackTwinNormal>
      handleZeroCopyVecOfPrimitiveTwinNormal({required int n, dynamic hint});

  ZeroCopyVecOfPrimitivePackTwinNormal
      handleZeroCopyVecOfPrimitiveSyncTwinNormal(
          {required int n, dynamic hint});

  Future<int> primitiveTypesTwinNormal(
      {required int myI32,
      required int myI64,
      required double myF64,
      required bool myBool,
      dynamic hint});

  Future<int> primitiveU32TwinNormal({required int myU32, dynamic hint});

  Future<MoreThanJustOneRawStringStructTwinNormal>
      testMoreThanJustOneRawStringStructTwinNormal({dynamic hint});

  Future<RawStringItemStructTwinNormal> testRawStringItemStructTwinNormal(
      {dynamic hint});

  Future<EnumOpaqueTwinNormalArray5> createArrayOpaqueEnumTwinNormal(
      {dynamic hint});

  Future<OpaqueNestedTwinNormal> createNestedOpaqueTwinNormal({dynamic hint});

  Future<HideData> createOpaqueTwinNormal({dynamic hint});

  Future<HideData?> createOptionOpaqueTwinNormal(
      {HideData? opaque, dynamic hint});

  Future<NonSendHideData> createSyncOpaqueTwinNormal({dynamic hint});

  Future<FrbOpaqueReturn> frbGeneratorTestTwinNormal({dynamic hint});

  Future<void> opaqueArrayRunTwinNormal(
      {required HideDataArray2 data, dynamic hint});

  Future<HideDataArray2> opaqueArrayTwinNormal({dynamic hint});

  Future<void> opaqueVecRunTwinNormal(
      {required List<HideData> data, dynamic hint});

  Future<List<HideData>> opaqueVecTwinNormal({dynamic hint});

  Future<String> runEnumOpaqueTwinNormal(
      {required EnumOpaqueTwinNormal opaque, dynamic hint});

  Future<void> runNestedOpaqueTwinNormal(
      {required OpaqueNestedTwinNormal opaque, dynamic hint});

  Future<String> runNonCloneTwinNormal(
      {required NonCloneData clone, dynamic hint});

  Future<String> runOpaqueTwinNormal({required HideData opaque, dynamic hint});

  Future<String> runOpaqueWithDelayTwinNormal(
      {required HideData opaque, dynamic hint});

  Future<String> unwrapRustOpaqueTwinNormal(
      {required HideData opaque, dynamic hint});

  FrbOpaqueSyncReturn frbSyncGeneratorTestTwinNormal({dynamic hint});

  NonCloneData syncCreateNonCloneTwinNormal({dynamic hint});

  HideData syncCreateOpaqueTwinNormal({dynamic hint});

  NonSendHideData syncCreateSyncOpaqueTwinNormal({dynamic hint});

  HideData? syncOptionRustOpaqueTwinNormal({dynamic hint});

  String syncRunOpaqueTwinNormal(
      {required NonSendHideData opaque, dynamic hint});

  Future<int> simpleAdderTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<String> funcStreamRealisticTwinNormal(
      {required String arg, dynamic hint});

  Stream<String> funcStreamReturnErrorTwinNormal({dynamic hint});

  Stream<String> funcStreamReturnPanicTwinNormal({dynamic hint});

  Stream<int> funcStreamSinkArgPositionTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<MyStreamEntryTwinNormal> handleStreamOfStructTwinNormal(
      {dynamic hint});

  Stream<LogTwinNormal> handleStreamSinkAt1TwinNormal(
      {required int key, required int max, dynamic hint});

  Stream<LogTwinNormal> handleStreamSinkAt2TwinNormal(
      {required int key, required int max, dynamic hint});

  Stream<LogTwinNormal> handleStreamSinkAt3TwinNormal(
      {required int key, required int max, dynamic hint});

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

  Future<void> testTuple2TwinNormal(
      {required List<(String, int)> value, dynamic hint});

  Future<(String, int)> testTupleTwinNormal(
      {(String, int)? value, dynamic hint});

  Future<int> handleTypeAliasIdTwinNormal({required int input, dynamic hint});

  Future<TestModelTwinNormal> handleTypeAliasModelTwinNormal(
      {required int input, dynamic hint});

  Future<int> handleTypeNestAliasIdTwinNormal(
      {required int input, dynamic hint});

  Future<FeatureUuidTwinNormal> handleNestedUuidsTwinNormal(
      {required FeatureUuidTwinNormal ids, dynamic hint});

  Future<UuidValue> handleUuidTwinNormal({required UuidValue id, dynamic hint});

  Future<List<UuidValue>> handleUuidsTwinNormal(
      {required List<UuidValue> ids, dynamic hint});

  OpaqueShareFnType get shareOpaqueMutexHideData;

  OpaqueDropFnType get dropOpaqueMutexHideData;

  OpaqueTypeFinalizer get mutexHideDataFinalizer;

  OpaqueShareFnType get shareOpaqueRwLockHideData;

  OpaqueDropFnType get dropOpaqueRwLockHideData;

  OpaqueTypeFinalizer get rwLockHideDataFinalizer;

  OpaqueShareFnType get shareOpaqueBoxDartDebug;

  OpaqueDropFnType get dropOpaqueBoxDartDebug;

  OpaqueTypeFinalizer get boxDartDebugFinalizer;

  OpaqueShareFnType get shareOpaqueFrbOpaqueReturn;

  OpaqueDropFnType get dropOpaqueFrbOpaqueReturn;

  OpaqueTypeFinalizer get frbOpaqueReturnFinalizer;

  OpaqueShareFnType get shareOpaqueFrbOpaqueSyncReturn;

  OpaqueDropFnType get dropOpaqueFrbOpaqueSyncReturn;

  OpaqueTypeFinalizer get frbOpaqueSyncReturnFinalizer;

  OpaqueShareFnType get shareOpaqueHideData;

  OpaqueDropFnType get dropOpaqueHideData;

  OpaqueTypeFinalizer get hideDataFinalizer;

  OpaqueShareFnType get shareOpaqueI32;

  OpaqueDropFnType get dropOpaqueI32;

  OpaqueTypeFinalizer get i32Finalizer;

  OpaqueShareFnType get shareOpaqueNonCloneData;

  OpaqueDropFnType get dropOpaqueNonCloneData;

  OpaqueTypeFinalizer get nonCloneDataFinalizer;

  OpaqueShareFnType get shareOpaqueNonSendHideData;

  OpaqueDropFnType get dropOpaqueNonSendHideData;

  OpaqueTypeFinalizer get nonSendHideDataFinalizer;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.dropPortManager,
  });

  @override
  Future<BlobTwinNormal> boxedBlobTwinNormal(
      {required U8Array1600 blob, dynamic hint}) {
    var arg0 = api2wire_box_u_8_array_1600(blob);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_boxed_blob_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_blob_twin_normal,
      parseErrorData: null,
      constMeta: kBoxedBlobTwinNormalConstMeta,
      argValues: [blob],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kBoxedBlobTwinNormalConstMeta => const TaskConstMeta(
        debugName: "boxed_blob_twin_normal",
        argNames: ["blob"],
      );

  @override
  Future<TestIdTwinNormal> funcTestIdTwinNormal(
      {required TestIdTwinNormal id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_test_id_twin_normal(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_test_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_test_id_twin_normal,
      parseErrorData: null,
      constMeta: kFuncTestIdTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncTestIdTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_test_id_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<U8Array5> getArrayTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_array_twin_normal(port_),
      parseSuccessData: _wire2api_u_8_array_5,
      parseErrorData: null,
      constMeta: kGetArrayTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetArrayTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_array_twin_normal",
        argNames: [],
      );

  @override
  Future<PointTwinNormalArray2> getComplexArrayTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_complex_array_twin_normal(port_),
      parseSuccessData: _wire2api_point_twin_normal_array_2,
      parseErrorData: null,
      constMeta: kGetComplexArrayTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetComplexArrayTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_complex_array_twin_normal",
        argNames: [],
      );

  @override
  Future<double> lastNumberTwinNormal(
      {required F64Array16 array, dynamic hint}) {
    var arg0 = api2wire_f_64_array_16(array);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_last_number_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kLastNumberTwinNormalConstMeta,
      argValues: [array],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLastNumberTwinNormalConstMeta => const TaskConstMeta(
        debugName: "last_number_twin_normal",
        argNames: ["array"],
      );

  @override
  Future<TestIdTwinNormalArray2> nestedIdTwinNormal(
      {required TestIdTwinNormalArray4 id, dynamic hint}) {
    var arg0 = api2wire_test_id_twin_normal_array_4(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_nested_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_test_id_twin_normal_array_2,
      parseErrorData: null,
      constMeta: kNestedIdTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNestedIdTwinNormalConstMeta => const TaskConstMeta(
        debugName: "nested_id_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<MessageIdTwinNormal> newMsgidTwinNormal(
      {required U8Array32 id, dynamic hint}) {
    var arg0 = api2wire_u_8_array_32(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_new_msgid_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_message_id_twin_normal,
      parseErrorData: null,
      constMeta: kNewMsgidTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNewMsgidTwinNormalConstMeta => const TaskConstMeta(
        debugName: "new_msgid_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<FeedIdTwinNormal> returnBoxedFeedIdTwinNormal(
      {required U8Array8 id, dynamic hint}) {
    var arg0 = api2wire_u_8_array_8(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_boxed_feed_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_box_feed_id_twin_normal,
      parseErrorData: null,
      constMeta: kReturnBoxedFeedIdTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnBoxedFeedIdTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_boxed_feed_id_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<U8Array8> returnBoxedRawFeedIdTwinNormal(
      {required FeedIdTwinNormal id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feed_id_twin_normal(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_boxed_raw_feed_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_box_u_8_array_8,
      parseErrorData: null,
      constMeta: kReturnBoxedRawFeedIdTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnBoxedRawFeedIdTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_boxed_raw_feed_id_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<U8Array1600> useBoxedBlobTwinNormal(
      {required BlobTwinNormal blob, dynamic hint}) {
    var arg0 = api2wire_box_blob_twin_normal(blob);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_boxed_blob_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_8_array_1600,
      parseErrorData: null,
      constMeta: kUseBoxedBlobTwinNormalConstMeta,
      argValues: [blob],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseBoxedBlobTwinNormalConstMeta => const TaskConstMeta(
        debugName: "use_boxed_blob_twin_normal",
        argNames: ["blob"],
      );

  @override
  Future<U8Array32> useMsgidTwinNormal(
      {required MessageIdTwinNormal id, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_message_id_twin_normal(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_msgid_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_8_array_32,
      parseErrorData: null,
      constMeta: kUseMsgidTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseMsgidTwinNormalConstMeta => const TaskConstMeta(
        debugName: "use_msgid_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<void> handleCustomizedStructTwinNormal(
      {required CustomizedTwinNormal val, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_customized_twin_normal(val);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_customized_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kHandleCustomizedStructTwinNormalConstMeta,
      argValues: [val],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleCustomizedStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_customized_struct_twin_normal",
        argNames: ["val"],
      );

  @override
  Future<UserIdTwinNormal> nextUserIdTwinNormal(
      {required UserIdTwinNormal userId, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_user_id_twin_normal(userId);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_next_user_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_user_id_twin_normal,
      parseErrorData: null,
      constMeta: kNextUserIdTwinNormalConstMeta,
      argValues: [userId],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNextUserIdTwinNormalConstMeta => const TaskConstMeta(
        debugName: "next_user_id_twin_normal",
        argNames: ["userId"],
      );

  @override
  Future<DateTime> datetimeLocalTwinNormal(
      {required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Local(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_datetime_local_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Local,
      parseErrorData: null,
      constMeta: kDatetimeLocalTwinNormalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDatetimeLocalTwinNormalConstMeta => const TaskConstMeta(
        debugName: "datetime_local_twin_normal",
        argNames: ["d"],
      );

  @override
  Future<DateTime> datetimeUtcTwinNormal({required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Utc(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_datetime_utc_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Utc,
      parseErrorData: null,
      constMeta: kDatetimeUtcTwinNormalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDatetimeUtcTwinNormalConstMeta => const TaskConstMeta(
        debugName: "datetime_utc_twin_normal",
        argNames: ["d"],
      );

  @override
  Future<Duration> durationTwinNormal({required Duration d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Duration(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_duration_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Duration,
      parseErrorData: null,
      constMeta: kDurationTwinNormalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDurationTwinNormalConstMeta => const TaskConstMeta(
        debugName: "duration_twin_normal",
        argNames: ["d"],
      );

  @override
  Future<List<DateTime>> handleDurationsTwinNormal(
      {required List<Duration> durations,
      required DateTime since,
      dynamic hint}) {
    var arg0 = api2wire_Chrono_DurationList(durations);
    var arg1 = api2wire_Chrono_Local(since);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_durations_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_Chrono_LocalList,
      parseErrorData: null,
      constMeta: kHandleDurationsTwinNormalConstMeta,
      argValues: [durations, since],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleDurationsTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_durations_twin_normal",
        argNames: ["durations", "since"],
      );

  @override
  Future<List<Duration>> handleTimestampsTwinNormal(
      {required List<DateTime> timestamps,
      required DateTime epoch,
      dynamic hint}) {
    var arg0 = api2wire_Chrono_NaiveList(timestamps);
    var arg1 = api2wire_Chrono_Naive(epoch);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_timestamps_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_Chrono_DurationList,
      parseErrorData: null,
      constMeta: kHandleTimestampsTwinNormalConstMeta,
      argValues: [timestamps, epoch],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTimestampsTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_timestamps_twin_normal",
        argNames: ["timestamps", "epoch"],
      );

  @override
  Future<Duration> howLongDoesItTakeTwinNormal(
      {required FeatureChronoTwinNormal mine, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feature_chrono_twin_normal(mine);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_how_long_does_it_take_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Duration,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHowLongDoesItTakeTwinNormalConstMeta,
      argValues: [mine],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHowLongDoesItTakeTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "how_long_does_it_take_twin_normal",
        argNames: ["mine"],
      );

  @override
  Future<DateTime> naivedatetimeTwinNormal(
      {required DateTime d, dynamic hint}) {
    var arg0 = api2wire_Chrono_Naive(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_naivedatetime_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Chrono_Naive,
      parseErrorData: null,
      constMeta: kNaivedatetimeTwinNormalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNaivedatetimeTwinNormalConstMeta => const TaskConstMeta(
        debugName: "naivedatetime_twin_normal",
        argNames: ["d"],
      );

  @override
  Future<DateTime?> optionalEmptyDatetimeUtcTwinNormal(
      {DateTime? d, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_Chrono_Utc(d);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_optional_empty_datetime_utc_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_Chrono_Utc,
      parseErrorData: null,
      constMeta: kOptionalEmptyDatetimeUtcTwinNormalConstMeta,
      argValues: [d],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOptionalEmptyDatetimeUtcTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "optional_empty_datetime_utc_twin_normal",
        argNames: ["d"],
      );

  @override
  Future<TestChronoTwinNormal> testChronoTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_chrono_twin_normal(port_),
      parseSuccessData: _wire2api_test_chrono_twin_normal,
      parseErrorData: null,
      constMeta: kTestChronoTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestChronoTwinNormalConstMeta => const TaskConstMeta(
        debugName: "test_chrono_twin_normal",
        argNames: [],
      );

  @override
  Future<TestChronoTwinNormal> testPreciseChronoTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_precise_chrono_twin_normal(port_),
      parseSuccessData: _wire2api_test_chrono_twin_normal,
      parseErrorData: null,
      constMeta: kTestPreciseChronoTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestPreciseChronoTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_precise_chrono_twin_normal",
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
  Future<dynamic> returnDartDynamicTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_dart_dynamic_twin_normal(port_),
      parseSuccessData: _wire2api_dartabi,
      parseErrorData: null,
      constMeta: kReturnDartDynamicTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnDartDynamicTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_dart_dynamic_twin_normal",
        argNames: [],
      );

  @override
  Future<String> asyncAcceptDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_async_accept_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kAsyncAcceptDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAsyncAcceptDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "async_accept_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<EnumDartOpaqueTwinNormal> createEnumDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_create_enum_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_enum_dart_opaque_twin_normal,
      parseErrorData: null,
      constMeta: kCreateEnumDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateEnumDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "create_enum_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<DartOpaqueNestedTwinNormal> createNestedDartOpaqueTwinNormal(
      {required Object opaque1, required Object opaque2, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque1);
    var arg1 = api2wire_DartOpaque(opaque2);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_create_nested_dart_opaque_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_dart_opaque_nested_twin_normal,
      parseErrorData: null,
      constMeta: kCreateNestedDartOpaqueTwinNormalConstMeta,
      argValues: [opaque1, opaque2],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateNestedDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "create_nested_dart_opaque_twin_normal",
        argNames: ["opaque1", "opaque2"],
      );

  @override
  Future<void> dropStaticDartOpaqueTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_drop_static_dart_opaque_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kDropStaticDartOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDropStaticDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "drop_static_dart_opaque_twin_normal",
        argNames: [],
      );

  @override
  Future<void> getEnumDartOpaqueTwinNormal(
      {required EnumDartOpaqueTwinNormal opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_dart_opaque_twin_normal(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_get_enum_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kGetEnumDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetEnumDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "get_enum_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> getNestedDartOpaqueTwinNormal(
      {required DartOpaqueNestedTwinNormal opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_dart_opaque_nested_twin_normal(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_get_nested_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kGetNestedDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetNestedDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "get_nested_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackArrayGetTwinNormal(
      {required ObjectArray1 opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque_array_1(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_loop_back_array_get_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackArrayGetTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackArrayGetTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_array_get_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<ObjectArray1> loopBackArrayTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_array_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_DartOpaque_array_1,
      parseErrorData: null,
      constMeta: kLoopBackArrayTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackArrayTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_array_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackOptionGetTwinNormal({Object? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_loop_back_option_get_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackOptionGetTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackOptionGetTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "loop_back_option_get_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<Object?> loopBackOptionTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_option_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackOptionTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackOptionTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_option_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<Object> loopBackTwinNormal({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackVecGetTwinNormal(
      {required List<Object> opaque, dynamic hint}) {
    var arg0 = api2wire_list_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_vec_get_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackVecGetTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackVecGetTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_vec_get_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<List<Object>> loopBackVecTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_vec_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackVecTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackVecTwinNormalConstMeta => const TaskConstMeta(
        debugName: "loop_back_vec_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> panicUnwrapDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_panic_unwrap_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kPanicUnwrapDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPanicUnwrapDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "panic_unwrap_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> setStaticDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_set_static_dart_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kSetStaticDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSetStaticDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "set_static_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Object returnNonDroppableDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_return_non_droppable_dart_opaque_twin_normal(arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kReturnNonDroppableDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnNonDroppableDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_non_droppable_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  String syncAcceptDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_accept_dart_opaque_twin_normal(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kSyncAcceptDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncAcceptDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_accept_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Object syncLoopbackTwinNormal({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_loopback_twin_normal(arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kSyncLoopbackTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncLoopbackTwinNormalConstMeta => const TaskConstMeta(
        debugName: "sync_loopback_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Object? syncOptionDartOpaqueTwinNormal(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_dart_opaque_twin_normal(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionDartOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_option_dart_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Object? syncOptionLoopbackTwinNormal({Object? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_loopback_twin_normal(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: null,
      constMeta: kSyncOptionLoopbackTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionLoopbackTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_option_loopback_twin_normal",
        argNames: ["opaque"],
      );

  @override
  String unwrapDartOpaqueTwinNormal({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_unwrap_dart_opaque_twin_normal(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kUnwrapDartOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUnwrapDartOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "unwrap_dart_opaque_twin_normal",
        argNames: ["opaque"],
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
  Future<WeekdaysTwinNormal> handleEnumParameterTwinNormal(
      {required WeekdaysTwinNormal weekday, dynamic hint}) {
    var arg0 = api2wire_weekdays_twin_normal(weekday);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_enum_parameter_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_weekdays_twin_normal,
      parseErrorData: null,
      constMeta: kHandleEnumParameterTwinNormalConstMeta,
      argValues: [weekday],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleEnumParameterTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_enum_parameter_twin_normal",
        argNames: ["weekday"],
      );

  @override
  Future<KitchenSinkTwinNormal> handleEnumStructTwinNormal(
      {required KitchenSinkTwinNormal val, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_kitchen_sink_twin_normal(val);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_enum_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_kitchen_sink_twin_normal,
      parseErrorData: null,
      constMeta: kHandleEnumStructTwinNormalConstMeta,
      argValues: [val],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleEnumStructTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_enum_struct_twin_normal",
        argNames: ["val"],
      );

  @override
  Future<WeekdaysTwinNormal?> handleReturnEnumTwinNormal(
      {required String input, dynamic hint}) {
    var arg0 = api2wire_String(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_return_enum_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_weekdays_twin_normal,
      parseErrorData: null,
      constMeta: kHandleReturnEnumTwinNormalConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleReturnEnumTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_return_enum_twin_normal",
        argNames: ["input"],
      );

  @override
  Future<MeasureTwinNormal?> multiplyByTenTwinNormal(
      {required MeasureTwinNormal measure, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_measure_twin_normal(measure);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_multiply_by_ten_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_measure_twin_normal,
      parseErrorData: null,
      constMeta: kMultiplyByTenTwinNormalConstMeta,
      argValues: [measure],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMultiplyByTenTwinNormalConstMeta => const TaskConstMeta(
        debugName: "multiply_by_ten_twin_normal",
        argNames: ["measure"],
      );

  @override
  Future<Uint8List> printNoteTwinNormal(
      {required NoteTwinNormal note, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_note_twin_normal(note);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_print_note_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_ZeroCopyBuffer_list_prim_u_8,
      parseErrorData: null,
      constMeta: kPrintNoteTwinNormalConstMeta,
      argValues: [note],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrintNoteTwinNormalConstMeta => const TaskConstMeta(
        debugName: "print_note_twin_normal",
        argNames: ["note"],
      );

  @override
  Future<String> eventTwinNormalAsStringTwinNormal(
      {required EventTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_event_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_EventTwinNormal_as_string_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kEventTwinNormalAsStringTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kEventTwinNormalAsStringTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "EventTwinNormal_as_string_twin_normal",
        argNames: ["that"],
      );

  @override
  Future<void> closeEventListenerTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_close_event_listener_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kCloseEventListenerTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCloseEventListenerTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "close_event_listener_twin_normal",
        argNames: [],
      );

  @override
  Future<void> createEventTwinNormal(
      {required String address, required String payload, dynamic hint}) {
    var arg0 = api2wire_String(address);
    var arg1 = api2wire_String(payload);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_event_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kCreateEventTwinNormalConstMeta,
      argValues: [address, payload],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateEventTwinNormalConstMeta => const TaskConstMeta(
        debugName: "create_event_twin_normal",
        argNames: ["address", "payload"],
      );

  @override
  Stream<EventTwinNormal> registerEventListenerTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_register_event_listener_twin_normal(port_),
      parseSuccessData: _wire2api_event_twin_normal,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kRegisterEventListenerTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRegisterEventListenerTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "register_event_listener_twin_normal",
        argNames: [],
      );

  @override
  Future<CustomStructTwinNormal> customStructTwinNormalNewTwinNormal(
      {required String message, dynamic hint}) {
    var arg0 = api2wire_String(message);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_CustomStructTwinNormal_new_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_custom_struct_twin_normal,
      parseErrorData: null,
      constMeta: kCustomStructTwinNormalNewTwinNormalConstMeta,
      argValues: [message],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructTwinNormalNewTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "CustomStructTwinNormal_new_twin_normal",
        argNames: ["message"],
      );

  @override
  Future<void> customStructTwinNormalNonstaticReturnCustomStructErrorTwinNormal(
      {required CustomStructTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta:
          kCustomStructTwinNormalNonstaticReturnCustomStructErrorTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kCustomStructTwinNormalNonstaticReturnCustomStructErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal",
            argNames: ["that"],
          );

  @override
  Future<int> customStructTwinNormalNonstaticReturnCustomStructOkTwinNormal(
      {required CustomStructTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta:
          kCustomStructTwinNormalNonstaticReturnCustomStructOkTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kCustomStructTwinNormalNonstaticReturnCustomStructOkTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal",
            argNames: ["that"],
          );

  @override
  Future<void> customStructTwinNormalStaticReturnCustomStructErrorTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal(
              port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta:
          kCustomStructTwinNormalStaticReturnCustomStructErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kCustomStructTwinNormalStaticReturnCustomStructErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "CustomStructTwinNormal_static_return_custom_struct_error_twin_normal",
            argNames: [],
          );

  @override
  Future<int> customStructTwinNormalStaticReturnCustomStructOkTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal(
              port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta:
          kCustomStructTwinNormalStaticReturnCustomStructOkTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kCustomStructTwinNormalStaticReturnCustomStructOkTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal",
            argNames: [],
          );

  @override
  Future<SomeStructTwinNormal> someStructTwinNormalNewTwinNormal(
      {required int value, dynamic hint}) {
    var arg0 = api2wire_u_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SomeStructTwinNormal_new_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_some_struct_twin_normal,
      parseErrorData: null,
      constMeta: kSomeStructTwinNormalNewTwinNormalConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructTwinNormalNewTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "SomeStructTwinNormal_new_twin_normal",
        argNames: ["value"],
      );

  @override
  Future<int> someStructTwinNormalNonStaticReturnErrCustomErrorTwinNormal(
      {required SomeStructTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_some_struct_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta:
          kSomeStructTwinNormalNonStaticReturnErrCustomErrorTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kSomeStructTwinNormalNonStaticReturnErrCustomErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal",
            argNames: ["that"],
          );

  @override
  Future<int> someStructTwinNormalNonStaticReturnOkCustomErrorTwinNormal(
      {required SomeStructTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_some_struct_twin_normal(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta:
          kSomeStructTwinNormalNonStaticReturnOkCustomErrorTwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kSomeStructTwinNormalNonStaticReturnOkCustomErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal",
            argNames: ["that"],
          );

  @override
  Future<int> someStructTwinNormalStaticReturnErrCustomErrorTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal(
              port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta:
          kSomeStructTwinNormalStaticReturnErrCustomErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kSomeStructTwinNormalStaticReturnErrCustomErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "SomeStructTwinNormal_static_return_err_custom_error_twin_normal",
            argNames: [],
          );

  @override
  Future<int> someStructTwinNormalStaticReturnOkCustomErrorTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal(
              port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta:
          kSomeStructTwinNormalStaticReturnOkCustomErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kSomeStructTwinNormalStaticReturnOkCustomErrorTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "SomeStructTwinNormal_static_return_ok_custom_error_twin_normal",
            argNames: [],
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
  Future<void> panicWithCustomResultTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_panic_with_custom_result_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta: kPanicWithCustomResultTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPanicWithCustomResultTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "panic_with_custom_result_twin_normal",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError1TwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_custom_nested_error_1_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_1_twin_normal,
      constMeta: kReturnCustomNestedError1TwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError1TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_nested_error_1_twin_normal",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError1Variant1TwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_custom_nested_error_1_variant1_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_1_twin_normal,
      constMeta: kReturnCustomNestedError1Variant1TwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError1Variant1TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_nested_error_1_variant1_twin_normal",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError2TwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_custom_nested_error_2_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_2_twin_normal,
      constMeta: kReturnCustomNestedError2TwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError2TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_nested_error_2_twin_normal",
        argNames: [],
      );

  @override
  Future<void> returnCustomStructErrorTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_custom_struct_error_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta: kReturnCustomStructErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomStructErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_struct_error_twin_normal",
        argNames: [],
      );

  @override
  Future<int> returnCustomStructOkTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_custom_struct_ok_twin_normal(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error_another_twin_normal,
      constMeta: kReturnCustomStructOkTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomStructOkTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_struct_ok_twin_normal",
        argNames: [],
      );

  @override
  Future<int> returnErrCustomErrorTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_err_custom_error_twin_normal(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta: kReturnErrCustomErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnErrCustomErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_err_custom_error_twin_normal",
        argNames: [],
      );

  @override
  Future<int> returnErrorVariantTwinNormal(
      {required int variant, dynamic hint}) {
    var arg0 = api2wire_u_32(variant);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_error_variant_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta: kReturnErrorVariantTwinNormalConstMeta,
      argValues: [variant],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnErrorVariantTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_error_variant_twin_normal",
        argNames: ["variant"],
      );

  @override
  Future<int> returnOkCustomErrorTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_ok_custom_error_twin_normal(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error_twin_normal,
      constMeta: kReturnOkCustomErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnOkCustomErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "return_ok_custom_error_twin_normal",
        argNames: [],
      );

  @override
  Stream<String> streamSinkThrowAnyhowTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_stream_sink_throw_anyhow_twin_normal(port_),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kStreamSinkThrowAnyhowTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStreamSinkThrowAnyhowTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "stream_sink_throw_anyhow_twin_normal",
        argNames: [],
      );

  @override
  void syncReturnCustomStructErrorTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_return_custom_struct_error_twin_normal(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error_twin_normal,
      constMeta: kSyncReturnCustomStructErrorTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncReturnCustomStructErrorTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_return_custom_struct_error_twin_normal",
        argNames: [],
      );

  @override
  Future<void> throwAnyhowTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_throw_anyhow_twin_normal(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kThrowAnyhowTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kThrowAnyhowTwinNormalConstMeta => const TaskConstMeta(
        debugName: "throw_anyhow_twin_normal",
        argNames: [],
      );

  @override
  Future<NewSimpleStruct> callNewModuleSystemTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_call_new_module_system_twin_normal(port_),
      parseSuccessData: _wire2api_new_simple_struct,
      parseErrorData: null,
      constMeta: kCallNewModuleSystemTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCallNewModuleSystemTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "call_new_module_system_twin_normal",
        argNames: [],
      );

  @override
  Future<OldSimpleStruct> callOldModuleSystemTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_call_old_module_system_twin_normal(port_),
      parseSuccessData: _wire2api_old_simple_struct,
      parseErrorData: null,
      constMeta: kCallOldModuleSystemTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCallOldModuleSystemTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "call_old_module_system_twin_normal",
        argNames: [],
      );

  @override
  Future<bool> useImportedEnumTwinNormal(
      {required MyEnum myEnum, dynamic hint}) {
    var arg0 = api2wire_my_enum(myEnum);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_imported_enum_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kUseImportedEnumTwinNormalConstMeta,
      argValues: [myEnum],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseImportedEnumTwinNormalConstMeta => const TaskConstMeta(
        debugName: "use_imported_enum_twin_normal",
        argNames: ["myEnum"],
      );

  @override
  Future<bool> useImportedStructTwinNormal(
      {required MyStruct myStruct, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_struct(myStruct);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_use_imported_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kUseImportedStructTwinNormalConstMeta,
      argValues: [myStruct],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseImportedStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "use_imported_struct_twin_normal",
        argNames: ["myStruct"],
      );

  @override
  Future<AnotherMacroStructTwinNormal> anotherMacroStructTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_another_macro_struct_twin_normal(port_),
      parseSuccessData: _wire2api_another_macro_struct_twin_normal,
      parseErrorData: null,
      constMeta: kAnotherMacroStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAnotherMacroStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "another_macro_struct_twin_normal",
        argNames: [],
      );

  @override
  Future<MacroStruct> funcMacroStructTwinNormal(
      {required MacroStruct arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_macro_struct(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_func_macro_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_macro_struct,
      parseErrorData: null,
      constMeta: kFuncMacroStructTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFuncMacroStructTwinNormalConstMeta => const TaskConstMeta(
        debugName: "func_macro_struct_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<String> concatenateWithTwinNormalConcatenateStaticTwinNormal(
      {required String a, required String b, dynamic hint}) {
    var arg0 = api2wire_String(a);
    var arg1 = api2wire_String(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal(
              port_, arg0, arg1),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kConcatenateWithTwinNormalConcatenateStaticTwinNormalConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithTwinNormalConcatenateStaticTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWithTwinNormal_concatenate_static_twin_normal",
            argNames: ["a", "b"],
          );

  @override
  Future<String> concatenateWithTwinNormalConcatenateTwinNormal(
      {required ConcatenateWithTwinNormal that,
      required String b,
      dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with_twin_normal(that);
    var arg1 = api2wire_String(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWithTwinNormal_concatenate_twin_normal(
              port_, arg0, arg1),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kConcatenateWithTwinNormalConcatenateTwinNormalConstMeta,
      argValues: [that, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithTwinNormalConcatenateTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWithTwinNormal_concatenate_twin_normal",
        argNames: ["that", "b"],
      );

  @override
  Stream<int>
      concatenateWithTwinNormalHandleSomeStaticStreamSinkSingleArgTwinNormal(
          {dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire
          .wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal(
              port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta:
          kConcatenateWithTwinNormalHandleSomeStaticStreamSinkSingleArgTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithTwinNormalHandleSomeStaticStreamSinkSingleArgTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal",
            argNames: [],
          );

  @override
  Stream<Log2TwinNormal>
      concatenateWithTwinNormalHandleSomeStaticStreamSinkTwinNormal(
          {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire
          .wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal(
              port_, arg0, arg1),
      parseSuccessData: _wire2api_log_2_twin_normal,
      parseErrorData: null,
      constMeta:
          kConcatenateWithTwinNormalHandleSomeStaticStreamSinkTwinNormalConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithTwinNormalHandleSomeStaticStreamSinkTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal",
            argNames: ["key", "max"],
          );

  @override
  Stream<int> concatenateWithTwinNormalHandleSomeStreamSinkAt1TwinNormal(
      {required ConcatenateWithTwinNormal that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with_twin_normal(that);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire
          .wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal(
              port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta:
          kConcatenateWithTwinNormalHandleSomeStreamSinkAt1TwinNormalConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithTwinNormalHandleSomeStreamSinkAt1TwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal",
            argNames: ["that"],
          );

  @override
  Stream<Log2TwinNormal>
      concatenateWithTwinNormalHandleSomeStreamSinkTwinNormal(
          {required ConcatenateWithTwinNormal that,
          required int key,
          required int max,
          dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with_twin_normal(that);
    var arg1 = api2wire_u_32(key);
    var arg2 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire
          .wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal(
              port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_log_2_twin_normal,
      parseErrorData: null,
      constMeta:
          kConcatenateWithTwinNormalHandleSomeStreamSinkTwinNormalConstMeta,
      argValues: [that, key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithTwinNormalHandleSomeStreamSinkTwinNormalConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal",
            argNames: ["that", "key", "max"],
          );

  @override
  Future<ConcatenateWithTwinNormal> concatenateWithTwinNormalNewTwinNormal(
      {required String a, dynamic hint}) {
    var arg0 = api2wire_String(a);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWithTwinNormal_new_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_concatenate_with_twin_normal,
      parseErrorData: null,
      constMeta: kConcatenateWithTwinNormalNewTwinNormalConstMeta,
      argValues: [a],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithTwinNormalNewTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWithTwinNormal_new_twin_normal",
        argNames: ["a"],
      );

  @override
  Future<int> sumWithTwinNormalSumTwinNormal(
      {required SumWithTwinNormal that,
      required int y,
      required int z,
      dynamic hint}) {
    var arg0 = api2wire_box_autoadd_sum_with_twin_normal(that);
    var arg1 = api2wire_u_32(y);
    var arg2 = api2wire_u_32(z);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SumWithTwinNormal_sum_twin_normal(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kSumWithTwinNormalSumTwinNormalConstMeta,
      argValues: [that, y, z],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSumWithTwinNormalSumTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "SumWithTwinNormal_sum_twin_normal",
        argNames: ["that", "y", "z"],
      );

  @override
  Future<SumWithTwinNormalArray3> getSumArrayTwinNormal(
      {required int a, required int b, required int c, dynamic hint}) {
    var arg0 = api2wire_u_32(a);
    var arg1 = api2wire_u_32(b);
    var arg2 = api2wire_u_32(c);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_get_sum_array_twin_normal(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_sum_with_twin_normal_array_3,
      parseErrorData: null,
      constMeta: kGetSumArrayTwinNormalConstMeta,
      argValues: [a, b, c],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetSumArrayTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_sum_array_twin_normal",
        argNames: ["a", "b", "c"],
      );

  @override
  Future<SumWithTwinNormal> getSumStructTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_sum_struct_twin_normal(port_),
      parseSuccessData: _wire2api_sum_with_twin_normal,
      parseErrorData: null,
      constMeta: kGetSumStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetSumStructTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_sum_struct_twin_normal",
        argNames: [],
      );

  @override
  Stream<ApplicationSettings> appSettingsStreamTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_app_settings_stream_twin_normal(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: null,
      constMeta: kAppSettingsStreamTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAppSettingsStreamTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "app_settings_stream_twin_normal",
        argNames: [],
      );

  @override
  Stream<List<ApplicationSettings>> appSettingsVecStreamTwinNormal(
      {dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_app_settings_vec_stream_twin_normal(port_),
      parseSuccessData: _wire2api_list_application_settings,
      parseErrorData: null,
      constMeta: kAppSettingsVecStreamTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAppSettingsVecStreamTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "app_settings_vec_stream_twin_normal",
        argNames: [],
      );

  @override
  Future<int?> firstNumberTwinNormal({required Numbers nums, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_numbers(nums);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_first_number_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kFirstNumberTwinNormalConstMeta,
      argValues: [nums],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFirstNumberTwinNormalConstMeta => const TaskConstMeta(
        debugName: "first_number_twin_normal",
        argNames: ["nums"],
      );

  @override
  Future<int?> firstSequenceTwinNormal(
      {required Sequences seqs, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_sequences(seqs);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_first_sequence_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kFirstSequenceTwinNormalConstMeta,
      argValues: [seqs],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFirstSequenceTwinNormalConstMeta => const TaskConstMeta(
        debugName: "first_sequence_twin_normal",
        argNames: ["seqs"],
      );

  @override
  Future<ApplicationSettings> getAppSettingsTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_app_settings_twin_normal(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: null,
      constMeta: kGetAppSettingsTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetAppSettingsTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_app_settings_twin_normal",
        argNames: [],
      );

  @override
  Future<ApplicationSettings> getFallibleAppSettingsTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_get_fallible_app_settings_twin_normal(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kGetFallibleAppSettingsTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetFallibleAppSettingsTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "get_fallible_app_settings_twin_normal",
        argNames: [],
      );

  @override
  Future<ApplicationMessage> getMessageTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_message_twin_normal(port_),
      parseSuccessData: _wire2api_application_message,
      parseErrorData: null,
      constMeta: kGetMessageTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetMessageTwinNormalConstMeta => const TaskConstMeta(
        debugName: "get_message_twin_normal",
        argNames: [],
      );

  @override
  Future<bool> isAppEmbeddedTwinNormal(
      {required ApplicationSettings appSettings, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_application_settings(appSettings);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_is_app_embedded_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kIsAppEmbeddedTwinNormalConstMeta,
      argValues: [appSettings],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kIsAppEmbeddedTwinNormalConstMeta => const TaskConstMeta(
        debugName: "is_app_embedded_twin_normal",
        argNames: ["appSettings"],
      );

  @override
  Stream<MirrorStructTwinNormal> mirrorStructStreamTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_mirror_struct_stream_twin_normal(port_),
      parseSuccessData: _wire2api_mirror_struct_twin_normal,
      parseErrorData: null,
      constMeta: kMirrorStructStreamTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMirrorStructStreamTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "mirror_struct_stream_twin_normal",
        argNames: [],
      );

  @override
  Stream<(ApplicationSettings, RawStringEnumMirrored)>
      mirrorTupleStreamTwinNormal({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_mirror_tuple_stream_twin_normal(port_),
      parseSuccessData:
          _wire2api_record_application_settings_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kMirrorTupleStreamTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMirrorTupleStreamTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "mirror_tuple_stream_twin_normal",
        argNames: [],
      );

  @override
  Future<Numbers> repeatNumberTwinNormal(
      {required int num, required int times, dynamic hint}) {
    var arg0 = api2wire_i_32(num);
    var arg1 = api2wire_usize(times);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_repeat_number_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_numbers,
      parseErrorData: null,
      constMeta: kRepeatNumberTwinNormalConstMeta,
      argValues: [num, times],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRepeatNumberTwinNormalConstMeta => const TaskConstMeta(
        debugName: "repeat_number_twin_normal",
        argNames: ["num", "times"],
      );

  @override
  Future<Sequences> repeatSequenceTwinNormal(
      {required int seq, required int times, dynamic hint}) {
    var arg0 = api2wire_i_32(seq);
    var arg1 = api2wire_usize(times);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_repeat_sequence_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_sequences,
      parseErrorData: null,
      constMeta: kRepeatSequenceTwinNormalConstMeta,
      argValues: [seq, times],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRepeatSequenceTwinNormalConstMeta => const TaskConstMeta(
        debugName: "repeat_sequence_twin_normal",
        argNames: ["seq", "times"],
      );

  @override
  Future<ContainsMirroredSubStructTwinNormal>
      testContainsMirroredSubStructTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_contains_mirrored_sub_struct_twin_normal(port_),
      parseSuccessData: _wire2api_contains_mirrored_sub_struct_twin_normal,
      parseErrorData: null,
      constMeta: kTestContainsMirroredSubStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestContainsMirroredSubStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_contains_mirrored_sub_struct_twin_normal",
        argNames: [],
      );

  @override
  Future<List<RawStringMirrored>> testFallibleOfRawStringMirroredTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_fallible_of_raw_string_mirrored_twin_normal(port_),
      parseSuccessData: _wire2api_list_raw_string_mirrored,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kTestFallibleOfRawStringMirroredTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestFallibleOfRawStringMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_fallible_of_raw_string_mirrored_twin_normal",
        argNames: [],
      );

  @override
  Future<List<RawStringEnumMirrored>> testListOfNestedEnumsMirroredTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_list_of_nested_enums_mirrored_twin_normal(port_),
      parseSuccessData: _wire2api_list_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kTestListOfNestedEnumsMirroredTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestListOfNestedEnumsMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_list_of_nested_enums_mirrored_twin_normal",
        argNames: [],
      );

  @override
  Future<ListOfNestedRawStringMirrored>
      testListOfRawNestedStringMirroredTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_list_of_raw_nested_string_mirrored_twin_normal(port_),
      parseSuccessData: _wire2api_list_of_nested_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestListOfRawNestedStringMirroredTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestListOfRawNestedStringMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_list_of_raw_nested_string_mirrored_twin_normal",
        argNames: [],
      );

  @override
  Future<NestedRawStringMirrored> testNestedRawStringMirroredTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_nested_raw_string_mirrored_twin_normal(port_),
      parseSuccessData: _wire2api_nested_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestNestedRawStringMirroredTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestNestedRawStringMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_nested_raw_string_mirrored_twin_normal",
        argNames: [],
      );

  @override
  Future<RawStringEnumMirrored> testRawStringEnumMirroredTwinNormal(
      {required bool nested, dynamic hint}) {
    var arg0 = api2wire_bool(nested);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_raw_string_enum_mirrored_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kTestRawStringEnumMirroredTwinNormalConstMeta,
      argValues: [nested],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringEnumMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_raw_string_enum_mirrored_twin_normal",
        argNames: ["nested"],
      );

  @override
  Future<RawStringMirrored> testRawStringMirroredTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_raw_string_mirrored_twin_normal(port_),
      parseSuccessData: _wire2api_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestRawStringMirroredTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringMirroredTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_raw_string_mirrored_twin_normal",
        argNames: [],
      );

  @override
  Future<BigBuffersTwinNormal> handleBigBuffersTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_big_buffers_twin_normal(port_),
      parseSuccessData: _wire2api_big_buffers_twin_normal,
      parseErrorData: null,
      constMeta: kHandleBigBuffersTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleBigBuffersTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_big_buffers_twin_normal",
        argNames: [],
      );

  @override
  Future<MyTreeNodeTwinNormal> handleComplexStructTwinNormal(
      {required MyTreeNodeTwinNormal s, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_tree_node_twin_normal(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_complex_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_my_tree_node_twin_normal,
      parseErrorData: null,
      constMeta: kHandleComplexStructTwinNormalConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleComplexStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_complex_struct_twin_normal",
        argNames: ["s"],
      );

  @override
  Future<MyNestedStructTwinNormal> handleNestedStructTwinNormal(
      {required MyNestedStructTwinNormal s, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_nested_struct_twin_normal(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_nested_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_my_nested_struct_twin_normal,
      parseErrorData: null,
      constMeta: kHandleNestedStructTwinNormalConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNestedStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_nested_struct_twin_normal",
        argNames: ["s"],
      );

  @override
  Future<String> handleStringTwinNormal({required String s, dynamic hint}) {
    var arg0 = api2wire_String(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_string_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kHandleStringTwinNormalConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStringTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_string_twin_normal",
        argNames: ["s"],
      );

  @override
  MySizeFreezedTwinNormal handleStructSyncFreezedTwinNormal(
      {required MySizeFreezedTwinNormal arg,
      required MySizeFreezedTwinNormal boxed,
      dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_size_freezed_twin_normal(arg);
    var arg1 = api2wire_box_my_size_freezed_twin_normal(boxed);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_handle_struct_sync_freezed_twin_normal(arg0, arg1),
      parseSuccessData: _wire2api_my_size_freezed_twin_normal,
      parseErrorData: null,
      constMeta: kHandleStructSyncFreezedTwinNormalConstMeta,
      argValues: [arg, boxed],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStructSyncFreezedTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_struct_sync_freezed_twin_normal",
        argNames: ["arg", "boxed"],
      );

  @override
  Future<MySize> handleStructTwinNormal(
      {required MySize arg, required MySize boxed, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_size(arg);
    var arg1 = api2wire_box_my_size(boxed);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_struct_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_my_size,
      parseErrorData: null,
      constMeta: kHandleStructTwinNormalConstMeta,
      argValues: [arg, boxed],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStructTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_struct_twin_normal",
        argNames: ["arg", "boxed"],
      );

  @override
  Future<Uint8List> handleVecU8TwinNormal(
      {required Uint8List v, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_8(v);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_vec_u8_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_8,
      parseErrorData: null,
      constMeta: kHandleVecU8TwinNormalConstMeta,
      argValues: [v],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecU8TwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_vec_u8_twin_normal",
        argNames: ["v"],
      );

  @override
  Future<List<WeekdaysTwinNormal>> listOfPrimitiveEnumsTwinNormal(
      {required List<WeekdaysTwinNormal> weekdays, dynamic hint}) {
    var arg0 = api2wire_list_weekdays_twin_normal(weekdays);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_list_of_primitive_enums_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_weekdays_twin_normal,
      parseErrorData: null,
      constMeta: kListOfPrimitiveEnumsTwinNormalConstMeta,
      argValues: [weekdays],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kListOfPrimitiveEnumsTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "list_of_primitive_enums_twin_normal",
        argNames: ["weekdays"],
      );

  @override
  Future<AbcTwinNormal> testAbcEnumTwinNormal(
      {required AbcTwinNormal abc, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_abc_twin_normal(abc);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_abc_enum_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_abc_twin_normal,
      parseErrorData: null,
      constMeta: kTestAbcEnumTwinNormalConstMeta,
      argValues: [abc],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestAbcEnumTwinNormalConstMeta => const TaskConstMeta(
        debugName: "test_abc_enum_twin_normal",
        argNames: ["abc"],
      );

  @override
  Future<StructWithEnumTwinNormal> testStructWithEnumTwinNormal(
      {required StructWithEnumTwinNormal se, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_struct_with_enum_twin_normal(se);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_struct_with_enum_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_struct_with_enum_twin_normal,
      parseErrorData: null,
      constMeta: kTestStructWithEnumTwinNormalConstMeta,
      argValues: [se],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestStructWithEnumTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_struct_with_enum_twin_normal",
        argNames: ["se"],
      );

  @override
  Future<EmptyTwinNormal> emptyStructTwinNormal(
      {required EmptyTwinNormal empty, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_empty_twin_normal(empty);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_empty_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_empty_twin_normal,
      parseErrorData: null,
      constMeta: kEmptyStructTwinNormalConstMeta,
      argValues: [empty],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kEmptyStructTwinNormalConstMeta => const TaskConstMeta(
        debugName: "empty_struct_twin_normal",
        argNames: ["empty"],
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
  Future<List<MySize>> handleListOfStructTwinNormal(
      {required List<MySize> l, dynamic hint}) {
    var arg0 = api2wire_list_my_size(l);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_list_of_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_list_my_size,
      parseErrorData: null,
      constMeta: kHandleListOfStructTwinNormalConstMeta,
      argValues: [l],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleListOfStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_list_of_struct_twin_normal",
        argNames: ["l"],
      );

  @override
  Future<List<String>> handleStringListTwinNormal(
      {required List<String> names, dynamic hint}) {
    var arg0 = api2wire_StringList(names);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_string_list_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_StringList,
      parseErrorData: null,
      constMeta: kHandleStringListTwinNormalConstMeta,
      argValues: [names],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStringListTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_string_list_twin_normal",
        argNames: ["names"],
      );

  @override
  Future<NewTypeIntTwinNormal> handleNewtypeTwinNormal(
      {required NewTypeIntTwinNormal arg, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_new_type_int_twin_normal(arg);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_newtype_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_new_type_int_twin_normal,
      parseErrorData: null,
      constMeta: kHandleNewtypeTwinNormalConstMeta,
      argValues: [arg],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNewtypeTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_newtype_twin_normal",
        argNames: ["arg"],
      );

  @override
  Future<double> handleIncrementBoxedOptionalTwinNormal(
      {double? opt, dynamic hint}) {
    var arg0 = api2wire_opt_box_f_64(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_increment_boxed_optional_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kHandleIncrementBoxedOptionalTwinNormalConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleIncrementBoxedOptionalTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_increment_boxed_optional_twin_normal",
        argNames: ["opt"],
      );

  @override
  Future<String> handleOptionBoxArgumentsTwinNormal(
      {int? i8Box,
      int? u8Box,
      int? i32Box,
      int? i64Box,
      double? f64Box,
      bool? boolbox,
      ExoticOptionalsTwinNormal? structbox,
      dynamic hint}) {
    var arg0 = api2wire_opt_box_i_8(i8Box);
    var arg1 = api2wire_opt_box_u_8(u8Box);
    var arg2 = api2wire_opt_box_i_32(i32Box);
    var arg3 = api2wire_opt_box_i_64(i64Box);
    var arg4 = api2wire_opt_box_f_64(f64Box);
    var arg5 = api2wire_opt_box_bool(boolbox);
    var arg6 = api2wire_opt_box_exotic_optionals_twin_normal(structbox);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_option_box_arguments_twin_normal(
          port_, arg0, arg1, arg2, arg3, arg4, arg5, arg6),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kHandleOptionBoxArgumentsTwinNormalConstMeta,
      argValues: [i8Box, u8Box, i32Box, i64Box, f64Box, boolbox, structbox],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionBoxArgumentsTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_option_box_arguments_twin_normal",
        argNames: [
          "i8Box",
          "u8Box",
          "i32Box",
          "i64Box",
          "f64Box",
          "boolbox",
          "structbox"
        ],
      );

  @override
  Future<ExoticOptionalsTwinNormal?> handleOptionalIncrementTwinNormal(
      {ExoticOptionalsTwinNormal? opt, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_exotic_optionals_twin_normal(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_optional_increment_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_exotic_optionals_twin_normal,
      parseErrorData: null,
      constMeta: kHandleOptionalIncrementTwinNormalConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalIncrementTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_optional_increment_twin_normal",
        argNames: ["opt"],
      );

  @override
  Future<double?> handleOptionalReturnTwinNormal(
      {required double left, required double right, dynamic hint}) {
    var arg0 = api2wire_f_64(left);
    var arg1 = api2wire_f_64(right);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_optional_return_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_opt_box_autoadd_f_64,
      parseErrorData: null,
      constMeta: kHandleOptionalReturnTwinNormalConstMeta,
      argValues: [left, right],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalReturnTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_optional_return_twin_normal",
        argNames: ["left", "right"],
      );

  @override
  Future<ElementTwinNormal?> handleOptionalStructTwinNormal(
      {String? document, dynamic hint}) {
    var arg0 = api2wire_opt_String(document);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_optional_struct_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_element_twin_normal,
      parseErrorData: null,
      constMeta: kHandleOptionalStructTwinNormalConstMeta,
      argValues: [document],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_optional_struct_twin_normal",
        argNames: ["document"],
      );

  @override
  Future<OptVecsTwinNormal> handleVecOfOptsTwinNormal(
      {required OptVecsTwinNormal opt, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_opt_vecs_twin_normal(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_vec_of_opts_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_vecs_twin_normal,
      parseErrorData: null,
      constMeta: kHandleVecOfOptsTwinNormalConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecOfOptsTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_vec_of_opts_twin_normal",
        argNames: ["opt"],
      );

  @override
  String? syncOptionNullTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_null_twin_normal(),
      parseSuccessData: _wire2api_opt_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionNullTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionNullTwinNormalConstMeta => const TaskConstMeta(
        debugName: "sync_option_null_twin_normal",
        argNames: [],
      );

  @override
  String? syncOptionTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_twin_normal(),
      parseSuccessData: _wire2api_opt_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionTwinNormalConstMeta => const TaskConstMeta(
        debugName: "sync_option_twin_normal",
        argNames: [],
      );

  @override
  Future<int?> primitiveOptionalTypesTwinNormal(
      {int? myI32, int? myI64, double? myF64, bool? myBool, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_32(myI32);
    var arg1 = api2wire_opt_box_autoadd_i_64(myI64);
    var arg2 = api2wire_opt_box_autoadd_f_64(myF64);
    var arg3 = api2wire_opt_box_autoadd_bool(myBool);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_primitive_optional_types_twin_normal(
          port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kPrimitiveOptionalTypesTwinNormalConstMeta,
      argValues: [myI32, myI64, myF64, myBool],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveOptionalTypesTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "primitive_optional_types_twin_normal",
        argNames: ["myI32", "myI64", "myF64", "myBool"],
      );

  @override
  Future<VecOfPrimitivePackTwinNormal> handleVecOfPrimitiveTwinNormal(
      {required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_vec_of_primitive_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_vec_of_primitive_pack_twin_normal,
      parseErrorData: null,
      constMeta: kHandleVecOfPrimitiveTwinNormalConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecOfPrimitiveTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_vec_of_primitive_twin_normal",
        argNames: ["n"],
      );

  @override
  Future<ZeroCopyVecOfPrimitivePackTwinNormal>
      handleZeroCopyVecOfPrimitiveTwinNormal({required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_zero_copy_vec_of_primitive_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_zero_copy_vec_of_primitive_pack_twin_normal,
      parseErrorData: null,
      constMeta: kHandleZeroCopyVecOfPrimitiveTwinNormalConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleZeroCopyVecOfPrimitiveTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_zero_copy_vec_of_primitive_twin_normal",
        argNames: ["n"],
      );

  @override
  ZeroCopyVecOfPrimitivePackTwinNormal
      handleZeroCopyVecOfPrimitiveSyncTwinNormal(
          {required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeSync(SyncTask(
      callFfi: () =>
          wire.wire_handle_zero_copy_vec_of_primitive_sync_twin_normal(arg0),
      parseSuccessData: _wire2api_zero_copy_vec_of_primitive_pack_twin_normal,
      parseErrorData: null,
      constMeta: kHandleZeroCopyVecOfPrimitiveSyncTwinNormalConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleZeroCopyVecOfPrimitiveSyncTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_zero_copy_vec_of_primitive_sync_twin_normal",
        argNames: ["n"],
      );

  @override
  Future<int> primitiveTypesTwinNormal(
      {required int myI32,
      required int myI64,
      required double myF64,
      required bool myBool,
      dynamic hint}) {
    var arg0 = api2wire_i_32(myI32);
    var arg1 = api2wire_i_64(myI64);
    var arg2 = api2wire_f_64(myF64);
    var arg3 = api2wire_bool(myBool);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_primitive_types_twin_normal(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kPrimitiveTypesTwinNormalConstMeta,
      argValues: [myI32, myI64, myF64, myBool],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveTypesTwinNormalConstMeta => const TaskConstMeta(
        debugName: "primitive_types_twin_normal",
        argNames: ["myI32", "myI64", "myF64", "myBool"],
      );

  @override
  Future<int> primitiveU32TwinNormal({required int myU32, dynamic hint}) {
    var arg0 = api2wire_u_32(myU32);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_primitive_u32_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kPrimitiveU32TwinNormalConstMeta,
      argValues: [myU32],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveU32TwinNormalConstMeta => const TaskConstMeta(
        debugName: "primitive_u32_twin_normal",
        argNames: ["myU32"],
      );

  @override
  Future<MoreThanJustOneRawStringStructTwinNormal>
      testMoreThanJustOneRawStringStructTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_test_more_than_just_one_raw_string_struct_twin_normal(port_),
      parseSuccessData:
          _wire2api_more_than_just_one_raw_string_struct_twin_normal,
      parseErrorData: null,
      constMeta: kTestMoreThanJustOneRawStringStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestMoreThanJustOneRawStringStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_more_than_just_one_raw_string_struct_twin_normal",
        argNames: [],
      );

  @override
  Future<RawStringItemStructTwinNormal> testRawStringItemStructTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_raw_string_item_struct_twin_normal(port_),
      parseSuccessData: _wire2api_raw_string_item_struct_twin_normal,
      parseErrorData: null,
      constMeta: kTestRawStringItemStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringItemStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "test_raw_string_item_struct_twin_normal",
        argNames: [],
      );

  @override
  Future<EnumOpaqueTwinNormalArray5> createArrayOpaqueEnumTwinNormal(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_array_opaque_enum_twin_normal(port_),
      parseSuccessData: _wire2api_enum_opaque_twin_normal_array_5,
      parseErrorData: null,
      constMeta: kCreateArrayOpaqueEnumTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateArrayOpaqueEnumTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "create_array_opaque_enum_twin_normal",
        argNames: [],
      );

  @override
  Future<OpaqueNestedTwinNormal> createNestedOpaqueTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_nested_opaque_twin_normal(port_),
      parseSuccessData: _wire2api_opaque_nested_twin_normal,
      parseErrorData: null,
      constMeta: kCreateNestedOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateNestedOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "create_nested_opaque_twin_normal",
        argNames: [],
      );

  @override
  Future<HideData> createOpaqueTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_opaque_twin_normal(port_),
      parseSuccessData: _wire2api_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kCreateOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "create_opaque_twin_normal",
        argNames: [],
      );

  @override
  Future<HideData?> createOptionOpaqueTwinNormal(
      {HideData? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_create_option_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kCreateOptionOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateOptionOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "create_option_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<NonSendHideData> createSyncOpaqueTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_sync_opaque_twin_normal(port_),
      parseSuccessData: _wire2api_RustOpaque_non_send_hide_data,
      parseErrorData: null,
      constMeta: kCreateSyncOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateSyncOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "create_sync_opaque_twin_normal",
        argNames: [],
      );

  @override
  Future<FrbOpaqueReturn> frbGeneratorTestTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_frb_generator_test_twin_normal(port_),
      parseSuccessData: _wire2api_RustOpaque_frb_opaque_return,
      parseErrorData: null,
      constMeta: kFrbGeneratorTestTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFrbGeneratorTestTwinNormalConstMeta => const TaskConstMeta(
        debugName: "frb_generator_test_twin_normal",
        argNames: [],
      );

  @override
  Future<void> opaqueArrayRunTwinNormal(
      {required HideDataArray2 data, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data_array_2(data);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_array_run_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kOpaqueArrayRunTwinNormalConstMeta,
      argValues: [data],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueArrayRunTwinNormalConstMeta => const TaskConstMeta(
        debugName: "opaque_array_run_twin_normal",
        argNames: ["data"],
      );

  @override
  Future<HideDataArray2> opaqueArrayTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_array_twin_normal(port_),
      parseSuccessData: _wire2api_RustOpaque_hide_data_array_2,
      parseErrorData: null,
      constMeta: kOpaqueArrayTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueArrayTwinNormalConstMeta => const TaskConstMeta(
        debugName: "opaque_array_twin_normal",
        argNames: [],
      );

  @override
  Future<void> opaqueVecRunTwinNormal(
      {required List<HideData> data, dynamic hint}) {
    var arg0 = api2wire_list_RustOpaque_hide_data(data);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_vec_run_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kOpaqueVecRunTwinNormalConstMeta,
      argValues: [data],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueVecRunTwinNormalConstMeta => const TaskConstMeta(
        debugName: "opaque_vec_run_twin_normal",
        argNames: ["data"],
      );

  @override
  Future<List<HideData>> opaqueVecTwinNormal({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_vec_twin_normal(port_),
      parseSuccessData: _wire2api_list_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kOpaqueVecTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueVecTwinNormalConstMeta => const TaskConstMeta(
        debugName: "opaque_vec_twin_normal",
        argNames: [],
      );

  @override
  Future<String> runEnumOpaqueTwinNormal(
      {required EnumOpaqueTwinNormal opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_opaque_twin_normal(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_enum_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunEnumOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunEnumOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "run_enum_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<void> runNestedOpaqueTwinNormal(
      {required OpaqueNestedTwinNormal opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_opaque_nested_twin_normal(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_nested_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kRunNestedOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunNestedOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "run_nested_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<String> runNonCloneTwinNormal(
      {required NonCloneData clone, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_non_clone_data(clone);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_non_clone_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunNonCloneTwinNormalConstMeta,
      argValues: [clone],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunNonCloneTwinNormalConstMeta => const TaskConstMeta(
        debugName: "run_non_clone_twin_normal",
        argNames: ["clone"],
      );

  @override
  Future<String> runOpaqueTwinNormal({required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "run_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<String> runOpaqueWithDelayTwinNormal(
      {required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_run_opaque_with_delay_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunOpaqueWithDelayTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunOpaqueWithDelayTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "run_opaque_with_delay_twin_normal",
        argNames: ["opaque"],
      );

  @override
  Future<String> unwrapRustOpaqueTwinNormal(
      {required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_unwrap_rust_opaque_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kUnwrapRustOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUnwrapRustOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "unwrap_rust_opaque_twin_normal",
        argNames: ["opaque"],
      );

  @override
  FrbOpaqueSyncReturn frbSyncGeneratorTestTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_frb_sync_generator_test_twin_normal(),
      parseSuccessData: _wire2api_RustOpaque_frb_opaque_sync_return,
      parseErrorData: null,
      constMeta: kFrbSyncGeneratorTestTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFrbSyncGeneratorTestTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "frb_sync_generator_test_twin_normal",
        argNames: [],
      );

  @override
  NonCloneData syncCreateNonCloneTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_non_clone_twin_normal(),
      parseSuccessData: _wire2api_RustOpaque_non_clone_data,
      parseErrorData: null,
      constMeta: kSyncCreateNonCloneTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateNonCloneTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_create_non_clone_twin_normal",
        argNames: [],
      );

  @override
  HideData syncCreateOpaqueTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_opaque_twin_normal(),
      parseSuccessData: _wire2api_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kSyncCreateOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "sync_create_opaque_twin_normal",
        argNames: [],
      );

  @override
  NonSendHideData syncCreateSyncOpaqueTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_sync_opaque_twin_normal(),
      parseSuccessData: _wire2api_RustOpaque_non_send_hide_data,
      parseErrorData: null,
      constMeta: kSyncCreateSyncOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateSyncOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_create_sync_opaque_twin_normal",
        argNames: [],
      );

  @override
  HideData? syncOptionRustOpaqueTwinNormal({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_rust_opaque_twin_normal(),
      parseSuccessData: _wire2api_opt_box_autoadd_RustOpaque_hide_data,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionRustOpaqueTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionRustOpaqueTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "sync_option_rust_opaque_twin_normal",
        argNames: [],
      );

  @override
  String syncRunOpaqueTwinNormal(
      {required NonSendHideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_non_send_hide_data(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_run_opaque_twin_normal(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kSyncRunOpaqueTwinNormalConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncRunOpaqueTwinNormalConstMeta => const TaskConstMeta(
        debugName: "sync_run_opaque_twin_normal",
        argNames: ["opaque"],
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
  Stream<MyStreamEntryTwinNormal> handleStreamOfStructTwinNormal(
      {dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_handle_stream_of_struct_twin_normal(port_),
      parseSuccessData: _wire2api_my_stream_entry_twin_normal,
      parseErrorData: null,
      constMeta: kHandleStreamOfStructTwinNormalConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamOfStructTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_stream_of_struct_twin_normal",
        argNames: [],
      );

  @override
  Stream<LogTwinNormal> handleStreamSinkAt1TwinNormal(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_handle_stream_sink_at_1_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_log_twin_normal,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt1TwinNormalConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt1TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_stream_sink_at_1_twin_normal",
        argNames: ["key", "max"],
      );

  @override
  Stream<LogTwinNormal> handleStreamSinkAt2TwinNormal(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_handle_stream_sink_at_2_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_log_twin_normal,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt2TwinNormalConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt2TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_stream_sink_at_2_twin_normal",
        argNames: ["key", "max"],
      );

  @override
  Stream<LogTwinNormal> handleStreamSinkAt3TwinNormal(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_handle_stream_sink_at_3_twin_normal(port_, arg0, arg1),
      parseSuccessData: _wire2api_log_twin_normal,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt3TwinNormalConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt3TwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_stream_sink_at_3_twin_normal",
        argNames: ["key", "max"],
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
  Future<void> testTuple2TwinNormal(
      {required List<(String, int)> value, dynamic hint}) {
    var arg0 = api2wire_list_record_string_i_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_tuple_2_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kTestTuple2TwinNormalConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestTuple2TwinNormalConstMeta => const TaskConstMeta(
        debugName: "test_tuple_2_twin_normal",
        argNames: ["value"],
      );

  @override
  Future<(String, int)> testTupleTwinNormal(
      {(String, int)? value, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_record_string_i_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_tuple_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_record_string_i_32,
      parseErrorData: null,
      constMeta: kTestTupleTwinNormalConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestTupleTwinNormalConstMeta => const TaskConstMeta(
        debugName: "test_tuple_twin_normal",
        argNames: ["value"],
      );

  @override
  Future<int> handleTypeAliasIdTwinNormal({required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_type_alias_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kHandleTypeAliasIdTwinNormalConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeAliasIdTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_type_alias_id_twin_normal",
        argNames: ["input"],
      );

  @override
  Future<TestModelTwinNormal> handleTypeAliasModelTwinNormal(
      {required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_type_alias_model_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_test_model_twin_normal,
      parseErrorData: null,
      constMeta: kHandleTypeAliasModelTwinNormalConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeAliasModelTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_type_alias_model_twin_normal",
        argNames: ["input"],
      );

  @override
  Future<int> handleTypeNestAliasIdTwinNormal(
      {required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_type_nest_alias_id_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kHandleTypeNestAliasIdTwinNormalConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeNestAliasIdTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_type_nest_alias_id_twin_normal",
        argNames: ["input"],
      );

  @override
  Future<FeatureUuidTwinNormal> handleNestedUuidsTwinNormal(
      {required FeatureUuidTwinNormal ids, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_feature_uuid_twin_normal(ids);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_nested_uuids_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_feature_uuid_twin_normal,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleNestedUuidsTwinNormalConstMeta,
      argValues: [ids],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleNestedUuidsTwinNormalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_nested_uuids_twin_normal",
        argNames: ["ids"],
      );

  @override
  Future<UuidValue> handleUuidTwinNormal(
      {required UuidValue id, dynamic hint}) {
    var arg0 = api2wire_Uuid(id);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_uuid_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Uuid,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleUuidTwinNormalConstMeta,
      argValues: [id],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleUuidTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_uuid_twin_normal",
        argNames: ["id"],
      );

  @override
  Future<List<UuidValue>> handleUuidsTwinNormal(
      {required List<UuidValue> ids, dynamic hint}) {
    var arg0 = api2wire_Uuids(ids);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_uuids_twin_normal(port_, arg0),
      parseSuccessData: _wire2api_Uuids,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kHandleUuidsTwinNormalConstMeta,
      argValues: [ids],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleUuidsTwinNormalConstMeta => const TaskConstMeta(
        debugName: "handle_uuids_twin_normal",
        argNames: ["ids"],
      );

  OpaqueShareFnType get shareOpaqueMutexHideData =>
      wire.share_opaque_RustOpaque_MutexHideData;

  OpaqueDropFnType get dropOpaqueMutexHideData =>
      wire.drop_opaque_RustOpaque_MutexHideData;

  OpaqueShareFnType get shareOpaqueRwLockHideData =>
      wire.share_opaque_RustOpaque_RwLockHideData;

  OpaqueDropFnType get dropOpaqueRwLockHideData =>
      wire.drop_opaque_RustOpaque_RwLockHideData;

  OpaqueShareFnType get shareOpaqueBoxDartDebug =>
      wire.share_opaque_RustOpaque_box_dynDartDebug;

  OpaqueDropFnType get dropOpaqueBoxDartDebug =>
      wire.drop_opaque_RustOpaque_box_dynDartDebug;

  OpaqueShareFnType get shareOpaqueFrbOpaqueReturn =>
      wire.share_opaque_RustOpaque_frb_opaque_return;

  OpaqueDropFnType get dropOpaqueFrbOpaqueReturn =>
      wire.drop_opaque_RustOpaque_frb_opaque_return;

  OpaqueShareFnType get shareOpaqueFrbOpaqueSyncReturn =>
      wire.share_opaque_RustOpaque_frb_opaque_sync_return;

  OpaqueDropFnType get dropOpaqueFrbOpaqueSyncReturn =>
      wire.drop_opaque_RustOpaque_frb_opaque_sync_return;

  OpaqueShareFnType get shareOpaqueHideData =>
      wire.share_opaque_RustOpaque_hide_data;

  OpaqueDropFnType get dropOpaqueHideData =>
      wire.drop_opaque_RustOpaque_hide_data;

  OpaqueShareFnType get shareOpaqueI32 => wire.share_opaque_RustOpaque_i_32;

  OpaqueDropFnType get dropOpaqueI32 => wire.drop_opaque_RustOpaque_i_32;

  OpaqueShareFnType get shareOpaqueNonCloneData =>
      wire.share_opaque_RustOpaque_non_clone_data;

  OpaqueDropFnType get dropOpaqueNonCloneData =>
      wire.drop_opaque_RustOpaque_non_clone_data;

  OpaqueShareFnType get shareOpaqueNonSendHideData =>
      wire.share_opaque_RustOpaque_non_send_hide_data;

  OpaqueDropFnType get dropOpaqueNonSendHideData =>
      wire.drop_opaque_RustOpaque_non_send_hide_data;

  AnyhowException _wire2api_AnyhowException(dynamic raw) {
    return AnyhowException(raw as String);
  }

  Duration _wire2api_Chrono_Duration(dynamic raw) {
    return wire2apiDuration(_wire2api_i_64(raw).toInt());
  }

  List<Duration> _wire2api_Chrono_DurationList(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_Chrono_Duration).toList();
  }

  DateTime _wire2api_Chrono_Local(dynamic raw) {
    return wire2apiTimestamp(ts: _wire2api_i_64(raw).toInt(), isUtc: false);
  }

  List<DateTime> _wire2api_Chrono_LocalList(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_Chrono_Local).toList();
  }

  DateTime _wire2api_Chrono_Naive(dynamic raw) {
    return wire2apiTimestamp(ts: _wire2api_i_64(raw).toInt(), isUtc: true);
  }

  DateTime _wire2api_Chrono_Utc(dynamic raw) {
    return wire2apiTimestamp(ts: _wire2api_i_64(raw).toInt(), isUtc: true);
  }

  Object _wire2api_DartOpaque(dynamic raw) {
    return generalizedFrbRustBinding.getDartObject(raw);
  }

  ObjectArray1 _wire2api_DartOpaque_array_1(dynamic raw) {
    return ObjectArray1(
        (raw as List<dynamic>).map(_wire2api_DartOpaque).toList());
  }

  MutexHideData _wire2api_RustOpaque_MutexHideData(dynamic raw) {
    return MutexHideData.fromRaw(raw[0], raw[1]);
  }

  RwLockHideData _wire2api_RustOpaque_RwLockHideData(dynamic raw) {
    return RwLockHideData.fromRaw(raw[0], raw[1]);
  }

  BoxDartDebug _wire2api_RustOpaque_box_dynDartDebug(dynamic raw) {
    return BoxDartDebug.fromRaw(raw[0], raw[1]);
  }

  FrbOpaqueReturn _wire2api_RustOpaque_frb_opaque_return(dynamic raw) {
    return FrbOpaqueReturn.fromRaw(raw[0], raw[1]);
  }

  FrbOpaqueSyncReturn _wire2api_RustOpaque_frb_opaque_sync_return(dynamic raw) {
    return FrbOpaqueSyncReturn.fromRaw(raw[0], raw[1]);
  }

  HideData _wire2api_RustOpaque_hide_data(dynamic raw) {
    return HideData.fromRaw(raw[0], raw[1]);
  }

  HideDataArray2 _wire2api_RustOpaque_hide_data_array_2(dynamic raw) {
    return HideDataArray2(
        (raw as List<dynamic>).map(_wire2api_RustOpaque_hide_data).toList());
  }

  I32 _wire2api_RustOpaque_i_32(dynamic raw) {
    return I32.fromRaw(raw[0], raw[1]);
  }

  NonCloneData _wire2api_RustOpaque_non_clone_data(dynamic raw) {
    return NonCloneData.fromRaw(raw[0], raw[1]);
  }

  NonSendHideData _wire2api_RustOpaque_non_send_hide_data(dynamic raw) {
    return NonSendHideData.fromRaw(raw[0], raw[1]);
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
    const kUuidSizeInBytes = 16;
    final bytes = _wire2api_list_prim_u_8(raw);
    return List.generate(
      bytes.lengthInBytes ~/ kUuidSizeInBytes,
      (i) => UuidValue.fromByteList(
          Uint8List.view(bytes.buffer, i * kUuidSizeInBytes, kUuidSizeInBytes)),
      growable: false,
    );
  }

  Float32List _wire2api_ZeroCopyBuffer_list_prim_f_32(dynamic raw) {
    return raw as Float32List;
  }

  Float64List _wire2api_ZeroCopyBuffer_list_prim_f_64(dynamic raw) {
    return raw as Float64List;
  }

  Int16List _wire2api_ZeroCopyBuffer_list_prim_i_16(dynamic raw) {
    return raw as Int16List;
  }

  Int32List _wire2api_ZeroCopyBuffer_list_prim_i_32(dynamic raw) {
    return raw as Int32List;
  }

  Int64List _wire2api_ZeroCopyBuffer_list_prim_i_64(dynamic raw) {
    return _wire2api_list_prim_i_64(raw);
  }

  Int8List _wire2api_ZeroCopyBuffer_list_prim_i_8(dynamic raw) {
    return raw as Int8List;
  }

  Uint16List _wire2api_ZeroCopyBuffer_list_prim_u_16(dynamic raw) {
    return raw as Uint16List;
  }

  Uint32List _wire2api_ZeroCopyBuffer_list_prim_u_32(dynamic raw) {
    return raw as Uint32List;
  }

  Uint64List _wire2api_ZeroCopyBuffer_list_prim_u_64(dynamic raw) {
    return _wire2api_list_prim_u_64(raw);
  }

  Uint8List _wire2api_ZeroCopyBuffer_list_prim_u_8(dynamic raw) {
    return raw as Uint8List;
  }

  ATwinNormal _wire2api_a_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ATwinNormal(
      a: _wire2api_String(arr[0]),
    );
  }

  AbcTwinNormal _wire2api_abc_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return AbcTwinNormal_A(
          _wire2api_box_autoadd_a_twin_normal(raw[1]),
        );
      case 1:
        return AbcTwinNormal_B(
          _wire2api_box_autoadd_b_twin_normal(raw[1]),
        );
      case 2:
        return AbcTwinNormal_C(
          _wire2api_box_autoadd_c_twin_normal(raw[1]),
        );
      case 3:
        return AbcTwinNormal_JustInt(
          _wire2api_i_32(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  AnotherMacroStructTwinNormal _wire2api_another_macro_struct_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return AnotherMacroStructTwinNormal(
      data: _wire2api_i_32(arr[0]),
      nonFinalData: _wire2api_i_32(arr[1]),
    );
  }

  AnotherTwinNormal _wire2api_another_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return AnotherTwinNormal(
      a: _wire2api_String(arr[0]),
    );
  }

  ApplicationEnv _wire2api_application_env(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ApplicationEnv(
      vars: _wire2api_list_application_env_var(arr[0]),
    );
  }

  ApplicationEnvVar _wire2api_application_env_var(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ApplicationEnvVar(
      field0: _wire2api_String(arr[0]),
      field1: _wire2api_bool(arr[1]),
    );
  }

  ApplicationMessage _wire2api_application_message(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return ApplicationMessage_DisplayMessage(
          _wire2api_String(raw[1]),
        );
      case 1:
        return ApplicationMessage_RenderPixel(
          x: _wire2api_i_32(raw[1]),
          y: _wire2api_i_32(raw[2]),
        );
      case 2:
        return ApplicationMessage_Exit();
      default:
        throw Exception("unreachable");
    }
  }

  ApplicationMode _wire2api_application_mode(dynamic raw) {
    return ApplicationMode.values[raw as int];
  }

  ApplicationSettings _wire2api_application_settings(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 5)
      throw Exception('unexpected arr length: expect 5 but see ${arr.length}');
    return ApplicationSettings(
      name: _wire2api_String(arr[0]),
      version: _wire2api_String(arr[1]),
      mode: _wire2api_application_mode(arr[2]),
      env: _wire2api_box_application_env(arr[3]),
      envOptional: _wire2api_opt_box_autoadd_application_env(arr[4]),
    );
  }

  AttributeTwinNormal _wire2api_attribute_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return AttributeTwinNormal(
      key: _wire2api_String(arr[0]),
      value: _wire2api_String(arr[1]),
    );
  }

  BTwinNormal _wire2api_b_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return BTwinNormal(
      b: _wire2api_i_32(arr[0]),
    );
  }

  BigBuffersTwinNormal _wire2api_big_buffers_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return BigBuffersTwinNormal(
      int64: _wire2api_list_prim_i_64(arr[0]),
      uint64: _wire2api_list_prim_u_64(arr[1]),
    );
  }

  BlobTwinNormal _wire2api_blob_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return BlobTwinNormal(
      field0: _wire2api_u_8_array_1600(arr[0]),
    );
  }

  bool _wire2api_bool(dynamic raw) {
    return raw as bool;
  }

  ApplicationEnv _wire2api_box_application_env(dynamic raw) {
    return _wire2api_application_env(raw);
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

  Object _wire2api_box_autoadd_DartOpaque(dynamic raw) {
    return _wire2api_DartOpaque(raw);
  }

  HideData _wire2api_box_autoadd_RustOpaque_hide_data(dynamic raw) {
    return _wire2api_RustOpaque_hide_data(raw);
  }

  ATwinNormal _wire2api_box_autoadd_a_twin_normal(dynamic raw) {
    return _wire2api_a_twin_normal(raw);
  }

  ApplicationEnv _wire2api_box_autoadd_application_env(dynamic raw) {
    return _wire2api_application_env(raw);
  }

  AttributeTwinNormal _wire2api_box_autoadd_attribute_twin_normal(dynamic raw) {
    return _wire2api_attribute_twin_normal(raw);
  }

  BTwinNormal _wire2api_box_autoadd_b_twin_normal(dynamic raw) {
    return _wire2api_b_twin_normal(raw);
  }

  bool _wire2api_box_autoadd_bool(dynamic raw) {
    return raw as bool;
  }

  CTwinNormal _wire2api_box_autoadd_c_twin_normal(dynamic raw) {
    return _wire2api_c_twin_normal(raw);
  }

  CustomNestedError2TwinNormal
      _wire2api_box_autoadd_custom_nested_error_2_twin_normal(dynamic raw) {
    return _wire2api_custom_nested_error_2_twin_normal(raw);
  }

  CustomNestedErrorInnerTwinNormal
      _wire2api_box_autoadd_custom_nested_error_inner_twin_normal(dynamic raw) {
    return _wire2api_custom_nested_error_inner_twin_normal(raw);
  }

  ElementTwinNormal _wire2api_box_autoadd_element_twin_normal(dynamic raw) {
    return _wire2api_element_twin_normal(raw);
  }

  ExoticOptionalsTwinNormal _wire2api_box_autoadd_exotic_optionals_twin_normal(
      dynamic raw) {
    return _wire2api_exotic_optionals_twin_normal(raw);
  }

  double _wire2api_box_autoadd_f_64(dynamic raw) {
    return raw as double;
  }

  int _wire2api_box_autoadd_i_32(dynamic raw) {
    return raw as int;
  }

  int _wire2api_box_autoadd_i_64(dynamic raw) {
    return _wire2api_i_64(raw);
  }

  ListOfNestedRawStringMirrored
      _wire2api_box_autoadd_list_of_nested_raw_string_mirrored(dynamic raw) {
    return _wire2api_list_of_nested_raw_string_mirrored(raw);
  }

  MeasureTwinNormal _wire2api_box_autoadd_measure_twin_normal(dynamic raw) {
    return _wire2api_measure_twin_normal(raw);
  }

  NestedRawStringMirrored _wire2api_box_autoadd_nested_raw_string_mirrored(
      dynamic raw) {
    return _wire2api_nested_raw_string_mirrored(raw);
  }

  NewTypeIntTwinNormal _wire2api_box_autoadd_new_type_int_twin_normal(
      dynamic raw) {
    return _wire2api_new_type_int_twin_normal(raw);
  }

  RawStringMirrored _wire2api_box_autoadd_raw_string_mirrored(dynamic raw) {
    return _wire2api_raw_string_mirrored(raw);
  }

  WeekdaysTwinNormal _wire2api_box_autoadd_weekdays_twin_normal(dynamic raw) {
    return _wire2api_weekdays_twin_normal(raw);
  }

  DistanceTwinNormal _wire2api_box_distance_twin_normal(dynamic raw) {
    return _wire2api_distance_twin_normal(raw);
  }

  FeedIdTwinNormal _wire2api_box_feed_id_twin_normal(dynamic raw) {
    return _wire2api_feed_id_twin_normal(raw);
  }

  KitchenSinkTwinNormal _wire2api_box_kitchen_sink_twin_normal(dynamic raw) {
    return _wire2api_kitchen_sink_twin_normal(raw);
  }

  SpeedTwinNormal _wire2api_box_speed_twin_normal(dynamic raw) {
    return _wire2api_speed_twin_normal(raw);
  }

  U8Array8 _wire2api_box_u_8_array_8(dynamic raw) {
    return _wire2api_u_8_array_8(raw);
  }

  CTwinNormal _wire2api_c_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CTwinNormal(
      c: _wire2api_bool(arr[0]),
    );
  }

  ConcatenateWithTwinNormal _wire2api_concatenate_with_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ConcatenateWithTwinNormal(
      a: _wire2api_String(arr[0]),
    );
  }

  ContainsMirroredSubStructTwinNormal
      _wire2api_contains_mirrored_sub_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ContainsMirroredSubStructTwinNormal(
      test: _wire2api_raw_string_mirrored(arr[0]),
      test2: _wire2api_another_twin_normal(arr[1]),
    );
  }

  CustomEnumErrorTwinNormal _wire2api_custom_enum_error_twin_normal(
      dynamic raw) {
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

  CustomErrorTwinNormal _wire2api_custom_error_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomErrorTwinNormal_Error0(
          e: _wire2api_String(raw[1]),
          backtrace: _wire2api_String(raw[2]),
        );
      case 1:
        return CustomErrorTwinNormal_Error1(
          e: _wire2api_u_32(raw[1]),
          backtrace: _wire2api_String(raw[2]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  CustomNestedError1TwinNormal _wire2api_custom_nested_error_1_twin_normal(
      dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomNestedError1TwinNormal_CustomNested1(
          _wire2api_String(raw[1]),
        );
      case 1:
        return CustomNestedError1TwinNormal_ErrorNested(
          _wire2api_box_autoadd_custom_nested_error_2_twin_normal(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  CustomNestedError2TwinNormal _wire2api_custom_nested_error_2_twin_normal(
      dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomNestedError2TwinNormal_CustomNested2(
          _wire2api_String(raw[1]),
        );
      case 1:
        return CustomNestedError2TwinNormal_CustomNested2Number(
          _wire2api_u_32(raw[1]),
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

  CustomStructErrorAnotherTwinNormal
      _wire2api_custom_struct_error_another_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CustomStructErrorAnotherTwinNormal(
      message: _wire2api_String(arr[0]),
    );
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

  CustomStructTwinNormal _wire2api_custom_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CustomStructTwinNormal(
      message: _wire2api_String(arr[0]),
    );
  }

  DartOpaqueNestedTwinNormal _wire2api_dart_opaque_nested_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return DartOpaqueNestedTwinNormal(
      first: _wire2api_DartOpaque(arr[0]),
      second: _wire2api_DartOpaque(arr[1]),
    );
  }

  dynamic _wire2api_dartabi(dynamic raw) {
    return raw;
  }

  DistanceTwinNormal _wire2api_distance_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return DistanceTwinNormal_Unknown();
      case 1:
        return DistanceTwinNormal_Map(
          _wire2api_f_64(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  ElementTwinNormal _wire2api_element_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return ElementTwinNormal(
      tag: _wire2api_opt_String(arr[0]),
      text: _wire2api_opt_String(arr[1]),
      attributes: _wire2api_opt_list_attribute_twin_normal(arr[2]),
      children: _wire2api_opt_list_element_twin_normal(arr[3]),
    );
  }

  EmptyTwinNormal _wire2api_empty_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 0)
      throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
    return EmptyTwinNormal();
  }

  EnumDartOpaqueTwinNormal _wire2api_enum_dart_opaque_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return EnumDartOpaqueTwinNormal_Primitive(
          _wire2api_i_32(raw[1]),
        );
      case 1:
        return EnumDartOpaqueTwinNormal_Opaque(
          _wire2api_DartOpaque(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  EnumOpaqueTwinNormal _wire2api_enum_opaque_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return EnumOpaqueTwinNormal_Struct(
          _wire2api_RustOpaque_hide_data(raw[1]),
        );
      case 1:
        return EnumOpaqueTwinNormal_Primitive(
          _wire2api_RustOpaque_i_32(raw[1]),
        );
      case 2:
        return EnumOpaqueTwinNormal_TraitObj(
          _wire2api_RustOpaque_box_dynDartDebug(raw[1]),
        );
      case 3:
        return EnumOpaqueTwinNormal_Mutex(
          _wire2api_RustOpaque_MutexHideData(raw[1]),
        );
      case 4:
        return EnumOpaqueTwinNormal_RwLock(
          _wire2api_RustOpaque_RwLockHideData(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  EnumOpaqueTwinNormalArray5 _wire2api_enum_opaque_twin_normal_array_5(
      dynamic raw) {
    return EnumOpaqueTwinNormalArray5(
        (raw as List<dynamic>).map(_wire2api_enum_opaque_twin_normal).toList());
  }

  EnumSimpleTwinNormal _wire2api_enum_simple_twin_normal(dynamic raw) {
    return EnumSimpleTwinNormal.values[raw as int];
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

  EventTwinNormal _wire2api_event_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return EventTwinNormal(
      address: _wire2api_String(arr[0]),
      payload: _wire2api_String(arr[1]),
    );
  }

  ExoticOptionalsTwinNormal _wire2api_exotic_optionals_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 14)
      throw Exception('unexpected arr length: expect 14 but see ${arr.length}');
    return ExoticOptionalsTwinNormal(
      int32: _wire2api_opt_box_autoadd_i_32(arr[0]),
      int64: _wire2api_opt_box_autoadd_i_64(arr[1]),
      float64: _wire2api_opt_box_autoadd_f_64(arr[2]),
      boolean: _wire2api_opt_box_autoadd_bool(arr[3]),
      zerocopy: _wire2api_opt_ZeroCopyBuffer_list_prim_u_8(arr[4]),
      int8List: _wire2api_opt_list_prim_i_8(arr[5]),
      uint8List: _wire2api_opt_list_prim_u_8(arr[6]),
      int32List: _wire2api_opt_list_prim_i_32(arr[7]),
      float32List: _wire2api_opt_list_prim_f_32(arr[8]),
      float64List: _wire2api_opt_list_prim_f_64(arr[9]),
      attributes: _wire2api_opt_list_attribute_twin_normal(arr[10]),
      attributesNullable:
          _wire2api_list_opt_box_autoadd_attribute_twin_normal(arr[11]),
      nullableAttributes:
          _wire2api_opt_list_opt_box_autoadd_attribute_twin_normal(arr[12]),
      newtypeint: _wire2api_opt_box_autoadd_new_type_int_twin_normal(arr[13]),
    );
  }

  double _wire2api_f_32(dynamic raw) {
    return raw as double;
  }

  double _wire2api_f_64(dynamic raw) {
    return raw as double;
  }

  FeatureUuidTwinNormal _wire2api_feature_uuid_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return FeatureUuidTwinNormal(
      one: _wire2api_Uuid(arr[0]),
      many: _wire2api_Uuids(arr[1]),
    );
  }

  FeedIdTwinNormal _wire2api_feed_id_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return FeedIdTwinNormal(
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

  int _wire2api_i_64(dynamic raw) {
    return wire2apiI64OrU64(raw);
  }

  int _wire2api_i_8(dynamic raw) {
    return raw as int;
  }

  KitchenSinkTwinNormal _wire2api_kitchen_sink_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return KitchenSinkTwinNormal_Empty();
      case 1:
        return KitchenSinkTwinNormal_Primitives(
          int32: _wire2api_i_32(raw[1]),
          float64: _wire2api_f_64(raw[2]),
          boolean: _wire2api_bool(raw[3]),
        );
      case 2:
        return KitchenSinkTwinNormal_Nested(
          _wire2api_i_32(raw[1]),
          _wire2api_box_kitchen_sink_twin_normal(raw[2]),
        );
      case 3:
        return KitchenSinkTwinNormal_Optional(
          _wire2api_opt_box_autoadd_i_32(raw[1]),
          _wire2api_opt_box_autoadd_i_32(raw[2]),
        );
      case 4:
        return KitchenSinkTwinNormal_Buffer(
          _wire2api_ZeroCopyBuffer_list_prim_u_8(raw[1]),
        );
      case 5:
        return KitchenSinkTwinNormal_Enums(
          _wire2api_weekdays_twin_normal(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  List<Object> _wire2api_list_DartOpaque(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_DartOpaque).toList();
  }

  List<HideData> _wire2api_list_RustOpaque_hide_data(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_RustOpaque_hide_data).toList();
  }

  List<ApplicationEnvVar> _wire2api_list_application_env_var(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_application_env_var).toList();
  }

  List<ApplicationSettings> _wire2api_list_application_settings(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_application_settings).toList();
  }

  List<AttributeTwinNormal> _wire2api_list_attribute_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_attribute_twin_normal).toList();
  }

  List<bool> _wire2api_list_bool(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_bool).toList();
  }

  List<ElementTwinNormal> _wire2api_list_element_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_element_twin_normal).toList();
  }

  List<EnumOpaqueTwinNormal> _wire2api_list_enum_opaque_twin_normal(
      dynamic raw) {
    return (raw as List<dynamic>)
        .map(_wire2api_enum_opaque_twin_normal)
        .toList();
  }

  List<MyEnum> _wire2api_list_my_enum(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_my_enum).toList();
  }

  List<MySize> _wire2api_list_my_size(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_my_size).toList();
  }

  List<MyTreeNodeTwinNormal> _wire2api_list_my_tree_node_twin_normal(
      dynamic raw) {
    return (raw as List<dynamic>)
        .map(_wire2api_my_tree_node_twin_normal)
        .toList();
  }

  List<NestedRawStringMirrored> _wire2api_list_nested_raw_string_mirrored(
      dynamic raw) {
    return (raw as List<dynamic>)
        .map(_wire2api_nested_raw_string_mirrored)
        .toList();
  }

  ListOfNestedRawStringMirrored _wire2api_list_of_nested_raw_string_mirrored(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ListOfNestedRawStringMirrored(
      raw: _wire2api_list_nested_raw_string_mirrored(arr[0]),
    );
  }

  List<String?> _wire2api_list_opt_String(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_String);
  }

  List<AttributeTwinNormal?>
      _wire2api_list_opt_box_autoadd_attribute_twin_normal(dynamic raw) {
    return mapNonNull(
        raw as List<dynamic>, _wire2api_box_autoadd_attribute_twin_normal);
  }

  List<int?> _wire2api_list_opt_box_autoadd_i_32(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_box_autoadd_i_32);
  }

  List<WeekdaysTwinNormal?> _wire2api_list_opt_box_autoadd_weekdays_twin_normal(
      dynamic raw) {
    return mapNonNull(
        raw as List<dynamic>, _wire2api_box_autoadd_weekdays_twin_normal);
  }

  List<Int32List?> _wire2api_list_opt_list_prim_i_32(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_list_prim_i_32);
  }

  List<PointTwinNormal> _wire2api_list_point_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_point_twin_normal).toList();
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

  List<RawStringEnumMirrored> _wire2api_list_raw_string_enum_mirrored(
      dynamic raw) {
    return (raw as List<dynamic>)
        .map(_wire2api_raw_string_enum_mirrored)
        .toList();
  }

  List<RawStringMirrored> _wire2api_list_raw_string_mirrored(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_raw_string_mirrored).toList();
  }

  List<SumWithTwinNormal> _wire2api_list_sum_with_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_sum_with_twin_normal).toList();
  }

  List<TestIdTwinNormal> _wire2api_list_test_id_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_test_id_twin_normal).toList();
  }

  List<WeekdaysTwinNormal> _wire2api_list_weekdays_twin_normal(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_weekdays_twin_normal).toList();
  }

  Log2TwinNormal _wire2api_log_2_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return Log2TwinNormal(
      key: _wire2api_u_32(arr[0]),
      value: _wire2api_String(arr[1]),
    );
  }

  LogTwinNormal _wire2api_log_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return LogTwinNormal(
      key: _wire2api_u_32(arr[0]),
      value: _wire2api_u_32(arr[1]),
    );
  }

  MacroStruct _wire2api_macro_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MacroStruct(
      data: _wire2api_i_32(arr[0]),
    );
  }

  MeasureTwinNormal _wire2api_measure_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return MeasureTwinNormal_Speed(
          _wire2api_box_speed_twin_normal(raw[1]),
        );
      case 1:
        return MeasureTwinNormal_Distance(
          _wire2api_box_distance_twin_normal(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  MessageIdTwinNormal _wire2api_message_id_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MessageIdTwinNormal(
      field0: _wire2api_u_8_array_32(arr[0]),
    );
  }

  MirrorStructTwinNormal _wire2api_mirror_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return MirrorStructTwinNormal(
      a: _wire2api_application_settings(arr[0]),
      b: _wire2api_my_struct(arr[1]),
      c: _wire2api_list_my_enum(arr[2]),
      d: _wire2api_list_application_settings(arr[3]),
    );
  }

  MoreThanJustOneRawStringStructTwinNormal
      _wire2api_more_than_just_one_raw_string_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return MoreThanJustOneRawStringStructTwinNormal(
      regular: _wire2api_String(arr[0]),
      type: _wire2api_String(arr[1]),
      async: _wire2api_bool(arr[2]),
      another: _wire2api_String(arr[3]),
    );
  }

  MyEnum _wire2api_my_enum(dynamic raw) {
    return MyEnum.values[raw as int];
  }

  MyNestedStructTwinNormal _wire2api_my_nested_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return MyNestedStructTwinNormal(
      treeNode: _wire2api_my_tree_node_twin_normal(arr[0]),
      weekday: _wire2api_weekdays_twin_normal(arr[1]),
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

  MySizeFreezedTwinNormal _wire2api_my_size_freezed_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return MySizeFreezedTwinNormal(
      width: _wire2api_i_32(arr[0]),
      height: _wire2api_i_32(arr[1]),
    );
  }

  MyStreamEntryTwinNormal _wire2api_my_stream_entry_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MyStreamEntryTwinNormal(
      hello: _wire2api_String(arr[0]),
    );
  }

  MyStruct _wire2api_my_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MyStruct(
      content: _wire2api_bool(arr[0]),
    );
  }

  MyTreeNodeTwinNormal _wire2api_my_tree_node_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return MyTreeNodeTwinNormal(
      valueI32: _wire2api_i_32(arr[0]),
      valueVecU8: _wire2api_list_prim_u_8(arr[1]),
      valueBoolean: _wire2api_bool(arr[2]),
      children: _wire2api_list_my_tree_node_twin_normal(arr[3]),
    );
  }

  NestedRawStringMirrored _wire2api_nested_raw_string_mirrored(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return NestedRawStringMirrored(
      raw: _wire2api_raw_string_mirrored(arr[0]),
    );
  }

  NewSimpleStruct _wire2api_new_simple_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return NewSimpleStruct(
      field: _wire2api_i_32(arr[0]),
    );
  }

  NewTypeIntTwinNormal _wire2api_new_type_int_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return NewTypeIntTwinNormal(
      field0: _wire2api_i_64(arr[0]),
    );
  }

  Numbers _wire2api_numbers(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return Numbers(
      field0: _wire2api_list_prim_i_32(arr[0]),
    );
  }

  OldSimpleStruct _wire2api_old_simple_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return OldSimpleStruct(
      field: _wire2api_i_32(arr[0]),
    );
  }

  OpaqueNestedTwinNormal _wire2api_opaque_nested_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return OpaqueNestedTwinNormal(
      first: _wire2api_RustOpaque_hide_data(arr[0]),
      second: _wire2api_RustOpaque_hide_data(arr[1]),
    );
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  Uint8List? _wire2api_opt_ZeroCopyBuffer_list_prim_u_8(dynamic raw) {
    return raw == null ? null : _wire2api_ZeroCopyBuffer_list_prim_u_8(raw);
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

  Object? _wire2api_opt_box_autoadd_DartOpaque(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_DartOpaque(raw);
  }

  HideData? _wire2api_opt_box_autoadd_RustOpaque_hide_data(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_RustOpaque_hide_data(raw);
  }

  ApplicationEnv? _wire2api_opt_box_autoadd_application_env(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_application_env(raw);
  }

  bool? _wire2api_opt_box_autoadd_bool(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_bool(raw);
  }

  ElementTwinNormal? _wire2api_opt_box_autoadd_element_twin_normal(
      dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_element_twin_normal(raw);
  }

  ExoticOptionalsTwinNormal?
      _wire2api_opt_box_autoadd_exotic_optionals_twin_normal(dynamic raw) {
    return raw == null
        ? null
        : _wire2api_box_autoadd_exotic_optionals_twin_normal(raw);
  }

  double? _wire2api_opt_box_autoadd_f_64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_f_64(raw);
  }

  int? _wire2api_opt_box_autoadd_i_32(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i_32(raw);
  }

  int? _wire2api_opt_box_autoadd_i_64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i_64(raw);
  }

  MeasureTwinNormal? _wire2api_opt_box_autoadd_measure_twin_normal(
      dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_measure_twin_normal(raw);
  }

  NewTypeIntTwinNormal? _wire2api_opt_box_autoadd_new_type_int_twin_normal(
      dynamic raw) {
    return raw == null
        ? null
        : _wire2api_box_autoadd_new_type_int_twin_normal(raw);
  }

  WeekdaysTwinNormal? _wire2api_opt_box_autoadd_weekdays_twin_normal(
      dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_weekdays_twin_normal(raw);
  }

  List<AttributeTwinNormal>? _wire2api_opt_list_attribute_twin_normal(
      dynamic raw) {
    return raw == null ? null : _wire2api_list_attribute_twin_normal(raw);
  }

  List<ElementTwinNormal>? _wire2api_opt_list_element_twin_normal(dynamic raw) {
    return raw == null ? null : _wire2api_list_element_twin_normal(raw);
  }

  List<AttributeTwinNormal?>?
      _wire2api_opt_list_opt_box_autoadd_attribute_twin_normal(dynamic raw) {
    return raw == null
        ? null
        : _wire2api_list_opt_box_autoadd_attribute_twin_normal(raw);
  }

  Float32List? _wire2api_opt_list_prim_f_32(dynamic raw) {
    return raw == null ? null : _wire2api_list_prim_f_32(raw);
  }

  Float64List? _wire2api_opt_list_prim_f_64(dynamic raw) {
    return raw == null ? null : _wire2api_list_prim_f_64(raw);
  }

  Int32List? _wire2api_opt_list_prim_i_32(dynamic raw) {
    return raw == null ? null : _wire2api_list_prim_i_32(raw);
  }

  Int8List? _wire2api_opt_list_prim_i_8(dynamic raw) {
    return raw == null ? null : _wire2api_list_prim_i_8(raw);
  }

  Uint8List? _wire2api_opt_list_prim_u_8(dynamic raw) {
    return raw == null ? null : _wire2api_list_prim_u_8(raw);
  }

  OptVecsTwinNormal _wire2api_opt_vecs_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return OptVecsTwinNormal(
      i32: _wire2api_list_opt_box_autoadd_i_32(arr[0]),
      enums: _wire2api_list_opt_box_autoadd_weekdays_twin_normal(arr[1]),
      strings: _wire2api_list_opt_String(arr[2]),
      buffers: _wire2api_list_opt_list_prim_i_32(arr[3]),
    );
  }

  PointTwinNormal _wire2api_point_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return PointTwinNormal(
      x: _wire2api_f_32(arr[0]),
      y: _wire2api_f_32(arr[1]),
    );
  }

  PointTwinNormalArray2 _wire2api_point_twin_normal_array_2(dynamic raw) {
    return PointTwinNormalArray2(
        (raw as List<dynamic>).map(_wire2api_point_twin_normal).toList());
  }

  RawStringEnumMirrored _wire2api_raw_string_enum_mirrored(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return RawStringEnumMirrored_Raw(
          _wire2api_box_autoadd_raw_string_mirrored(raw[1]),
        );
      case 1:
        return RawStringEnumMirrored_Nested(
          _wire2api_box_autoadd_nested_raw_string_mirrored(raw[1]),
        );
      case 2:
        return RawStringEnumMirrored_ListOfNested(
          _wire2api_box_autoadd_list_of_nested_raw_string_mirrored(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  RawStringItemStructTwinNormal _wire2api_raw_string_item_struct_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return RawStringItemStructTwinNormal(
      type: _wire2api_String(arr[0]),
    );
  }

  RawStringMirrored _wire2api_raw_string_mirrored(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return RawStringMirrored(
      value: _wire2api_String(arr[0]),
    );
  }

  (ApplicationSettings, RawStringEnumMirrored)
      _wire2api_record_application_settings_raw_string_enum_mirrored(
          dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2) {
      throw Exception('Expected 2 elements, got ${arr.length}');
    }
    return (
      _wire2api_application_settings(arr[0]),
      _wire2api_raw_string_enum_mirrored(arr[1]),
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

  Sequences _wire2api_sequences(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return Sequences(
      field0: _wire2api_list_prim_i_32(arr[0]),
    );
  }

  SomeStructTwinNormal _wire2api_some_struct_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SomeStructTwinNormal(
      value: _wire2api_u_32(arr[0]),
    );
  }

  SpeedTwinNormal _wire2api_speed_twin_normal(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return SpeedTwinNormal_Unknown();
      case 1:
        return SpeedTwinNormal_GPS(
          _wire2api_f_64(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  StructWithEnumTwinNormal _wire2api_struct_with_enum_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return StructWithEnumTwinNormal(
      abc1: _wire2api_abc_twin_normal(arr[0]),
      abc2: _wire2api_abc_twin_normal(arr[1]),
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

  StructWithZeroFieldTwinNormal _wire2api_struct_with_zero_field_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 0)
      throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
    return StructWithZeroFieldTwinNormal();
  }

  SumWithTwinNormal _wire2api_sum_with_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SumWithTwinNormal(
      x: _wire2api_u_32(arr[0]),
    );
  }

  SumWithTwinNormalArray3 _wire2api_sum_with_twin_normal_array_3(dynamic raw) {
    return SumWithTwinNormalArray3(
        (raw as List<dynamic>).map(_wire2api_sum_with_twin_normal).toList());
  }

  TestChronoTwinNormal _wire2api_test_chrono_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return TestChronoTwinNormal(
      dt: _wire2api_opt_box_autoadd_Chrono_Utc(arr[0]),
      dt2: _wire2api_opt_box_autoadd_Chrono_Naive(arr[1]),
      du: _wire2api_opt_box_autoadd_Chrono_Duration(arr[2]),
    );
  }

  TestIdTwinNormal _wire2api_test_id_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return TestIdTwinNormal(
      field0: _wire2api_i_32_array_2(arr[0]),
    );
  }

  TestIdTwinNormalArray2 _wire2api_test_id_twin_normal_array_2(dynamic raw) {
    return TestIdTwinNormalArray2(
        (raw as List<dynamic>).map(_wire2api_test_id_twin_normal).toList());
  }

  TestModelTwinNormal _wire2api_test_model_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return TestModelTwinNormal(
      id: _wire2api_u_64(arr[0]),
      name: _wire2api_String(arr[1]),
      aliasEnum: _wire2api_my_enum(arr[2]),
      aliasStruct: _wire2api_my_struct(arr[3]),
    );
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

  int _wire2api_u_16(dynamic raw) {
    return raw as int;
  }

  int _wire2api_u_32(dynamic raw) {
    return raw as int;
  }

  int _wire2api_u_64(dynamic raw) {
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

  UserIdTwinNormal _wire2api_user_id_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return UserIdTwinNormal(
      value: _wire2api_u_32(arr[0]),
    );
  }

  VecOfPrimitivePackTwinNormal _wire2api_vec_of_primitive_pack_twin_normal(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 11)
      throw Exception('unexpected arr length: expect 11 but see ${arr.length}');
    return VecOfPrimitivePackTwinNormal(
      int8List: _wire2api_list_prim_i_8(arr[0]),
      uint8List: _wire2api_list_prim_u_8(arr[1]),
      int16List: _wire2api_list_prim_i_16(arr[2]),
      uint16List: _wire2api_list_prim_u_16(arr[3]),
      uint32List: _wire2api_list_prim_u_32(arr[4]),
      int32List: _wire2api_list_prim_i_32(arr[5]),
      uint64List: _wire2api_list_prim_u_64(arr[6]),
      int64List: _wire2api_list_prim_i_64(arr[7]),
      float32List: _wire2api_list_prim_f_32(arr[8]),
      float64List: _wire2api_list_prim_f_64(arr[9]),
      boolList: _wire2api_list_bool(arr[10]),
    );
  }

  WeekdaysTwinNormal _wire2api_weekdays_twin_normal(dynamic raw) {
    return WeekdaysTwinNormal.values[raw as int];
  }

  ZeroCopyVecOfPrimitivePackTwinNormal
      _wire2api_zero_copy_vec_of_primitive_pack_twin_normal(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 10)
      throw Exception('unexpected arr length: expect 10 but see ${arr.length}');
    return ZeroCopyVecOfPrimitivePackTwinNormal(
      int8List: _wire2api_ZeroCopyBuffer_list_prim_i_8(arr[0]),
      uint8List: _wire2api_ZeroCopyBuffer_list_prim_u_8(arr[1]),
      int16List: _wire2api_ZeroCopyBuffer_list_prim_i_16(arr[2]),
      uint16List: _wire2api_ZeroCopyBuffer_list_prim_u_16(arr[3]),
      uint32List: _wire2api_ZeroCopyBuffer_list_prim_u_32(arr[4]),
      int32List: _wire2api_ZeroCopyBuffer_list_prim_i_32(arr[5]),
      uint64List: _wire2api_ZeroCopyBuffer_list_prim_u_64(arr[6]),
      int64List: _wire2api_ZeroCopyBuffer_list_prim_i_64(arr[7]),
      float32List: _wire2api_ZeroCopyBuffer_list_prim_f_32(arr[8]),
      float64List: _wire2api_ZeroCopyBuffer_list_prim_f_64(arr[9]),
    );
  }
}

// Section: api2wire_funcs

int api2wire_application_mode(ApplicationMode raw) {
  return api2wire_i_32(raw.index);
}

bool api2wire_bool(bool raw) {
  return raw;
}

int api2wire_enum_simple_twin_normal(EnumSimpleTwinNormal raw) {
  return api2wire_i_32(raw.index);
}

double api2wire_f_32(double raw) {
  return raw;
}

double api2wire_f_64(double raw) {
  return raw;
}

int api2wire_i_32(int raw) {
  return raw;
}

int api2wire_i_8(int raw) {
  return raw;
}

int api2wire_my_enum(MyEnum raw) {
  return api2wire_i_32(raw.index);
}

int api2wire_u_32(int raw) {
  return raw;
}

int api2wire_u_8(int raw) {
  return raw;
}

int api2wire_usize(int raw) {
  return raw;
}

int api2wire_weekdays_twin_normal(WeekdaysTwinNormal raw) {
  return api2wire_i_32(raw.index);
}
