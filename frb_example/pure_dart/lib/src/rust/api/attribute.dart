// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:meta/meta.dart' as meta;
part 'attribute.freezed.dart';

Future<void> handleCustomizedStruct({required Customized val, dynamic hint}) =>
    RustLib.instance.api.handleCustomizedStruct(val: val, hint: hint);

Future<UserId> nextUserId({required UserId userId, dynamic hint}) =>
    RustLib.instance.api.nextUserId(userId: userId, hint: hint);

class Customized {
  final String finalField;
  String? nonFinalField;

  Customized({
    required this.finalField,
    this.nonFinalField,
  });

  @override
  int get hashCode => finalField.hashCode ^ nonFinalField.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Customized &&
          runtimeType == other.runtimeType &&
          finalField == other.finalField &&
          nonFinalField == other.nonFinalField;
}

/// Example for @freezed and @meta.immutable
@freezed
@meta.immutable
class UserId with _$UserId {
  const factory UserId({
    @Default(0) int value,
  }) = _UserId;
}
