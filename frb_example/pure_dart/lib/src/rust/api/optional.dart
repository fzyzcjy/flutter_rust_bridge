// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'misc_example.dart';
import 'newtype_pattern.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<double?> handleOptionalReturn(
        {required double left, required double right, dynamic hint}) =>
    RustLib.instance.api
        .handleOptionalReturn(left: left, right: right, hint: hint);

Future<Element?> handleOptionalStruct({String? document, dynamic hint}) =>
    RustLib.instance.api.handleOptionalStruct(document: document, hint: hint);

Future<ExoticOptionals?> handleOptionalIncrement(
        {ExoticOptionals? opt, dynamic hint}) =>
    RustLib.instance.api.handleOptionalIncrement(opt: opt, hint: hint);

Future<double> handleIncrementBoxedOptional({double? opt, dynamic hint}) =>
    RustLib.instance.api.handleIncrementBoxedOptional(opt: opt, hint: hint);

Future<OptVecs> handleVecOfOpts({required OptVecs opt, dynamic hint}) =>
    RustLib.instance.api.handleVecOfOpts(opt: opt, hint: hint);

Future<String> handleOptionBoxArguments(
        {int? i8Box,
        int? u8Box,
        int? i32Box,
        BigInt? i64Box,
        double? f64Box,
        bool? boolbox,
        ExoticOptionals? structbox,
        dynamic hint}) =>
    RustLib.instance.api.handleOptionBoxArguments(
        i8Box: i8Box,
        u8Box: u8Box,
        i32Box: i32Box,
        i64Box: i64Box,
        f64Box: f64Box,
        boolbox: boolbox,
        structbox: structbox,
        hint: hint);

String? syncOption({dynamic hint}) =>
    RustLib.instance.api.syncOption(hint: hint);

String? syncOptionNull({dynamic hint}) =>
    RustLib.instance.api.syncOptionNull(hint: hint);

class Attribute {
  final String key;
  final String value;

  const Attribute({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Attribute &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class Element {
  final String? tag;
  final String? text;
  final List<Attribute>? attributes;
  final List<Element>? children;

  const Element({
    this.tag,
    this.text,
    this.attributes,
    this.children,
  });

  @override
  int get hashCode =>
      tag.hashCode ^ text.hashCode ^ attributes.hashCode ^ children.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Element &&
          runtimeType == other.runtimeType &&
          tag == other.tag &&
          text == other.text &&
          attributes == other.attributes &&
          children == other.children;
}

class ExoticOptionals {
  final int? int32;
  final BigInt? int64;
  final double? float64;
  final bool? boolean;
  final Uint8List? zerocopy;
  final Int8List? int8List;
  final Uint8List? uint8List;
  final Int32List? int32List;
  final Float32List? float32List;
  final Float64List? float64List;
  final List<Attribute>? attributes;
  final List<Attribute?> attributesNullable;
  final List<Attribute?>? nullableAttributes;
  final NewTypeInt? newtypeint;

  const ExoticOptionals({
    this.int32,
    this.int64,
    this.float64,
    this.boolean,
    this.zerocopy,
    this.int8List,
    this.uint8List,
    this.int32List,
    this.float32List,
    this.float64List,
    this.attributes,
    required this.attributesNullable,
    this.nullableAttributes,
    this.newtypeint,
  });

  @override
  int get hashCode =>
      int32.hashCode ^
      int64.hashCode ^
      float64.hashCode ^
      boolean.hashCode ^
      zerocopy.hashCode ^
      int8List.hashCode ^
      uint8List.hashCode ^
      int32List.hashCode ^
      float32List.hashCode ^
      float64List.hashCode ^
      attributes.hashCode ^
      attributesNullable.hashCode ^
      nullableAttributes.hashCode ^
      newtypeint.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ExoticOptionals &&
          runtimeType == other.runtimeType &&
          int32 == other.int32 &&
          int64 == other.int64 &&
          float64 == other.float64 &&
          boolean == other.boolean &&
          zerocopy == other.zerocopy &&
          int8List == other.int8List &&
          uint8List == other.uint8List &&
          int32List == other.int32List &&
          float32List == other.float32List &&
          float64List == other.float64List &&
          attributes == other.attributes &&
          attributesNullable == other.attributesNullable &&
          nullableAttributes == other.nullableAttributes &&
          newtypeint == other.newtypeint;
}

class OptVecs {
  final List<int?> i32;
  final List<Weekdays?> enums;
  final List<String?> strings;
  final List<Int32List?> buffers;

  const OptVecs({
    required this.i32,
    required this.enums,
    required this.strings,
    required this.buffers,
  });

  @override
  int get hashCode =>
      i32.hashCode ^ enums.hashCode ^ strings.hashCode ^ buffers.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OptVecs &&
          runtimeType == other.runtimeType &&
          i32 == other.i32 &&
          enums == other.enums &&
          strings == other.strings &&
          buffers == other.buffers;
}
