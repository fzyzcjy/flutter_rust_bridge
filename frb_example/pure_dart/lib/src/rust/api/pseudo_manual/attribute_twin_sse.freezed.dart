// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinSse {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinSseCopyWith<UserIdTwinSse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinSseCopyWith<$Res> {
  factory $UserIdTwinSseCopyWith(
          UserIdTwinSse value, $Res Function(UserIdTwinSse) then) =
      _$UserIdTwinSseCopyWithImpl<$Res, UserIdTwinSse>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinSseCopyWithImpl<$Res, $Val extends UserIdTwinSse>
    implements $UserIdTwinSseCopyWith<$Res> {
  _$UserIdTwinSseCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdTwinSseImplCopyWith<$Res>
    implements $UserIdTwinSseCopyWith<$Res> {
  factory _$$UserIdTwinSseImplCopyWith(
          _$UserIdTwinSseImpl value, $Res Function(_$UserIdTwinSseImpl) then) =
      __$$UserIdTwinSseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinSseImplCopyWithImpl<$Res>
    extends _$UserIdTwinSseCopyWithImpl<$Res, _$UserIdTwinSseImpl>
    implements _$$UserIdTwinSseImplCopyWith<$Res> {
  __$$UserIdTwinSseImplCopyWithImpl(
      _$UserIdTwinSseImpl _value, $Res Function(_$UserIdTwinSseImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinSseImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinSseImpl implements _UserIdTwinSse {
  const _$UserIdTwinSseImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinSse(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinSseImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinSseImplCopyWith<_$UserIdTwinSseImpl> get copyWith =>
      __$$UserIdTwinSseImplCopyWithImpl<_$UserIdTwinSseImpl>(this, _$identity);
}

abstract class _UserIdTwinSse implements UserIdTwinSse {
  const factory _UserIdTwinSse({final int value}) = _$UserIdTwinSseImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinSseImplCopyWith<_$UserIdTwinSseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
