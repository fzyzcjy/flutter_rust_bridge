// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target

part of 'bridge_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more informations: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
class _$FoobarTearOff {
  const _$FoobarTearOff();

  Foo foo() {
    return const Foo();
  }

  Bar bar(String field0) {
    return Bar(
      field0,
    );
  }

  Baz baz({required String name}) {
    return Baz(
      name: name,
    );
  }
}

/// @nodoc
const $Foobar = _$FoobarTearOff();

/// @nodoc
mixin _$Foobar {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() foo,
    required TResult Function(String field0) bar,
    required TResult Function(String name) baz,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Foo value) foo,
    required TResult Function(Bar value) bar,
    required TResult Function(Baz value) baz,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FoobarCopyWith<$Res> {
  factory $FoobarCopyWith(Foobar value, $Res Function(Foobar) then) =
      _$FoobarCopyWithImpl<$Res>;
}

/// @nodoc
class _$FoobarCopyWithImpl<$Res> implements $FoobarCopyWith<$Res> {
  _$FoobarCopyWithImpl(this._value, this._then);

  final Foobar _value;
  // ignore: unused_field
  final $Res Function(Foobar) _then;
}

/// @nodoc
abstract class $FooCopyWith<$Res> {
  factory $FooCopyWith(Foo value, $Res Function(Foo) then) =
      _$FooCopyWithImpl<$Res>;
}

/// @nodoc
class _$FooCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res>
    implements $FooCopyWith<$Res> {
  _$FooCopyWithImpl(Foo _value, $Res Function(Foo) _then)
      : super(_value, (v) => _then(v as Foo));

  @override
  Foo get _value => super._value as Foo;
}

/// @nodoc

class _$Foo implements Foo {
  const _$Foo();

  @override
  String toString() {
    return 'Foobar.foo()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is Foo);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() foo,
    required TResult Function(String field0) bar,
    required TResult Function(String name) baz,
  }) {
    return foo();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
  }) {
    return foo?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
    required TResult orElse(),
  }) {
    if (foo != null) {
      return foo();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Foo value) foo,
    required TResult Function(Bar value) bar,
    required TResult Function(Baz value) baz,
  }) {
    return foo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
  }) {
    return foo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
    required TResult orElse(),
  }) {
    if (foo != null) {
      return foo(this);
    }
    return orElse();
  }
}

abstract class Foo implements Foobar {
  const factory Foo() = _$Foo;
}

/// @nodoc
abstract class $BarCopyWith<$Res> {
  factory $BarCopyWith(Bar value, $Res Function(Bar) then) =
      _$BarCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class _$BarCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res>
    implements $BarCopyWith<$Res> {
  _$BarCopyWithImpl(Bar _value, $Res Function(Bar) _then)
      : super(_value, (v) => _then(v as Bar));

  @override
  Bar get _value => super._value as Bar;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Bar(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Bar implements Bar {
  const _$Bar(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'Foobar.bar(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Bar &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $BarCopyWith<Bar> get copyWith => _$BarCopyWithImpl<Bar>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() foo,
    required TResult Function(String field0) bar,
    required TResult Function(String name) baz,
  }) {
    return bar(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
  }) {
    return bar?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
    required TResult orElse(),
  }) {
    if (bar != null) {
      return bar(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Foo value) foo,
    required TResult Function(Bar value) bar,
    required TResult Function(Baz value) baz,
  }) {
    return bar(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
  }) {
    return bar?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
    required TResult orElse(),
  }) {
    if (bar != null) {
      return bar(this);
    }
    return orElse();
  }
}

abstract class Bar implements Foobar {
  const factory Bar(String field0) = _$Bar;

  String get field0;
  @JsonKey(ignore: true)
  $BarCopyWith<Bar> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BazCopyWith<$Res> {
  factory $BazCopyWith(Baz value, $Res Function(Baz) then) =
      _$BazCopyWithImpl<$Res>;
  $Res call({String name});
}

/// @nodoc
class _$BazCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res>
    implements $BazCopyWith<$Res> {
  _$BazCopyWithImpl(Baz _value, $Res Function(Baz) _then)
      : super(_value, (v) => _then(v as Baz));

  @override
  Baz get _value => super._value as Baz;

  @override
  $Res call({
    Object? name = freezed,
  }) {
    return _then(Baz(
      name: name == freezed
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Baz implements Baz {
  const _$Baz({required this.name});

  @override
  final String name;

  @override
  String toString() {
    return 'Foobar.baz(name: $name)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Baz &&
            const DeepCollectionEquality().equals(other.name, name));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(name));

  @JsonKey(ignore: true)
  @override
  $BazCopyWith<Baz> get copyWith => _$BazCopyWithImpl<Baz>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() foo,
    required TResult Function(String field0) bar,
    required TResult Function(String name) baz,
  }) {
    return baz(name);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
  }) {
    return baz?.call(name);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? foo,
    TResult Function(String field0)? bar,
    TResult Function(String name)? baz,
    required TResult orElse(),
  }) {
    if (baz != null) {
      return baz(name);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Foo value) foo,
    required TResult Function(Bar value) bar,
    required TResult Function(Baz value) baz,
  }) {
    return baz(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
  }) {
    return baz?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Foo value)? foo,
    TResult Function(Bar value)? bar,
    TResult Function(Baz value)? baz,
    required TResult orElse(),
  }) {
    if (baz != null) {
      return baz(this);
    }
    return orElse();
  }
}

abstract class Baz implements Foobar {
  const factory Baz({required String name}) = _$Baz;

  String get name;
  @JsonKey(ignore: true)
  $BazCopyWith<Baz> get copyWith => throw _privateConstructorUsedError;
}
