// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

UuidValue handleUuidTwinSync({required UuidValue id, dynamic hint}) =>
    RustLib.instance.api.handleUuidTwinSync(id: id, hint: hint);

List<UuidValue> handleUuidsTwinSync(
        {required List<UuidValue> ids, dynamic hint}) =>
    RustLib.instance.api.handleUuidsTwinSync(ids: ids, hint: hint);

FeatureUuidTwinSync handleNestedUuidsTwinSync(
        {required FeatureUuidTwinSync ids, dynamic hint}) =>
    RustLib.instance.api.handleNestedUuidsTwinSync(ids: ids, hint: hint);

class FeatureUuidTwinSync {
  final UuidValue one;
  final List<UuidValue> many;

  const FeatureUuidTwinSync({
    required this.one,
    required this.many,
  });

  @override
  int get hashCode => one.hashCode ^ many.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinSync &&
          runtimeType == other.runtimeType &&
          one == other.one &&
          many == other.many;
}
