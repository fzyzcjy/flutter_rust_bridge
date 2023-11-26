// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../auxiliary/sample_types.dart';
import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> handleTypeAliasId({required int input, dynamic hint}) =>
    RustLib.instance.api.handleTypeAliasId(input: input, hint: hint);

Future<int> handleTypeNestAliasId({required int input, dynamic hint}) =>
    RustLib.instance.api.handleTypeNestAliasId(input: input, hint: hint);

Future<TestModel> handleTypeAliasModel({required int input, dynamic hint}) =>
    RustLib.instance.api.handleTypeAliasModel(input: input, hint: hint);

class TestModel {
  final int id;
  final String name;
  final MyEnum aliasEnum;
  final MyStruct aliasStruct;

  const TestModel({
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
      other is TestModel &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          name == other.name &&
          aliasEnum == other.aliasEnum &&
          aliasStruct == other.aliasStruct;
}
