// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$UserIdTwinNormal {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdTwinNormalCopyWith<UserIdTwinNormal> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdTwinNormalCopyWith<$Res> {
  factory $UserIdTwinNormalCopyWith(
          UserIdTwinNormal value, $Res Function(UserIdTwinNormal) then) =
      _$UserIdTwinNormalCopyWithImpl<$Res, UserIdTwinNormal>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinNormalCopyWithImpl<$Res, $Val extends UserIdTwinNormal>
    implements $UserIdTwinNormalCopyWith<$Res> {
  _$UserIdTwinNormalCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdTwinNormalImplCopyWith<$Res>
    implements $UserIdTwinNormalCopyWith<$Res> {
  factory _$$UserIdTwinNormalImplCopyWith(_$UserIdTwinNormalImpl value,
          $Res Function(_$UserIdTwinNormalImpl) then) =
      __$$UserIdTwinNormalImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdTwinNormalImplCopyWithImpl<$Res>
    extends _$UserIdTwinNormalCopyWithImpl<$Res, _$UserIdTwinNormalImpl>
    implements _$$UserIdTwinNormalImplCopyWith<$Res> {
  __$$UserIdTwinNormalImplCopyWithImpl(_$UserIdTwinNormalImpl _value,
      $Res Function(_$UserIdTwinNormalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdTwinNormalImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdTwinNormalImpl implements _UserIdTwinNormal {
  const _$UserIdTwinNormalImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserIdTwinNormal(value: $value)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdTwinNormalImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdTwinNormalImplCopyWith<_$UserIdTwinNormalImpl> get copyWith =>
      __$$UserIdTwinNormalImplCopyWithImpl<_$UserIdTwinNormalImpl>(
          this, _$identity);
}

abstract class _UserIdTwinNormal implements UserIdTwinNormal {
  const factory _UserIdTwinNormal({final int value}) = _$UserIdTwinNormalImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdTwinNormalImplCopyWith<_$UserIdTwinNormalImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
