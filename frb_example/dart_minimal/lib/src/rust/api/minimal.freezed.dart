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
mixin _$MyEnum {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(int field0) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(int field0)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_A value) a,
    required TResult Function(MyEnum_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_A value)? a,
    TResult? Function(MyEnum_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_A value)? a,
    TResult Function(MyEnum_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyEnumCopyWith<$Res> {
  factory $MyEnumCopyWith(MyEnum value, $Res Function(MyEnum) then) =
      _$MyEnumCopyWithImpl<$Res, MyEnum>;
}

/// @nodoc
class _$MyEnumCopyWithImpl<$Res, $Val extends MyEnum>
    implements $MyEnumCopyWith<$Res> {
  _$MyEnumCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$MyEnum_AImplCopyWith<$Res> {
  factory _$$MyEnum_AImplCopyWith(
          _$MyEnum_AImpl value, $Res Function(_$MyEnum_AImpl) then) =
      __$$MyEnum_AImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$MyEnum_AImplCopyWithImpl<$Res>
    extends _$MyEnumCopyWithImpl<$Res, _$MyEnum_AImpl>
    implements _$$MyEnum_AImplCopyWith<$Res> {
  __$$MyEnum_AImplCopyWithImpl(
      _$MyEnum_AImpl _value, $Res Function(_$MyEnum_AImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$MyEnum_AImpl extends MyEnum_A {
  const _$MyEnum_AImpl() : super._();

  @override
  String toString() {
    return 'MyEnum.a()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$MyEnum_AImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(int field0) b,
  }) {
    return a();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(int field0)? b,
  }) {
    return a?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_A value) a,
    required TResult Function(MyEnum_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_A value)? a,
    TResult? Function(MyEnum_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_A value)? a,
    TResult Function(MyEnum_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class MyEnum_A extends MyEnum {
  const factory MyEnum_A() = _$MyEnum_AImpl;
  const MyEnum_A._() : super._();
}

/// @nodoc
abstract class _$$MyEnum_BImplCopyWith<$Res> {
  factory _$$MyEnum_BImplCopyWith(
          _$MyEnum_BImpl value, $Res Function(_$MyEnum_BImpl) then) =
      __$$MyEnum_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$MyEnum_BImplCopyWithImpl<$Res>
    extends _$MyEnumCopyWithImpl<$Res, _$MyEnum_BImpl>
    implements _$$MyEnum_BImplCopyWith<$Res> {
  __$$MyEnum_BImplCopyWithImpl(
      _$MyEnum_BImpl _value, $Res Function(_$MyEnum_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnum_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$MyEnum_BImpl extends MyEnum_B {
  const _$MyEnum_BImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'MyEnum.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnum_BImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnum_BImplCopyWith<_$MyEnum_BImpl> get copyWith =>
      __$$MyEnum_BImplCopyWithImpl<_$MyEnum_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(int field0) b,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(int field0)? b,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_A value) a,
    required TResult Function(MyEnum_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_A value)? a,
    TResult? Function(MyEnum_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_A value)? a,
    TResult Function(MyEnum_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class MyEnum_B extends MyEnum {
  const factory MyEnum_B(final int field0) = _$MyEnum_BImpl;
  const MyEnum_B._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$MyEnum_BImplCopyWith<_$MyEnum_BImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
