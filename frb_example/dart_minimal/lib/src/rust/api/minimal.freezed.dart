// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'minimal.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$TheEnum {
  int get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) theVariant,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? theVariant,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? theVariant,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TheEnum_TheVariant value) theVariant,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TheEnum_TheVariant value)? theVariant,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TheEnum_TheVariant value)? theVariant,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $TheEnumCopyWith<TheEnum> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TheEnumCopyWith<$Res> {
  factory $TheEnumCopyWith(TheEnum value, $Res Function(TheEnum) then) =
      _$TheEnumCopyWithImpl<$Res, TheEnum>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$TheEnumCopyWithImpl<$Res, $Val extends TheEnum>
    implements $TheEnumCopyWith<$Res> {
  _$TheEnumCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_value.copyWith(
      field0: null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$TheEnum_TheVariantImplCopyWith<$Res>
    implements $TheEnumCopyWith<$Res> {
  factory _$$TheEnum_TheVariantImplCopyWith(_$TheEnum_TheVariantImpl value,
          $Res Function(_$TheEnum_TheVariantImpl) then) =
      __$$TheEnum_TheVariantImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$TheEnum_TheVariantImplCopyWithImpl<$Res>
    extends _$TheEnumCopyWithImpl<$Res, _$TheEnum_TheVariantImpl>
    implements _$$TheEnum_TheVariantImplCopyWith<$Res> {
  __$$TheEnum_TheVariantImplCopyWithImpl(_$TheEnum_TheVariantImpl _value,
      $Res Function(_$TheEnum_TheVariantImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$TheEnum_TheVariantImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$TheEnum_TheVariantImpl implements TheEnum_TheVariant {
  const _$TheEnum_TheVariantImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'TheEnum.theVariant(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TheEnum_TheVariantImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$TheEnum_TheVariantImplCopyWith<_$TheEnum_TheVariantImpl> get copyWith =>
      __$$TheEnum_TheVariantImplCopyWithImpl<_$TheEnum_TheVariantImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) theVariant,
  }) {
    return theVariant(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? theVariant,
  }) {
    return theVariant?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? theVariant,
    required TResult orElse(),
  }) {
    if (theVariant != null) {
      return theVariant(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(TheEnum_TheVariant value) theVariant,
  }) {
    return theVariant(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(TheEnum_TheVariant value)? theVariant,
  }) {
    return theVariant?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(TheEnum_TheVariant value)? theVariant,
    required TResult orElse(),
  }) {
    if (theVariant != null) {
      return theVariant(this);
    }
    return orElse();
  }
}

abstract class TheEnum_TheVariant implements TheEnum {
  const factory TheEnum_TheVariant(final int field0) = _$TheEnum_TheVariantImpl;

  @override
  int get field0;
  @override
  @JsonKey(ignore: true)
  _$$TheEnum_TheVariantImplCopyWith<_$TheEnum_TheVariantImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
