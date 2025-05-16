// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinSync {
  int get value;

  /// Create a copy of UserIdTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinSyncCopyWith<UserIdTwinSync> get copyWith =>
      _$UserIdTwinSyncCopyWithImpl<UserIdTwinSync>(
          this as UserIdTwinSync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinSync &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSync(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinSyncCopyWith<$Res> {
  factory $UserIdTwinSyncCopyWith(
          UserIdTwinSync value, $Res Function(UserIdTwinSync) _then) =
      _$UserIdTwinSyncCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSyncCopyWithImpl<$Res>
    implements $UserIdTwinSyncCopyWith<$Res> {
  _$UserIdTwinSyncCopyWithImpl(this._self, this._then);

  final UserIdTwinSync _self;
  final $Res Function(UserIdTwinSync) _then;

  /// Create a copy of UserIdTwinSync
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

class _UserIdTwinSync implements UserIdTwinSync {
  const _UserIdTwinSync({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinSyncCopyWith<_UserIdTwinSync> get copyWith =>
      __$UserIdTwinSyncCopyWithImpl<_UserIdTwinSync>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinSync &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinSync(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinSyncCopyWith<$Res>
    implements $UserIdTwinSyncCopyWith<$Res> {
  factory _$UserIdTwinSyncCopyWith(
          _UserIdTwinSync value, $Res Function(_UserIdTwinSync) _then) =
      __$UserIdTwinSyncCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinSyncCopyWithImpl<$Res>
    implements _$UserIdTwinSyncCopyWith<$Res> {
  __$UserIdTwinSyncCopyWithImpl(this._self, this._then);

  final _UserIdTwinSync _self;
  final $Res Function(_UserIdTwinSync) _then;

  /// Create a copy of UserIdTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinSync(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
