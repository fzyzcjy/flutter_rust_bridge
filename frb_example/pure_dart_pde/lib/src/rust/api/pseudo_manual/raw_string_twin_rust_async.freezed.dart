// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'raw_string_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$RawStringItemEnumTwinRustAsync {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String regular) regular,
    required TResult Function(String type) raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String regular)? regular,
    TResult? Function(String type)? raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String regular)? regular,
    TResult Function(String type)? raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringItemEnumTwinRustAsync_Regular value)
        regular,
    required TResult Function(RawStringItemEnumTwinRustAsync_Raw value) raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RawStringItemEnumTwinRustAsyncCopyWith<$Res> {
  factory $RawStringItemEnumTwinRustAsyncCopyWith(
          RawStringItemEnumTwinRustAsync value,
          $Res Function(RawStringItemEnumTwinRustAsync) then) =
      _$RawStringItemEnumTwinRustAsyncCopyWithImpl<$Res,
          RawStringItemEnumTwinRustAsync>;
}

/// @nodoc
class _$RawStringItemEnumTwinRustAsyncCopyWithImpl<$Res,
        $Val extends RawStringItemEnumTwinRustAsync>
    implements $RawStringItemEnumTwinRustAsyncCopyWith<$Res> {
  _$RawStringItemEnumTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$RawStringItemEnumTwinRustAsync_RegularImplCopyWith<$Res> {
  factory _$$RawStringItemEnumTwinRustAsync_RegularImplCopyWith(
          _$RawStringItemEnumTwinRustAsync_RegularImpl value,
          $Res Function(_$RawStringItemEnumTwinRustAsync_RegularImpl) then) =
      __$$RawStringItemEnumTwinRustAsync_RegularImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String regular});
}

/// @nodoc
class __$$RawStringItemEnumTwinRustAsync_RegularImplCopyWithImpl<$Res>
    extends _$RawStringItemEnumTwinRustAsyncCopyWithImpl<$Res,
        _$RawStringItemEnumTwinRustAsync_RegularImpl>
    implements _$$RawStringItemEnumTwinRustAsync_RegularImplCopyWith<$Res> {
  __$$RawStringItemEnumTwinRustAsync_RegularImplCopyWithImpl(
      _$RawStringItemEnumTwinRustAsync_RegularImpl _value,
      $Res Function(_$RawStringItemEnumTwinRustAsync_RegularImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? regular = null,
  }) {
    return _then(_$RawStringItemEnumTwinRustAsync_RegularImpl(
      regular: null == regular
          ? _value.regular
          : regular // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RawStringItemEnumTwinRustAsync_RegularImpl
    extends RawStringItemEnumTwinRustAsync_Regular {
  const _$RawStringItemEnumTwinRustAsync_RegularImpl({required this.regular})
      : super._();

  @override
  final String regular;

  @override
  String toString() {
    return 'RawStringItemEnumTwinRustAsync.regular(regular: $regular)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringItemEnumTwinRustAsync_RegularImpl &&
            (identical(other.regular, regular) || other.regular == regular));
  }

  @override
  int get hashCode => Object.hash(runtimeType, regular);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringItemEnumTwinRustAsync_RegularImplCopyWith<
          _$RawStringItemEnumTwinRustAsync_RegularImpl>
      get copyWith =>
          __$$RawStringItemEnumTwinRustAsync_RegularImplCopyWithImpl<
              _$RawStringItemEnumTwinRustAsync_RegularImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String regular) regular,
    required TResult Function(String type) raw,
  }) {
    return regular(this.regular);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String regular)? regular,
    TResult? Function(String type)? raw,
  }) {
    return regular?.call(this.regular);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String regular)? regular,
    TResult Function(String type)? raw,
    required TResult orElse(),
  }) {
    if (regular != null) {
      return regular(this.regular);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringItemEnumTwinRustAsync_Regular value)
        regular,
    required TResult Function(RawStringItemEnumTwinRustAsync_Raw value) raw,
  }) {
    return regular(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
  }) {
    return regular?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (regular != null) {
      return regular(this);
    }
    return orElse();
  }
}

