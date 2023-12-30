// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinSync {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinSyncCopyWith<UserIdTwinSync> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinSyncCopyWith<$Res> {
  factory $UserIdTwinSyncCopyWith(
          UserIdTwinSync value, $Res Function(UserIdTwinSync) then) =
      _$UserIdTwinSyncCopyWithImpl<$Res, UserIdTwinSync>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSyncCopyWithImpl<$Res, $Val extends UserIdTwinSync>
    implements $UserIdTwinSyncCopyWith<$Res> {
  _$UserIdTwinSyncCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdTwinSyncImplCopyWith<$Res>
    implements $UserIdTwinSyncCopyWith<$Res> {
  factory _$$UserIdTwinSyncImplCopyWith(_$UserIdTwinSyncImpl value,
          $Res Function(_$UserIdTwinSyncImpl) then) =
      __$$UserIdTwinSyncImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinSyncImplCopyWithImpl<$Res>
    extends _$UserIdTwinSyncCopyWithImpl<$Res, _$UserIdTwinSyncImpl>
    implements _$$UserIdTwinSyncImplCopyWith<$Res> {
  __$$UserIdTwinSyncImplCopyWithImpl(
      _$UserIdTwinSyncImpl _value, $Res Function(_$UserIdTwinSyncImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinSyncImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinSyncImpl implements _UserIdTwinSync {
  const _$UserIdTwinSyncImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinSync(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinSyncImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinSyncImplCopyWith<_$UserIdTwinSyncImpl> get copyWith =>
      __$$UserIdTwinSyncImplCopyWithImpl<_$UserIdTwinSyncImpl>(
          this, _$identity);
}

abstract class _UserIdTwinSync implements UserIdTwinSync {
  const factory _UserIdTwinSync({final int value}) = _$UserIdTwinSyncImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinSyncImplCopyWith<_$UserIdTwinSyncImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
