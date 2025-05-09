// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinSyncSse {
  int get value;

  /// Create a copy of UserIdTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinSyncSseCopyWith<UserIdTwinSyncSse> get copyWith =>
      _$UserIdTwinSyncSseCopyWithImpl<UserIdTwinSyncSse>(
          this as UserIdTwinSyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinSyncSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSyncSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinSyncSseCopyWith<$Res> {
  factory $UserIdTwinSyncSseCopyWith(
          UserIdTwinSyncSse value, $Res Function(UserIdTwinSyncSse) _then) =
      _$UserIdTwinSyncSseCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSyncSseCopyWithImpl<$Res>
    implements $UserIdTwinSyncSseCopyWith<$Res> {
  _$UserIdTwinSyncSseCopyWithImpl(this._self, this._then);

  final UserIdTwinSyncSse _self;
  final $Res Function(UserIdTwinSyncSse) _then;

  /// Create a copy of UserIdTwinSyncSse
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

class _UserIdTwinSyncSse implements UserIdTwinSyncSse {
  const _UserIdTwinSyncSse({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinSyncSseCopyWith<_UserIdTwinSyncSse> get copyWith =>
      __$UserIdTwinSyncSseCopyWithImpl<_UserIdTwinSyncSse>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinSyncSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSyncSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinSyncSseCopyWith<$Res>
    implements $UserIdTwinSyncSseCopyWith<$Res> {
  factory _$UserIdTwinSyncSseCopyWith(
          _UserIdTwinSyncSse value, $Res Function(_UserIdTwinSyncSse) _then) =
      __$UserIdTwinSyncSseCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinSyncSseCopyWithImpl<$Res>
    implements _$UserIdTwinSyncSseCopyWith<$Res> {
  __$UserIdTwinSyncSseCopyWithImpl(this._self, this._then);

  final _UserIdTwinSyncSse _self;
  final $Res Function(_UserIdTwinSyncSse) _then;

  /// Create a copy of UserIdTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinSyncSse(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
