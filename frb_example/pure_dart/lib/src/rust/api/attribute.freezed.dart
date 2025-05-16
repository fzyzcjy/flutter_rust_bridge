// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinNormal {
  int get value;

  /// Create a copy of UserIdTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinNormalCopyWith<UserIdTwinNormal> get copyWith =>
      _$UserIdTwinNormalCopyWithImpl<UserIdTwinNormal>(
          this as UserIdTwinNormal, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinNormal &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinNormal(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinNormalCopyWith<$Res> {
  factory $UserIdTwinNormalCopyWith(
          UserIdTwinNormal value, $Res Function(UserIdTwinNormal) _then) =
      _$UserIdTwinNormalCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinNormalCopyWithImpl<$Res>
    implements $UserIdTwinNormalCopyWith<$Res> {
  _$UserIdTwinNormalCopyWithImpl(this._self, this._then);

  final UserIdTwinNormal _self;
  final $Res Function(UserIdTwinNormal) _then;

  /// Create a copy of UserIdTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_self.copyWith(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _UserIdTwinNormal implements UserIdTwinNormal {
  const _UserIdTwinNormal({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinNormalCopyWith<_UserIdTwinNormal> get copyWith =>
      __$UserIdTwinNormalCopyWithImpl<_UserIdTwinNormal>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinNormal &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinNormal(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinNormalCopyWith<$Res>
    implements $UserIdTwinNormalCopyWith<$Res> {
  factory _$UserIdTwinNormalCopyWith(
          _UserIdTwinNormal value, $Res Function(_UserIdTwinNormal) _then) =
      __$UserIdTwinNormalCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinNormalCopyWithImpl<$Res>
    implements _$UserIdTwinNormalCopyWith<$Res> {
  __$UserIdTwinNormalCopyWithImpl(this._self, this._then);

  final _UserIdTwinNormal _self;
  final $Res Function(_UserIdTwinNormal) _then;

  /// Create a copy of UserIdTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinNormal(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
