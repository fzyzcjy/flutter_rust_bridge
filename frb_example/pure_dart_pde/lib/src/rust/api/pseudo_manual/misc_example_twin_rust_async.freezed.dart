// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

AbcTwinRustAsync _$AbcTwinRustAsyncFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return AbcTwinRustAsync_A.fromJson(json);
    case 'b':
      return AbcTwinRustAsync_B.fromJson(json);
    case 'c':
      return AbcTwinRustAsync_C.fromJson(json);
    case 'justInt':
      return AbcTwinRustAsync_JustInt.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'AbcTwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$AbcTwinRustAsync {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AbcTwinRustAsyncCopyWith<$Res> {
  factory $AbcTwinRustAsyncCopyWith(
          AbcTwinRustAsync value, $Res Function(AbcTwinRustAsync) then) =
      _$AbcTwinRustAsyncCopyWithImpl<$Res, AbcTwinRustAsync>;
}

/// @nodoc
class _$AbcTwinRustAsyncCopyWithImpl<$Res, $Val extends AbcTwinRustAsync>
    implements $AbcTwinRustAsyncCopyWith<$Res> {
  _$AbcTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$AbcTwinRustAsync_AImplCopyWith<$Res> {
  factory _$$AbcTwinRustAsync_AImplCopyWith(_$AbcTwinRustAsync_AImpl value,
          $Res Function(_$AbcTwinRustAsync_AImpl) then) =
      __$$AbcTwinRustAsync_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ATwinRustAsync field0});
}

