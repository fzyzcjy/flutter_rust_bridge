// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../auxiliary/sample_types.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'mirror.freezed.dart';

Future<ApplicationSettings> getAppSettingsTwinNormal({dynamic hint}) =>
    RustLib.instance.api.getAppSettingsTwinNormal(hint: hint);

Future<ApplicationSettings> getFallibleAppSettingsTwinNormal({dynamic hint}) =>
    RustLib.instance.api.getFallibleAppSettingsTwinNormal(hint: hint);

Future<bool> isAppEmbeddedTwinNormal(
        {required ApplicationSettings appSettings, dynamic hint}) =>
    RustLib.instance.api
        .isAppEmbeddedTwinNormal(appSettings: appSettings, hint: hint);

Stream<ApplicationSettings> appSettingsStreamTwinNormal({dynamic hint}) =>
    RustLib.instance.api.appSettingsStreamTwinNormal(hint: hint);

Stream<List<ApplicationSettings>> appSettingsVecStreamTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.appSettingsVecStreamTwinNormal(hint: hint);

Stream<MirrorStructTwinNormal> mirrorStructStreamTwinNormal({dynamic hint}) =>
    RustLib.instance.api.mirrorStructStreamTwinNormal(hint: hint);

Stream<(ApplicationSettings, RawStringEnumMirrored)>
    mirrorTupleStreamTwinNormal({dynamic hint}) =>
        RustLib.instance.api.mirrorTupleStreamTwinNormal(hint: hint);

Future<ApplicationMessage> getMessageTwinNormal({dynamic hint}) =>
    RustLib.instance.api.getMessageTwinNormal(hint: hint);

Future<Numbers> repeatNumberTwinNormal(
        {required int num, required int times, dynamic hint}) =>
    RustLib.instance.api
        .repeatNumberTwinNormal(num: num, times: times, hint: hint);

Future<Sequences> repeatSequenceTwinNormal(
        {required int seq, required int times, dynamic hint}) =>
    RustLib.instance.api
        .repeatSequenceTwinNormal(seq: seq, times: times, hint: hint);

Future<int?> firstNumberTwinNormal({required Numbers nums, dynamic hint}) =>
    RustLib.instance.api.firstNumberTwinNormal(nums: nums, hint: hint);

Future<int?> firstSequenceTwinNormal({required Sequences seqs, dynamic hint}) =>
    RustLib.instance.api.firstSequenceTwinNormal(seqs: seqs, hint: hint);

Future<RawStringMirrored> testRawStringMirroredTwinNormal({dynamic hint}) =>
    RustLib.instance.api.testRawStringMirroredTwinNormal(hint: hint);

Future<NestedRawStringMirrored> testNestedRawStringMirroredTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.testNestedRawStringMirroredTwinNormal(hint: hint);

Future<RawStringEnumMirrored> testRawStringEnumMirroredTwinNormal(
        {required bool nested, dynamic hint}) =>
    RustLib.instance.api
        .testRawStringEnumMirroredTwinNormal(nested: nested, hint: hint);

Future<ListOfNestedRawStringMirrored>
    testListOfRawNestedStringMirroredTwinNormal({dynamic hint}) =>
        RustLib.instance.api
            .testListOfRawNestedStringMirroredTwinNormal(hint: hint);

Future<List<RawStringMirrored>> testFallibleOfRawStringMirroredTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.testFallibleOfRawStringMirroredTwinNormal(hint: hint);

Future<List<RawStringEnumMirrored>> testListOfNestedEnumsMirroredTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.testListOfNestedEnumsMirroredTwinNormal(hint: hint);

Future<ContainsMirroredSubStructTwinNormal>
    testContainsMirroredSubStructTwinNormal({dynamic hint}) =>
        RustLib.instance.api
            .testContainsMirroredSubStructTwinNormal(hint: hint);

class AnotherTwinNormal {
  final String a;

