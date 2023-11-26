// ignore_for_file: unused_import, unused_element

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

  Future<void> handleCustomizedStruct({required Customized val, dynamic hint});

  Future<UserId> nextUserId({required UserId userId, dynamic hint});

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

  Future<String> asyncAcceptDartOpaque({required Object opaque, dynamic hint});

  Future<EnumDartOpaque> createEnumDartOpaque(
      {required Object opaque, dynamic hint});

  Future<DartOpaqueNested> createNestedDartOpaque(
      {required Object opaque1, required Object opaque2, dynamic hint});

  Future<void> dropStaticDartOpaque({dynamic hint});

  Future<void> getEnumDartOpaque(
      {required EnumDartOpaque opaque, dynamic hint});

  Future<void> getNestedDartOpaque(
      {required DartOpaqueNested opaque, dynamic hint});

  Future<Object> loopBack({required Object opaque, dynamic hint});

  Future<ObjectArray1> loopBackArray({required Object opaque, dynamic hint});

  Future<void> loopBackArrayGet({required ObjectArray1 opaque, dynamic hint});

  Future<Object?> loopBackOption({required Object opaque, dynamic hint});

  Future<void> loopBackOptionGet({Object? opaque, dynamic hint});

  Future<List<Object>> loopBackVec({required Object opaque, dynamic hint});

  Future<void> loopBackVecGet({required List<Object> opaque, dynamic hint});

  Future<void> panicUnwrapDartOpaque({required Object opaque, dynamic hint});

  Future<void> setStaticDartOpaque({required Object opaque, dynamic hint});

  Object returnNonDroppableDartOpaque({required Object opaque, dynamic hint});

  String syncAcceptDartOpaque({required Object opaque, dynamic hint});

  Object syncLoopback({required Object opaque, dynamic hint});

  Object? syncOptionDartOpaque({required Object opaque, dynamic hint});

  Object? syncOptionLoopback({Object? opaque, dynamic hint});

  String unwrapDartOpaque({required Object opaque, dynamic hint});

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

  Future<KitchenSink> handleEnumStruct(
      {required KitchenSink val, dynamic hint});

  Future<Weekdays?> handleReturnEnum({required String input, dynamic hint});

  Future<Measure?> multiplyByTen({required Measure measure, dynamic hint});

  Future<Uint8List> printNote({required Note note, dynamic hint});

  Future<String> eventAsString({required Event that, dynamic hint});

  Future<void> closeEventListener({dynamic hint});

  Future<void> createEvent(
      {required String address, required String payload, dynamic hint});

  Stream<Event> registerEventListener({dynamic hint});

  Future<CustomStruct> customStructNew({required String message, dynamic hint});

  Future<void> customStructNonstaticReturnCustomStructError(
      {required CustomStruct that, dynamic hint});

  Future<int> customStructNonstaticReturnCustomStructOk(
      {required CustomStruct that, dynamic hint});

  Future<void> customStructStaticReturnCustomStructError({dynamic hint});

  Future<int> customStructStaticReturnCustomStructOk({dynamic hint});

  Future<SomeStruct> someStructNew({required int value, dynamic hint});

  Future<int> someStructNonStaticReturnErrCustomError(
      {required SomeStruct that, dynamic hint});

  Future<int> someStructNonStaticReturnOkCustomError(
      {required SomeStruct that, dynamic hint});

  Future<int> someStructStaticReturnErrCustomError({dynamic hint});

  Future<int> someStructStaticReturnOkCustomError({dynamic hint});

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

  Future<void> panicWithCustomResult({dynamic hint});

  Future<void> returnCustomNestedError1({dynamic hint});

  Future<void> returnCustomNestedError1Variant1({dynamic hint});

  Future<void> returnCustomNestedError2({dynamic hint});

  Future<void> returnCustomStructError({dynamic hint});

  Future<int> returnCustomStructOk({dynamic hint});

  Future<int> returnErrCustomError({dynamic hint});

  Future<int> returnErrorVariant({required int variant, dynamic hint});

  Future<int> returnOkCustomError({dynamic hint});

  Stream<String> streamSinkThrowAnyhow({dynamic hint});

  void syncReturnCustomStructError({dynamic hint});

  Future<void> throwAnyhow({dynamic hint});

  Future<NewSimpleStruct> callNewModuleSystem({dynamic hint});

  Future<OldSimpleStruct> callOldModuleSystem({dynamic hint});

  Future<bool> useImportedEnum({required MyEnum myEnum, dynamic hint});

  Future<bool> useImportedStruct({required MyStruct myStruct, dynamic hint});

  Future<AnotherMacroStruct> anotherMacroStruct({dynamic hint});

  Future<MacroStruct> funcMacroStruct({required MacroStruct arg, dynamic hint});

  Future<String> concatenateWithConcatenate(
      {required ConcatenateWith that, required String b, dynamic hint});

  Future<String> concatenateWithConcatenateStatic(
      {required String a, required String b, dynamic hint});

  Stream<Log2> concatenateWithHandleSomeStaticStreamSink(
      {required int key, required int max, dynamic hint});

  Stream<int> concatenateWithHandleSomeStaticStreamSinkSingleArg(
      {dynamic hint});

  Stream<Log2> concatenateWithHandleSomeStreamSink(
      {required ConcatenateWith that,
      required int key,
      required int max,
      dynamic hint});

  Stream<int> concatenateWithHandleSomeStreamSinkAt1(
      {required ConcatenateWith that, dynamic hint});

  Future<ConcatenateWith> concatenateWithNew({required String a, dynamic hint});

  Future<int> sumWithSum(
      {required SumWith that, required int y, required int z, dynamic hint});

  Future<SumWithArray3> getSumArray(
      {required int a, required int b, required int c, dynamic hint});

  Future<SumWith> getSumStruct({dynamic hint});

  Stream<ApplicationSettings> appSettingsStream({dynamic hint});

  Stream<List<ApplicationSettings>> appSettingsVecStream({dynamic hint});

  Future<int?> firstNumber({required Numbers nums, dynamic hint});

  Future<int?> firstSequence({required Sequences seqs, dynamic hint});

  Future<ApplicationSettings> getAppSettings({dynamic hint});

  Future<ApplicationSettings> getFallibleAppSettings({dynamic hint});

  Future<ApplicationMessage> getMessage({dynamic hint});

  Future<bool> isAppEmbedded(
      {required ApplicationSettings appSettings, dynamic hint});

  Stream<MirrorStruct> mirrorStructStream({dynamic hint});

  Stream<(ApplicationSettings, RawStringEnumMirrored)> mirrorTupleStream(
      {dynamic hint});

  Future<Numbers> repeatNumber(
      {required int num, required int times, dynamic hint});

  Future<Sequences> repeatSequence(
      {required int seq, required int times, dynamic hint});

  Future<ContainsMirroredSubStruct> testContainsMirroredSubStruct(
      {dynamic hint});

  Future<List<RawStringMirrored>> testFallibleOfRawStringMirrored(
      {dynamic hint});

  Future<List<RawStringEnumMirrored>> testListOfNestedEnumsMirrored(
      {dynamic hint});

  Future<ListOfNestedRawStringMirrored> testListOfRawNestedStringMirrored(
      {dynamic hint});

  Future<NestedRawStringMirrored> testNestedRawStringMirrored({dynamic hint});

  Future<RawStringEnumMirrored> testRawStringEnumMirrored(
      {required bool nested, dynamic hint});

  Future<RawStringMirrored> testRawStringMirrored({dynamic hint});

  Future<BigBuffers> handleBigBuffers({dynamic hint});

  Future<MyTreeNode> handleComplexStruct({required MyTreeNode s, dynamic hint});

  Future<MyNestedStruct> handleNestedStruct(
      {required MyNestedStruct s, dynamic hint});

  Future<String> handleString({required String s, dynamic hint});

  Future<MySize> handleStruct(
      {required MySize arg, required MySize boxed, dynamic hint});

  MySizeFreezed handleStructSyncFreezed(
      {required MySizeFreezed arg, required MySizeFreezed boxed, dynamic hint});

  Future<Uint8List> handleVecU8({required Uint8List v, dynamic hint});

  Future<List<Weekdays>> listOfPrimitiveEnums(
      {required List<Weekdays> weekdays, dynamic hint});

  Future<Abc> testAbcEnum({required Abc abc, dynamic hint});

  Future<StructWithEnum> testStructWithEnum(
      {required StructWithEnum se, dynamic hint});

  Future<Empty> emptyStruct({required Empty empty, dynamic hint});

  Future<void> funcReturnUnitTwinNormal({dynamic hint});

  Future<String> funcStringTwinNormal({required String arg, dynamic hint});

  Future<List<MySize>> handleListOfStruct(
      {required List<MySize> l, dynamic hint});

  Future<List<String>> handleStringList(
      {required List<String> names, dynamic hint});

  Future<NewTypeInt> handleNewtype({required NewTypeInt arg, dynamic hint});

  Future<double> handleIncrementBoxedOptional({double? opt, dynamic hint});

  Future<String> handleOptionBoxArguments(
      {int? i8Box,
      int? u8Box,
      int? i32Box,
      int? i64Box,
      double? f64Box,
      bool? boolbox,
      ExoticOptionals? structbox,
      dynamic hint});

  Future<ExoticOptionals?> handleOptionalIncrement(
      {ExoticOptionals? opt, dynamic hint});

  Future<double?> handleOptionalReturn(
      {required double left, required double right, dynamic hint});

  Future<Element?> handleOptionalStruct({String? document, dynamic hint});

  Future<OptVecs> handleVecOfOpts({required OptVecs opt, dynamic hint});

  String? syncOption({dynamic hint});

  String? syncOptionNull({dynamic hint});

  Future<int?> primitiveOptionalTypes(
      {int? myI32, int? myI64, double? myF64, bool? myBool, dynamic hint});

  Future<VecOfPrimitivePack> handleVecOfPrimitive(
      {required int n, dynamic hint});

  Future<ZeroCopyVecOfPrimitivePack> handleZeroCopyVecOfPrimitive(
      {required int n, dynamic hint});

  ZeroCopyVecOfPrimitivePack handleZeroCopyVecOfPrimitiveSync(
      {required int n, dynamic hint});

  Future<int> primitiveTypes(
      {required int myI32,
      required int myI64,
      required double myF64,
      required bool myBool,
      dynamic hint});

  Future<int> primitiveU32({required int myU32, dynamic hint});

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

  Future<int?> exampleOptionalPrimitiveTypeI64TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeI8TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU16TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU32TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU64TwinNormal(
      {int? arg, dynamic hint});

  Future<int?> exampleOptionalPrimitiveTypeU8TwinNormal(
      {int? arg, dynamic hint});

  bool? exampleOptionalPrimitiveTypeBoolTwinSync({bool? arg, dynamic hint});

  double? exampleOptionalPrimitiveTypeF32TwinSync({double? arg, dynamic hint});

  double? exampleOptionalPrimitiveTypeF64TwinSync({double? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI16TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI32TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI64TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeI8TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU16TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU32TwinSync({int? arg, dynamic hint});

  int? exampleOptionalPrimitiveTypeU64TwinSync({int? arg, dynamic hint});

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

  Future<int> examplePrimitiveTypeI64TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeI8TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU16TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU32TwinNormal(
      {required int arg, dynamic hint});

  Future<int> examplePrimitiveTypeU64TwinNormal(
      {required int arg, dynamic hint});

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

  int examplePrimitiveTypeI64TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeI8TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeU16TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeU32TwinSync({required int arg, dynamic hint});

  int examplePrimitiveTypeU64TwinSync({required int arg, dynamic hint});

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

  Future<EnumOpaqueArray5> createArrayOpaqueEnum({dynamic hint});

  Future<OpaqueNested> createNestedOpaque({dynamic hint});

  Future<HideData> createOpaque({dynamic hint});

  Future<HideData?> createOptionOpaque({HideData? opaque, dynamic hint});

  Future<NonSendHideData> createSyncOpaque({dynamic hint});

  Future<FrbOpaqueReturn> frbGeneratorTest({dynamic hint});

  Future<HideDataArray2> opaqueArray({dynamic hint});

  Future<void> opaqueArrayRun({required HideDataArray2 data, dynamic hint});

  Future<List<HideData>> opaqueVec({dynamic hint});

  Future<void> opaqueVecRun({required List<HideData> data, dynamic hint});

  Future<String> runEnumOpaque({required EnumOpaque opaque, dynamic hint});

  Future<void> runNestedOpaque({required OpaqueNested opaque, dynamic hint});

  Future<String> runNonClone({required NonCloneData clone, dynamic hint});

  Future<String> runOpaque({required HideData opaque, dynamic hint});

  Future<String> runOpaqueWithDelay({required HideData opaque, dynamic hint});

  Future<String> unwrapRustOpaque({required HideData opaque, dynamic hint});

  FrbOpaqueSyncReturn frbSyncGeneratorTest({dynamic hint});

  NonCloneData syncCreateNonClone({dynamic hint});

  HideData syncCreateOpaque({dynamic hint});

  NonSendHideData syncCreateSyncOpaque({dynamic hint});

  HideData? syncOptionRustOpaque({dynamic hint});

  String syncRunOpaque({required NonSendHideData opaque, dynamic hint});

  Future<int> simpleAdderTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<String> funcStreamRealisticTwinNormal(
      {required String arg, dynamic hint});

  Stream<String> funcStreamReturnErrorTwinNormal({dynamic hint});

  Stream<String> funcStreamReturnPanicTwinNormal({dynamic hint});

  Stream<int> funcStreamSinkArgPositionTwinNormal(
      {required int a, required int b, dynamic hint});

  Stream<MyStreamEntry> handleStreamOfStruct({dynamic hint});

  Stream<Log> handleStreamSinkAt1(
      {required int key, required int max, dynamic hint});

  Stream<Log> handleStreamSinkAt2(
      {required int key, required int max, dynamic hint});

  Stream<Log> handleStreamSinkAt3(
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

  Future<(String, int)> testTuple({(String, int)? value, dynamic hint});

  Future<void> testTuple2({required List<(String, int)> value, dynamic hint});

  Future<int> handleTypeAliasId({required int input, dynamic hint});

  Future<TestModel> handleTypeAliasModel({required int input, dynamic hint});

  Future<int> handleTypeNestAliasId({required int input, dynamic hint});

  Future<FeatureUuid> handleNestedUuids(
      {required FeatureUuid ids, dynamic hint});

  Future<UuidValue> handleUuid({required UuidValue id, dynamic hint});

  Future<List<UuidValue>> handleUuids(
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
  Future<void> handleCustomizedStruct({required Customized val, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_customized(val);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_customized_struct(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kHandleCustomizedStructConstMeta,
      argValues: [val],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleCustomizedStructConstMeta => const TaskConstMeta(
        debugName: "handle_customized_struct",
        argNames: ["val"],
      );

  @override
  Future<UserId> nextUserId({required UserId userId, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_user_id(userId);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_next_user_id(port_, arg0),
      parseSuccessData: _wire2api_user_id,
      parseErrorData: null,
      constMeta: kNextUserIdConstMeta,
      argValues: [userId],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kNextUserIdConstMeta => const TaskConstMeta(
        debugName: "next_user_id",
        argNames: ["userId"],
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
  Future<String> asyncAcceptDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_async_accept_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kAsyncAcceptDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAsyncAcceptDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "async_accept_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<EnumDartOpaque> createEnumDartOpaque(
      {required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_enum_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_enum_dart_opaque,
      parseErrorData: null,
      constMeta: kCreateEnumDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateEnumDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_enum_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<DartOpaqueNested> createNestedDartOpaque(
      {required Object opaque1, required Object opaque2, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque1);
    var arg1 = api2wire_DartOpaque(opaque2);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_create_nested_dart_opaque(port_, arg0, arg1),
      parseSuccessData: _wire2api_dart_opaque_nested,
      parseErrorData: null,
      constMeta: kCreateNestedDartOpaqueConstMeta,
      argValues: [opaque1, opaque2],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateNestedDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_nested_dart_opaque",
        argNames: ["opaque1", "opaque2"],
      );

  @override
  Future<void> dropStaticDartOpaque({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_drop_static_dart_opaque(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kDropStaticDartOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kDropStaticDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "drop_static_dart_opaque",
        argNames: [],
      );

  @override
  Future<void> getEnumDartOpaque(
      {required EnumDartOpaque opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_dart_opaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_enum_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kGetEnumDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetEnumDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "get_enum_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<void> getNestedDartOpaque(
      {required DartOpaqueNested opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_dart_opaque_nested(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_nested_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kGetNestedDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetNestedDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "get_nested_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<Object> loopBack({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back(port_, arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackConstMeta => const TaskConstMeta(
        debugName: "loop_back",
        argNames: ["opaque"],
      );

  @override
  Future<ObjectArray1> loopBackArray({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_array(port_, arg0),
      parseSuccessData: _wire2api_DartOpaque_array_1,
      parseErrorData: null,
      constMeta: kLoopBackArrayConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackArrayConstMeta => const TaskConstMeta(
        debugName: "loop_back_array",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackArrayGet({required ObjectArray1 opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque_array_1(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_array_get(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackArrayGetConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackArrayGetConstMeta => const TaskConstMeta(
        debugName: "loop_back_array_get",
        argNames: ["opaque"],
      );

  @override
  Future<Object?> loopBackOption({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_option(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackOptionConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackOptionConstMeta => const TaskConstMeta(
        debugName: "loop_back_option",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackOptionGet({Object? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_option_get(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackOptionGetConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackOptionGetConstMeta => const TaskConstMeta(
        debugName: "loop_back_option_get",
        argNames: ["opaque"],
      );

  @override
  Future<List<Object>> loopBackVec({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_vec(port_, arg0),
      parseSuccessData: _wire2api_list_DartOpaque,
      parseErrorData: null,
      constMeta: kLoopBackVecConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackVecConstMeta => const TaskConstMeta(
        debugName: "loop_back_vec",
        argNames: ["opaque"],
      );

  @override
  Future<void> loopBackVecGet({required List<Object> opaque, dynamic hint}) {
    var arg0 = api2wire_list_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_loop_back_vec_get(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kLoopBackVecGetConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kLoopBackVecGetConstMeta => const TaskConstMeta(
        debugName: "loop_back_vec_get",
        argNames: ["opaque"],
      );

  @override
  Future<void> panicUnwrapDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_panic_unwrap_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kPanicUnwrapDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPanicUnwrapDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "panic_unwrap_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<void> setStaticDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_set_static_dart_opaque(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kSetStaticDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSetStaticDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "set_static_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Object returnNonDroppableDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_return_non_droppable_dart_opaque(arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kReturnNonDroppableDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnNonDroppableDartOpaqueConstMeta =>
      const TaskConstMeta(
        debugName: "return_non_droppable_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  String syncAcceptDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_accept_dart_opaque(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kSyncAcceptDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncAcceptDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_accept_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Object syncLoopback({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_loopback(arg0),
      parseSuccessData: _wire2api_DartOpaque,
      parseErrorData: null,
      constMeta: kSyncLoopbackConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncLoopbackConstMeta => const TaskConstMeta(
        debugName: "sync_loopback",
        argNames: ["opaque"],
      );

  @override
  Object? syncOptionDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_dart_opaque(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_option_dart_opaque",
        argNames: ["opaque"],
      );

  @override
  Object? syncOptionLoopback({Object? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_loopback(arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_DartOpaque,
      parseErrorData: null,
      constMeta: kSyncOptionLoopbackConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionLoopbackConstMeta => const TaskConstMeta(
        debugName: "sync_option_loopback",
        argNames: ["opaque"],
      );

  @override
  String unwrapDartOpaque({required Object opaque, dynamic hint}) {
    var arg0 = api2wire_DartOpaque(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_unwrap_dart_opaque(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kUnwrapDartOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUnwrapDartOpaqueConstMeta => const TaskConstMeta(
        debugName: "unwrap_dart_opaque",
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
  Future<KitchenSink> handleEnumStruct(
      {required KitchenSink val, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_kitchen_sink(val);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_enum_struct(port_, arg0),
      parseSuccessData: _wire2api_kitchen_sink,
      parseErrorData: null,
      constMeta: kHandleEnumStructConstMeta,
      argValues: [val],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleEnumStructConstMeta => const TaskConstMeta(
        debugName: "handle_enum_struct",
        argNames: ["val"],
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
  Future<String> eventAsString({required Event that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_event(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_Event_as_string(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kEventAsStringConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kEventAsStringConstMeta => const TaskConstMeta(
        debugName: "Event_as_string",
        argNames: ["that"],
      );

  @override
  Future<void> closeEventListener({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_close_event_listener(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kCloseEventListenerConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCloseEventListenerConstMeta => const TaskConstMeta(
        debugName: "close_event_listener",
        argNames: [],
      );

  @override
  Future<void> createEvent(
      {required String address, required String payload, dynamic hint}) {
    var arg0 = api2wire_String(address);
    var arg1 = api2wire_String(payload);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_event(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kCreateEventConstMeta,
      argValues: [address, payload],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateEventConstMeta => const TaskConstMeta(
        debugName: "create_event",
        argNames: ["address", "payload"],
      );

  @override
  Stream<Event> registerEventListener({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_register_event_listener(port_),
      parseSuccessData: _wire2api_event,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kRegisterEventListenerConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRegisterEventListenerConstMeta => const TaskConstMeta(
        debugName: "register_event_listener",
        argNames: [],
      );

  @override
  Future<CustomStruct> customStructNew(
      {required String message, dynamic hint}) {
    var arg0 = api2wire_String(message);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_CustomStruct_new(port_, arg0),
      parseSuccessData: _wire2api_custom_struct,
      parseErrorData: null,
      constMeta: kCustomStructNewConstMeta,
      argValues: [message],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructNewConstMeta => const TaskConstMeta(
        debugName: "CustomStruct_new",
        argNames: ["message"],
      );

  @override
  Future<void> customStructNonstaticReturnCustomStructError(
      {required CustomStruct that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire
          .wire_CustomStruct_nonstatic_return_custom_struct_error(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kCustomStructNonstaticReturnCustomStructErrorConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructNonstaticReturnCustomStructErrorConstMeta =>
      const TaskConstMeta(
        debugName: "CustomStruct_nonstatic_return_custom_struct_error",
        argNames: ["that"],
      );

  @override
  Future<int> customStructNonstaticReturnCustomStructOk(
      {required CustomStruct that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_custom_struct(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_CustomStruct_nonstatic_return_custom_struct_ok(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kCustomStructNonstaticReturnCustomStructOkConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructNonstaticReturnCustomStructOkConstMeta =>
      const TaskConstMeta(
        debugName: "CustomStruct_nonstatic_return_custom_struct_ok",
        argNames: ["that"],
      );

  @override
  Future<void> customStructStaticReturnCustomStructError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_CustomStruct_static_return_custom_struct_error(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kCustomStructStaticReturnCustomStructErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructStaticReturnCustomStructErrorConstMeta =>
      const TaskConstMeta(
        debugName: "CustomStruct_static_return_custom_struct_error",
        argNames: [],
      );

  @override
  Future<int> customStructStaticReturnCustomStructOk({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_CustomStruct_static_return_custom_struct_ok(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kCustomStructStaticReturnCustomStructOkConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCustomStructStaticReturnCustomStructOkConstMeta =>
      const TaskConstMeta(
        debugName: "CustomStruct_static_return_custom_struct_ok",
        argNames: [],
      );

  @override
  Future<SomeStruct> someStructNew({required int value, dynamic hint}) {
    var arg0 = api2wire_u_32(value);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_SomeStruct_new(port_, arg0),
      parseSuccessData: _wire2api_some_struct,
      parseErrorData: null,
      constMeta: kSomeStructNewConstMeta,
      argValues: [value],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructNewConstMeta => const TaskConstMeta(
        debugName: "SomeStruct_new",
        argNames: ["value"],
      );

  @override
  Future<int> someStructNonStaticReturnErrCustomError(
      {required SomeStruct that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_some_struct(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SomeStruct_non_static_return_err_custom_error(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kSomeStructNonStaticReturnErrCustomErrorConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructNonStaticReturnErrCustomErrorConstMeta =>
      const TaskConstMeta(
        debugName: "SomeStruct_non_static_return_err_custom_error",
        argNames: ["that"],
      );

  @override
  Future<int> someStructNonStaticReturnOkCustomError(
      {required SomeStruct that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_some_struct(that);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SomeStruct_non_static_return_ok_custom_error(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kSomeStructNonStaticReturnOkCustomErrorConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructNonStaticReturnOkCustomErrorConstMeta =>
      const TaskConstMeta(
        debugName: "SomeStruct_non_static_return_ok_custom_error",
        argNames: ["that"],
      );

  @override
  Future<int> someStructStaticReturnErrCustomError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SomeStruct_static_return_err_custom_error(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kSomeStructStaticReturnErrCustomErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructStaticReturnErrCustomErrorConstMeta =>
      const TaskConstMeta(
        debugName: "SomeStruct_static_return_err_custom_error",
        argNames: [],
      );

  @override
  Future<int> someStructStaticReturnOkCustomError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_SomeStruct_static_return_ok_custom_error(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kSomeStructStaticReturnOkCustomErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSomeStructStaticReturnOkCustomErrorConstMeta =>
      const TaskConstMeta(
        debugName: "SomeStruct_static_return_ok_custom_error",
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
  Future<void> panicWithCustomResult({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_panic_with_custom_result(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_error,
      constMeta: kPanicWithCustomResultConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPanicWithCustomResultConstMeta => const TaskConstMeta(
        debugName: "panic_with_custom_result",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError1({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_custom_nested_error_1(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_1,
      constMeta: kReturnCustomNestedError1ConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError1ConstMeta => const TaskConstMeta(
        debugName: "return_custom_nested_error_1",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError1Variant1({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_return_custom_nested_error_1_variant1(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_1,
      constMeta: kReturnCustomNestedError1Variant1ConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError1Variant1ConstMeta =>
      const TaskConstMeta(
        debugName: "return_custom_nested_error_1_variant1",
        argNames: [],
      );

  @override
  Future<void> returnCustomNestedError2({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_custom_nested_error_2(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_nested_error_2,
      constMeta: kReturnCustomNestedError2ConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomNestedError2ConstMeta => const TaskConstMeta(
        debugName: "return_custom_nested_error_2",
        argNames: [],
      );

  @override
  Future<void> returnCustomStructError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_custom_struct_error(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kReturnCustomStructErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomStructErrorConstMeta => const TaskConstMeta(
        debugName: "return_custom_struct_error",
        argNames: [],
      );

  @override
  Future<int> returnCustomStructOk({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_custom_struct_ok(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kReturnCustomStructOkConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnCustomStructOkConstMeta => const TaskConstMeta(
        debugName: "return_custom_struct_ok",
        argNames: [],
      );

  @override
  Future<int> returnErrCustomError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_err_custom_error(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kReturnErrCustomErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnErrCustomErrorConstMeta => const TaskConstMeta(
        debugName: "return_err_custom_error",
        argNames: [],
      );

  @override
  Future<int> returnErrorVariant({required int variant, dynamic hint}) {
    var arg0 = api2wire_u_32(variant);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_error_variant(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kReturnErrorVariantConstMeta,
      argValues: [variant],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnErrorVariantConstMeta => const TaskConstMeta(
        debugName: "return_error_variant",
        argNames: ["variant"],
      );

  @override
  Future<int> returnOkCustomError({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_return_ok_custom_error(port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: _wire2api_custom_error,
      constMeta: kReturnOkCustomErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kReturnOkCustomErrorConstMeta => const TaskConstMeta(
        debugName: "return_ok_custom_error",
        argNames: [],
      );

  @override
  Stream<String> streamSinkThrowAnyhow({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_stream_sink_throw_anyhow(port_),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kStreamSinkThrowAnyhowConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kStreamSinkThrowAnyhowConstMeta => const TaskConstMeta(
        debugName: "stream_sink_throw_anyhow",
        argNames: [],
      );

  @override
  void syncReturnCustomStructError({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_return_custom_struct_error(),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_custom_struct_error,
      constMeta: kSyncReturnCustomStructErrorConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncReturnCustomStructErrorConstMeta =>
      const TaskConstMeta(
        debugName: "sync_return_custom_struct_error",
        argNames: [],
      );

  @override
  Future<void> throwAnyhow({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_throw_anyhow(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kThrowAnyhowConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kThrowAnyhowConstMeta => const TaskConstMeta(
        debugName: "throw_anyhow",
        argNames: [],
      );

  @override
  Future<NewSimpleStruct> callNewModuleSystem({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_call_new_module_system(port_),
      parseSuccessData: _wire2api_new_simple_struct,
      parseErrorData: null,
      constMeta: kCallNewModuleSystemConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCallNewModuleSystemConstMeta => const TaskConstMeta(
        debugName: "call_new_module_system",
        argNames: [],
      );

  @override
  Future<OldSimpleStruct> callOldModuleSystem({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_call_old_module_system(port_),
      parseSuccessData: _wire2api_old_simple_struct,
      parseErrorData: null,
      constMeta: kCallOldModuleSystemConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCallOldModuleSystemConstMeta => const TaskConstMeta(
        debugName: "call_old_module_system",
        argNames: [],
      );

  @override
  Future<bool> useImportedEnum({required MyEnum myEnum, dynamic hint}) {
    var arg0 = api2wire_my_enum(myEnum);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_imported_enum(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kUseImportedEnumConstMeta,
      argValues: [myEnum],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseImportedEnumConstMeta => const TaskConstMeta(
        debugName: "use_imported_enum",
        argNames: ["myEnum"],
      );

  @override
  Future<bool> useImportedStruct({required MyStruct myStruct, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_struct(myStruct);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_use_imported_struct(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kUseImportedStructConstMeta,
      argValues: [myStruct],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUseImportedStructConstMeta => const TaskConstMeta(
        debugName: "use_imported_struct",
        argNames: ["myStruct"],
      );

  @override
  Future<AnotherMacroStruct> anotherMacroStruct({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_another_macro_struct(port_),
      parseSuccessData: _wire2api_another_macro_struct,
      parseErrorData: null,
      constMeta: kAnotherMacroStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAnotherMacroStructConstMeta => const TaskConstMeta(
        debugName: "another_macro_struct",
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
  Future<String> concatenateWithConcatenate(
      {required ConcatenateWith that, required String b, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with(that);
    var arg1 = api2wire_String(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWith_concatenate(port_, arg0, arg1),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kConcatenateWithConcatenateConstMeta,
      argValues: [that, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithConcatenateConstMeta => const TaskConstMeta(
        debugName: "ConcatenateWith_concatenate",
        argNames: ["that", "b"],
      );

  @override
  Future<String> concatenateWithConcatenateStatic(
      {required String a, required String b, dynamic hint}) {
    var arg0 = api2wire_String(a);
    var arg1 = api2wire_String(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWith_concatenate_static(port_, arg0, arg1),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kConcatenateWithConcatenateStaticConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithConcatenateStaticConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWith_concatenate_static",
        argNames: ["a", "b"],
      );

  @override
  Stream<Log2> concatenateWithHandleSomeStaticStreamSink(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWith_handle_some_static_stream_sink(
              port_, arg0, arg1),
      parseSuccessData: _wire2api_log_2,
      parseErrorData: null,
      constMeta: kConcatenateWithHandleSomeStaticStreamSinkConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithHandleSomeStaticStreamSinkConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWith_handle_some_static_stream_sink",
        argNames: ["key", "max"],
      );

  @override
  Stream<int> concatenateWithHandleSomeStaticStreamSinkSingleArg(
      {dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
              port_),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kConcatenateWithHandleSomeStaticStreamSinkSingleArgConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta
      get kConcatenateWithHandleSomeStaticStreamSinkSingleArgConstMeta =>
          const TaskConstMeta(
            debugName:
                "ConcatenateWith_handle_some_static_stream_sink_single_arg",
            argNames: [],
          );

  @override
  Stream<Log2> concatenateWithHandleSomeStreamSink(
      {required ConcatenateWith that,
      required int key,
      required int max,
      dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with(that);
    var arg1 = api2wire_u_32(key);
    var arg2 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_ConcatenateWith_handle_some_stream_sink(
          port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_log_2,
      parseErrorData: null,
      constMeta: kConcatenateWithHandleSomeStreamSinkConstMeta,
      argValues: [that, key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithHandleSomeStreamSinkConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWith_handle_some_stream_sink",
        argNames: ["that", "key", "max"],
      );

  @override
  Stream<int> concatenateWithHandleSomeStreamSinkAt1(
      {required ConcatenateWith that, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_concatenate_with(that);
    return handler.executeStream(StreamTask(
      callFfi: (port_) =>
          wire.wire_ConcatenateWith_handle_some_stream_sink_at_1(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kConcatenateWithHandleSomeStreamSinkAt1ConstMeta,
      argValues: [that],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithHandleSomeStreamSinkAt1ConstMeta =>
      const TaskConstMeta(
        debugName: "ConcatenateWith_handle_some_stream_sink_at_1",
        argNames: ["that"],
      );

  @override
  Future<ConcatenateWith> concatenateWithNew(
      {required String a, dynamic hint}) {
    var arg0 = api2wire_String(a);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_ConcatenateWith_new(port_, arg0),
      parseSuccessData: _wire2api_concatenate_with,
      parseErrorData: null,
      constMeta: kConcatenateWithNewConstMeta,
      argValues: [a],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kConcatenateWithNewConstMeta => const TaskConstMeta(
        debugName: "ConcatenateWith_new",
        argNames: ["a"],
      );

  @override
  Future<int> sumWithSum(
      {required SumWith that, required int y, required int z, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_sum_with(that);
    var arg1 = api2wire_u_32(y);
    var arg2 = api2wire_u_32(z);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_SumWith_sum(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kSumWithSumConstMeta,
      argValues: [that, y, z],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSumWithSumConstMeta => const TaskConstMeta(
        debugName: "SumWith_sum",
        argNames: ["that", "y", "z"],
      );

  @override
  Future<SumWithArray3> getSumArray(
      {required int a, required int b, required int c, dynamic hint}) {
    var arg0 = api2wire_u_32(a);
    var arg1 = api2wire_u_32(b);
    var arg2 = api2wire_u_32(c);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_sum_array(port_, arg0, arg1, arg2),
      parseSuccessData: _wire2api_sum_with_array_3,
      parseErrorData: null,
      constMeta: kGetSumArrayConstMeta,
      argValues: [a, b, c],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetSumArrayConstMeta => const TaskConstMeta(
        debugName: "get_sum_array",
        argNames: ["a", "b", "c"],
      );

  @override
  Future<SumWith> getSumStruct({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_sum_struct(port_),
      parseSuccessData: _wire2api_sum_with,
      parseErrorData: null,
      constMeta: kGetSumStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetSumStructConstMeta => const TaskConstMeta(
        debugName: "get_sum_struct",
        argNames: [],
      );

  @override
  Stream<ApplicationSettings> appSettingsStream({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_app_settings_stream(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: null,
      constMeta: kAppSettingsStreamConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAppSettingsStreamConstMeta => const TaskConstMeta(
        debugName: "app_settings_stream",
        argNames: [],
      );

  @override
  Stream<List<ApplicationSettings>> appSettingsVecStream({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_app_settings_vec_stream(port_),
      parseSuccessData: _wire2api_list_application_settings,
      parseErrorData: null,
      constMeta: kAppSettingsVecStreamConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAppSettingsVecStreamConstMeta => const TaskConstMeta(
        debugName: "app_settings_vec_stream",
        argNames: [],
      );

  @override
  Future<int?> firstNumber({required Numbers nums, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_numbers(nums);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_first_number(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kFirstNumberConstMeta,
      argValues: [nums],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFirstNumberConstMeta => const TaskConstMeta(
        debugName: "first_number",
        argNames: ["nums"],
      );

  @override
  Future<int?> firstSequence({required Sequences seqs, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_sequences(seqs);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_first_sequence(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kFirstSequenceConstMeta,
      argValues: [seqs],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFirstSequenceConstMeta => const TaskConstMeta(
        debugName: "first_sequence",
        argNames: ["seqs"],
      );

  @override
  Future<ApplicationSettings> getAppSettings({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_app_settings(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: null,
      constMeta: kGetAppSettingsConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetAppSettingsConstMeta => const TaskConstMeta(
        debugName: "get_app_settings",
        argNames: [],
      );

  @override
  Future<ApplicationSettings> getFallibleAppSettings({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_fallible_app_settings(port_),
      parseSuccessData: _wire2api_application_settings,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kGetFallibleAppSettingsConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetFallibleAppSettingsConstMeta => const TaskConstMeta(
        debugName: "get_fallible_app_settings",
        argNames: [],
      );

  @override
  Future<ApplicationMessage> getMessage({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_get_message(port_),
      parseSuccessData: _wire2api_application_message,
      parseErrorData: null,
      constMeta: kGetMessageConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kGetMessageConstMeta => const TaskConstMeta(
        debugName: "get_message",
        argNames: [],
      );

  @override
  Future<bool> isAppEmbedded(
      {required ApplicationSettings appSettings, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_application_settings(appSettings);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_is_app_embedded(port_, arg0),
      parseSuccessData: _wire2api_bool,
      parseErrorData: null,
      constMeta: kIsAppEmbeddedConstMeta,
      argValues: [appSettings],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kIsAppEmbeddedConstMeta => const TaskConstMeta(
        debugName: "is_app_embedded",
        argNames: ["appSettings"],
      );

  @override
  Stream<MirrorStruct> mirrorStructStream({dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_mirror_struct_stream(port_),
      parseSuccessData: _wire2api_mirror_struct,
      parseErrorData: null,
      constMeta: kMirrorStructStreamConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMirrorStructStreamConstMeta => const TaskConstMeta(
        debugName: "mirror_struct_stream",
        argNames: [],
      );

  @override
  Stream<(ApplicationSettings, RawStringEnumMirrored)> mirrorTupleStream(
      {dynamic hint}) {
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_mirror_tuple_stream(port_),
      parseSuccessData:
          _wire2api_record_application_settings_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kMirrorTupleStreamConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMirrorTupleStreamConstMeta => const TaskConstMeta(
        debugName: "mirror_tuple_stream",
        argNames: [],
      );

  @override
  Future<Numbers> repeatNumber(
      {required int num, required int times, dynamic hint}) {
    var arg0 = api2wire_i_32(num);
    var arg1 = api2wire_usize(times);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_repeat_number(port_, arg0, arg1),
      parseSuccessData: _wire2api_numbers,
      parseErrorData: null,
      constMeta: kRepeatNumberConstMeta,
      argValues: [num, times],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRepeatNumberConstMeta => const TaskConstMeta(
        debugName: "repeat_number",
        argNames: ["num", "times"],
      );

  @override
  Future<Sequences> repeatSequence(
      {required int seq, required int times, dynamic hint}) {
    var arg0 = api2wire_i_32(seq);
    var arg1 = api2wire_usize(times);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_repeat_sequence(port_, arg0, arg1),
      parseSuccessData: _wire2api_sequences,
      parseErrorData: null,
      constMeta: kRepeatSequenceConstMeta,
      argValues: [seq, times],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRepeatSequenceConstMeta => const TaskConstMeta(
        debugName: "repeat_sequence",
        argNames: ["seq", "times"],
      );

  @override
  Future<ContainsMirroredSubStruct> testContainsMirroredSubStruct(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_contains_mirrored_sub_struct(port_),
      parseSuccessData: _wire2api_contains_mirrored_sub_struct,
      parseErrorData: null,
      constMeta: kTestContainsMirroredSubStructConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestContainsMirroredSubStructConstMeta =>
      const TaskConstMeta(
        debugName: "test_contains_mirrored_sub_struct",
        argNames: [],
      );

  @override
  Future<List<RawStringMirrored>> testFallibleOfRawStringMirrored(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_fallible_of_raw_string_mirrored(port_),
      parseSuccessData: _wire2api_list_raw_string_mirrored,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kTestFallibleOfRawStringMirroredConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestFallibleOfRawStringMirroredConstMeta =>
      const TaskConstMeta(
        debugName: "test_fallible_of_raw_string_mirrored",
        argNames: [],
      );

  @override
  Future<List<RawStringEnumMirrored>> testListOfNestedEnumsMirrored(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_list_of_nested_enums_mirrored(port_),
      parseSuccessData: _wire2api_list_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kTestListOfNestedEnumsMirroredConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestListOfNestedEnumsMirroredConstMeta =>
      const TaskConstMeta(
        debugName: "test_list_of_nested_enums_mirrored",
        argNames: [],
      );

  @override
  Future<ListOfNestedRawStringMirrored> testListOfRawNestedStringMirrored(
      {dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_test_list_of_raw_nested_string_mirrored(port_),
      parseSuccessData: _wire2api_list_of_nested_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestListOfRawNestedStringMirroredConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestListOfRawNestedStringMirroredConstMeta =>
      const TaskConstMeta(
        debugName: "test_list_of_raw_nested_string_mirrored",
        argNames: [],
      );

  @override
  Future<NestedRawStringMirrored> testNestedRawStringMirrored({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_nested_raw_string_mirrored(port_),
      parseSuccessData: _wire2api_nested_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestNestedRawStringMirroredConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestNestedRawStringMirroredConstMeta =>
      const TaskConstMeta(
        debugName: "test_nested_raw_string_mirrored",
        argNames: [],
      );

  @override
  Future<RawStringEnumMirrored> testRawStringEnumMirrored(
      {required bool nested, dynamic hint}) {
    var arg0 = api2wire_bool(nested);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_raw_string_enum_mirrored(port_, arg0),
      parseSuccessData: _wire2api_raw_string_enum_mirrored,
      parseErrorData: null,
      constMeta: kTestRawStringEnumMirroredConstMeta,
      argValues: [nested],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringEnumMirroredConstMeta => const TaskConstMeta(
        debugName: "test_raw_string_enum_mirrored",
        argNames: ["nested"],
      );

  @override
  Future<RawStringMirrored> testRawStringMirrored({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_test_raw_string_mirrored(port_),
      parseSuccessData: _wire2api_raw_string_mirrored,
      parseErrorData: null,
      constMeta: kTestRawStringMirroredConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kTestRawStringMirroredConstMeta => const TaskConstMeta(
        debugName: "test_raw_string_mirrored",
        argNames: [],
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
  Future<String> handleString({required String s, dynamic hint}) {
    var arg0 = api2wire_String(s);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_string(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kHandleStringConstMeta,
      argValues: [s],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStringConstMeta => const TaskConstMeta(
        debugName: "handle_string",
        argNames: ["s"],
      );

  @override
  Future<MySize> handleStruct(
      {required MySize arg, required MySize boxed, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_size(arg);
    var arg1 = api2wire_box_my_size(boxed);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_struct(port_, arg0, arg1),
      parseSuccessData: _wire2api_my_size,
      parseErrorData: null,
      constMeta: kHandleStructConstMeta,
      argValues: [arg, boxed],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStructConstMeta => const TaskConstMeta(
        debugName: "handle_struct",
        argNames: ["arg", "boxed"],
      );

  @override
  MySizeFreezed handleStructSyncFreezed(
      {required MySizeFreezed arg,
      required MySizeFreezed boxed,
      dynamic hint}) {
    var arg0 = api2wire_box_autoadd_my_size_freezed(arg);
    var arg1 = api2wire_box_my_size_freezed(boxed);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_handle_struct_sync_freezed(arg0, arg1),
      parseSuccessData: _wire2api_my_size_freezed,
      parseErrorData: null,
      constMeta: kHandleStructSyncFreezedConstMeta,
      argValues: [arg, boxed],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStructSyncFreezedConstMeta => const TaskConstMeta(
        debugName: "handle_struct_sync_freezed",
        argNames: ["arg", "boxed"],
      );

  @override
  Future<Uint8List> handleVecU8({required Uint8List v, dynamic hint}) {
    var arg0 = api2wire_list_prim_u_8(v);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_vec_u8(port_, arg0),
      parseSuccessData: _wire2api_list_prim_u_8,
      parseErrorData: null,
      constMeta: kHandleVecU8ConstMeta,
      argValues: [v],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecU8ConstMeta => const TaskConstMeta(
        debugName: "handle_vec_u8",
        argNames: ["v"],
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
  Future<Empty> emptyStruct({required Empty empty, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_empty(empty);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_empty_struct(port_, arg0),
      parseSuccessData: _wire2api_empty,
      parseErrorData: null,
      constMeta: kEmptyStructConstMeta,
      argValues: [empty],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kEmptyStructConstMeta => const TaskConstMeta(
        debugName: "empty_struct",
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
  Future<double> handleIncrementBoxedOptional({double? opt, dynamic hint}) {
    var arg0 = api2wire_opt_box_f_64(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_increment_boxed_optional(port_, arg0),
      parseSuccessData: _wire2api_f_64,
      parseErrorData: null,
      constMeta: kHandleIncrementBoxedOptionalConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleIncrementBoxedOptionalConstMeta =>
      const TaskConstMeta(
        debugName: "handle_increment_boxed_optional",
        argNames: ["opt"],
      );

  @override
  Future<String> handleOptionBoxArguments(
      {int? i8Box,
      int? u8Box,
      int? i32Box,
      int? i64Box,
      double? f64Box,
      bool? boolbox,
      ExoticOptionals? structbox,
      dynamic hint}) {
    var arg0 = api2wire_opt_box_i_8(i8Box);
    var arg1 = api2wire_opt_box_u_8(u8Box);
    var arg2 = api2wire_opt_box_i_32(i32Box);
    var arg3 = api2wire_opt_box_i_64(i64Box);
    var arg4 = api2wire_opt_box_f_64(f64Box);
    var arg5 = api2wire_opt_box_bool(boolbox);
    var arg6 = api2wire_opt_box_exotic_optionals(structbox);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_option_box_arguments(
          port_, arg0, arg1, arg2, arg3, arg4, arg5, arg6),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kHandleOptionBoxArgumentsConstMeta,
      argValues: [i8Box, u8Box, i32Box, i64Box, f64Box, boolbox, structbox],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionBoxArgumentsConstMeta => const TaskConstMeta(
        debugName: "handle_option_box_arguments",
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
  Future<ExoticOptionals?> handleOptionalIncrement(
      {ExoticOptionals? opt, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_exotic_optionals(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_optional_increment(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_exotic_optionals,
      parseErrorData: null,
      constMeta: kHandleOptionalIncrementConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalIncrementConstMeta => const TaskConstMeta(
        debugName: "handle_optional_increment",
        argNames: ["opt"],
      );

  @override
  Future<double?> handleOptionalReturn(
      {required double left, required double right, dynamic hint}) {
    var arg0 = api2wire_f_64(left);
    var arg1 = api2wire_f_64(right);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_optional_return(port_, arg0, arg1),
      parseSuccessData: _wire2api_opt_box_autoadd_f_64,
      parseErrorData: null,
      constMeta: kHandleOptionalReturnConstMeta,
      argValues: [left, right],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalReturnConstMeta => const TaskConstMeta(
        debugName: "handle_optional_return",
        argNames: ["left", "right"],
      );

  @override
  Future<Element?> handleOptionalStruct({String? document, dynamic hint}) {
    var arg0 = api2wire_opt_String(document);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_optional_struct(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_element,
      parseErrorData: null,
      constMeta: kHandleOptionalStructConstMeta,
      argValues: [document],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleOptionalStructConstMeta => const TaskConstMeta(
        debugName: "handle_optional_struct",
        argNames: ["document"],
      );

  @override
  Future<OptVecs> handleVecOfOpts({required OptVecs opt, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_opt_vecs(opt);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_vec_of_opts(port_, arg0),
      parseSuccessData: _wire2api_opt_vecs,
      parseErrorData: null,
      constMeta: kHandleVecOfOptsConstMeta,
      argValues: [opt],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecOfOptsConstMeta => const TaskConstMeta(
        debugName: "handle_vec_of_opts",
        argNames: ["opt"],
      );

  @override
  String? syncOption({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option(),
      parseSuccessData: _wire2api_opt_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionConstMeta => const TaskConstMeta(
        debugName: "sync_option",
        argNames: [],
      );

  @override
  String? syncOptionNull({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_null(),
      parseSuccessData: _wire2api_opt_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionNullConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionNullConstMeta => const TaskConstMeta(
        debugName: "sync_option_null",
        argNames: [],
      );

  @override
  Future<int?> primitiveOptionalTypes(
      {int? myI32, int? myI64, double? myF64, bool? myBool, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_i_32(myI32);
    var arg1 = api2wire_opt_box_autoadd_i_64(myI64);
    var arg2 = api2wire_opt_box_autoadd_f_64(myF64);
    var arg3 = api2wire_opt_box_autoadd_bool(myBool);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_primitive_optional_types(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_opt_box_autoadd_i_32,
      parseErrorData: null,
      constMeta: kPrimitiveOptionalTypesConstMeta,
      argValues: [myI32, myI64, myF64, myBool],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveOptionalTypesConstMeta => const TaskConstMeta(
        debugName: "primitive_optional_types",
        argNames: ["myI32", "myI64", "myF64", "myBool"],
      );

  @override
  Future<VecOfPrimitivePack> handleVecOfPrimitive(
      {required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_vec_of_primitive(port_, arg0),
      parseSuccessData: _wire2api_vec_of_primitive_pack,
      parseErrorData: null,
      constMeta: kHandleVecOfPrimitiveConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleVecOfPrimitiveConstMeta => const TaskConstMeta(
        debugName: "handle_vec_of_primitive",
        argNames: ["n"],
      );

  @override
  Future<ZeroCopyVecOfPrimitivePack> handleZeroCopyVecOfPrimitive(
      {required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) =>
          wire.wire_handle_zero_copy_vec_of_primitive(port_, arg0),
      parseSuccessData: _wire2api_zero_copy_vec_of_primitive_pack,
      parseErrorData: null,
      constMeta: kHandleZeroCopyVecOfPrimitiveConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleZeroCopyVecOfPrimitiveConstMeta =>
      const TaskConstMeta(
        debugName: "handle_zero_copy_vec_of_primitive",
        argNames: ["n"],
      );

  @override
  ZeroCopyVecOfPrimitivePack handleZeroCopyVecOfPrimitiveSync(
      {required int n, dynamic hint}) {
    var arg0 = api2wire_i_32(n);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_handle_zero_copy_vec_of_primitive_sync(arg0),
      parseSuccessData: _wire2api_zero_copy_vec_of_primitive_pack,
      parseErrorData: null,
      constMeta: kHandleZeroCopyVecOfPrimitiveSyncConstMeta,
      argValues: [n],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleZeroCopyVecOfPrimitiveSyncConstMeta =>
      const TaskConstMeta(
        debugName: "handle_zero_copy_vec_of_primitive_sync",
        argNames: ["n"],
      );

  @override
  Future<int> primitiveTypes(
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
          wire.wire_primitive_types(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kPrimitiveTypesConstMeta,
      argValues: [myI32, myI64, myF64, myBool],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveTypesConstMeta => const TaskConstMeta(
        debugName: "primitive_types",
        argNames: ["myI32", "myI64", "myF64", "myBool"],
      );

  @override
  Future<int> primitiveU32({required int myU32, dynamic hint}) {
    var arg0 = api2wire_u_32(myU32);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_primitive_u32(port_, arg0),
      parseSuccessData: _wire2api_u_32,
      parseErrorData: null,
      constMeta: kPrimitiveU32ConstMeta,
      argValues: [myU32],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kPrimitiveU32ConstMeta => const TaskConstMeta(
        debugName: "primitive_u32",
        argNames: ["myU32"],
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
  Future<int?> exampleOptionalPrimitiveTypeI64TwinNormal(
      {int? arg, dynamic hint}) {
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
  Future<int?> exampleOptionalPrimitiveTypeU64TwinNormal(
      {int? arg, dynamic hint}) {
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
  int? exampleOptionalPrimitiveTypeI64TwinSync({int? arg, dynamic hint}) {
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
  int? exampleOptionalPrimitiveTypeU64TwinSync({int? arg, dynamic hint}) {
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
  Future<int> examplePrimitiveTypeI64TwinNormal(
      {required int arg, dynamic hint}) {
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
  Future<int> examplePrimitiveTypeU64TwinNormal(
      {required int arg, dynamic hint}) {
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
  int examplePrimitiveTypeI64TwinSync({required int arg, dynamic hint}) {
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
  int examplePrimitiveTypeU64TwinSync({required int arg, dynamic hint}) {
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
  Future<EnumOpaqueArray5> createArrayOpaqueEnum({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_array_opaque_enum(port_),
      parseSuccessData: _wire2api_enum_opaque_array_5,
      parseErrorData: null,
      constMeta: kCreateArrayOpaqueEnumConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateArrayOpaqueEnumConstMeta => const TaskConstMeta(
        debugName: "create_array_opaque_enum",
        argNames: [],
      );

  @override
  Future<OpaqueNested> createNestedOpaque({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_nested_opaque(port_),
      parseSuccessData: _wire2api_opaque_nested,
      parseErrorData: null,
      constMeta: kCreateNestedOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateNestedOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_nested_opaque",
        argNames: [],
      );

  @override
  Future<HideData> createOpaque({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_opaque(port_),
      parseSuccessData: _wire2api_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kCreateOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_opaque",
        argNames: [],
      );

  @override
  Future<HideData?> createOptionOpaque({HideData? opaque, dynamic hint}) {
    var arg0 = api2wire_opt_box_autoadd_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_option_opaque(port_, arg0),
      parseSuccessData: _wire2api_opt_box_autoadd_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kCreateOptionOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateOptionOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_option_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<NonSendHideData> createSyncOpaque({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_create_sync_opaque(port_),
      parseSuccessData: _wire2api_RustOpaque_non_send_hide_data,
      parseErrorData: null,
      constMeta: kCreateSyncOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kCreateSyncOpaqueConstMeta => const TaskConstMeta(
        debugName: "create_sync_opaque",
        argNames: [],
      );

  @override
  Future<FrbOpaqueReturn> frbGeneratorTest({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_frb_generator_test(port_),
      parseSuccessData: _wire2api_RustOpaque_frb_opaque_return,
      parseErrorData: null,
      constMeta: kFrbGeneratorTestConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFrbGeneratorTestConstMeta => const TaskConstMeta(
        debugName: "frb_generator_test",
        argNames: [],
      );

  @override
  Future<HideDataArray2> opaqueArray({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_array(port_),
      parseSuccessData: _wire2api_RustOpaque_hide_data_array_2,
      parseErrorData: null,
      constMeta: kOpaqueArrayConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueArrayConstMeta => const TaskConstMeta(
        debugName: "opaque_array",
        argNames: [],
      );

  @override
  Future<void> opaqueArrayRun({required HideDataArray2 data, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data_array_2(data);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_array_run(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kOpaqueArrayRunConstMeta,
      argValues: [data],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueArrayRunConstMeta => const TaskConstMeta(
        debugName: "opaque_array_run",
        argNames: ["data"],
      );

  @override
  Future<List<HideData>> opaqueVec({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_vec(port_),
      parseSuccessData: _wire2api_list_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kOpaqueVecConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueVecConstMeta => const TaskConstMeta(
        debugName: "opaque_vec",
        argNames: [],
      );

  @override
  Future<void> opaqueVecRun({required List<HideData> data, dynamic hint}) {
    var arg0 = api2wire_list_RustOpaque_hide_data(data);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_opaque_vec_run(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kOpaqueVecRunConstMeta,
      argValues: [data],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kOpaqueVecRunConstMeta => const TaskConstMeta(
        debugName: "opaque_vec_run",
        argNames: ["data"],
      );

  @override
  Future<String> runEnumOpaque({required EnumOpaque opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_enum_opaque(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_enum_opaque(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunEnumOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunEnumOpaqueConstMeta => const TaskConstMeta(
        debugName: "run_enum_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<void> runNestedOpaque({required OpaqueNested opaque, dynamic hint}) {
    var arg0 = api2wire_box_autoadd_opaque_nested(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_nested_opaque(port_, arg0),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kRunNestedOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunNestedOpaqueConstMeta => const TaskConstMeta(
        debugName: "run_nested_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<String> runNonClone({required NonCloneData clone, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_non_clone_data(clone);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_non_clone(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunNonCloneConstMeta,
      argValues: [clone],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunNonCloneConstMeta => const TaskConstMeta(
        debugName: "run_non_clone",
        argNames: ["clone"],
      );

  @override
  Future<String> runOpaque({required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_opaque(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunOpaqueConstMeta => const TaskConstMeta(
        debugName: "run_opaque",
        argNames: ["opaque"],
      );

  @override
  Future<String> runOpaqueWithDelay({required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_run_opaque_with_delay(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kRunOpaqueWithDelayConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kRunOpaqueWithDelayConstMeta => const TaskConstMeta(
        debugName: "run_opaque_with_delay",
        argNames: ["opaque"],
      );

  @override
  Future<String> unwrapRustOpaque({required HideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_hide_data(opaque);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_unwrap_rust_opaque(port_, arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kUnwrapRustOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kUnwrapRustOpaqueConstMeta => const TaskConstMeta(
        debugName: "unwrap_rust_opaque",
        argNames: ["opaque"],
      );

  @override
  FrbOpaqueSyncReturn frbSyncGeneratorTest({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_frb_sync_generator_test(),
      parseSuccessData: _wire2api_RustOpaque_frb_opaque_sync_return,
      parseErrorData: null,
      constMeta: kFrbSyncGeneratorTestConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kFrbSyncGeneratorTestConstMeta => const TaskConstMeta(
        debugName: "frb_sync_generator_test",
        argNames: [],
      );

  @override
  NonCloneData syncCreateNonClone({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_non_clone(),
      parseSuccessData: _wire2api_RustOpaque_non_clone_data,
      parseErrorData: null,
      constMeta: kSyncCreateNonCloneConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateNonCloneConstMeta => const TaskConstMeta(
        debugName: "sync_create_non_clone",
        argNames: [],
      );

  @override
  HideData syncCreateOpaque({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_opaque(),
      parseSuccessData: _wire2api_RustOpaque_hide_data,
      parseErrorData: null,
      constMeta: kSyncCreateOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_create_opaque",
        argNames: [],
      );

  @override
  NonSendHideData syncCreateSyncOpaque({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_create_sync_opaque(),
      parseSuccessData: _wire2api_RustOpaque_non_send_hide_data,
      parseErrorData: null,
      constMeta: kSyncCreateSyncOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncCreateSyncOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_create_sync_opaque",
        argNames: [],
      );

  @override
  HideData? syncOptionRustOpaque({dynamic hint}) {
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_option_rust_opaque(),
      parseSuccessData: _wire2api_opt_box_autoadd_RustOpaque_hide_data,
      parseErrorData: _wire2api_AnyhowException,
      constMeta: kSyncOptionRustOpaqueConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncOptionRustOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_option_rust_opaque",
        argNames: [],
      );

  @override
  String syncRunOpaque({required NonSendHideData opaque, dynamic hint}) {
    var arg0 = api2wire_RustOpaque_non_send_hide_data(opaque);
    return handler.executeSync(SyncTask(
      callFfi: () => wire.wire_sync_run_opaque(arg0),
      parseSuccessData: _wire2api_String,
      parseErrorData: null,
      constMeta: kSyncRunOpaqueConstMeta,
      argValues: [opaque],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSyncRunOpaqueConstMeta => const TaskConstMeta(
        debugName: "sync_run_opaque",
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
  Stream<Log> handleStreamSinkAt1(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_handle_stream_sink_at_1(port_, arg0, arg1),
      parseSuccessData: _wire2api_log,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt1ConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt1ConstMeta => const TaskConstMeta(
        debugName: "handle_stream_sink_at_1",
        argNames: ["key", "max"],
      );

  @override
  Stream<Log> handleStreamSinkAt2(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_handle_stream_sink_at_2(port_, arg0, arg1),
      parseSuccessData: _wire2api_log,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt2ConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt2ConstMeta => const TaskConstMeta(
        debugName: "handle_stream_sink_at_2",
        argNames: ["key", "max"],
      );

  @override
  Stream<Log> handleStreamSinkAt3(
      {required int key, required int max, dynamic hint}) {
    var arg0 = api2wire_u_32(key);
    var arg1 = api2wire_u_32(max);
    return handler.executeStream(StreamTask(
      callFfi: (port_) => wire.wire_handle_stream_sink_at_3(port_, arg0, arg1),
      parseSuccessData: _wire2api_log,
      parseErrorData: null,
      constMeta: kHandleStreamSinkAt3ConstMeta,
      argValues: [key, max],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleStreamSinkAt3ConstMeta => const TaskConstMeta(
        debugName: "handle_stream_sink_at_3",
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
  Future<int> handleTypeAliasId({required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_type_alias_id(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kHandleTypeAliasIdConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeAliasIdConstMeta => const TaskConstMeta(
        debugName: "handle_type_alias_id",
        argNames: ["input"],
      );

  @override
  Future<TestModel> handleTypeAliasModel({required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_type_alias_model(port_, arg0),
      parseSuccessData: _wire2api_test_model,
      parseErrorData: null,
      constMeta: kHandleTypeAliasModelConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeAliasModelConstMeta => const TaskConstMeta(
        debugName: "handle_type_alias_model",
        argNames: ["input"],
      );

  @override
  Future<int> handleTypeNestAliasId({required int input, dynamic hint}) {
    var arg0 = api2wire_u_64(input);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_handle_type_nest_alias_id(port_, arg0),
      parseSuccessData: _wire2api_u_64,
      parseErrorData: null,
      constMeta: kHandleTypeNestAliasIdConstMeta,
      argValues: [input],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kHandleTypeNestAliasIdConstMeta => const TaskConstMeta(
        debugName: "handle_type_nest_alias_id",
        argNames: ["input"],
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

  Another _wire2api_another(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return Another(
      a: _wire2api_String(arr[0]),
    );
  }

  AnotherMacroStruct _wire2api_another_macro_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return AnotherMacroStruct(
      data: _wire2api_i_32(arr[0]),
      nonFinalData: _wire2api_i_32(arr[1]),
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

  Attribute _wire2api_attribute(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return Attribute(
      key: _wire2api_String(arr[0]),
      value: _wire2api_String(arr[1]),
    );
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

  A _wire2api_box_autoadd_a(dynamic raw) {
    return _wire2api_a(raw);
  }

  ApplicationEnv _wire2api_box_autoadd_application_env(dynamic raw) {
    return _wire2api_application_env(raw);
  }

  Attribute _wire2api_box_autoadd_attribute(dynamic raw) {
    return _wire2api_attribute(raw);
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

  CustomNestedError2 _wire2api_box_autoadd_custom_nested_error_2(dynamic raw) {
    return _wire2api_custom_nested_error_2(raw);
  }

  CustomNestedErrorInnerTwinNormal
      _wire2api_box_autoadd_custom_nested_error_inner_twin_normal(dynamic raw) {
    return _wire2api_custom_nested_error_inner_twin_normal(raw);
  }

  CustomNestedErrorInnerTwinSync
      _wire2api_box_autoadd_custom_nested_error_inner_twin_sync(dynamic raw) {
    return _wire2api_custom_nested_error_inner_twin_sync(raw);
  }

  Element _wire2api_box_autoadd_element(dynamic raw) {
    return _wire2api_element(raw);
  }

  ExoticOptionals _wire2api_box_autoadd_exotic_optionals(dynamic raw) {
    return _wire2api_exotic_optionals(raw);
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

  int _wire2api_box_autoadd_i_64(dynamic raw) {
    return _wire2api_i_64(raw);
  }

  int _wire2api_box_autoadd_i_8(dynamic raw) {
    return raw as int;
  }

  ListOfNestedRawStringMirrored
      _wire2api_box_autoadd_list_of_nested_raw_string_mirrored(dynamic raw) {
    return _wire2api_list_of_nested_raw_string_mirrored(raw);
  }

  Measure _wire2api_box_autoadd_measure(dynamic raw) {
    return _wire2api_measure(raw);
  }

  NestedRawStringMirrored _wire2api_box_autoadd_nested_raw_string_mirrored(
      dynamic raw) {
    return _wire2api_nested_raw_string_mirrored(raw);
  }

  NewTypeInt _wire2api_box_autoadd_new_type_int(dynamic raw) {
    return _wire2api_new_type_int(raw);
  }

  RawStringMirrored _wire2api_box_autoadd_raw_string_mirrored(dynamic raw) {
    return _wire2api_raw_string_mirrored(raw);
  }

  int _wire2api_box_autoadd_u_16(dynamic raw) {
    return raw as int;
  }

  int _wire2api_box_autoadd_u_32(dynamic raw) {
    return raw as int;
  }

  int _wire2api_box_autoadd_u_64(dynamic raw) {
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

  KitchenSink _wire2api_box_kitchen_sink(dynamic raw) {
    return _wire2api_kitchen_sink(raw);
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

  ConcatenateWith _wire2api_concatenate_with(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ConcatenateWith(
      a: _wire2api_String(arr[0]),
    );
  }

  ContainsMirroredSubStruct _wire2api_contains_mirrored_sub_struct(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return ContainsMirroredSubStruct(
      test: _wire2api_raw_string_mirrored(arr[0]),
      test2: _wire2api_another(arr[1]),
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

  CustomError _wire2api_custom_error(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomError_Error0(
          e: _wire2api_String(raw[1]),
          backtrace: _wire2api_String(raw[2]),
        );
      case 1:
        return CustomError_Error1(
          e: _wire2api_u_32(raw[1]),
          backtrace: _wire2api_String(raw[2]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  CustomNestedError1 _wire2api_custom_nested_error_1(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomNestedError1_CustomNested1(
          _wire2api_String(raw[1]),
        );
      case 1:
        return CustomNestedError1_ErrorNested(
          _wire2api_box_autoadd_custom_nested_error_2(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  CustomNestedError2 _wire2api_custom_nested_error_2(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return CustomNestedError2_CustomNested2(
          _wire2api_String(raw[1]),
        );
      case 1:
        return CustomNestedError2_CustomNested2Number(
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

  CustomStruct _wire2api_custom_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CustomStruct(
      message: _wire2api_String(arr[0]),
    );
  }

  CustomStructError _wire2api_custom_struct_error(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CustomStructError(
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

  CustomStructErrorTwinSync _wire2api_custom_struct_error_twin_sync(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return CustomStructErrorTwinSync(
      a: _wire2api_String(arr[0]),
    );
  }

  DartOpaqueNested _wire2api_dart_opaque_nested(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return DartOpaqueNested(
      first: _wire2api_DartOpaque(arr[0]),
      second: _wire2api_DartOpaque(arr[1]),
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

  Element _wire2api_element(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return Element(
      tag: _wire2api_opt_String(arr[0]),
      text: _wire2api_opt_String(arr[1]),
      attributes: _wire2api_opt_list_attribute(arr[2]),
      children: _wire2api_opt_list_element(arr[3]),
    );
  }

  Empty _wire2api_empty(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 0)
      throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
    return Empty();
  }

  EnumDartOpaque _wire2api_enum_dart_opaque(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return EnumDartOpaque_Primitive(
          _wire2api_i_32(raw[1]),
        );
      case 1:
        return EnumDartOpaque_Opaque(
          _wire2api_DartOpaque(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  EnumOpaque _wire2api_enum_opaque(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return EnumOpaque_Struct(
          _wire2api_RustOpaque_hide_data(raw[1]),
        );
      case 1:
        return EnumOpaque_Primitive(
          _wire2api_RustOpaque_i_32(raw[1]),
        );
      case 2:
        return EnumOpaque_TraitObj(
          _wire2api_RustOpaque_box_dynDartDebug(raw[1]),
        );
      case 3:
        return EnumOpaque_Mutex(
          _wire2api_RustOpaque_MutexHideData(raw[1]),
        );
      case 4:
        return EnumOpaque_RwLock(
          _wire2api_RustOpaque_RwLockHideData(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  EnumOpaqueArray5 _wire2api_enum_opaque_array_5(dynamic raw) {
    return EnumOpaqueArray5(
        (raw as List<dynamic>).map(_wire2api_enum_opaque).toList());
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

  Event _wire2api_event(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return Event(
      address: _wire2api_String(arr[0]),
      payload: _wire2api_String(arr[1]),
    );
  }

  ExoticOptionals _wire2api_exotic_optionals(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 14)
      throw Exception('unexpected arr length: expect 14 but see ${arr.length}');
    return ExoticOptionals(
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
      attributes: _wire2api_opt_list_attribute(arr[10]),
      attributesNullable: _wire2api_list_opt_box_autoadd_attribute(arr[11]),
      nullableAttributes: _wire2api_opt_list_opt_box_autoadd_attribute(arr[12]),
      newtypeint: _wire2api_opt_box_autoadd_new_type_int(arr[13]),
    );
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

  int _wire2api_i_64(dynamic raw) {
    return wire2apiI64OrU64(raw);
  }

  int _wire2api_i_8(dynamic raw) {
    return raw as int;
  }

  KitchenSink _wire2api_kitchen_sink(dynamic raw) {
    switch (raw[0]) {
      case 0:
        return KitchenSink_Empty();
      case 1:
        return KitchenSink_Primitives(
          int32: _wire2api_i_32(raw[1]),
          float64: _wire2api_f_64(raw[2]),
          boolean: _wire2api_bool(raw[3]),
        );
      case 2:
        return KitchenSink_Nested(
          _wire2api_i_32(raw[1]),
          _wire2api_box_kitchen_sink(raw[2]),
        );
      case 3:
        return KitchenSink_Optional(
          _wire2api_opt_box_autoadd_i_32(raw[1]),
          _wire2api_opt_box_autoadd_i_32(raw[2]),
        );
      case 4:
        return KitchenSink_Buffer(
          _wire2api_ZeroCopyBuffer_list_prim_u_8(raw[1]),
        );
      case 5:
        return KitchenSink_Enums(
          _wire2api_weekdays(raw[1]),
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

  List<Attribute> _wire2api_list_attribute(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_attribute).toList();
  }

  List<bool> _wire2api_list_bool(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_bool).toList();
  }

  List<Element> _wire2api_list_element(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_element).toList();
  }

  List<EnumOpaque> _wire2api_list_enum_opaque(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_enum_opaque).toList();
  }

  List<MyEnum> _wire2api_list_my_enum(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_my_enum).toList();
  }

  List<MySize> _wire2api_list_my_size(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_my_size).toList();
  }

  List<MyTreeNode> _wire2api_list_my_tree_node(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_my_tree_node).toList();
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

  List<Attribute?> _wire2api_list_opt_box_autoadd_attribute(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_box_autoadd_attribute);
  }

  List<int?> _wire2api_list_opt_box_autoadd_i_32(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_box_autoadd_i_32);
  }

  List<Weekdays?> _wire2api_list_opt_box_autoadd_weekdays(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_box_autoadd_weekdays);
  }

  List<Int32List?> _wire2api_list_opt_list_prim_i_32(dynamic raw) {
    return mapNonNull(raw as List<dynamic>, _wire2api_list_prim_i_32);
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

  List<RawStringEnumMirrored> _wire2api_list_raw_string_enum_mirrored(
      dynamic raw) {
    return (raw as List<dynamic>)
        .map(_wire2api_raw_string_enum_mirrored)
        .toList();
  }

  List<RawStringMirrored> _wire2api_list_raw_string_mirrored(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_raw_string_mirrored).toList();
  }

  List<SumWith> _wire2api_list_sum_with(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_sum_with).toList();
  }

  List<TestId> _wire2api_list_test_id(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_test_id).toList();
  }

  List<Weekdays> _wire2api_list_weekdays(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_weekdays).toList();
  }

  Log _wire2api_log(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return Log(
      key: _wire2api_u_32(arr[0]),
      value: _wire2api_u_32(arr[1]),
    );
  }

  Log2 _wire2api_log_2(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return Log2(
      key: _wire2api_u_32(arr[0]),
      value: _wire2api_String(arr[1]),
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

  MirrorStruct _wire2api_mirror_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return MirrorStruct(
      a: _wire2api_application_settings(arr[0]),
      b: _wire2api_my_struct(arr[1]),
      c: _wire2api_list_my_enum(arr[2]),
      d: _wire2api_list_application_settings(arr[3]),
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

  MyEnum _wire2api_my_enum(dynamic raw) {
    return MyEnum.values[raw as int];
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

  MySizeFreezed _wire2api_my_size_freezed(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return MySizeFreezed(
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

  MyStruct _wire2api_my_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return MyStruct(
      content: _wire2api_bool(arr[0]),
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

  NewTypeInt _wire2api_new_type_int(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return NewTypeInt(
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

  OpaqueNested _wire2api_opaque_nested(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 2)
      throw Exception('unexpected arr length: expect 2 but see ${arr.length}');
    return OpaqueNested(
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

  Element? _wire2api_opt_box_autoadd_element(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_element(raw);
  }

  ExoticOptionals? _wire2api_opt_box_autoadd_exotic_optionals(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_exotic_optionals(raw);
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

  int? _wire2api_opt_box_autoadd_i_64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i_64(raw);
  }

  int? _wire2api_opt_box_autoadd_i_8(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i_8(raw);
  }

  Measure? _wire2api_opt_box_autoadd_measure(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_measure(raw);
  }

  NewTypeInt? _wire2api_opt_box_autoadd_new_type_int(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_new_type_int(raw);
  }

  int? _wire2api_opt_box_autoadd_u_16(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_u_16(raw);
  }

  int? _wire2api_opt_box_autoadd_u_32(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_u_32(raw);
  }

  int? _wire2api_opt_box_autoadd_u_64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_u_64(raw);
  }

  int? _wire2api_opt_box_autoadd_u_8(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_u_8(raw);
  }

  Weekdays? _wire2api_opt_box_autoadd_weekdays(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_weekdays(raw);
  }

  List<Attribute>? _wire2api_opt_list_attribute(dynamic raw) {
    return raw == null ? null : _wire2api_list_attribute(raw);
  }

  List<Element>? _wire2api_opt_list_element(dynamic raw) {
    return raw == null ? null : _wire2api_list_element(raw);
  }

  List<Attribute?>? _wire2api_opt_list_opt_box_autoadd_attribute(dynamic raw) {
    return raw == null ? null : _wire2api_list_opt_box_autoadd_attribute(raw);
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

  OptVecs _wire2api_opt_vecs(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return OptVecs(
      i32: _wire2api_list_opt_box_autoadd_i_32(arr[0]),
      enums: _wire2api_list_opt_box_autoadd_weekdays(arr[1]),
      strings: _wire2api_list_opt_String(arr[2]),
      buffers: _wire2api_list_opt_list_prim_i_32(arr[3]),
    );
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

  RawStringItemStruct _wire2api_raw_string_item_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return RawStringItemStruct(
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

  SomeStruct _wire2api_some_struct(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SomeStruct(
      value: _wire2api_u_32(arr[0]),
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

  SumWith _wire2api_sum_with(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return SumWith(
      x: _wire2api_u_32(arr[0]),
    );
  }

  SumWithArray3 _wire2api_sum_with_array_3(dynamic raw) {
    return SumWithArray3(
        (raw as List<dynamic>).map(_wire2api_sum_with).toList());
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

  TestModel _wire2api_test_model(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return TestModel(
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

  TupleStructWithOneFieldTwinSync
      _wire2api_tuple_struct_with_one_field_twin_sync(dynamic raw) {
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

  TupleStructWithTwoFieldTwinSync
      _wire2api_tuple_struct_with_two_field_twin_sync(dynamic raw) {
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

  UserId _wire2api_user_id(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return UserId(
      value: _wire2api_u_32(arr[0]),
    );
  }

  VecOfPrimitivePack _wire2api_vec_of_primitive_pack(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 11)
      throw Exception('unexpected arr length: expect 11 but see ${arr.length}');
    return VecOfPrimitivePack(
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

  Weekdays _wire2api_weekdays(dynamic raw) {
    return Weekdays.values[raw as int];
  }

  ZeroCopyVecOfPrimitivePack _wire2api_zero_copy_vec_of_primitive_pack(
      dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 10)
      throw Exception('unexpected arr length: expect 10 but see ${arr.length}');
    return ZeroCopyVecOfPrimitivePack(
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

int api2wire_my_enum(MyEnum raw) {
  return api2wire_i_32(raw.index);
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

int api2wire_usize(int raw) {
  return raw;
}

int api2wire_weekdays(Weekdays raw) {
  return api2wire_i_32(raw.index);
}
