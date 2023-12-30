// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinRustAsync {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinRustAsyncCopyWith<UserIdTwinRustAsync> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinRustAsyncCopyWith<$Res> {
  factory $UserIdTwinRustAsyncCopyWith(
          UserIdTwinRustAsync value, $Res Function(UserIdTwinRustAsync) then) =
      _$UserIdTwinRustAsyncCopyWithImpl<$Res, UserIdTwinRustAsync>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinRustAsyncCopyWithImpl<$Res, $Val extends UserIdTwinRustAsync>
    implements $UserIdTwinRustAsyncCopyWith<$Res> {
  _$UserIdTwinRustAsyncCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdTwinRustAsyncImplCopyWith<$Res>
    implements $UserIdTwinRustAsyncCopyWith<$Res> {
  factory _$$UserIdTwinRustAsyncImplCopyWith(_$UserIdTwinRustAsyncImpl value,
          $Res Function(_$UserIdTwinRustAsyncImpl) then) =
      __$$UserIdTwinRustAsyncImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinRustAsyncImplCopyWithImpl<$Res>
    extends _$UserIdTwinRustAsyncCopyWithImpl<$Res, _$UserIdTwinRustAsyncImpl>
    implements _$$UserIdTwinRustAsyncImplCopyWith<$Res> {
  __$$UserIdTwinRustAsyncImplCopyWithImpl(_$UserIdTwinRustAsyncImpl _value,
      $Res Function(_$UserIdTwinRustAsyncImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinRustAsyncImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinRustAsyncImpl implements _UserIdTwinRustAsync {
  const _$UserIdTwinRustAsyncImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinRustAsync(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinRustAsyncImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinRustAsyncImplCopyWith<_$UserIdTwinRustAsyncImpl> get copyWith =>
      __$$UserIdTwinRustAsyncImplCopyWithImpl<_$UserIdTwinRustAsyncImpl>(
          this, _$identity);
}

abstract class _UserIdTwinRustAsync implements UserIdTwinRustAsync {
  const factory _UserIdTwinRustAsync({final int value}) =
      _$UserIdTwinRustAsyncImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinRustAsyncImplCopyWith<_$UserIdTwinRustAsyncImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
