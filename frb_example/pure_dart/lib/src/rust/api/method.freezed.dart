// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'method.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$SimpleEnumTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinNormal()';
  }
}

/// @nodoc
class $SimpleEnumTwinNormalCopyWith<$Res> {
  $SimpleEnumTwinNormalCopyWith(
      SimpleEnumTwinNormal _, $Res Function(SimpleEnumTwinNormal) __);
}

/// @nodoc

class SimpleEnumTwinNormal_First extends SimpleEnumTwinNormal {
  const SimpleEnumTwinNormal_First() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinNormal_First);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinNormal.first()';
  }
}

/// @nodoc

class SimpleEnumTwinNormal_Second extends SimpleEnumTwinNormal {
  const SimpleEnumTwinNormal_Second(this.field0) : super._();

  final String field0;

  /// Create a copy of SimpleEnumTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SimpleEnumTwinNormal_SecondCopyWith<SimpleEnumTwinNormal_Second>
      get copyWith => _$SimpleEnumTwinNormal_SecondCopyWithImpl<
          SimpleEnumTwinNormal_Second>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinNormal_Second &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SimpleEnumTwinNormal.second(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SimpleEnumTwinNormal_SecondCopyWith<$Res>
    implements $SimpleEnumTwinNormalCopyWith<$Res> {
  factory $SimpleEnumTwinNormal_SecondCopyWith(
          SimpleEnumTwinNormal_Second value,
          $Res Function(SimpleEnumTwinNormal_Second) _then) =
      _$SimpleEnumTwinNormal_SecondCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$SimpleEnumTwinNormal_SecondCopyWithImpl<$Res>
    implements $SimpleEnumTwinNormal_SecondCopyWith<$Res> {
  _$SimpleEnumTwinNormal_SecondCopyWithImpl(this._self, this._then);

  final SimpleEnumTwinNormal_Second _self;
  final $Res Function(SimpleEnumTwinNormal_Second) _then;

  /// Create a copy of SimpleEnumTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SimpleEnumTwinNormal_Second(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
