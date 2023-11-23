// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Abc {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(A field0) a,
    required TResult Function(B field0) b,
    required TResult Function(C field0) c,
    required TResult Function(int field0) justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(A field0)? a,
    TResult? Function(B field0)? b,
    TResult? Function(C field0)? c,
    TResult? Function(int field0)? justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(A field0)? a,
    TResult Function(B field0)? b,
    TResult Function(C field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Abc_A value) a,
    required TResult Function(Abc_B value) b,
    required TResult Function(Abc_C value) c,
    required TResult Function(Abc_JustInt value) justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Abc_A value)? a,
    TResult? Function(Abc_B value)? b,
    TResult? Function(Abc_C value)? c,
    TResult? Function(Abc_JustInt value)? justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Abc_A value)? a,
    TResult Function(Abc_B value)? b,
    TResult Function(Abc_C value)? c,
    TResult Function(Abc_JustInt value)? justInt,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AbcCopyWith<$Res> {
  factory $AbcCopyWith(Abc value, $Res Function(Abc) then) =
      _$AbcCopyWithImpl<$Res, Abc>;
}

/// @nodoc
class _$AbcCopyWithImpl<$Res, $Val extends Abc> implements $AbcCopyWith<$Res> {
  _$AbcCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Abc_AImplCopyWith<$Res> {
  factory _$$Abc_AImplCopyWith(
          _$Abc_AImpl value, $Res Function(_$Abc_AImpl) then) =
      __$$Abc_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({A field0});
}

/// @nodoc
class __$$Abc_AImplCopyWithImpl<$Res>
    extends _$AbcCopyWithImpl<$Res, _$Abc_AImpl>
    implements _$$Abc_AImplCopyWith<$Res> {
  __$$Abc_AImplCopyWithImpl(
      _$Abc_AImpl _value, $Res Function(_$Abc_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Abc_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as A,
    ));
  }
}

/// @nodoc

class _$Abc_AImpl implements Abc_A {
  const _$Abc_AImpl(this.field0);

  @override
  final A field0;

