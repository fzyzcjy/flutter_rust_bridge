// ignore_for_file: invalid_use_of_internal_member, unused_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../frb_generated.dart';

Future<UuidValue> handleUuid({required UuidValue id, dynamic hint}) =>
    RustLib.instance.api.handleUuid(id: id, hint: hint);

Future<List<UuidValue>> handleUuids({required List<UuidValue> ids, dynamic hint}) =>
    RustLib.instance.api.handleUuids(ids: ids, hint: hint);

Future<FeatureUuid> handleNestedUuids({required FeatureUuid ids, dynamic hint}) =>
    RustLib.instance.api.handleNestedUuids(ids: ids, hint: hint);

class FeatureUuid {
  final UuidValue one;
  final List<UuidValue> many;

  const FeatureUuid({
    required this.one,
    required this.many,
  });

  @override
  int get hashCode => one.hashCode ^ many.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuid && runtimeType == other.runtimeType && one == other.one && many == other.many;
}
