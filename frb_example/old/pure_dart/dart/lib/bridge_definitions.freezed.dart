// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

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
  factory $AbcCopyWith(Abc value, $Res Function(Abc) then) = _$AbcCopyWithImpl<$Res, Abc>;
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
  factory _$$Abc_AImplCopyWith(_$Abc_AImpl value, $Res Function(_$Abc_AImpl) then) = __$$Abc_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({A field0});
}

/// @nodoc
class __$$Abc_AImplCopyWithImpl<$Res> extends _$AbcCopyWithImpl<$Res, _$Abc_AImpl>
    implements _$$Abc_AImplCopyWith<$Res> {
  __$$Abc_AImplCopyWithImpl(_$Abc_AImpl _value, $Res Function(_$Abc_AImpl) _then) : super(_value, _then);

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
  _$$Abc_AImplCopyWith<_$Abc_AImpl> get copyWith => __$$Abc_AImplCopyWithImpl<_$Abc_AImpl>(this, _$identity);

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
  _$$Abc_AImplCopyWith<_$Abc_AImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_BImplCopyWith<$Res> {
  factory _$$Abc_BImplCopyWith(_$Abc_BImpl value, $Res Function(_$Abc_BImpl) then) = __$$Abc_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({B field0});
}

/// @nodoc
class __$$Abc_BImplCopyWithImpl<$Res> extends _$AbcCopyWithImpl<$Res, _$Abc_BImpl>
    implements _$$Abc_BImplCopyWith<$Res> {
  __$$Abc_BImplCopyWithImpl(_$Abc_BImpl _value, $Res Function(_$Abc_BImpl) _then) : super(_value, _then);

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
  _$$Abc_BImplCopyWith<_$Abc_BImpl> get copyWith => __$$Abc_BImplCopyWithImpl<_$Abc_BImpl>(this, _$identity);

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
  _$$Abc_BImplCopyWith<_$Abc_BImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_CImplCopyWith<$Res> {
  factory _$$Abc_CImplCopyWith(_$Abc_CImpl value, $Res Function(_$Abc_CImpl) then) = __$$Abc_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({C field0});
}

/// @nodoc
class __$$Abc_CImplCopyWithImpl<$Res> extends _$AbcCopyWithImpl<$Res, _$Abc_CImpl>
    implements _$$Abc_CImplCopyWith<$Res> {
  __$$Abc_CImplCopyWithImpl(_$Abc_CImpl _value, $Res Function(_$Abc_CImpl) _then) : super(_value, _then);

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
  _$$Abc_CImplCopyWith<_$Abc_CImpl> get copyWith => __$$Abc_CImplCopyWithImpl<_$Abc_CImpl>(this, _$identity);

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
  _$$Abc_CImplCopyWith<_$Abc_CImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Abc_JustIntImplCopyWith<$Res> {
  factory _$$Abc_JustIntImplCopyWith(_$Abc_JustIntImpl value, $Res Function(_$Abc_JustIntImpl) then) =
      __$$Abc_JustIntImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Abc_JustIntImplCopyWithImpl<$Res> extends _$AbcCopyWithImpl<$Res, _$Abc_JustIntImpl>
    implements _$$Abc_JustIntImplCopyWith<$Res> {
  __$$Abc_JustIntImplCopyWithImpl(_$Abc_JustIntImpl _value, $Res Function(_$Abc_JustIntImpl) _then)
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
  _$$Abc_JustIntImplCopyWith<_$Abc_JustIntImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ApplicationMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ApplicationMessageCopyWith<$Res> {
  factory $ApplicationMessageCopyWith(ApplicationMessage value, $Res Function(ApplicationMessage) then) =
      _$ApplicationMessageCopyWithImpl<$Res, ApplicationMessage>;
}

/// @nodoc
class _$ApplicationMessageCopyWithImpl<$Res, $Val extends ApplicationMessage>
    implements $ApplicationMessageCopyWith<$Res> {
  _$ApplicationMessageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$ApplicationMessage_DisplayMessageImplCopyWith<$Res> {
  factory _$$ApplicationMessage_DisplayMessageImplCopyWith(
          _$ApplicationMessage_DisplayMessageImpl value, $Res Function(_$ApplicationMessage_DisplayMessageImpl) then) =
      __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_DisplayMessageImpl>
    implements _$$ApplicationMessage_DisplayMessageImplCopyWith<$Res> {
  __$$ApplicationMessage_DisplayMessageImplCopyWithImpl(
      _$ApplicationMessage_DisplayMessageImpl _value, $Res Function(_$ApplicationMessage_DisplayMessageImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$ApplicationMessage_DisplayMessageImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_DisplayMessageImpl implements ApplicationMessage_DisplayMessage {
  const _$ApplicationMessage_DisplayMessageImpl(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'ApplicationMessage.displayMessage(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_DisplayMessageImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ApplicationMessage_DisplayMessageImplCopyWith<_$ApplicationMessage_DisplayMessageImpl> get copyWith =>
      __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<_$ApplicationMessage_DisplayMessageImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return displayMessage(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
  }) {
    return displayMessage?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return displayMessage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
  }) {
    return displayMessage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_DisplayMessage implements ApplicationMessage {
  const factory ApplicationMessage_DisplayMessage(final String field0) = _$ApplicationMessage_DisplayMessageImpl;

  String get field0;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_DisplayMessageImplCopyWith<_$ApplicationMessage_DisplayMessageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_RenderPixelImplCopyWith<$Res> {
  factory _$$ApplicationMessage_RenderPixelImplCopyWith(
          _$ApplicationMessage_RenderPixelImpl value, $Res Function(_$ApplicationMessage_RenderPixelImpl) then) =
      __$$ApplicationMessage_RenderPixelImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int x, int y});
}

/// @nodoc
class __$$ApplicationMessage_RenderPixelImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_RenderPixelImpl>
    implements _$$ApplicationMessage_RenderPixelImplCopyWith<$Res> {
  __$$ApplicationMessage_RenderPixelImplCopyWithImpl(
      _$ApplicationMessage_RenderPixelImpl _value, $Res Function(_$ApplicationMessage_RenderPixelImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? x = null,
    Object? y = null,
  }) {
    return _then(_$ApplicationMessage_RenderPixelImpl(
      x: null == x
          ? _value.x
          : x // ignore: cast_nullable_to_non_nullable
              as int,
      y: null == y
          ? _value.y
          : y // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_RenderPixelImpl implements ApplicationMessage_RenderPixel {
  const _$ApplicationMessage_RenderPixelImpl({required this.x, required this.y});

  @override
  final int x;
  @override
  final int y;

  @override
  String toString() {
    return 'ApplicationMessage.renderPixel(x: $x, y: $y)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_RenderPixelImpl &&
            (identical(other.x, x) || other.x == x) &&
            (identical(other.y, y) || other.y == y));
  }

  @override
  int get hashCode => Object.hash(runtimeType, x, y);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ApplicationMessage_RenderPixelImplCopyWith<_$ApplicationMessage_RenderPixelImpl> get copyWith =>
      __$$ApplicationMessage_RenderPixelImplCopyWithImpl<_$ApplicationMessage_RenderPixelImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return renderPixel(x, y);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
  }) {
    return renderPixel?.call(x, y);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(x, y);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return renderPixel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
  }) {
    return renderPixel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_RenderPixel implements ApplicationMessage {
  const factory ApplicationMessage_RenderPixel({required final int x, required final int y}) =
      _$ApplicationMessage_RenderPixelImpl;

  int get x;
  int get y;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_RenderPixelImplCopyWith<_$ApplicationMessage_RenderPixelImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_ExitImplCopyWith<$Res> {
  factory _$$ApplicationMessage_ExitImplCopyWith(
          _$ApplicationMessage_ExitImpl value, $Res Function(_$ApplicationMessage_ExitImpl) then) =
      __$$ApplicationMessage_ExitImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ApplicationMessage_ExitImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_ExitImpl>
    implements _$$ApplicationMessage_ExitImplCopyWith<$Res> {
  __$$ApplicationMessage_ExitImplCopyWithImpl(
      _$ApplicationMessage_ExitImpl _value, $Res Function(_$ApplicationMessage_ExitImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$ApplicationMessage_ExitImpl implements ApplicationMessage_Exit {
  const _$ApplicationMessage_ExitImpl();

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$ApplicationMessage_ExitImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return exit();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
  }) {
    return exit?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return exit(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
  }) {
    return exit?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_Exit implements ApplicationMessage {
  const factory ApplicationMessage_Exit() = _$ApplicationMessage_ExitImpl;
}

/// @nodoc
mixin _$CustomError {
  Object get e => throw _privateConstructorUsedError;
  String get backtrace => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomError_Error0 value) error0,
    required TResult Function(CustomError_Error1 value) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomError_Error0 value)? error0,
    TResult? Function(CustomError_Error1 value)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomError_Error0 value)? error0,
    TResult Function(CustomError_Error1 value)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $CustomErrorCopyWith<CustomError> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomErrorCopyWith<$Res> {
  factory $CustomErrorCopyWith(CustomError value, $Res Function(CustomError) then) =
      _$CustomErrorCopyWithImpl<$Res, CustomError>;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorCopyWithImpl<$Res, $Val extends CustomError> implements $CustomErrorCopyWith<$Res> {
  _$CustomErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? backtrace = null,
  }) {
    return _then(_value.copyWith(
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CustomError_Error0ImplCopyWith<$Res> implements $CustomErrorCopyWith<$Res> {
  factory _$$CustomError_Error0ImplCopyWith(
          _$CustomError_Error0Impl value, $Res Function(_$CustomError_Error0Impl) then) =
      __$$CustomError_Error0ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class __$$CustomError_Error0ImplCopyWithImpl<$Res> extends _$CustomErrorCopyWithImpl<$Res, _$CustomError_Error0Impl>
    implements _$$CustomError_Error0ImplCopyWith<$Res> {
  __$$CustomError_Error0ImplCopyWithImpl(_$CustomError_Error0Impl _value, $Res Function(_$CustomError_Error0Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomError_Error0Impl(
      e: null == e
          ? _value.e
          : e // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomError_Error0Impl implements CustomError_Error0 {
  const _$CustomError_Error0Impl({required this.e, required this.backtrace});

  @override
  final String e;
  @override
  final String backtrace;

  @override
  String toString() {
    return 'CustomError.error0(e: $e, backtrace: $backtrace)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomError_Error0Impl &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) || other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomError_Error0ImplCopyWith<_$CustomError_Error0Impl> get copyWith =>
      __$$CustomError_Error0ImplCopyWithImpl<_$CustomError_Error0Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) {
    return error0(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) {
    return error0?.call(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(e, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomError_Error0 value) error0,
    required TResult Function(CustomError_Error1 value) error1,
  }) {
    return error0(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomError_Error0 value)? error0,
    TResult? Function(CustomError_Error1 value)? error1,
  }) {
    return error0?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomError_Error0 value)? error0,
    TResult Function(CustomError_Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(this);
    }
    return orElse();
  }
}

abstract class CustomError_Error0 implements CustomError, FrbBacktracedException {
  const factory CustomError_Error0({required final String e, required final String backtrace}) =
      _$CustomError_Error0Impl;

  @override
  String get e;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomError_Error0ImplCopyWith<_$CustomError_Error0Impl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomError_Error1ImplCopyWith<$Res> implements $CustomErrorCopyWith<$Res> {
  factory _$$CustomError_Error1ImplCopyWith(
          _$CustomError_Error1Impl value, $Res Function(_$CustomError_Error1Impl) then) =
      __$$CustomError_Error1ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class __$$CustomError_Error1ImplCopyWithImpl<$Res> extends _$CustomErrorCopyWithImpl<$Res, _$CustomError_Error1Impl>
    implements _$$CustomError_Error1ImplCopyWith<$Res> {
  __$$CustomError_Error1ImplCopyWithImpl(_$CustomError_Error1Impl _value, $Res Function(_$CustomError_Error1Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomError_Error1Impl(
      e: null == e
          ? _value.e
          : e // ignore: cast_nullable_to_non_nullable
              as int,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomError_Error1Impl implements CustomError_Error1 {
  const _$CustomError_Error1Impl({required this.e, required this.backtrace});

  @override
  final int e;
  @override
  final String backtrace;

  @override
  String toString() {
    return 'CustomError.error1(e: $e, backtrace: $backtrace)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomError_Error1Impl &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) || other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomError_Error1ImplCopyWith<_$CustomError_Error1Impl> get copyWith =>
      __$$CustomError_Error1ImplCopyWithImpl<_$CustomError_Error1Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) {
    return error1(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) {
    return error1?.call(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(e, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomError_Error0 value) error0,
    required TResult Function(CustomError_Error1 value) error1,
  }) {
    return error1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomError_Error0 value)? error0,
    TResult? Function(CustomError_Error1 value)? error1,
  }) {
    return error1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomError_Error0 value)? error0,
    TResult Function(CustomError_Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(this);
    }
    return orElse();
  }
}

abstract class CustomError_Error1 implements CustomError, FrbBacktracedException {
  const factory CustomError_Error1({required final int e, required final String backtrace}) = _$CustomError_Error1Impl;

  @override
  int get e;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomError_Error1ImplCopyWith<_$CustomError_Error1Impl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$CustomNestedError1 {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2 field0)? errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError1_CustomNested1 value) customNested1,
    required TResult Function(CustomNestedError1_ErrorNested value) errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult? Function(CustomNestedError1_ErrorNested value)? errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult Function(CustomNestedError1_ErrorNested value)? errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError1CopyWith<$Res> {
  factory $CustomNestedError1CopyWith(CustomNestedError1 value, $Res Function(CustomNestedError1) then) =
      _$CustomNestedError1CopyWithImpl<$Res, CustomNestedError1>;
}

/// @nodoc
class _$CustomNestedError1CopyWithImpl<$Res, $Val extends CustomNestedError1>
    implements $CustomNestedError1CopyWith<$Res> {
  _$CustomNestedError1CopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedError1_CustomNested1ImplCopyWith<$Res> {
  factory _$$CustomNestedError1_CustomNested1ImplCopyWith(
          _$CustomNestedError1_CustomNested1Impl value, $Res Function(_$CustomNestedError1_CustomNested1Impl) then) =
      __$$CustomNestedError1_CustomNested1ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedError1_CustomNested1ImplCopyWithImpl<$Res>
    extends _$CustomNestedError1CopyWithImpl<$Res, _$CustomNestedError1_CustomNested1Impl>
    implements _$$CustomNestedError1_CustomNested1ImplCopyWith<$Res> {
  __$$CustomNestedError1_CustomNested1ImplCopyWithImpl(
      _$CustomNestedError1_CustomNested1Impl _value, $Res Function(_$CustomNestedError1_CustomNested1Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError1_CustomNested1Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomNestedError1_CustomNested1Impl implements CustomNestedError1_CustomNested1 {
  const _$CustomNestedError1_CustomNested1Impl(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'CustomNestedError1.customNested1(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError1_CustomNested1Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError1_CustomNested1ImplCopyWith<_$CustomNestedError1_CustomNested1Impl> get copyWith =>
      __$$CustomNestedError1_CustomNested1ImplCopyWithImpl<_$CustomNestedError1_CustomNested1Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) {
    return customNested1(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2 field0)? errorNested,
  }) {
    return customNested1?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) {
    if (customNested1 != null) {
      return customNested1(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError1_CustomNested1 value) customNested1,
    required TResult Function(CustomNestedError1_ErrorNested value) errorNested,
  }) {
    return customNested1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult? Function(CustomNestedError1_ErrorNested value)? errorNested,
  }) {
    return customNested1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult Function(CustomNestedError1_ErrorNested value)? errorNested,
    required TResult orElse(),
  }) {
    if (customNested1 != null) {
      return customNested1(this);
    }
    return orElse();
  }
}

abstract class CustomNestedError1_CustomNested1 implements CustomNestedError1 {
  const factory CustomNestedError1_CustomNested1(final String field0) = _$CustomNestedError1_CustomNested1Impl;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError1_CustomNested1ImplCopyWith<_$CustomNestedError1_CustomNested1Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedError1_ErrorNestedImplCopyWith<$Res> {
  factory _$$CustomNestedError1_ErrorNestedImplCopyWith(
          _$CustomNestedError1_ErrorNestedImpl value, $Res Function(_$CustomNestedError1_ErrorNestedImpl) then) =
      __$$CustomNestedError1_ErrorNestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({CustomNestedError2 field0});

  $CustomNestedError2CopyWith<$Res> get field0;
}

/// @nodoc
class __$$CustomNestedError1_ErrorNestedImplCopyWithImpl<$Res>
    extends _$CustomNestedError1CopyWithImpl<$Res, _$CustomNestedError1_ErrorNestedImpl>
    implements _$$CustomNestedError1_ErrorNestedImplCopyWith<$Res> {
  __$$CustomNestedError1_ErrorNestedImplCopyWithImpl(
      _$CustomNestedError1_ErrorNestedImpl _value, $Res Function(_$CustomNestedError1_ErrorNestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError1_ErrorNestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2CopyWith<$Res> get field0 {
    return $CustomNestedError2CopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$CustomNestedError1_ErrorNestedImpl implements CustomNestedError1_ErrorNested {
  const _$CustomNestedError1_ErrorNestedImpl(this.field0);

  @override
  final CustomNestedError2 field0;

  @override
  String toString() {
    return 'CustomNestedError1.errorNested(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError1_ErrorNestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError1_ErrorNestedImplCopyWith<_$CustomNestedError1_ErrorNestedImpl> get copyWith =>
      __$$CustomNestedError1_ErrorNestedImplCopyWithImpl<_$CustomNestedError1_ErrorNestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) {
    return errorNested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2 field0)? errorNested,
  }) {
    return errorNested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) {
    if (errorNested != null) {
      return errorNested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError1_CustomNested1 value) customNested1,
    required TResult Function(CustomNestedError1_ErrorNested value) errorNested,
  }) {
    return errorNested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult? Function(CustomNestedError1_ErrorNested value)? errorNested,
  }) {
    return errorNested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1_CustomNested1 value)? customNested1,
    TResult Function(CustomNestedError1_ErrorNested value)? errorNested,
    required TResult orElse(),
  }) {
    if (errorNested != null) {
      return errorNested(this);
    }
    return orElse();
  }
}

abstract class CustomNestedError1_ErrorNested implements CustomNestedError1 {
  const factory CustomNestedError1_ErrorNested(final CustomNestedError2 field0) = _$CustomNestedError1_ErrorNestedImpl;

  @override
  CustomNestedError2 get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError1_ErrorNestedImplCopyWith<_$CustomNestedError1_ErrorNestedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$CustomNestedError2 {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError2_CustomNested2 value) customNested2,
    required TResult Function(CustomNestedError2_CustomNested2Number value) customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult? Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError2CopyWith<$Res> {
  factory $CustomNestedError2CopyWith(CustomNestedError2 value, $Res Function(CustomNestedError2) then) =
      _$CustomNestedError2CopyWithImpl<$Res, CustomNestedError2>;
}

/// @nodoc
class _$CustomNestedError2CopyWithImpl<$Res, $Val extends CustomNestedError2>
    implements $CustomNestedError2CopyWith<$Res> {
  _$CustomNestedError2CopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedError2_CustomNested2ImplCopyWith<$Res> {
  factory _$$CustomNestedError2_CustomNested2ImplCopyWith(
          _$CustomNestedError2_CustomNested2Impl value, $Res Function(_$CustomNestedError2_CustomNested2Impl) then) =
      __$$CustomNestedError2_CustomNested2ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedError2_CustomNested2ImplCopyWithImpl<$Res>
    extends _$CustomNestedError2CopyWithImpl<$Res, _$CustomNestedError2_CustomNested2Impl>
    implements _$$CustomNestedError2_CustomNested2ImplCopyWith<$Res> {
  __$$CustomNestedError2_CustomNested2ImplCopyWithImpl(
      _$CustomNestedError2_CustomNested2Impl _value, $Res Function(_$CustomNestedError2_CustomNested2Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError2_CustomNested2Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomNestedError2_CustomNested2Impl implements CustomNestedError2_CustomNested2 {
  const _$CustomNestedError2_CustomNested2Impl(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'CustomNestedError2.customNested2(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError2_CustomNested2Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError2_CustomNested2ImplCopyWith<_$CustomNestedError2_CustomNested2Impl> get copyWith =>
      __$$CustomNestedError2_CustomNested2ImplCopyWithImpl<_$CustomNestedError2_CustomNested2Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) {
    return customNested2(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
  }) {
    return customNested2?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2 != null) {
      return customNested2(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError2_CustomNested2 value) customNested2,
    required TResult Function(CustomNestedError2_CustomNested2Number value) customNested2Number,
  }) {
    return customNested2(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult? Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
  }) {
    return customNested2?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2 != null) {
      return customNested2(this);
    }
    return orElse();
  }
}

abstract class CustomNestedError2_CustomNested2 implements CustomNestedError2 {
  const factory CustomNestedError2_CustomNested2(final String field0) = _$CustomNestedError2_CustomNested2Impl;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError2_CustomNested2ImplCopyWith<_$CustomNestedError2_CustomNested2Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedError2_CustomNested2NumberImplCopyWith<$Res> {
  factory _$$CustomNestedError2_CustomNested2NumberImplCopyWith(_$CustomNestedError2_CustomNested2NumberImpl value,
          $Res Function(_$CustomNestedError2_CustomNested2NumberImpl) then) =
      __$$CustomNestedError2_CustomNested2NumberImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$CustomNestedError2_CustomNested2NumberImplCopyWithImpl<$Res>
    extends _$CustomNestedError2CopyWithImpl<$Res, _$CustomNestedError2_CustomNested2NumberImpl>
    implements _$$CustomNestedError2_CustomNested2NumberImplCopyWith<$Res> {
  __$$CustomNestedError2_CustomNested2NumberImplCopyWithImpl(_$CustomNestedError2_CustomNested2NumberImpl _value,
      $Res Function(_$CustomNestedError2_CustomNested2NumberImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError2_CustomNested2NumberImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$CustomNestedError2_CustomNested2NumberImpl implements CustomNestedError2_CustomNested2Number {
  const _$CustomNestedError2_CustomNested2NumberImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'CustomNestedError2.customNested2Number(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError2_CustomNested2NumberImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError2_CustomNested2NumberImplCopyWith<_$CustomNestedError2_CustomNested2NumberImpl> get copyWith =>
      __$$CustomNestedError2_CustomNested2NumberImplCopyWithImpl<_$CustomNestedError2_CustomNested2NumberImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) {
    return customNested2Number(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
  }) {
    return customNested2Number?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2Number != null) {
      return customNested2Number(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedError2_CustomNested2 value) customNested2,
    required TResult Function(CustomNestedError2_CustomNested2Number value) customNested2Number,
  }) {
    return customNested2Number(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult? Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
  }) {
    return customNested2Number?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2_CustomNested2 value)? customNested2,
    TResult Function(CustomNestedError2_CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2Number != null) {
      return customNested2Number(this);
    }
    return orElse();
  }
}

abstract class CustomNestedError2_CustomNested2Number implements CustomNestedError2 {
  const factory CustomNestedError2_CustomNested2Number(final int field0) = _$CustomNestedError2_CustomNested2NumberImpl;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError2_CustomNested2NumberImplCopyWith<_$CustomNestedError2_CustomNested2NumberImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Distance {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DistanceCopyWith<$Res> {
  factory $DistanceCopyWith(Distance value, $Res Function(Distance) then) = _$DistanceCopyWithImpl<$Res, Distance>;
}

/// @nodoc
class _$DistanceCopyWithImpl<$Res, $Val extends Distance> implements $DistanceCopyWith<$Res> {
  _$DistanceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Distance_UnknownImplCopyWith<$Res> {
  factory _$$Distance_UnknownImplCopyWith(_$Distance_UnknownImpl value, $Res Function(_$Distance_UnknownImpl) then) =
      __$$Distance_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Distance_UnknownImplCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res, _$Distance_UnknownImpl>
    implements _$$Distance_UnknownImplCopyWith<$Res> {
  __$$Distance_UnknownImplCopyWithImpl(_$Distance_UnknownImpl _value, $Res Function(_$Distance_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Distance_UnknownImpl implements Distance_Unknown {
  const _$Distance_UnknownImpl();

  @override
  String toString() {
    return 'Distance.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Distance_UnknownImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Distance_Unknown implements Distance {
  const factory Distance_Unknown() = _$Distance_UnknownImpl;
}

/// @nodoc
abstract class _$$Distance_MapImplCopyWith<$Res> {
  factory _$$Distance_MapImplCopyWith(_$Distance_MapImpl value, $Res Function(_$Distance_MapImpl) then) =
      __$$Distance_MapImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$Distance_MapImplCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res, _$Distance_MapImpl>
    implements _$$Distance_MapImplCopyWith<$Res> {
  __$$Distance_MapImplCopyWithImpl(_$Distance_MapImpl _value, $Res Function(_$Distance_MapImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Distance_MapImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Distance_MapImpl implements Distance_Map {
  const _$Distance_MapImpl(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'Distance.map(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Distance_MapImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Distance_MapImplCopyWith<_$Distance_MapImpl> get copyWith =>
      __$$Distance_MapImplCopyWithImpl<_$Distance_MapImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return map(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return map?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }
}

abstract class Distance_Map implements Distance {
  const factory Distance_Map(final double field0) = _$Distance_MapImpl;

  double get field0;
  @JsonKey(ignore: true)
  _$$Distance_MapImplCopyWith<_$Distance_MapImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumDartOpaque {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumDartOpaqueCopyWith<$Res> {
  factory $EnumDartOpaqueCopyWith(EnumDartOpaque value, $Res Function(EnumDartOpaque) then) =
      _$EnumDartOpaqueCopyWithImpl<$Res, EnumDartOpaque>;
}

/// @nodoc
class _$EnumDartOpaqueCopyWithImpl<$Res, $Val extends EnumDartOpaque> implements $EnumDartOpaqueCopyWith<$Res> {
  _$EnumDartOpaqueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumDartOpaque_PrimitiveImplCopyWith<$Res> {
  factory _$$EnumDartOpaque_PrimitiveImplCopyWith(
          _$EnumDartOpaque_PrimitiveImpl value, $Res Function(_$EnumDartOpaque_PrimitiveImpl) then) =
      __$$EnumDartOpaque_PrimitiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$EnumDartOpaque_PrimitiveImplCopyWithImpl<$Res>
    extends _$EnumDartOpaqueCopyWithImpl<$Res, _$EnumDartOpaque_PrimitiveImpl>
    implements _$$EnumDartOpaque_PrimitiveImplCopyWith<$Res> {
  __$$EnumDartOpaque_PrimitiveImplCopyWithImpl(
      _$EnumDartOpaque_PrimitiveImpl _value, $Res Function(_$EnumDartOpaque_PrimitiveImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumDartOpaque_PrimitiveImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$EnumDartOpaque_PrimitiveImpl implements EnumDartOpaque_Primitive {
  const _$EnumDartOpaque_PrimitiveImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'EnumDartOpaque.primitive(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumDartOpaque_PrimitiveImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumDartOpaque_PrimitiveImplCopyWith<_$EnumDartOpaque_PrimitiveImpl> get copyWith =>
      __$$EnumDartOpaque_PrimitiveImplCopyWithImpl<_$EnumDartOpaque_PrimitiveImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }
}

abstract class EnumDartOpaque_Primitive implements EnumDartOpaque {
  const factory EnumDartOpaque_Primitive(final int field0) = _$EnumDartOpaque_PrimitiveImpl;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$EnumDartOpaque_PrimitiveImplCopyWith<_$EnumDartOpaque_PrimitiveImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumDartOpaque_OpaqueImplCopyWith<$Res> {
  factory _$$EnumDartOpaque_OpaqueImplCopyWith(
          _$EnumDartOpaque_OpaqueImpl value, $Res Function(_$EnumDartOpaque_OpaqueImpl) then) =
      __$$EnumDartOpaque_OpaqueImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Object field0});
}

/// @nodoc
class __$$EnumDartOpaque_OpaqueImplCopyWithImpl<$Res>
    extends _$EnumDartOpaqueCopyWithImpl<$Res, _$EnumDartOpaque_OpaqueImpl>
    implements _$$EnumDartOpaque_OpaqueImplCopyWith<$Res> {
  __$$EnumDartOpaque_OpaqueImplCopyWithImpl(
      _$EnumDartOpaque_OpaqueImpl _value, $Res Function(_$EnumDartOpaque_OpaqueImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumDartOpaque_OpaqueImpl(
      null == field0 ? _value.field0 : field0,
    ));
  }
}

/// @nodoc

class _$EnumDartOpaque_OpaqueImpl implements EnumDartOpaque_Opaque {
  const _$EnumDartOpaque_OpaqueImpl(this.field0);

  @override
  final Object field0;

  @override
  String toString() {
    return 'EnumDartOpaque.opaque(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumDartOpaque_OpaqueImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumDartOpaque_OpaqueImplCopyWith<_$EnumDartOpaque_OpaqueImpl> get copyWith =>
      __$$EnumDartOpaque_OpaqueImplCopyWithImpl<_$EnumDartOpaque_OpaqueImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) {
    return opaque(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) {
    return opaque?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) {
    if (opaque != null) {
      return opaque(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) {
    return opaque(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) {
    return opaque?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    if (opaque != null) {
      return opaque(this);
    }
    return orElse();
  }
}

abstract class EnumDartOpaque_Opaque implements EnumDartOpaque {
  const factory EnumDartOpaque_Opaque(final Object field0) = _$EnumDartOpaque_OpaqueImpl;

  @override
  Object get field0;
  @JsonKey(ignore: true)
  _$$EnumDartOpaque_OpaqueImplCopyWith<_$EnumDartOpaque_OpaqueImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumOpaque {
  FrbOpaque get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumOpaqueCopyWith<$Res> {
  factory $EnumOpaqueCopyWith(EnumOpaque value, $Res Function(EnumOpaque) then) =
      _$EnumOpaqueCopyWithImpl<$Res, EnumOpaque>;
}

/// @nodoc
class _$EnumOpaqueCopyWithImpl<$Res, $Val extends EnumOpaque> implements $EnumOpaqueCopyWith<$Res> {
  _$EnumOpaqueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumOpaque_StructImplCopyWith<$Res> {
  factory _$$EnumOpaque_StructImplCopyWith(_$EnumOpaque_StructImpl value, $Res Function(_$EnumOpaque_StructImpl) then) =
      __$$EnumOpaque_StructImplCopyWithImpl<$Res>;
  @useResult
  $Res call({HideData field0});
}

/// @nodoc
class __$$EnumOpaque_StructImplCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_StructImpl>
    implements _$$EnumOpaque_StructImplCopyWith<$Res> {
  __$$EnumOpaque_StructImplCopyWithImpl(_$EnumOpaque_StructImpl _value, $Res Function(_$EnumOpaque_StructImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_StructImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_StructImpl implements EnumOpaque_Struct {
  const _$EnumOpaque_StructImpl(this.field0);

  @override
  final HideData field0;

  @override
  String toString() {
    return 'EnumOpaque.struct(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_StructImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_StructImplCopyWith<_$EnumOpaque_StructImpl> get copyWith =>
      __$$EnumOpaque_StructImplCopyWithImpl<_$EnumOpaque_StructImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return struct(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return struct?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return struct(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return struct?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Struct implements EnumOpaque {
  const factory EnumOpaque_Struct(final HideData field0) = _$EnumOpaque_StructImpl;

  @override
  HideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_StructImplCopyWith<_$EnumOpaque_StructImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_PrimitiveImplCopyWith<$Res> {
  factory _$$EnumOpaque_PrimitiveImplCopyWith(
          _$EnumOpaque_PrimitiveImpl value, $Res Function(_$EnumOpaque_PrimitiveImpl) then) =
      __$$EnumOpaque_PrimitiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class __$$EnumOpaque_PrimitiveImplCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_PrimitiveImpl>
    implements _$$EnumOpaque_PrimitiveImplCopyWith<$Res> {
  __$$EnumOpaque_PrimitiveImplCopyWithImpl(
      _$EnumOpaque_PrimitiveImpl _value, $Res Function(_$EnumOpaque_PrimitiveImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_PrimitiveImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_PrimitiveImpl implements EnumOpaque_Primitive {
  const _$EnumOpaque_PrimitiveImpl(this.field0);

  @override
  final I32 field0;

  @override
  String toString() {
    return 'EnumOpaque.primitive(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_PrimitiveImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_PrimitiveImplCopyWith<_$EnumOpaque_PrimitiveImpl> get copyWith =>
      __$$EnumOpaque_PrimitiveImplCopyWithImpl<_$EnumOpaque_PrimitiveImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Primitive implements EnumOpaque {
  const factory EnumOpaque_Primitive(final I32 field0) = _$EnumOpaque_PrimitiveImpl;

  @override
  I32 get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_PrimitiveImplCopyWith<_$EnumOpaque_PrimitiveImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_TraitObjImplCopyWith<$Res> {
  factory _$$EnumOpaque_TraitObjImplCopyWith(
          _$EnumOpaque_TraitObjImpl value, $Res Function(_$EnumOpaque_TraitObjImpl) then) =
      __$$EnumOpaque_TraitObjImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BoxDartDebug field0});
}

/// @nodoc
class __$$EnumOpaque_TraitObjImplCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_TraitObjImpl>
    implements _$$EnumOpaque_TraitObjImplCopyWith<$Res> {
  __$$EnumOpaque_TraitObjImplCopyWithImpl(
      _$EnumOpaque_TraitObjImpl _value, $Res Function(_$EnumOpaque_TraitObjImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_TraitObjImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebug,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_TraitObjImpl implements EnumOpaque_TraitObj {
  const _$EnumOpaque_TraitObjImpl(this.field0);

  @override
  final BoxDartDebug field0;

  @override
  String toString() {
    return 'EnumOpaque.traitObj(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_TraitObjImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_TraitObjImplCopyWith<_$EnumOpaque_TraitObjImpl> get copyWith =>
      __$$EnumOpaque_TraitObjImplCopyWithImpl<_$EnumOpaque_TraitObjImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return traitObj(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return traitObj?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return traitObj(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return traitObj?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_TraitObj implements EnumOpaque {
  const factory EnumOpaque_TraitObj(final BoxDartDebug field0) = _$EnumOpaque_TraitObjImpl;

  @override
  BoxDartDebug get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_TraitObjImplCopyWith<_$EnumOpaque_TraitObjImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_MutexImplCopyWith<$Res> {
  factory _$$EnumOpaque_MutexImplCopyWith(_$EnumOpaque_MutexImpl value, $Res Function(_$EnumOpaque_MutexImpl) then) =
      __$$EnumOpaque_MutexImplCopyWithImpl<$Res>;
  @useResult
  $Res call({MutexHideData field0});
}

/// @nodoc
class __$$EnumOpaque_MutexImplCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_MutexImpl>
    implements _$$EnumOpaque_MutexImplCopyWith<$Res> {
  __$$EnumOpaque_MutexImplCopyWithImpl(_$EnumOpaque_MutexImpl _value, $Res Function(_$EnumOpaque_MutexImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_MutexImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_MutexImpl implements EnumOpaque_Mutex {
  const _$EnumOpaque_MutexImpl(this.field0);

  @override
  final MutexHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.mutex(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_MutexImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_MutexImplCopyWith<_$EnumOpaque_MutexImpl> get copyWith =>
      __$$EnumOpaque_MutexImplCopyWithImpl<_$EnumOpaque_MutexImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return mutex(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return mutex?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return mutex(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return mutex?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Mutex implements EnumOpaque {
  const factory EnumOpaque_Mutex(final MutexHideData field0) = _$EnumOpaque_MutexImpl;

  @override
  MutexHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_MutexImplCopyWith<_$EnumOpaque_MutexImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_RwLockImplCopyWith<$Res> {
  factory _$$EnumOpaque_RwLockImplCopyWith(_$EnumOpaque_RwLockImpl value, $Res Function(_$EnumOpaque_RwLockImpl) then) =
      __$$EnumOpaque_RwLockImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RwLockHideData field0});
}

/// @nodoc
class __$$EnumOpaque_RwLockImplCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_RwLockImpl>
    implements _$$EnumOpaque_RwLockImplCopyWith<$Res> {
  __$$EnumOpaque_RwLockImplCopyWithImpl(_$EnumOpaque_RwLockImpl _value, $Res Function(_$EnumOpaque_RwLockImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_RwLockImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_RwLockImpl implements EnumOpaque_RwLock {
  const _$EnumOpaque_RwLockImpl(this.field0);

  @override
  final RwLockHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.rwLock(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_RwLockImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_RwLockImplCopyWith<_$EnumOpaque_RwLockImpl> get copyWith =>
      __$$EnumOpaque_RwLockImplCopyWithImpl<_$EnumOpaque_RwLockImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return rwLock(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return rwLock?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return rwLock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return rwLock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_RwLock implements EnumOpaque {
  const factory EnumOpaque_RwLock(final RwLockHideData field0) = _$EnumOpaque_RwLockImpl;

  @override
  RwLockHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_RwLockImplCopyWith<_$EnumOpaque_RwLockImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Event {
  FlutterRustBridgeExampleSingleBlockTest get bridge => throw _privateConstructorUsedError;
  String get address => throw _privateConstructorUsedError;
  String get payload => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $EventCopyWith<Event> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EventCopyWith<$Res> {
  factory $EventCopyWith(Event value, $Res Function(Event) then) = _$EventCopyWithImpl<$Res, Event>;
  @useResult
  $Res call({FlutterRustBridgeExampleSingleBlockTest bridge, String address, String payload});
}

/// @nodoc
class _$EventCopyWithImpl<$Res, $Val extends Event> implements $EventCopyWith<$Res> {
  _$EventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bridge = null,
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_value.copyWith(
      bridge: null == bridge
          ? _value.bridge
          : bridge // ignore: cast_nullable_to_non_nullable
              as FlutterRustBridgeExampleSingleBlockTest,
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$EventImplCopyWith<$Res> implements $EventCopyWith<$Res> {
  factory _$$EventImplCopyWith(_$EventImpl value, $Res Function(_$EventImpl) then) = __$$EventImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({FlutterRustBridgeExampleSingleBlockTest bridge, String address, String payload});
}

/// @nodoc
class __$$EventImplCopyWithImpl<$Res> extends _$EventCopyWithImpl<$Res, _$EventImpl>
    implements _$$EventImplCopyWith<$Res> {
  __$$EventImplCopyWithImpl(_$EventImpl _value, $Res Function(_$EventImpl) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bridge = null,
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_$EventImpl(
      bridge: null == bridge
          ? _value.bridge
          : bridge // ignore: cast_nullable_to_non_nullable
              as FlutterRustBridgeExampleSingleBlockTest,
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$EventImpl extends _Event {
  const _$EventImpl({required this.bridge, required this.address, required this.payload}) : super._();

  @override
  final FlutterRustBridgeExampleSingleBlockTest bridge;
  @override
  final String address;
  @override
  final String payload;

  @override
  String toString() {
    return 'Event(bridge: $bridge, address: $address, payload: $payload)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EventImpl &&
            (identical(other.bridge, bridge) || other.bridge == bridge) &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, bridge, address, payload);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EventImplCopyWith<_$EventImpl> get copyWith => __$$EventImplCopyWithImpl<_$EventImpl>(this, _$identity);
}

abstract class _Event extends Event {
  const factory _Event(
      {required final FlutterRustBridgeExampleSingleBlockTest bridge,
      required final String address,
      required final String payload}) = _$EventImpl;
  const _Event._() : super._();

  @override
  FlutterRustBridgeExampleSingleBlockTest get bridge;
  @override
  String get address;
  @override
  String get payload;
  @override
  @JsonKey(ignore: true)
  _$$EventImplCopyWith<_$EventImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkCopyWith<$Res> {
  factory $KitchenSinkCopyWith(KitchenSink value, $Res Function(KitchenSink) then) =
      _$KitchenSinkCopyWithImpl<$Res, KitchenSink>;
}

/// @nodoc
class _$KitchenSinkCopyWithImpl<$Res, $Val extends KitchenSink> implements $KitchenSinkCopyWith<$Res> {
  _$KitchenSinkCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$KitchenSink_EmptyImplCopyWith<$Res> {
  factory _$$KitchenSink_EmptyImplCopyWith(_$KitchenSink_EmptyImpl value, $Res Function(_$KitchenSink_EmptyImpl) then) =
      __$$KitchenSink_EmptyImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSink_EmptyImplCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_EmptyImpl>
    implements _$$KitchenSink_EmptyImplCopyWith<$Res> {
  __$$KitchenSink_EmptyImplCopyWithImpl(_$KitchenSink_EmptyImpl _value, $Res Function(_$KitchenSink_EmptyImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$KitchenSink_EmptyImpl implements KitchenSink_Empty {
  const _$KitchenSink_EmptyImpl();

  @override
  String toString() {
    return 'KitchenSink.empty()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$KitchenSink_EmptyImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Empty implements KitchenSink {
  const factory KitchenSink_Empty() = _$KitchenSink_EmptyImpl;
}

/// @nodoc
abstract class _$$KitchenSink_PrimitivesImplCopyWith<$Res> {
  factory _$$KitchenSink_PrimitivesImplCopyWith(
          _$KitchenSink_PrimitivesImpl value, $Res Function(_$KitchenSink_PrimitivesImpl) then) =
      __$$KitchenSink_PrimitivesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSink_PrimitivesImplCopyWithImpl<$Res>
    extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_PrimitivesImpl>
    implements _$$KitchenSink_PrimitivesImplCopyWith<$Res> {
  __$$KitchenSink_PrimitivesImplCopyWithImpl(
      _$KitchenSink_PrimitivesImpl _value, $Res Function(_$KitchenSink_PrimitivesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(_$KitchenSink_PrimitivesImpl(
      int32: null == int32
          ? _value.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: null == float64
          ? _value.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: null == boolean
          ? _value.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$KitchenSink_PrimitivesImpl implements KitchenSink_Primitives {
  const _$KitchenSink_PrimitivesImpl({this.int32 = -1, required this.float64, required this.boolean});

  /// Dart field comment
  @override
  @JsonKey()
  final int int32;
  @override
  final double float64;
  @override
  final bool boolean;

  @override
  String toString() {
    return 'KitchenSink.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_PrimitivesImpl &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_PrimitivesImplCopyWith<_$KitchenSink_PrimitivesImpl> get copyWith =>
      __$$KitchenSink_PrimitivesImplCopyWithImpl<_$KitchenSink_PrimitivesImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(int32, float64, boolean);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Primitives implements KitchenSink {
  const factory KitchenSink_Primitives({final int int32, required final double float64, required final bool boolean}) =
      _$KitchenSink_PrimitivesImpl;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$KitchenSink_PrimitivesImplCopyWith<_$KitchenSink_PrimitivesImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_NestedImplCopyWith<$Res> {
  factory _$$KitchenSink_NestedImplCopyWith(
          _$KitchenSink_NestedImpl value, $Res Function(_$KitchenSink_NestedImpl) then) =
      __$$KitchenSink_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, KitchenSink field1});

  $KitchenSinkCopyWith<$Res> get field1;
}

/// @nodoc
class __$$KitchenSink_NestedImplCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_NestedImpl>
    implements _$$KitchenSink_NestedImplCopyWith<$Res> {
  __$$KitchenSink_NestedImplCopyWithImpl(_$KitchenSink_NestedImpl _value, $Res Function(_$KitchenSink_NestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$KitchenSink_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSink,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkCopyWith<$Res> get field1 {
    return $KitchenSinkCopyWith<$Res>(_value.field1, (value) {
      return _then(_value.copyWith(field1: value));
    });
  }
}

/// @nodoc

class _$KitchenSink_NestedImpl implements KitchenSink_Nested {
  const _$KitchenSink_NestedImpl(this.field0, [this.field1 = const KitchenSink.empty()]);

  @override
  final int field0;
  @override
  @JsonKey()
  final KitchenSink field1;

  @override
  String toString() {
    return 'KitchenSink.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_NestedImplCopyWith<_$KitchenSink_NestedImpl> get copyWith =>
      __$$KitchenSink_NestedImplCopyWithImpl<_$KitchenSink_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Nested implements KitchenSink {
  const factory KitchenSink_Nested(final int field0, [final KitchenSink field1]) = _$KitchenSink_NestedImpl;

  int get field0;
  KitchenSink get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_NestedImplCopyWith<_$KitchenSink_NestedImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_OptionalImplCopyWith<$Res> {
  factory _$$KitchenSink_OptionalImplCopyWith(
          _$KitchenSink_OptionalImpl value, $Res Function(_$KitchenSink_OptionalImpl) then) =
      __$$KitchenSink_OptionalImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSink_OptionalImplCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_OptionalImpl>
    implements _$$KitchenSink_OptionalImplCopyWith<$Res> {
  __$$KitchenSink_OptionalImplCopyWithImpl(
      _$KitchenSink_OptionalImpl _value, $Res Function(_$KitchenSink_OptionalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_OptionalImpl(
      freezed == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      freezed == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$KitchenSink_OptionalImpl implements KitchenSink_Optional {
  const _$KitchenSink_OptionalImpl([this.field0 = -1, this.field1]);

  /// Comment on anonymous field
  @override
  @JsonKey()
  final int? field0;
  @override
  final int? field1;

  @override
  String toString() {
    return 'KitchenSink.optional(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_OptionalImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_OptionalImplCopyWith<_$KitchenSink_OptionalImpl> get copyWith =>
      __$$KitchenSink_OptionalImplCopyWithImpl<_$KitchenSink_OptionalImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Optional implements KitchenSink {
  const factory KitchenSink_Optional([final int? field0, final int? field1]) = _$KitchenSink_OptionalImpl;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_OptionalImplCopyWith<_$KitchenSink_OptionalImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_BufferImplCopyWith<$Res> {
  factory _$$KitchenSink_BufferImplCopyWith(
          _$KitchenSink_BufferImpl value, $Res Function(_$KitchenSink_BufferImpl) then) =
      __$$KitchenSink_BufferImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSink_BufferImplCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_BufferImpl>
    implements _$$KitchenSink_BufferImplCopyWith<$Res> {
  __$$KitchenSink_BufferImplCopyWithImpl(_$KitchenSink_BufferImpl _value, $Res Function(_$KitchenSink_BufferImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSink_BufferImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$KitchenSink_BufferImpl implements KitchenSink_Buffer {
  const _$KitchenSink_BufferImpl(this.field0);

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'KitchenSink.buffer(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_BufferImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_BufferImplCopyWith<_$KitchenSink_BufferImpl> get copyWith =>
      __$$KitchenSink_BufferImplCopyWithImpl<_$KitchenSink_BufferImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Buffer implements KitchenSink {
  const factory KitchenSink_Buffer(final Uint8List field0) = _$KitchenSink_BufferImpl;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_BufferImplCopyWith<_$KitchenSink_BufferImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_EnumsImplCopyWith<$Res> {
  factory _$$KitchenSink_EnumsImplCopyWith(_$KitchenSink_EnumsImpl value, $Res Function(_$KitchenSink_EnumsImpl) then) =
      __$$KitchenSink_EnumsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Weekdays field0});
}

/// @nodoc
class __$$KitchenSink_EnumsImplCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_EnumsImpl>
    implements _$$KitchenSink_EnumsImplCopyWith<$Res> {
  __$$KitchenSink_EnumsImplCopyWithImpl(_$KitchenSink_EnumsImpl _value, $Res Function(_$KitchenSink_EnumsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSink_EnumsImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$KitchenSink_EnumsImpl implements KitchenSink_Enums {
  const _$KitchenSink_EnumsImpl([this.field0 = Weekdays.sunday]);

  @override
  @JsonKey()
  final Weekdays field0;

  @override
  String toString() {
    return 'KitchenSink.enums(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_EnumsImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_EnumsImplCopyWith<_$KitchenSink_EnumsImpl> get copyWith =>
      __$$KitchenSink_EnumsImplCopyWithImpl<_$KitchenSink_EnumsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Enums implements KitchenSink {
  const factory KitchenSink_Enums([final Weekdays field0]) = _$KitchenSink_EnumsImpl;

  Weekdays get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_EnumsImplCopyWith<_$KitchenSink_EnumsImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Measure {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MeasureCopyWith<$Res> {
  factory $MeasureCopyWith(Measure value, $Res Function(Measure) then) = _$MeasureCopyWithImpl<$Res, Measure>;
}

/// @nodoc
class _$MeasureCopyWithImpl<$Res, $Val extends Measure> implements $MeasureCopyWith<$Res> {
  _$MeasureCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Measure_SpeedImplCopyWith<$Res> {
  factory _$$Measure_SpeedImplCopyWith(_$Measure_SpeedImpl value, $Res Function(_$Measure_SpeedImpl) then) =
      __$$Measure_SpeedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Speed field0});

  $SpeedCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_SpeedImplCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res, _$Measure_SpeedImpl>
    implements _$$Measure_SpeedImplCopyWith<$Res> {
  __$$Measure_SpeedImplCopyWithImpl(_$Measure_SpeedImpl _value, $Res Function(_$Measure_SpeedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Measure_SpeedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Speed,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SpeedCopyWith<$Res> get field0 {
    return $SpeedCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Measure_SpeedImpl implements Measure_Speed {
  const _$Measure_SpeedImpl(this.field0);

  @override
  final Speed field0;

  @override
  String toString() {
    return 'Measure.speed(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Measure_SpeedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Measure_SpeedImplCopyWith<_$Measure_SpeedImpl> get copyWith =>
      __$$Measure_SpeedImplCopyWithImpl<_$Measure_SpeedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) {
    return speed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
  }) {
    return speed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }
}

abstract class Measure_Speed implements Measure {
  const factory Measure_Speed(final Speed field0) = _$Measure_SpeedImpl;

  @override
  Speed get field0;
  @JsonKey(ignore: true)
  _$$Measure_SpeedImplCopyWith<_$Measure_SpeedImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Measure_DistanceImplCopyWith<$Res> {
  factory _$$Measure_DistanceImplCopyWith(_$Measure_DistanceImpl value, $Res Function(_$Measure_DistanceImpl) then) =
      __$$Measure_DistanceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Distance field0});

  $DistanceCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_DistanceImplCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res, _$Measure_DistanceImpl>
    implements _$$Measure_DistanceImplCopyWith<$Res> {
  __$$Measure_DistanceImplCopyWithImpl(_$Measure_DistanceImpl _value, $Res Function(_$Measure_DistanceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Measure_DistanceImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Distance,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DistanceCopyWith<$Res> get field0 {
    return $DistanceCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Measure_DistanceImpl implements Measure_Distance {
  const _$Measure_DistanceImpl(this.field0);

  @override
  final Distance field0;

  @override
  String toString() {
    return 'Measure.distance(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Measure_DistanceImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Measure_DistanceImplCopyWith<_$Measure_DistanceImpl> get copyWith =>
      __$$Measure_DistanceImplCopyWithImpl<_$Measure_DistanceImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) {
    return distance(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
  }) {
    return distance?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }
}

abstract class Measure_Distance implements Measure {
  const factory Measure_Distance(final Distance field0) = _$Measure_DistanceImpl;

  @override
  Distance get field0;
  @JsonKey(ignore: true)
  _$$Measure_DistanceImplCopyWith<_$Measure_DistanceImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$MyEnumFreezed {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) a,
    required TResult Function(String field0) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? a,
    TResult? Function(String field0)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? a,
    TResult Function(String field0)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumFreezed_A value) a,
    required TResult Function(MyEnumFreezed_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumFreezed_A value)? a,
    TResult? Function(MyEnumFreezed_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumFreezed_A value)? a,
    TResult Function(MyEnumFreezed_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyEnumFreezedCopyWith<$Res> {
  factory $MyEnumFreezedCopyWith(MyEnumFreezed value, $Res Function(MyEnumFreezed) then) =
      _$MyEnumFreezedCopyWithImpl<$Res, MyEnumFreezed>;
}

/// @nodoc
class _$MyEnumFreezedCopyWithImpl<$Res, $Val extends MyEnumFreezed> implements $MyEnumFreezedCopyWith<$Res> {
  _$MyEnumFreezedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$MyEnumFreezed_AImplCopyWith<$Res> {
  factory _$$MyEnumFreezed_AImplCopyWith(_$MyEnumFreezed_AImpl value, $Res Function(_$MyEnumFreezed_AImpl) then) =
      __$$MyEnumFreezed_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$MyEnumFreezed_AImplCopyWithImpl<$Res> extends _$MyEnumFreezedCopyWithImpl<$Res, _$MyEnumFreezed_AImpl>
    implements _$$MyEnumFreezed_AImplCopyWith<$Res> {
  __$$MyEnumFreezed_AImplCopyWithImpl(_$MyEnumFreezed_AImpl _value, $Res Function(_$MyEnumFreezed_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnumFreezed_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$MyEnumFreezed_AImpl implements MyEnumFreezed_A {
  const _$MyEnumFreezed_AImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'MyEnumFreezed.a(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumFreezed_AImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumFreezed_AImplCopyWith<_$MyEnumFreezed_AImpl> get copyWith =>
      __$$MyEnumFreezed_AImplCopyWithImpl<_$MyEnumFreezed_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) a,
    required TResult Function(String field0) b,
  }) {
    return a(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? a,
    TResult? Function(String field0)? b,
  }) {
    return a?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? a,
    TResult Function(String field0)? b,
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
    required TResult Function(MyEnumFreezed_A value) a,
    required TResult Function(MyEnumFreezed_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumFreezed_A value)? a,
    TResult? Function(MyEnumFreezed_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumFreezed_A value)? a,
    TResult Function(MyEnumFreezed_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class MyEnumFreezed_A implements MyEnumFreezed {
  const factory MyEnumFreezed_A(final int field0) = _$MyEnumFreezed_AImpl;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$MyEnumFreezed_AImplCopyWith<_$MyEnumFreezed_AImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MyEnumFreezed_BImplCopyWith<$Res> {
  factory _$$MyEnumFreezed_BImplCopyWith(_$MyEnumFreezed_BImpl value, $Res Function(_$MyEnumFreezed_BImpl) then) =
      __$$MyEnumFreezed_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$MyEnumFreezed_BImplCopyWithImpl<$Res> extends _$MyEnumFreezedCopyWithImpl<$Res, _$MyEnumFreezed_BImpl>
    implements _$$MyEnumFreezed_BImplCopyWith<$Res> {
  __$$MyEnumFreezed_BImplCopyWithImpl(_$MyEnumFreezed_BImpl _value, $Res Function(_$MyEnumFreezed_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnumFreezed_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$MyEnumFreezed_BImpl implements MyEnumFreezed_B {
  const _$MyEnumFreezed_BImpl(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'MyEnumFreezed.b(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumFreezed_BImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumFreezed_BImplCopyWith<_$MyEnumFreezed_BImpl> get copyWith =>
      __$$MyEnumFreezed_BImplCopyWithImpl<_$MyEnumFreezed_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) a,
    required TResult Function(String field0) b,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? a,
    TResult? Function(String field0)? b,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? a,
    TResult Function(String field0)? b,
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
    required TResult Function(MyEnumFreezed_A value) a,
    required TResult Function(MyEnumFreezed_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumFreezed_A value)? a,
    TResult? Function(MyEnumFreezed_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumFreezed_A value)? a,
    TResult Function(MyEnumFreezed_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class MyEnumFreezed_B implements MyEnumFreezed {
  const factory MyEnumFreezed_B(final String field0) = _$MyEnumFreezed_BImpl;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$MyEnumFreezed_BImplCopyWith<_$MyEnumFreezed_BImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$MySizeFreezed {
  int get width => throw _privateConstructorUsedError;
  int get height => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MySizeFreezedCopyWith<MySizeFreezed> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MySizeFreezedCopyWith<$Res> {
  factory $MySizeFreezedCopyWith(MySizeFreezed value, $Res Function(MySizeFreezed) then) =
      _$MySizeFreezedCopyWithImpl<$Res, MySizeFreezed>;
  @useResult
  $Res call({int width, int height});
}

/// @nodoc
class _$MySizeFreezedCopyWithImpl<$Res, $Val extends MySizeFreezed> implements $MySizeFreezedCopyWith<$Res> {
  _$MySizeFreezedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
  }) {
    return _then(_value.copyWith(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MySizeFreezedImplCopyWith<$Res> implements $MySizeFreezedCopyWith<$Res> {
  factory _$$MySizeFreezedImplCopyWith(_$MySizeFreezedImpl value, $Res Function(_$MySizeFreezedImpl) then) =
      __$$MySizeFreezedImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int width, int height});
}

/// @nodoc
class __$$MySizeFreezedImplCopyWithImpl<$Res> extends _$MySizeFreezedCopyWithImpl<$Res, _$MySizeFreezedImpl>
    implements _$$MySizeFreezedImplCopyWith<$Res> {
  __$$MySizeFreezedImplCopyWithImpl(_$MySizeFreezedImpl _value, $Res Function(_$MySizeFreezedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
  }) {
    return _then(_$MySizeFreezedImpl(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$MySizeFreezedImpl implements _MySizeFreezed {
  const _$MySizeFreezedImpl({required this.width, required this.height});

  @override
  final int width;
  @override
  final int height;

  @override
  String toString() {
    return 'MySizeFreezed(width: $width, height: $height)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MySizeFreezedImpl &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height));
  }

  @override
  int get hashCode => Object.hash(runtimeType, width, height);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MySizeFreezedImplCopyWith<_$MySizeFreezedImpl> get copyWith =>
      __$$MySizeFreezedImplCopyWithImpl<_$MySizeFreezedImpl>(this, _$identity);
}

abstract class _MySizeFreezed implements MySizeFreezed {
  const factory _MySizeFreezed({required final int width, required final int height}) = _$MySizeFreezedImpl;

  @override
  int get width;
  @override
  int get height;
  @override
  @JsonKey(ignore: true)
  _$$MySizeFreezedImplCopyWith<_$MySizeFreezedImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$RawStringEnumMirrored {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0) listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value) listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RawStringEnumMirroredCopyWith<$Res> {
  factory $RawStringEnumMirroredCopyWith(RawStringEnumMirrored value, $Res Function(RawStringEnumMirrored) then) =
      _$RawStringEnumMirroredCopyWithImpl<$Res, RawStringEnumMirrored>;
}

/// @nodoc
class _$RawStringEnumMirroredCopyWithImpl<$Res, $Val extends RawStringEnumMirrored>
    implements $RawStringEnumMirroredCopyWith<$Res> {
  _$RawStringEnumMirroredCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_RawImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_RawImplCopyWith(
          _$RawStringEnumMirrored_RawImpl value, $Res Function(_$RawStringEnumMirrored_RawImpl) then) =
      __$$RawStringEnumMirrored_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_RawImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res, _$RawStringEnumMirrored_RawImpl>
    implements _$$RawStringEnumMirrored_RawImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_RawImplCopyWithImpl(
      _$RawStringEnumMirrored_RawImpl _value, $Res Function(_$RawStringEnumMirrored_RawImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_RawImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_RawImpl implements RawStringEnumMirrored_Raw {
  const _$RawStringEnumMirrored_RawImpl(this.field0);

  @override
  final RawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.raw(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_RawImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_RawImplCopyWith<_$RawStringEnumMirrored_RawImpl> get copyWith =>
      __$$RawStringEnumMirrored_RawImplCopyWithImpl<_$RawStringEnumMirrored_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0) listOfNested,
  }) {
    return raw(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return raw?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value) listOfNested,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_Raw implements RawStringEnumMirrored {
  const factory RawStringEnumMirrored_Raw(final RawStringMirrored field0) = _$RawStringEnumMirrored_RawImpl;

  @override
  RawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_RawImplCopyWith<_$RawStringEnumMirrored_RawImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_NestedImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_NestedImplCopyWith(
          _$RawStringEnumMirrored_NestedImpl value, $Res Function(_$RawStringEnumMirrored_NestedImpl) then) =
      __$$RawStringEnumMirrored_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({NestedRawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_NestedImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res, _$RawStringEnumMirrored_NestedImpl>
    implements _$$RawStringEnumMirrored_NestedImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_NestedImplCopyWithImpl(
      _$RawStringEnumMirrored_NestedImpl _value, $Res Function(_$RawStringEnumMirrored_NestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NestedRawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_NestedImpl implements RawStringEnumMirrored_Nested {
  const _$RawStringEnumMirrored_NestedImpl(this.field0);

  @override
  final NestedRawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.nested(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_NestedImplCopyWith<_$RawStringEnumMirrored_NestedImpl> get copyWith =>
      __$$RawStringEnumMirrored_NestedImplCopyWithImpl<_$RawStringEnumMirrored_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0) listOfNested,
  }) {
    return nested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return nested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value) listOfNested,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_Nested implements RawStringEnumMirrored {
  const factory RawStringEnumMirrored_Nested(final NestedRawStringMirrored field0) = _$RawStringEnumMirrored_NestedImpl;

  @override
  NestedRawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_NestedImplCopyWith<_$RawStringEnumMirrored_NestedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_ListOfNestedImplCopyWith(_$RawStringEnumMirrored_ListOfNestedImpl value,
          $Res Function(_$RawStringEnumMirrored_ListOfNestedImpl) then) =
      __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ListOfNestedRawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res, _$RawStringEnumMirrored_ListOfNestedImpl>
    implements _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl(
      _$RawStringEnumMirrored_ListOfNestedImpl _value, $Res Function(_$RawStringEnumMirrored_ListOfNestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_ListOfNestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ListOfNestedRawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_ListOfNestedImpl implements RawStringEnumMirrored_ListOfNested {
  const _$RawStringEnumMirrored_ListOfNestedImpl(this.field0);

  @override
  final ListOfNestedRawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.listOfNested(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_ListOfNestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<_$RawStringEnumMirrored_ListOfNestedImpl> get copyWith =>
      __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<_$RawStringEnumMirrored_ListOfNestedImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0) listOfNested,
  }) {
    return listOfNested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return listOfNested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) {
    if (listOfNested != null) {
      return listOfNested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value) listOfNested,
  }) {
    return listOfNested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return listOfNested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (listOfNested != null) {
      return listOfNested(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_ListOfNested implements RawStringEnumMirrored {
  const factory RawStringEnumMirrored_ListOfNested(final ListOfNestedRawStringMirrored field0) =
      _$RawStringEnumMirrored_ListOfNestedImpl;

  @override
  ListOfNestedRawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<_$RawStringEnumMirrored_ListOfNestedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Speed {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpeedCopyWith<$Res> {
  factory $SpeedCopyWith(Speed value, $Res Function(Speed) then) = _$SpeedCopyWithImpl<$Res, Speed>;
}

/// @nodoc
class _$SpeedCopyWithImpl<$Res, $Val extends Speed> implements $SpeedCopyWith<$Res> {
  _$SpeedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Speed_UnknownImplCopyWith<$Res> {
  factory _$$Speed_UnknownImplCopyWith(_$Speed_UnknownImpl value, $Res Function(_$Speed_UnknownImpl) then) =
      __$$Speed_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Speed_UnknownImplCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res, _$Speed_UnknownImpl>
    implements _$$Speed_UnknownImplCopyWith<$Res> {
  __$$Speed_UnknownImplCopyWithImpl(_$Speed_UnknownImpl _value, $Res Function(_$Speed_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Speed_UnknownImpl implements Speed_Unknown {
  const _$Speed_UnknownImpl();

  @override
  String toString() {
    return 'Speed.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Speed_UnknownImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Speed_Unknown implements Speed {
  const factory Speed_Unknown() = _$Speed_UnknownImpl;
}

/// @nodoc
abstract class _$$Speed_GPSImplCopyWith<$Res> {
  factory _$$Speed_GPSImplCopyWith(_$Speed_GPSImpl value, $Res Function(_$Speed_GPSImpl) then) =
      __$$Speed_GPSImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$Speed_GPSImplCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res, _$Speed_GPSImpl>
    implements _$$Speed_GPSImplCopyWith<$Res> {
  __$$Speed_GPSImplCopyWithImpl(_$Speed_GPSImpl _value, $Res Function(_$Speed_GPSImpl) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Speed_GPSImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Speed_GPSImpl implements Speed_GPS {
  const _$Speed_GPSImpl(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'Speed.gps(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Speed_GPSImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Speed_GPSImplCopyWith<_$Speed_GPSImpl> get copyWith =>
      __$$Speed_GPSImplCopyWithImpl<_$Speed_GPSImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return gps(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return gps?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }
}

abstract class Speed_GPS implements Speed {
  const factory Speed_GPS(final double field0) = _$Speed_GPSImpl;

  double get field0;
  @JsonKey(ignore: true)
  _$$Speed_GPSImplCopyWith<_$Speed_GPSImpl> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$UserId {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdCopyWith<UserId> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdCopyWith<$Res> {
  factory $UserIdCopyWith(UserId value, $Res Function(UserId) then) = _$UserIdCopyWithImpl<$Res, UserId>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdCopyWithImpl<$Res, $Val extends UserId> implements $UserIdCopyWith<$Res> {
  _$UserIdCopyWithImpl(this._value, this._then);

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
abstract class _$$UserIdImplCopyWith<$Res> implements $UserIdCopyWith<$Res> {
  factory _$$UserIdImplCopyWith(_$UserIdImpl value, $Res Function(_$UserIdImpl) then) =
      __$$UserIdImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$UserIdImplCopyWithImpl<$Res> extends _$UserIdCopyWithImpl<$Res, _$UserIdImpl>
    implements _$$UserIdImplCopyWith<$Res> {
  __$$UserIdImplCopyWithImpl(_$UserIdImpl _value, $Res Function(_$UserIdImpl) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$UserIdImpl(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$UserIdImpl implements _UserId {
  const _$UserIdImpl({this.value = 0});

  @override
  @JsonKey()
  final int value;

  @override
  String toString() {
    return 'UserId(value: $value)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$UserIdImpl &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$UserIdImplCopyWith<_$UserIdImpl> get copyWith => __$$UserIdImplCopyWithImpl<_$UserIdImpl>(this, _$identity);
}

abstract class _UserId implements UserId {
  const factory _UserId({final int value}) = _$UserIdImpl;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$UserIdImplCopyWith<_$UserIdImpl> get copyWith => throw _privateConstructorUsedError;
}