  @override
  String toString() {
    return 'Abc.a(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Abc_AImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Abc_AImplCopyWith<_$Abc_AImpl> get copyWith =>
      __$$Abc_AImplCopyWithImpl<_$Abc_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(A field0) a,
    required TResult Function(B field0) b,
    required TResult Function(C field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return a(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(A field0)? a,
    TResult? Function(B field0)? b,
    TResult? Function(C field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return a?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(A field0)? a,
    TResult Function(B field0)? b,
    TResult Function(C field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Abc_A value) a,
    required TResult Function(Abc_B value) b,
    required TResult Function(Abc_C value) c,
    required TResult Function(Abc_JustInt value) justInt,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Abc_A value)? a,
    TResult? Function(Abc_B value)? b,
    TResult? Function(Abc_C value)? c,
    TResult? Function(Abc_JustInt value)? justInt,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Abc_A value)? a,
    TResult Function(Abc_B value)? b,
    TResult Function(Abc_C value)? c,
    TResult Function(Abc_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class Abc_A implements Abc {
  const factory Abc_A(final A field0) = _$Abc_AImpl;

  @override
  A get field0;
  @JsonKey(ignore: true)
  _$$Abc_AImplCopyWith<_$Abc_AImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_BImplCopyWith<$Res> {
  factory _$$Abc_BImplCopyWith(
          _$Abc_BImpl value, $Res Function(_$Abc_BImpl) then) =
      __$$Abc_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({B field0});
}

/// @nodoc
class __$$Abc_BImplCopyWithImpl<$Res>
    extends _$AbcCopyWithImpl<$Res, _$Abc_BImpl>
    implements _$$Abc_BImplCopyWith<$Res> {
  __$$Abc_BImplCopyWithImpl(
      _$Abc_BImpl _value, $Res Function(_$Abc_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Abc_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as B,
    ));
  }
}

/// @nodoc

class _$Abc_BImpl implements Abc_B {
  const _$Abc_BImpl(this.field0);

  @override
  final B field0;

  @override
  String toString() {
    return 'Abc.b(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Abc_BImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Abc_BImplCopyWith<_$Abc_BImpl> get copyWith =>
      __$$Abc_BImplCopyWithImpl<_$Abc_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(A field0) a,
    required TResult Function(B field0) b,
    required TResult Function(C field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(A field0)? a,
    TResult? Function(B field0)? b,
    TResult? Function(C field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(A field0)? a,
    TResult Function(B field0)? b,
    TResult Function(C field0)? c,
    TResult Function(int field0)? justInt,
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
    required TResult Function(Abc_A value) a,
    required TResult Function(Abc_B value) b,
    required TResult Function(Abc_C value) c,
    required TResult Function(Abc_JustInt value) justInt,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Abc_A value)? a,
    TResult? Function(Abc_B value)? b,
    TResult? Function(Abc_C value)? c,
    TResult? Function(Abc_JustInt value)? justInt,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Abc_A value)? a,
    TResult Function(Abc_B value)? b,
    TResult Function(Abc_C value)? c,
    TResult Function(Abc_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class Abc_B implements Abc {
  const factory Abc_B(final B field0) = _$Abc_BImpl;

  @override
  B get field0;
  @JsonKey(ignore: true)
  _$$Abc_BImplCopyWith<_$Abc_BImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_CImplCopyWith<$Res> {
  factory _$$Abc_CImplCopyWith(
          _$Abc_CImpl value, $Res Function(_$Abc_CImpl) then) =
      __$$Abc_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({C field0});
}

/// @nodoc
class __$$Abc_CImplCopyWithImpl<$Res>
    extends _$AbcCopyWithImpl<$Res, _$Abc_CImpl>
    implements _$$Abc_CImplCopyWith<$Res> {
  __$$Abc_CImplCopyWithImpl(
      _$Abc_CImpl _value, $Res Function(_$Abc_CImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Abc_CImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as C,
    ));
  }
}

/// @nodoc

class _$Abc_CImpl implements Abc_C {
  const _$Abc_CImpl(this.field0);

  @override
  final C field0;

  @override
  String toString() {
    return 'Abc.c(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Abc_CImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Abc_CImplCopyWith<_$Abc_CImpl> get copyWith =>
      __$$Abc_CImplCopyWithImpl<_$Abc_CImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(A field0) a,
    required TResult Function(B field0) b,
    required TResult Function(C field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return c(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(A field0)? a,
    TResult? Function(B field0)? b,
    TResult? Function(C field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return c?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(A field0)? a,
    TResult Function(B field0)? b,
    TResult Function(C field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Abc_A value) a,
    required TResult Function(Abc_B value) b,
    required TResult Function(Abc_C value) c,
    required TResult Function(Abc_JustInt value) justInt,
  }) {
    return c(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Abc_A value)? a,
    TResult? Function(Abc_B value)? b,
    TResult? Function(Abc_C value)? c,
    TResult? Function(Abc_JustInt value)? justInt,
  }) {
    return c?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Abc_A value)? a,
    TResult Function(Abc_B value)? b,
    TResult Function(Abc_C value)? c,
    TResult Function(Abc_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(this);
    }
    return orElse();
  }
}

abstract class Abc_C implements Abc {
  const factory Abc_C(final C field0) = _$Abc_CImpl;

  @override
  C get field0;
  @JsonKey(ignore: true)
  _$$Abc_CImplCopyWith<_$Abc_CImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_JustIntImplCopyWith<$Res> {
  factory _$$Abc_JustIntImplCopyWith(
          _$Abc_JustIntImpl value, $Res Function(_$Abc_JustIntImpl) then) =
      __$$Abc_JustIntImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Abc_JustIntImplCopyWithImpl<$Res>
    extends _$AbcCopyWithImpl<$Res, _$Abc_JustIntImpl>
    implements _$$Abc_JustIntImplCopyWith<$Res> {
  __$$Abc_JustIntImplCopyWithImpl(
      _$Abc_JustIntImpl _value, $Res Function(_$Abc_JustIntImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Abc_JustIntImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Abc_JustIntImpl implements Abc_JustInt {
  const _$Abc_JustIntImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'Abc.justInt(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Abc_JustIntImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Abc_JustIntImplCopyWith<_$Abc_JustIntImpl> get copyWith =>
      __$$Abc_JustIntImplCopyWithImpl<_$Abc_JustIntImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(A field0) a,
    required TResult Function(B field0) b,
    required TResult Function(C field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return justInt(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(A field0)? a,
    TResult? Function(B field0)? b,
    TResult? Function(C field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return justInt?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(A field0)? a,
    TResult Function(B field0)? b,
    TResult Function(C field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    if (justInt != null) {
      return justInt(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Abc_A value) a,
    required TResult Function(Abc_B value) b,
    required TResult Function(Abc_C value) c,
    required TResult Function(Abc_JustInt value) justInt,
  }) {
    return justInt(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Abc_A value)? a,
    TResult? Function(Abc_B value)? b,
    TResult? Function(Abc_C value)? c,
    TResult? Function(Abc_JustInt value)? justInt,
  }) {
    return justInt?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Abc_A value)? a,
    TResult Function(Abc_B value)? b,
    TResult Function(Abc_C value)? c,
    TResult Function(Abc_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (justInt != null) {
      return justInt(this);
    }
    return orElse();
  }
}

abstract class Abc_JustInt implements Abc {
  const factory Abc_JustInt(final int field0) = _$Abc_JustIntImpl;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$Abc_JustIntImplCopyWith<_$Abc_JustIntImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
