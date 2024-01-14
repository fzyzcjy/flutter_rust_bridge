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
    required TResult Function() apple,
    required TResult Function(Uint8List field0) orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(Uint8List field0)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(Uint8List field0)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_Apple value) apple,
    required TResult Function(MyEnum_Orange value) orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_Apple value)? apple,
    TResult? Function(MyEnum_Orange value)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_Apple value)? apple,
    TResult Function(MyEnum_Orange value)? orange,
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
abstract class _$$MyEnum_AppleImplCopyWith<$Res> {
  factory _$$MyEnum_AppleImplCopyWith(
          _$MyEnum_AppleImpl value, $Res Function(_$MyEnum_AppleImpl) then) =
      __$$MyEnum_AppleImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$MyEnum_AppleImplCopyWithImpl<$Res>
    extends _$MyEnumCopyWithImpl<$Res, _$MyEnum_AppleImpl>
    implements _$$MyEnum_AppleImplCopyWith<$Res> {
  __$$MyEnum_AppleImplCopyWithImpl(
      _$MyEnum_AppleImpl _value, $Res Function(_$MyEnum_AppleImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$MyEnum_AppleImpl implements MyEnum_Apple {
  const _$MyEnum_AppleImpl();

  @override
  String toString() {
    return 'MyEnum.apple()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$MyEnum_AppleImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(Uint8List field0) orange,
  }) {
    return apple();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(Uint8List field0)? orange,
  }) {
    return apple?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(Uint8List field0)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_Apple value) apple,
    required TResult Function(MyEnum_Orange value) orange,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_Apple value)? apple,
    TResult? Function(MyEnum_Orange value)? orange,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_Apple value)? apple,
    TResult Function(MyEnum_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }
}

abstract class MyEnum_Apple implements MyEnum {
  const factory MyEnum_Apple() = _$MyEnum_AppleImpl;
}

/// @nodoc
abstract class _$$MyEnum_OrangeImplCopyWith<$Res> {
  factory _$$MyEnum_OrangeImplCopyWith(
          _$MyEnum_OrangeImpl value, $Res Function(_$MyEnum_OrangeImpl) then) =
      __$$MyEnum_OrangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$MyEnum_OrangeImplCopyWithImpl<$Res>
    extends _$MyEnumCopyWithImpl<$Res, _$MyEnum_OrangeImpl>
    implements _$$MyEnum_OrangeImplCopyWith<$Res> {
  __$$MyEnum_OrangeImplCopyWithImpl(
      _$MyEnum_OrangeImpl _value, $Res Function(_$MyEnum_OrangeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnum_OrangeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$MyEnum_OrangeImpl implements MyEnum_Orange {
  const _$MyEnum_OrangeImpl(this.field0);

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'MyEnum.orange(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnum_OrangeImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnum_OrangeImplCopyWith<_$MyEnum_OrangeImpl> get copyWith =>
      __$$MyEnum_OrangeImplCopyWithImpl<_$MyEnum_OrangeImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(Uint8List field0) orange,
  }) {
    return orange(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(Uint8List field0)? orange,
  }) {
    return orange?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(Uint8List field0)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnum_Apple value) apple,
    required TResult Function(MyEnum_Orange value) orange,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnum_Apple value)? apple,
    TResult? Function(MyEnum_Orange value)? orange,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnum_Apple value)? apple,
    TResult Function(MyEnum_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }
}

abstract class MyEnum_Orange implements MyEnum {
  const factory MyEnum_Orange(final Uint8List field0) = _$MyEnum_OrangeImpl;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$MyEnum_OrangeImplCopyWith<_$MyEnum_OrangeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
