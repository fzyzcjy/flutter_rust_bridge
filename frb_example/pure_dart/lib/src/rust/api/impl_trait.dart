// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class StructWithDefaultValue {
  final int val;

  const StructWithDefaultValue({
    required this.val,
  });

  static Future<StructWithDefaultValue> traitFun({dynamic hint}) =>
      RustLib.instance.api.structWithDefaultValueTraitFun(hint: hint);

  @override
  int get hashCode => val.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithDefaultValue &&
          runtimeType == other.runtimeType &&
          val == other.val;
}