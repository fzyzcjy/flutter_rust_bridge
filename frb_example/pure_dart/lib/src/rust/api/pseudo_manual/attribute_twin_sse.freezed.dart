// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinSse {
  int get value;

  /// Create a copy of UserIdTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinSseCopyWith<UserIdTwinSse> get copyWith =>
      _$UserIdTwinSseCopyWithImpl<UserIdTwinSse>(
          this as UserIdTwinSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinSseCopyWith<$Res> {
  factory $UserIdTwinSseCopyWith(
          UserIdTwinSse value, $Res Function(UserIdTwinSse) _then) =
      _$UserIdTwinSseCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSseCopyWithImpl<$Res>
    implements $UserIdTwinSseCopyWith<$Res> {
  _$UserIdTwinSseCopyWithImpl(this._self, this._then);

  final UserIdTwinSse _self;
  final $Res Function(UserIdTwinSse) _then;

  /// Create a copy of UserIdTwinSse
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

class _UserIdTwinSse implements UserIdTwinSse {
  const _UserIdTwinSse({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinSseCopyWith<_UserIdTwinSse> get copyWith =>
      __$UserIdTwinSseCopyWithImpl<_UserIdTwinSse>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinSseCopyWith<$Res>
    implements $UserIdTwinSseCopyWith<$Res> {
  factory _$UserIdTwinSseCopyWith(
          _UserIdTwinSse value, $Res Function(_UserIdTwinSse) _then) =
      __$UserIdTwinSseCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinSseCopyWithImpl<$Res>
    implements _$UserIdTwinSseCopyWith<$Res> {
  __$UserIdTwinSseCopyWithImpl(this._self, this._then);

  final _UserIdTwinSse _self;
  final $Res Function(_UserIdTwinSse) _then;

  /// Create a copy of UserIdTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinSse(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
