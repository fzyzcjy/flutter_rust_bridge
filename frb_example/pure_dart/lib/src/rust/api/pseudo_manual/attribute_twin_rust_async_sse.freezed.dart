// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinRustAsyncSse {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinRustAsyncSseCopyWith<UserIdTwinRustAsyncSse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinRustAsyncSseCopyWith<$Res> {
  factory $UserIdTwinRustAsyncSseCopyWith(UserIdTwinRustAsyncSse value,
          $Res Function(UserIdTwinRustAsyncSse) then) =
      _$UserIdTwinRustAsyncSseCopyWithImpl<$Res, UserIdTwinRustAsyncSse>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends UserIdTwinRustAsyncSse>
    implements $UserIdTwinRustAsyncSseCopyWith<$Res> {
  _$UserIdTwinRustAsyncSseCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdTwinRustAsyncSseImplCopyWith<$Res>
    implements $UserIdTwinRustAsyncSseCopyWith<$Res> {
  factory _$$UserIdTwinRustAsyncSseImplCopyWith(
          _$UserIdTwinRustAsyncSseImpl value,
          $Res Function(_$UserIdTwinRustAsyncSseImpl) then) =
      __$$UserIdTwinRustAsyncSseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinRustAsyncSseImplCopyWithImpl<$Res>
    extends _$UserIdTwinRustAsyncSseCopyWithImpl<$Res,
        _$UserIdTwinRustAsyncSseImpl>
    implements _$$UserIdTwinRustAsyncSseImplCopyWith<$Res> {
  __$$UserIdTwinRustAsyncSseImplCopyWithImpl(
      _$UserIdTwinRustAsyncSseImpl _value,
      $Res Function(_$UserIdTwinRustAsyncSseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinRustAsyncSseImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinRustAsyncSseImpl implements _UserIdTwinRustAsyncSse {
  const _$UserIdTwinRustAsyncSseImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinRustAsyncSse(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinRustAsyncSseImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinRustAsyncSseImplCopyWith<_$UserIdTwinRustAsyncSseImpl>
      get copyWith => __$$UserIdTwinRustAsyncSseImplCopyWithImpl<
          _$UserIdTwinRustAsyncSseImpl>(this, _$identity);
}

abstract class _UserIdTwinRustAsyncSse implements UserIdTwinRustAsyncSse {
  const factory _UserIdTwinRustAsyncSse({final int value}) =
      _$UserIdTwinRustAsyncSseImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinRustAsyncSseImplCopyWith<_$UserIdTwinRustAsyncSseImpl>
      get copyWith => throw _privateConstructorUsedError;
}