/// @nodoc
class __$$AbcTwinRustAsync_AImplCopyWithImpl<$Res>
    extends _$AbcTwinRustAsyncCopyWithImpl<$Res, _$AbcTwinRustAsync_AImpl>
    implements _$$AbcTwinRustAsync_AImplCopyWith<$Res> {
  __$$AbcTwinRustAsync_AImplCopyWithImpl(_$AbcTwinRustAsync_AImpl _value,
      $Res Function(_$AbcTwinRustAsync_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AbcTwinRustAsync_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinRustAsync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AbcTwinRustAsync_AImpl extends AbcTwinRustAsync_A {
  const _$AbcTwinRustAsync_AImpl(this.field0, {final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$AbcTwinRustAsync_AImpl.fromJson(Map<String, dynamic> json) =>
      _$$AbcTwinRustAsync_AImplFromJson(json);

  @override
  final ATwinRustAsync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'AbcTwinRustAsync.a(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AbcTwinRustAsync_AImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AbcTwinRustAsync_AImplCopyWith<_$AbcTwinRustAsync_AImpl> get copyWith =>
      __$$AbcTwinRustAsync_AImplCopyWithImpl<_$AbcTwinRustAsync_AImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return a(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return a?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
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
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$AbcTwinRustAsync_AImplToJson(
      this,
    );
  }
}

abstract class AbcTwinRustAsync_A extends AbcTwinRustAsync {
  const factory AbcTwinRustAsync_A(final ATwinRustAsync field0) =
      _$AbcTwinRustAsync_AImpl;
  const AbcTwinRustAsync_A._() : super._();

  factory AbcTwinRustAsync_A.fromJson(Map<String, dynamic> json) =
      _$AbcTwinRustAsync_AImpl.fromJson;

  @override
  ATwinRustAsync get field0;
  @JsonKey(ignore: true)
  _$$AbcTwinRustAsync_AImplCopyWith<_$AbcTwinRustAsync_AImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AbcTwinRustAsync_BImplCopyWith<$Res> {
  factory _$$AbcTwinRustAsync_BImplCopyWith(_$AbcTwinRustAsync_BImpl value,
          $Res Function(_$AbcTwinRustAsync_BImpl) then) =
      __$$AbcTwinRustAsync_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BTwinRustAsync field0});
}

/// @nodoc
class __$$AbcTwinRustAsync_BImplCopyWithImpl<$Res>
    extends _$AbcTwinRustAsyncCopyWithImpl<$Res, _$AbcTwinRustAsync_BImpl>
    implements _$$AbcTwinRustAsync_BImplCopyWith<$Res> {
  __$$AbcTwinRustAsync_BImplCopyWithImpl(_$AbcTwinRustAsync_BImpl _value,
      $Res Function(_$AbcTwinRustAsync_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AbcTwinRustAsync_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinRustAsync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AbcTwinRustAsync_BImpl extends AbcTwinRustAsync_B {
  const _$AbcTwinRustAsync_BImpl(this.field0, {final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$AbcTwinRustAsync_BImpl.fromJson(Map<String, dynamic> json) =>
      _$$AbcTwinRustAsync_BImplFromJson(json);

  @override
  final BTwinRustAsync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'AbcTwinRustAsync.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AbcTwinRustAsync_BImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AbcTwinRustAsync_BImplCopyWith<_$AbcTwinRustAsync_BImpl> get copyWith =>
      __$$AbcTwinRustAsync_BImplCopyWithImpl<_$AbcTwinRustAsync_BImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
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
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$AbcTwinRustAsync_BImplToJson(
      this,
    );
  }
}

abstract class AbcTwinRustAsync_B extends AbcTwinRustAsync {
  const factory AbcTwinRustAsync_B(final BTwinRustAsync field0) =
      _$AbcTwinRustAsync_BImpl;
  const AbcTwinRustAsync_B._() : super._();

  factory AbcTwinRustAsync_B.fromJson(Map<String, dynamic> json) =
      _$AbcTwinRustAsync_BImpl.fromJson;

  @override
  BTwinRustAsync get field0;
  @JsonKey(ignore: true)
  _$$AbcTwinRustAsync_BImplCopyWith<_$AbcTwinRustAsync_BImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AbcTwinRustAsync_CImplCopyWith<$Res> {
  factory _$$AbcTwinRustAsync_CImplCopyWith(_$AbcTwinRustAsync_CImpl value,
          $Res Function(_$AbcTwinRustAsync_CImpl) then) =
      __$$AbcTwinRustAsync_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({CTwinRustAsync field0});
}

/// @nodoc
class __$$AbcTwinRustAsync_CImplCopyWithImpl<$Res>
    extends _$AbcTwinRustAsyncCopyWithImpl<$Res, _$AbcTwinRustAsync_CImpl>
    implements _$$AbcTwinRustAsync_CImplCopyWith<$Res> {
  __$$AbcTwinRustAsync_CImplCopyWithImpl(_$AbcTwinRustAsync_CImpl _value,
      $Res Function(_$AbcTwinRustAsync_CImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AbcTwinRustAsync_CImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinRustAsync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AbcTwinRustAsync_CImpl extends AbcTwinRustAsync_C {
  const _$AbcTwinRustAsync_CImpl(this.field0, {final String? $type})
      : $type = $type ?? 'c',
        super._();

  factory _$AbcTwinRustAsync_CImpl.fromJson(Map<String, dynamic> json) =>
      _$$AbcTwinRustAsync_CImplFromJson(json);

  @override
  final CTwinRustAsync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'AbcTwinRustAsync.c(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AbcTwinRustAsync_CImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AbcTwinRustAsync_CImplCopyWith<_$AbcTwinRustAsync_CImpl> get copyWith =>
      __$$AbcTwinRustAsync_CImplCopyWithImpl<_$AbcTwinRustAsync_CImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return c(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return c?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
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
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) {
    return c(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) {
    return c?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$AbcTwinRustAsync_CImplToJson(
      this,
    );
  }
}

abstract class AbcTwinRustAsync_C extends AbcTwinRustAsync {
  const factory AbcTwinRustAsync_C(final CTwinRustAsync field0) =
      _$AbcTwinRustAsync_CImpl;
  const AbcTwinRustAsync_C._() : super._();

  factory AbcTwinRustAsync_C.fromJson(Map<String, dynamic> json) =
      _$AbcTwinRustAsync_CImpl.fromJson;

  @override
  CTwinRustAsync get field0;
  @JsonKey(ignore: true)
  _$$AbcTwinRustAsync_CImplCopyWith<_$AbcTwinRustAsync_CImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AbcTwinRustAsync_JustIntImplCopyWith<$Res> {
  factory _$$AbcTwinRustAsync_JustIntImplCopyWith(
          _$AbcTwinRustAsync_JustIntImpl value,
          $Res Function(_$AbcTwinRustAsync_JustIntImpl) then) =
      __$$AbcTwinRustAsync_JustIntImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$AbcTwinRustAsync_JustIntImplCopyWithImpl<$Res>
    extends _$AbcTwinRustAsyncCopyWithImpl<$Res, _$AbcTwinRustAsync_JustIntImpl>
    implements _$$AbcTwinRustAsync_JustIntImplCopyWith<$Res> {
  __$$AbcTwinRustAsync_JustIntImplCopyWithImpl(
      _$AbcTwinRustAsync_JustIntImpl _value,
      $Res Function(_$AbcTwinRustAsync_JustIntImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AbcTwinRustAsync_JustIntImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AbcTwinRustAsync_JustIntImpl extends AbcTwinRustAsync_JustInt {
  const _$AbcTwinRustAsync_JustIntImpl(this.field0, {final String? $type})
      : $type = $type ?? 'justInt',
        super._();

  factory _$AbcTwinRustAsync_JustIntImpl.fromJson(Map<String, dynamic> json) =>
      _$$AbcTwinRustAsync_JustIntImplFromJson(json);

  @override
  final int field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'AbcTwinRustAsync.justInt(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AbcTwinRustAsync_JustIntImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AbcTwinRustAsync_JustIntImplCopyWith<_$AbcTwinRustAsync_JustIntImpl>
      get copyWith => __$$AbcTwinRustAsync_JustIntImplCopyWithImpl<
          _$AbcTwinRustAsync_JustIntImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    return justInt(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    return justInt?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
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
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) {
    return justInt(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) {
    return justInt?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    if (justInt != null) {
      return justInt(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$AbcTwinRustAsync_JustIntImplToJson(
      this,
    );
  }
}

abstract class AbcTwinRustAsync_JustInt extends AbcTwinRustAsync {
  const factory AbcTwinRustAsync_JustInt(final int field0) =
      _$AbcTwinRustAsync_JustIntImpl;
  const AbcTwinRustAsync_JustInt._() : super._();

  factory AbcTwinRustAsync_JustInt.fromJson(Map<String, dynamic> json) =
      _$AbcTwinRustAsync_JustIntImpl.fromJson;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$AbcTwinRustAsync_JustIntImplCopyWith<_$AbcTwinRustAsync_JustIntImpl>
      get copyWith => throw _privateConstructorUsedError;
}
