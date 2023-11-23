// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<RawStringItemStruct> testRawStringItemStruct({dynamic hint}) =>
    RustLib.instance.api.testRawStringItemStruct(hint: hint);

Future<MoreThanJustOneRawStringStruct> testMoreThanJustOneRawStringStruct(
        {dynamic hint}) =>
    RustLib.instance.api.testMoreThanJustOneRawStringStruct(hint: hint);

class MoreThanJustOneRawStringStruct {
  final String regular;
  final String type;
  final bool async;
  final String another;

  const MoreThanJustOneRawStringStruct({
    required this.regular,
    required this.type,
    required this.async,
    required this.another,
  });

  @override
  int get hashCode =>
      regular.hashCode ^ type.hashCode ^ async.hashCode ^ another.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MoreThanJustOneRawStringStruct &&
          runtimeType == other.runtimeType &&
          regular == other.regular &&
          type == other.type &&
          async == other.async &&
          another == other.another;
}

class RawStringItemStruct {
  final String type;

  const RawStringItemStruct({
    required this.type,
  });

  @override
  int get hashCode => type.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RawStringItemStruct &&
          runtimeType == other.runtimeType &&
          type == other.type;
}
