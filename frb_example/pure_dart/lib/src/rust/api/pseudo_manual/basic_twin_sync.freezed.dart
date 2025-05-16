// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is BasicGeneralEnumTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSync()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinSyncCopyWith<$Res> {
  $BasicGeneralEnumTwinSyncCopyWith(
      BasicGeneralEnumTwinSync _, $Res Function(BasicGeneralEnumTwinSync) __);
}

/// @nodoc

class BasicGeneralEnumTwinSync_Apple extends BasicGeneralEnumTwinSync {
  const BasicGeneralEnumTwinSync_Apple({required this.field}) : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinSync_AppleCopyWith<BasicGeneralEnumTwinSync_Apple>
      get copyWith => _$BasicGeneralEnumTwinSync_AppleCopyWithImpl<
          BasicGeneralEnumTwinSync_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSync_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSync.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinSync_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinSyncCopyWith<$Res> {
  factory $BasicGeneralEnumTwinSync_AppleCopyWith(
          BasicGeneralEnumTwinSync_Apple value,
          $Res Function(BasicGeneralEnumTwinSync_Apple) _then) =
      _$BasicGeneralEnumTwinSync_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinSync_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinSync_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinSync_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinSync_Apple _self;
  final $Res Function(BasicGeneralEnumTwinSync_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinSync_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinSync_Orange extends BasicGeneralEnumTwinSync {
  const BasicGeneralEnumTwinSync_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSync_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSync.orange()';
  }
}

// dart format on
