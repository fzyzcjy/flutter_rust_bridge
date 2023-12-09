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
mixin _$Hello {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(int field0) orange,
    required TResult Function(int helloWorld, int anotherField) raspi,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(int field0)? orange,
    TResult? Function(int helloWorld, int anotherField)? raspi,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(int field0)? orange,
    TResult Function(int helloWorld, int anotherField)? raspi,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Hello_Apple value) apple,
    required TResult Function(Hello_Orange value) orange,
    required TResult Function(Hello_Raspi value) raspi,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Hello_Apple value)? apple,
    TResult? Function(Hello_Orange value)? orange,
    TResult? Function(Hello_Raspi value)? raspi,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Hello_Apple value)? apple,
    TResult Function(Hello_Orange value)? orange,
    TResult Function(Hello_Raspi value)? raspi,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $HelloCopyWith<$Res> {
  factory $HelloCopyWith(Hello value, $Res Function(Hello) then) =
      _$HelloCopyWithImpl<$Res, Hello>;
}

/// @nodoc
class _$HelloCopyWithImpl<$Res, $Val extends Hello>
    implements $HelloCopyWith<$Res> {
  _$HelloCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Hello_AppleImplCopyWith<$Res> {
  factory _$$Hello_AppleImplCopyWith(
          _$Hello_AppleImpl value, $Res Function(_$Hello_AppleImpl) then) =
      __$$Hello_AppleImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Hello_AppleImplCopyWithImpl<$Res>
    extends _$HelloCopyWithImpl<$Res, _$Hello_AppleImpl>
    implements _$$Hello_AppleImplCopyWith<$Res> {
  __$$Hello_AppleImplCopyWithImpl(
      _$Hello_AppleImpl _value, $Res Function(_$Hello_AppleImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Hello_AppleImpl implements Hello_Apple {
  const _$Hello_AppleImpl();

  @override
  String toString() {
    return 'Hello.apple()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Hello_AppleImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(int field0) orange,
    required TResult Function(int helloWorld, int anotherField) raspi,
  }) {
    return apple();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(int field0)? orange,
    TResult? Function(int helloWorld, int anotherField)? raspi,
  }) {
    return apple?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(int field0)? orange,
    TResult Function(int helloWorld, int anotherField)? raspi,
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
    required TResult Function(Hello_Apple value) apple,
    required TResult Function(Hello_Orange value) orange,
    required TResult Function(Hello_Raspi value) raspi,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Hello_Apple value)? apple,
    TResult? Function(Hello_Orange value)? orange,
    TResult? Function(Hello_Raspi value)? raspi,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Hello_Apple value)? apple,
    TResult Function(Hello_Orange value)? orange,
    TResult Function(Hello_Raspi value)? raspi,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }
}

abstract class Hello_Apple implements Hello {
  const factory Hello_Apple() = _$Hello_AppleImpl;
}

/// @nodoc
abstract class _$$Hello_OrangeImplCopyWith<$Res> {
  factory _$$Hello_OrangeImplCopyWith(
          _$Hello_OrangeImpl value, $Res Function(_$Hello_OrangeImpl) then) =
      __$$Hello_OrangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Hello_OrangeImplCopyWithImpl<$Res>
    extends _$HelloCopyWithImpl<$Res, _$Hello_OrangeImpl>
    implements _$$Hello_OrangeImplCopyWith<$Res> {
  __$$Hello_OrangeImplCopyWithImpl(
      _$Hello_OrangeImpl _value, $Res Function(_$Hello_OrangeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Hello_OrangeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Hello_OrangeImpl implements Hello_Orange {
  const _$Hello_OrangeImpl(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'Hello.orange(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Hello_OrangeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Hello_OrangeImplCopyWith<_$Hello_OrangeImpl> get copyWith =>
      __$$Hello_OrangeImplCopyWithImpl<_$Hello_OrangeImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(int field0) orange,
    required TResult Function(int helloWorld, int anotherField) raspi,
  }) {
    return orange(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(int field0)? orange,
    TResult? Function(int helloWorld, int anotherField)? raspi,
  }) {
    return orange?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(int field0)? orange,
    TResult Function(int helloWorld, int anotherField)? raspi,
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
    required TResult Function(Hello_Apple value) apple,
    required TResult Function(Hello_Orange value) orange,
    required TResult Function(Hello_Raspi value) raspi,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Hello_Apple value)? apple,
    TResult? Function(Hello_Orange value)? orange,
    TResult? Function(Hello_Raspi value)? raspi,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Hello_Apple value)? apple,
    TResult Function(Hello_Orange value)? orange,
    TResult Function(Hello_Raspi value)? raspi,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }
}

abstract class Hello_Orange implements Hello {
  const factory Hello_Orange(final int field0) = _$Hello_OrangeImpl;

  int get field0;
  @JsonKey(ignore: true)
  _$$Hello_OrangeImplCopyWith<_$Hello_OrangeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Hello_RaspiImplCopyWith<$Res> {
  factory _$$Hello_RaspiImplCopyWith(
          _$Hello_RaspiImpl value, $Res Function(_$Hello_RaspiImpl) then) =
      __$$Hello_RaspiImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int helloWorld, int anotherField});
}

/// @nodoc
class __$$Hello_RaspiImplCopyWithImpl<$Res>
    extends _$HelloCopyWithImpl<$Res, _$Hello_RaspiImpl>
    implements _$$Hello_RaspiImplCopyWith<$Res> {
  __$$Hello_RaspiImplCopyWithImpl(
      _$Hello_RaspiImpl _value, $Res Function(_$Hello_RaspiImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? helloWorld = null,
    Object? anotherField = null,
  }) {
    return _then(_$Hello_RaspiImpl(
      helloWorld: null == helloWorld
          ? _value.helloWorld
          : helloWorld // ignore: cast_nullable_to_non_nullable
              as int,
      anotherField: null == anotherField
          ? _value.anotherField
          : anotherField // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Hello_RaspiImpl implements Hello_Raspi {
  const _$Hello_RaspiImpl(
      {required this.helloWorld, required this.anotherField});

  @override
  final int helloWorld;
  @override
  final int anotherField;

  @override
  String toString() {
    return 'Hello.raspi(helloWorld: $helloWorld, anotherField: $anotherField)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Hello_RaspiImpl &&
            (identical(other.helloWorld, helloWorld) ||
                other.helloWorld == helloWorld) &&
            (identical(other.anotherField, anotherField) ||
                other.anotherField == anotherField));
  }

  @override
  int get hashCode => Object.hash(runtimeType, helloWorld, anotherField);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Hello_RaspiImplCopyWith<_$Hello_RaspiImpl> get copyWith =>
      __$$Hello_RaspiImplCopyWithImpl<_$Hello_RaspiImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() apple,
    required TResult Function(int field0) orange,
    required TResult Function(int helloWorld, int anotherField) raspi,
  }) {
    return raspi(helloWorld, anotherField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? apple,
    TResult? Function(int field0)? orange,
    TResult? Function(int helloWorld, int anotherField)? raspi,
  }) {
    return raspi?.call(helloWorld, anotherField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? apple,
    TResult Function(int field0)? orange,
    TResult Function(int helloWorld, int anotherField)? raspi,
    required TResult orElse(),
  }) {
    if (raspi != null) {
      return raspi(helloWorld, anotherField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Hello_Apple value) apple,
    required TResult Function(Hello_Orange value) orange,
    required TResult Function(Hello_Raspi value) raspi,
  }) {
    return raspi(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Hello_Apple value)? apple,
    TResult? Function(Hello_Orange value)? orange,
    TResult? Function(Hello_Raspi value)? raspi,
  }) {
    return raspi?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Hello_Apple value)? apple,
    TResult Function(Hello_Orange value)? orange,
    TResult Function(Hello_Raspi value)? raspi,
    required TResult orElse(),
  }) {
    if (raspi != null) {
      return raspi(this);
    }
    return orElse();
  }
}

abstract class Hello_Raspi implements Hello {
  const factory Hello_Raspi(
      {required final int helloWorld,
      required final int anotherField}) = _$Hello_RaspiImpl;

  int get helloWorld;
  int get anotherField;
  @JsonKey(ignore: true)
  _$$Hello_RaspiImplCopyWith<_$Hello_RaspiImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
