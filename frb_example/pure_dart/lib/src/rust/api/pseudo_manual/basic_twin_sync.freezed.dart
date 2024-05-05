// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$BasicGeneralEnumTwinSync {
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
    required TResult Function(BasicGeneralEnumTwinSync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinSync_Orange value) orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinSync_Orange value)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinSync_Orange value)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BasicGeneralEnumTwinSyncCopyWith<$Res> {
  factory $BasicGeneralEnumTwinSyncCopyWith(BasicGeneralEnumTwinSync value,
          $Res Function(BasicGeneralEnumTwinSync) then) =
      _$BasicGeneralEnumTwinSyncCopyWithImpl<$Res, BasicGeneralEnumTwinSync>;
}

/// @nodoc
class _$BasicGeneralEnumTwinSyncCopyWithImpl<$Res,
        $Val extends BasicGeneralEnumTwinSync>
    implements $BasicGeneralEnumTwinSyncCopyWith<$Res> {
  _$BasicGeneralEnumTwinSyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$BasicGeneralEnumTwinSync_AppleImplCopyWith<$Res> {
  factory _$$BasicGeneralEnumTwinSync_AppleImplCopyWith(
          _$BasicGeneralEnumTwinSync_AppleImpl value,
          $Res Function(_$BasicGeneralEnumTwinSync_AppleImpl) then) =
      __$$BasicGeneralEnumTwinSync_AppleImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field});
}

/// @nodoc
class __$$BasicGeneralEnumTwinSync_AppleImplCopyWithImpl<$Res>
    extends _$BasicGeneralEnumTwinSyncCopyWithImpl<$Res,
        _$BasicGeneralEnumTwinSync_AppleImpl>
    implements _$$BasicGeneralEnumTwinSync_AppleImplCopyWith<$Res> {
  __$$BasicGeneralEnumTwinSync_AppleImplCopyWithImpl(
      _$BasicGeneralEnumTwinSync_AppleImpl _value,
      $Res Function(_$BasicGeneralEnumTwinSync_AppleImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field = null,
  }) {
    return _then(_$BasicGeneralEnumTwinSync_AppleImpl(
      field: null == field
          ? _value.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$BasicGeneralEnumTwinSync_AppleImpl
    extends BasicGeneralEnumTwinSync_Apple {
  const _$BasicGeneralEnumTwinSync_AppleImpl({required this.field}) : super._();

  @override
  final String field;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSync.apple(field: $field)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BasicGeneralEnumTwinSync_AppleImpl &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$BasicGeneralEnumTwinSync_AppleImplCopyWith<
          _$BasicGeneralEnumTwinSync_AppleImpl>
      get copyWith => __$$BasicGeneralEnumTwinSync_AppleImplCopyWithImpl<
          _$BasicGeneralEnumTwinSync_AppleImpl>(this, _$identity);

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
    required TResult Function(BasicGeneralEnumTwinSync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinSync_Orange value) orange,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinSync_Orange value)? orange,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinSync_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }
}

abstract class BasicGeneralEnumTwinSync_Apple extends BasicGeneralEnumTwinSync {
  const factory BasicGeneralEnumTwinSync_Apple({required final String field}) =
      _$BasicGeneralEnumTwinSync_AppleImpl;
  const BasicGeneralEnumTwinSync_Apple._() : super._();

  String get field;
  @JsonKey(ignore: true)
  _$$BasicGeneralEnumTwinSync_AppleImplCopyWith<
          _$BasicGeneralEnumTwinSync_AppleImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BasicGeneralEnumTwinSync_OrangeImplCopyWith<$Res> {
  factory _$$BasicGeneralEnumTwinSync_OrangeImplCopyWith(
          _$BasicGeneralEnumTwinSync_OrangeImpl value,
          $Res Function(_$BasicGeneralEnumTwinSync_OrangeImpl) then) =
      __$$BasicGeneralEnumTwinSync_OrangeImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$BasicGeneralEnumTwinSync_OrangeImplCopyWithImpl<$Res>
    extends _$BasicGeneralEnumTwinSyncCopyWithImpl<$Res,
        _$BasicGeneralEnumTwinSync_OrangeImpl>
    implements _$$BasicGeneralEnumTwinSync_OrangeImplCopyWith<$Res> {
  __$$BasicGeneralEnumTwinSync_OrangeImplCopyWithImpl(
      _$BasicGeneralEnumTwinSync_OrangeImpl _value,
      $Res Function(_$BasicGeneralEnumTwinSync_OrangeImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$BasicGeneralEnumTwinSync_OrangeImpl
    extends BasicGeneralEnumTwinSync_Orange {
  const _$BasicGeneralEnumTwinSync_OrangeImpl() : super._();

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSync.orange()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$BasicGeneralEnumTwinSync_OrangeImpl);
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
    required TResult Function(BasicGeneralEnumTwinSync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinSync_Orange value) orange,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinSync_Orange value)? orange,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(BasicGeneralEnumTwinSync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinSync_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }
}

abstract class BasicGeneralEnumTwinSync_Orange
    extends BasicGeneralEnumTwinSync {
  const factory BasicGeneralEnumTwinSync_Orange() =
      _$BasicGeneralEnumTwinSync_OrangeImpl;
  const BasicGeneralEnumTwinSync_Orange._() : super._();
}
