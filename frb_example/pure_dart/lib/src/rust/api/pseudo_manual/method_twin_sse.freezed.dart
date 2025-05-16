// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'method_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$SimpleEnumTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinSse()';
  }
}

/// @nodoc
class $SimpleEnumTwinSseCopyWith<$Res> {
  $SimpleEnumTwinSseCopyWith(
      SimpleEnumTwinSse _, $Res Function(SimpleEnumTwinSse) __);
}

/// @nodoc

class SimpleEnumTwinSse_First extends SimpleEnumTwinSse {
  const SimpleEnumTwinSse_First() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinSse_First);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinSse.first()';
  }
}

/// @nodoc

class SimpleEnumTwinSse_Second extends SimpleEnumTwinSse {
  const SimpleEnumTwinSse_Second(this.field0) : super._();

  final String field0;

  /// Create a copy of SimpleEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SimpleEnumTwinSse_SecondCopyWith<SimpleEnumTwinSse_Second> get copyWith =>
      _$SimpleEnumTwinSse_SecondCopyWithImpl<SimpleEnumTwinSse_Second>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinSse_Second &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SimpleEnumTwinSse.second(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SimpleEnumTwinSse_SecondCopyWith<$Res>
    implements $SimpleEnumTwinSseCopyWith<$Res> {
  factory $SimpleEnumTwinSse_SecondCopyWith(SimpleEnumTwinSse_Second value,
          $Res Function(SimpleEnumTwinSse_Second) _then) =
      _$SimpleEnumTwinSse_SecondCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$SimpleEnumTwinSse_SecondCopyWithImpl<$Res>
    implements $SimpleEnumTwinSse_SecondCopyWith<$Res> {
  _$SimpleEnumTwinSse_SecondCopyWithImpl(this._self, this._then);

  final SimpleEnumTwinSse_Second _self;
  final $Res Function(SimpleEnumTwinSse_Second) _then;

  /// Create a copy of SimpleEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SimpleEnumTwinSse_Second(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
