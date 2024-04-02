// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$BasicGeneralEnumTwinRustAsync {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field) apple,
    required TResult Function() orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field)? apple,
    TResult? Function()? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field)? apple,
    TResult Function()? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BasicGeneralEnumTwinRustAsync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)
        orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BasicGeneralEnumTwinRustAsyncCopyWith<$Res> {
  factory $BasicGeneralEnumTwinRustAsyncCopyWith(
          BasicGeneralEnumTwinRustAsync value,
          $Res Function(BasicGeneralEnumTwinRustAsync) then) =
      _$BasicGeneralEnumTwinRustAsyncCopyWithImpl<$Res,
          BasicGeneralEnumTwinRustAsync>;
}

/// @nodoc
class _$BasicGeneralEnumTwinRustAsyncCopyWithImpl<$Res,
        $Val extends BasicGeneralEnumTwinRustAsync>
    implements $BasicGeneralEnumTwinRustAsyncCopyWith<$Res> {
  _$BasicGeneralEnumTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWith<$Res> {
  factory _$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWith(
          _$BasicGeneralEnumTwinRustAsync_AppleImpl value,
          $Res Function(_$BasicGeneralEnumTwinRustAsync_AppleImpl) then) =
      __$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field});
}

/// @nodoc
class __$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWithImpl<$Res>
    extends _$BasicGeneralEnumTwinRustAsyncCopyWithImpl<$Res,
        _$BasicGeneralEnumTwinRustAsync_AppleImpl>
    implements _$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWith<$Res> {
  __$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWithImpl(
      _$BasicGeneralEnumTwinRustAsync_AppleImpl _value,
      $Res Function(_$BasicGeneralEnumTwinRustAsync_AppleImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field = null,
  }) {
    return _then(_$BasicGeneralEnumTwinRustAsync_AppleImpl(
      field: null == field
          ? _value.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$BasicGeneralEnumTwinRustAsync_AppleImpl
    extends BasicGeneralEnumTwinRustAsync_Apple {
  const _$BasicGeneralEnumTwinRustAsync_AppleImpl({required this.field})
      : super._();

  @override
  final String field;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsync.apple(field: $field)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BasicGeneralEnumTwinRustAsync_AppleImpl &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWith<
          _$BasicGeneralEnumTwinRustAsync_AppleImpl>
      get copyWith => __$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWithImpl<
          _$BasicGeneralEnumTwinRustAsync_AppleImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field) apple,
    required TResult Function() orange,
  }) {
    return apple(field);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field)? apple,
    TResult? Function()? orange,
  }) {
    return apple?.call(field);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field)? apple,
    TResult Function()? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(field);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BasicGeneralEnumTwinRustAsync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)
        orange,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }
}

abstract class BasicGeneralEnumTwinRustAsync_Apple
    extends BasicGeneralEnumTwinRustAsync {
  const factory BasicGeneralEnumTwinRustAsync_Apple(
          {required final String field}) =
      _$BasicGeneralEnumTwinRustAsync_AppleImpl;
  const BasicGeneralEnumTwinRustAsync_Apple._() : super._();

  String get field;
  @JsonKey(ignore: true)
  _$$BasicGeneralEnumTwinRustAsync_AppleImplCopyWith<
          _$BasicGeneralEnumTwinRustAsync_AppleImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWith<$Res> {
  factory _$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWith(
          _$BasicGeneralEnumTwinRustAsync_OrangeImpl value,
          $Res Function(_$BasicGeneralEnumTwinRustAsync_OrangeImpl) then) =
      __$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWithImpl<$Res>
    extends _$BasicGeneralEnumTwinRustAsyncCopyWithImpl<$Res,
        _$BasicGeneralEnumTwinRustAsync_OrangeImpl>
    implements _$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWith<$Res> {
  __$$BasicGeneralEnumTwinRustAsync_OrangeImplCopyWithImpl(
      _$BasicGeneralEnumTwinRustAsync_OrangeImpl _value,
      $Res Function(_$BasicGeneralEnumTwinRustAsync_OrangeImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BasicGeneralEnumTwinRustAsync_OrangeImpl
    extends BasicGeneralEnumTwinRustAsync_Orange {
  const _$BasicGeneralEnumTwinRustAsync_OrangeImpl() : super._();

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsync.orange()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BasicGeneralEnumTwinRustAsync_OrangeImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field) apple,
    required TResult Function() orange,
  }) {
    return orange();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field)? apple,
    TResult? Function()? orange,
  }) {
    return orange?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field)? apple,
    TResult Function()? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(BasicGeneralEnumTwinRustAsync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)
        orange,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }
}

abstract class BasicGeneralEnumTwinRustAsync_Orange
    extends BasicGeneralEnumTwinRustAsync {
  const factory BasicGeneralEnumTwinRustAsync_Orange() =
      _$BasicGeneralEnumTwinRustAsync_OrangeImpl;
  const BasicGeneralEnumTwinRustAsync_Orange._() : super._();
}
