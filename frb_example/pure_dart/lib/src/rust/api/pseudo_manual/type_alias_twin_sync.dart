// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

BigInt handleTypeAliasIdTwinSync({required BigInt input}) =>
    RustLib.instance.api
        .crateApiPseudoManualTypeAliasTwinSyncHandleTypeAliasIdTwinSync(
            input: input);

BigInt handleTypeNestAliasIdTwinSync({required BigInt input}) =>
    RustLib.instance.api
        .crateApiPseudoManualTypeAliasTwinSyncHandleTypeNestAliasIdTwinSync(
            input: input);

TestModelTwinSync handleTypeAliasModelTwinSync({required BigInt input}) =>
    RustLib.instance.api
        .crateApiPseudoManualTypeAliasTwinSyncHandleTypeAliasModelTwinSync(
            input: input);

class TestModelTwinSync {
  final BigInt id;
  final String name;
  final MyEnum aliasEnum;
  final MyStruct aliasStruct;

  const TestModelTwinSync({
    required this.id,
    required this.name,
    required this.aliasEnum,
    required this.aliasStruct,
  });

  @override
  int get hashCode =>
      id.hashCode ^ name.hashCode ^ aliasEnum.hashCode ^ aliasStruct.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TestModelTwinSync &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          name == other.name &&
          aliasEnum == other.aliasEnum &&
          aliasStruct == other.aliasStruct;
}
