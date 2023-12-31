// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinSyncSse {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinSyncSseCopyWith<UserIdTwinSyncSse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinSyncSseCopyWith<$Res> {
  factory $UserIdTwinSyncSseCopyWith(
          UserIdTwinSyncSse value, $Res Function(UserIdTwinSyncSse) then) =
      _$UserIdTwinSyncSseCopyWithImpl<$Res, UserIdTwinSyncSse>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSyncSseCopyWithImpl<$Res, $Val extends UserIdTwinSyncSse>
    implements $UserIdTwinSyncSseCopyWith<$Res> {
  _$UserIdTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_value.copyWith(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$UserIdTwinSyncSseImplCopyWith<$Res>
    implements $UserIdTwinSyncSseCopyWith<$Res> {
  factory _$$UserIdTwinSyncSseImplCopyWith(_$UserIdTwinSyncSseImpl value,
          $Res Function(_$UserIdTwinSyncSseImpl) then) =
      __$$UserIdTwinSyncSseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinSyncSseImplCopyWithImpl<$Res>
    extends _$UserIdTwinSyncSseCopyWithImpl<$Res, _$UserIdTwinSyncSseImpl>
    implements _$$UserIdTwinSyncSseImplCopyWith<$Res> {
  __$$UserIdTwinSyncSseImplCopyWithImpl(_$UserIdTwinSyncSseImpl _value,
      $Res Function(_$UserIdTwinSyncSseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinSyncSseImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinSyncSseImpl implements _UserIdTwinSyncSse {
  const _$UserIdTwinSyncSseImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinSyncSse(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinSyncSseImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinSyncSseImplCopyWith<_$UserIdTwinSyncSseImpl> get copyWith =>
      __$$UserIdTwinSyncSseImplCopyWithImpl<_$UserIdTwinSyncSseImpl>(
          this, _$identity);
}

abstract class _UserIdTwinSyncSse implements UserIdTwinSyncSse {
  const factory _UserIdTwinSyncSse({final int value}) = _$UserIdTwinSyncSseImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinSyncSseImplCopyWith<_$UserIdTwinSyncSseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
