// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'raw_string_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$RawStringItemEnumTwinSse {
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
    required TResult Function(RawStringItemEnumTwinSse_Regular value) regular,
    required TResult Function(RawStringItemEnumTwinSse_Raw value) raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSse_Raw value)? raw,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSse_Raw value)? raw,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RawStringItemEnumTwinSseCopyWith<$Res> {
  factory $RawStringItemEnumTwinSseCopyWith(RawStringItemEnumTwinSse value,
          $Res Function(RawStringItemEnumTwinSse) then) =
      _$RawStringItemEnumTwinSseCopyWithImpl<$Res, RawStringItemEnumTwinSse>;
}

/// @nodoc
class _$RawStringItemEnumTwinSseCopyWithImpl<$Res,
        $Val extends RawStringItemEnumTwinSse>
    implements $RawStringItemEnumTwinSseCopyWith<$Res> {
  _$RawStringItemEnumTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$RawStringItemEnumTwinSse_RegularImplCopyWith<$Res> {
  factory _$$RawStringItemEnumTwinSse_RegularImplCopyWith(
          _$RawStringItemEnumTwinSse_RegularImpl value,
          $Res Function(_$RawStringItemEnumTwinSse_RegularImpl) then) =
      __$$RawStringItemEnumTwinSse_RegularImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String regular});
}

/// @nodoc
class __$$RawStringItemEnumTwinSse_RegularImplCopyWithImpl<$Res>
    extends _$RawStringItemEnumTwinSseCopyWithImpl<$Res,
        _$RawStringItemEnumTwinSse_RegularImpl>
    implements _$$RawStringItemEnumTwinSse_RegularImplCopyWith<$Res> {
  __$$RawStringItemEnumTwinSse_RegularImplCopyWithImpl(
      _$RawStringItemEnumTwinSse_RegularImpl _value,
      $Res Function(_$RawStringItemEnumTwinSse_RegularImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? regular = null,
  }) {
    return _then(_$RawStringItemEnumTwinSse_RegularImpl(
      regular: null == regular
          ? _value.regular
          : regular // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RawStringItemEnumTwinSse_RegularImpl
    extends RawStringItemEnumTwinSse_Regular {
  const _$RawStringItemEnumTwinSse_RegularImpl({required this.regular})
      : super._();

  @override
  final String regular;

  @override
  String toString() {
    return 'RawStringItemEnumTwinSse.regular(regular: $regular)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringItemEnumTwinSse_RegularImpl &&
            (identical(other.regular, regular) || other.regular == regular));
  }

  @override
  int get hashCode => Object.hash(runtimeType, regular);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringItemEnumTwinSse_RegularImplCopyWith<
          _$RawStringItemEnumTwinSse_RegularImpl>
      get copyWith => __$$RawStringItemEnumTwinSse_RegularImplCopyWithImpl<
          _$RawStringItemEnumTwinSse_RegularImpl>(this, _$identity);

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
    required TResult Function(RawStringItemEnumTwinSse_Regular value) regular,
    required TResult Function(RawStringItemEnumTwinSse_Raw value) raw,
  }) {
    return regular(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSse_Raw value)? raw,
  }) {
    return regular?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSse_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (regular != null) {
      return regular(this);
    }
    return orElse();
  }
}

abstract class RawStringItemEnumTwinSse_Regular
    extends RawStringItemEnumTwinSse {
  const factory RawStringItemEnumTwinSse_Regular(
      {required final String regular}) = _$RawStringItemEnumTwinSse_RegularImpl;
  const RawStringItemEnumTwinSse_Regular._() : super._();

  String get regular;
  @JsonKey(ignore: true)
  _$$RawStringItemEnumTwinSse_RegularImplCopyWith<
          _$RawStringItemEnumTwinSse_RegularImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringItemEnumTwinSse_RawImplCopyWith<$Res> {
  factory _$$RawStringItemEnumTwinSse_RawImplCopyWith(
          _$RawStringItemEnumTwinSse_RawImpl value,
          $Res Function(_$RawStringItemEnumTwinSse_RawImpl) then) =
      __$$RawStringItemEnumTwinSse_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String type});
}

/// @nodoc
class __$$RawStringItemEnumTwinSse_RawImplCopyWithImpl<$Res>
    extends _$RawStringItemEnumTwinSseCopyWithImpl<$Res,
        _$RawStringItemEnumTwinSse_RawImpl>
    implements _$$RawStringItemEnumTwinSse_RawImplCopyWith<$Res> {
  __$$RawStringItemEnumTwinSse_RawImplCopyWithImpl(
      _$RawStringItemEnumTwinSse_RawImpl _value,
      $Res Function(_$RawStringItemEnumTwinSse_RawImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? type = null,
  }) {
    return _then(_$RawStringItemEnumTwinSse_RawImpl(
      type: null == type
          ? _value.type
          : type // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RawStringItemEnumTwinSse_RawImpl extends RawStringItemEnumTwinSse_Raw {
  const _$RawStringItemEnumTwinSse_RawImpl({required this.type}) : super._();

  @override
  final String type;

  @override
  String toString() {
    return 'RawStringItemEnumTwinSse.raw(type: $type)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringItemEnumTwinSse_RawImpl &&
            (identical(other.type, type) || other.type == type));
  }

  @override
  int get hashCode => Object.hash(runtimeType, type);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringItemEnumTwinSse_RawImplCopyWith<
          _$RawStringItemEnumTwinSse_RawImpl>
      get copyWith => __$$RawStringItemEnumTwinSse_RawImplCopyWithImpl<
          _$RawStringItemEnumTwinSse_RawImpl>(this, _$identity);

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
    required TResult Function(RawStringItemEnumTwinSse_Regular value) regular,
    required TResult Function(RawStringItemEnumTwinSse_Raw value) raw,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSse_Raw value)? raw,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSse_Raw value)? raw,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class RawStringItemEnumTwinSse_Raw extends RawStringItemEnumTwinSse {
  const factory RawStringItemEnumTwinSse_Raw({required final String type}) =
      _$RawStringItemEnumTwinSse_RawImpl;
  const RawStringItemEnumTwinSse_Raw._() : super._();

  String get type;
  @JsonKey(ignore: true)
  _$$RawStringItemEnumTwinSse_RawImplCopyWith<
          _$RawStringItemEnumTwinSse_RawImpl>
      get copyWith => throw _privateConstructorUsedError;
}