abstract class RawStringItemEnumTwinRustAsync_Regular
    extends RawStringItemEnumTwinRustAsync {
  const factory RawStringItemEnumTwinRustAsync_Regular(
          {required final String regular}) =
      _$RawStringItemEnumTwinRustAsync_RegularImpl;
  const RawStringItemEnumTwinRustAsync_Regular._() : super._();

  String get regular;
  @JsonKey(ignore: true)
  _$$RawStringItemEnumTwinRustAsync_RegularImplCopyWith<
          _$RawStringItemEnumTwinRustAsync_RegularImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringItemEnumTwinRustAsync_RawImplCopyWith<$Res> {
  factory _$$RawStringItemEnumTwinRustAsync_RawImplCopyWith(
          _$RawStringItemEnumTwinRustAsync_RawImpl value,
          $Res Function(_$RawStringItemEnumTwinRustAsync_RawImpl) then) =
      __$$RawStringItemEnumTwinRustAsync_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String type});
}

/// @nodoc
class __$$RawStringItemEnumTwinRustAsync_RawImplCopyWithImpl<$Res>
    extends _$RawStringItemEnumTwinRustAsyncCopyWithImpl<$Res,
        _$RawStringItemEnumTwinRustAsync_RawImpl>
    implements _$$RawStringItemEnumTwinRustAsync_RawImplCopyWith<$Res> {
  __$$RawStringItemEnumTwinRustAsync_RawImplCopyWithImpl(
      _$RawStringItemEnumTwinRustAsync_RawImpl _value,
      $Res Function(_$RawStringItemEnumTwinRustAsync_RawImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? type = null,
  }) {
    return _then(_$RawStringItemEnumTwinRustAsync_RawImpl(
      type: null == type
          ? _value.type
          : type // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RawStringItemEnumTwinRustAsync_RawImpl
    extends RawStringItemEnumTwinRustAsync_Raw {
  const _$RawStringItemEnumTwinRustAsync_RawImpl({required this.type})
      : super._();

  @override
  final String type;

  @override
  String toString() {
    return 'RawStringItemEnumTwinRustAsync.raw(type: $type)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringItemEnumTwinRustAsync_RawImpl &&
            (identical(other.type, type) || other.type == type));
  }

  @override
  int get hashCode => Object.hash(runtimeType, type);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringItemEnumTwinRustAsync_RawImplCopyWith<
          _$RawStringItemEnumTwinRustAsync_RawImpl>
      get copyWith => __$$RawStringItemEnumTwinRustAsync_RawImplCopyWithImpl<
          _$RawStringItemEnumTwinRustAsync_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String regular) regular,
    required TResult Function(String type) raw,
  }) {
    return raw(type);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String regular)? regular,
    TResult? Function(String type)? raw,
  }) {
    return raw?.call(type);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String regular)? regular,
    TResult Function(String type)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(type);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringItemEnumTwinRustAsync_Regular value)
        regular,
    required TResult Function(RawStringItemEnumTwinRustAsync_Raw value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinRustAsync_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinRustAsync_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class RawStringItemEnumTwinRustAsync_Raw
    extends RawStringItemEnumTwinRustAsync {
  const factory RawStringItemEnumTwinRustAsync_Raw(
      {required final String type}) = _$RawStringItemEnumTwinRustAsync_RawImpl;
  const RawStringItemEnumTwinRustAsync_Raw._() : super._();

  String get type;
  @JsonKey(ignore: true)
  _$$RawStringItemEnumTwinRustAsync_RawImplCopyWith<
          _$RawStringItemEnumTwinRustAsync_RawImpl>
      get copyWith => throw _privateConstructorUsedError;
}