  const AnotherTwinNormal({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AnotherTwinNormal &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class ApplicationEnv {
  final List<ApplicationEnvVar> vars;

  const ApplicationEnv({
    required this.vars,
  });

  @override
  int get hashCode => vars.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApplicationEnv &&
          runtimeType == other.runtimeType &&
          vars == other.vars;
}

class ApplicationEnvVar {
  final String field0;
  final bool field1;

  const ApplicationEnvVar({
    required this.field0,
    required this.field1,
  });

  @override
  int get hashCode => field0.hashCode ^ field1.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApplicationEnvVar &&
          runtimeType == other.runtimeType &&
          field0 == other.field0 &&
          field1 == other.field1;
}

@freezed
sealed class ApplicationMessage with _$ApplicationMessage {
  const factory ApplicationMessage.displayMessage(
    String field0,
  ) = ApplicationMessage_DisplayMessage;
  const factory ApplicationMessage.renderPixel({
    required int x,
    required int y,
  }) = ApplicationMessage_RenderPixel;
  const factory ApplicationMessage.exit() = ApplicationMessage_Exit;
}

enum ApplicationMode {
  standalone,
  embedded,
}

class ApplicationSettings {
  final String name;
  final String version;
  final ApplicationMode mode;
  final ApplicationEnv env;
  final ApplicationEnv? envOptional;

  const ApplicationSettings({
    required this.name,
    required this.version,
    required this.mode,
    required this.env,
    this.envOptional,
  });

  @override
  int get hashCode =>
      name.hashCode ^
      version.hashCode ^
      mode.hashCode ^
      env.hashCode ^
      envOptional.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApplicationSettings &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          version == other.version &&
          mode == other.mode &&
          env == other.env &&
          envOptional == other.envOptional;
}

class ContainsMirroredSubStructTwinNormal {
  final RawStringMirrored test;
  final AnotherTwinNormal test2;

  const ContainsMirroredSubStructTwinNormal({
    required this.test,
    required this.test2,
  });

  @override
  int get hashCode => test.hashCode ^ test2.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ContainsMirroredSubStructTwinNormal &&
          runtimeType == other.runtimeType &&
          test == other.test &&
          test2 == other.test2;
}

class ListOfNestedRawStringMirrored {
  final List<NestedRawStringMirrored> raw;

  const ListOfNestedRawStringMirrored({
    required this.raw,
  });

  @override
  int get hashCode => raw.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ListOfNestedRawStringMirrored &&
          runtimeType == other.runtimeType &&
          raw == other.raw;
}

class MirrorStructTwinNormal {
  final ApplicationSettings a;
  final MyStruct b;
  final List<MyEnum> c;
  final List<ApplicationSettings> d;

  const MirrorStructTwinNormal({
    required this.a,
    required this.b,
    required this.c,
    required this.d,
  });

  @override
  int get hashCode => a.hashCode ^ b.hashCode ^ c.hashCode ^ d.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MirrorStructTwinNormal &&
          runtimeType == other.runtimeType &&
          a == other.a &&
          b == other.b &&
          c == other.c &&
          d == other.d;
}

class NestedRawStringMirrored {
  final RawStringMirrored raw;

  const NestedRawStringMirrored({
    required this.raw,
  });

  @override
  int get hashCode => raw.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NestedRawStringMirrored &&
          runtimeType == other.runtimeType &&
          raw == other.raw;
}

class Numbers {
  final Int32List field0;

  const Numbers({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Numbers &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

@freezed
sealed class RawStringEnumMirrored with _$RawStringEnumMirrored {
  const factory RawStringEnumMirrored.raw(
    RawStringMirrored field0,
  ) = RawStringEnumMirrored_Raw;
  const factory RawStringEnumMirrored.nested(
    NestedRawStringMirrored field0,
  ) = RawStringEnumMirrored_Nested;
  const factory RawStringEnumMirrored.listOfNested(
    ListOfNestedRawStringMirrored field0,
  ) = RawStringEnumMirrored_ListOfNested;
}

class RawStringMirrored {
  final String value;

  const RawStringMirrored({
    required this.value,
  });

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RawStringMirrored &&
          runtimeType == other.runtimeType &&
          value == other.value;
}

class Sequences {
  final Int32List field0;

  const Sequences({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Sequences &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}
