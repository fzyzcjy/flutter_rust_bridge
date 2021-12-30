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
  factory $FoobarCopyWith(Foobar value, $Res Function(Foobar) then) = _$FoobarCopyWithImpl<$Res>;
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
  factory $FooCopyWith(Foo value, $Res Function(Foo) then) = _$FooCopyWithImpl<$Res>;
}

/// @nodoc
class _$FooCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res> implements $FooCopyWith<$Res> {
  _$FooCopyWithImpl(Foo _value, $Res Function(Foo) _then) : super(_value, (v) => _then(v as Foo));

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
    return identical(this, other) || (other.runtimeType == runtimeType && other is Foo);
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
  factory $BarCopyWith(Bar value, $Res Function(Bar) then) = _$BarCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class _$BarCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res> implements $BarCopyWith<$Res> {
  _$BarCopyWithImpl(Bar _value, $Res Function(Bar) _then) : super(_value, (v) => _then(v as Bar));

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
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

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
  factory $BazCopyWith(Baz value, $Res Function(Baz) then) = _$BazCopyWithImpl<$Res>;
  $Res call({String name});
}

/// @nodoc
class _$BazCopyWithImpl<$Res> extends _$FoobarCopyWithImpl<$Res> implements $BazCopyWith<$Res> {
  _$BazCopyWithImpl(Baz _value, $Res Function(Baz) _then) : super(_value, (v) => _then(v as Baz));

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
        (other.runtimeType == runtimeType && other is Baz && const DeepCollectionEquality().equals(other.name, name));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(name));

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

/// @nodoc
class _$KitchenSinkTearOff {
  const _$KitchenSinkTearOff();

  Empty empty() {
    return const Empty();
  }

  Nested nested(KitchenSink field0) {
    return Nested(
      field0,
    );
  }

  Optional optional(int field0, [int? field1]) {
    return Optional(
      field0,
      field1,
    );
  }

  Boxed boxed(int field0) {
    return Boxed(
      field0,
    );
  }

  Buffer buffer(Uint8List field0) {
    return Buffer(
      field0,
    );
  }

  Enums enums(Weekdays field0) {
    return Enums(
      field0,
    );
  }

  Structlike structlike({required Foobar foo, int? bar}) {
    return Structlike(
      foo: foo,
      bar: bar,
    );
  }
}

/// @nodoc
const $KitchenSink = _$KitchenSinkTearOff();

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkCopyWith<$Res> {
  factory $KitchenSinkCopyWith(KitchenSink value, $Res Function(KitchenSink) then) = _$KitchenSinkCopyWithImpl<$Res>;
}

/// @nodoc
class _$KitchenSinkCopyWithImpl<$Res> implements $KitchenSinkCopyWith<$Res> {
  _$KitchenSinkCopyWithImpl(this._value, this._then);

  final KitchenSink _value;
  // ignore: unused_field
  final $Res Function(KitchenSink) _then;
}

/// @nodoc
abstract class $EmptyCopyWith<$Res> {
  factory $EmptyCopyWith(Empty value, $Res Function(Empty) then) = _$EmptyCopyWithImpl<$Res>;
}

/// @nodoc
class _$EmptyCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $EmptyCopyWith<$Res> {
  _$EmptyCopyWithImpl(Empty _value, $Res Function(Empty) _then) : super(_value, (v) => _then(v as Empty));

  @override
  Empty get _value => super._value as Empty;
}

/// @nodoc

class _$Empty implements Empty {
  const _$Empty();

  @override
  String toString() {
    return 'KitchenSink.empty()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class Empty implements KitchenSink {
  const factory Empty() = _$Empty;
}

/// @nodoc
abstract class $NestedCopyWith<$Res> {
  factory $NestedCopyWith(Nested value, $Res Function(Nested) then) = _$NestedCopyWithImpl<$Res>;
  $Res call({KitchenSink field0});

  $KitchenSinkCopyWith<$Res> get field0;
}

/// @nodoc
class _$NestedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $NestedCopyWith<$Res> {
  _$NestedCopyWithImpl(Nested _value, $Res Function(Nested) _then) : super(_value, (v) => _then(v as Nested));

  @override
  Nested get _value => super._value as Nested;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Nested(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KitchenSink,
    ));
  }

  @override
  $KitchenSinkCopyWith<$Res> get field0 {
    return $KitchenSinkCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Nested implements Nested {
  const _$Nested(this.field0);

  @override
  final KitchenSink field0;

  @override
  String toString() {
    return 'KitchenSink.nested(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Nested &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $NestedCopyWith<Nested> get copyWith => _$NestedCopyWithImpl<Nested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return nested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return nested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class Nested implements KitchenSink {
  const factory Nested(KitchenSink field0) = _$Nested;

  KitchenSink get field0;
  @JsonKey(ignore: true)
  $NestedCopyWith<Nested> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OptionalCopyWith<$Res> {
  factory $OptionalCopyWith(Optional value, $Res Function(Optional) then) = _$OptionalCopyWithImpl<$Res>;
  $Res call({int field0, int? field1});
}

/// @nodoc
class _$OptionalCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $OptionalCopyWith<$Res> {
  _$OptionalCopyWithImpl(Optional _value, $Res Function(Optional) _then) : super(_value, (v) => _then(v as Optional));

  @override
  Optional get _value => super._value as Optional;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(Optional(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$Optional implements Optional {
  const _$Optional(this.field0, [this.field1]);

  @override
  final int field0;
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
            other is Optional &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  $OptionalCopyWith<Optional> get copyWith => _$OptionalCopyWithImpl<Optional>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class Optional implements KitchenSink {
  const factory Optional(int field0, [int? field1]) = _$Optional;

  int get field0;
  int? get field1;
  @JsonKey(ignore: true)
  $OptionalCopyWith<Optional> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BoxedCopyWith<$Res> {
  factory $BoxedCopyWith(Boxed value, $Res Function(Boxed) then) = _$BoxedCopyWithImpl<$Res>;
  $Res call({int field0});
}

/// @nodoc
class _$BoxedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $BoxedCopyWith<$Res> {
  _$BoxedCopyWithImpl(Boxed _value, $Res Function(Boxed) _then) : super(_value, (v) => _then(v as Boxed));

  @override
  Boxed get _value => super._value as Boxed;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Boxed(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Boxed implements Boxed {
  const _$Boxed(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'KitchenSink.boxed(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Boxed &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $BoxedCopyWith<Boxed> get copyWith => _$BoxedCopyWithImpl<Boxed>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return boxed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return boxed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
    required TResult orElse(),
  }) {
    if (boxed != null) {
      return boxed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return boxed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return boxed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (boxed != null) {
      return boxed(this);
    }
    return orElse();
  }
}

abstract class Boxed implements KitchenSink {
  const factory Boxed(int field0) = _$Boxed;

  int get field0;
  @JsonKey(ignore: true)
  $BoxedCopyWith<Boxed> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BufferCopyWith<$Res> {
  factory $BufferCopyWith(Buffer value, $Res Function(Buffer) then) = _$BufferCopyWithImpl<$Res>;
  $Res call({Uint8List field0});
}

/// @nodoc
class _$BufferCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $BufferCopyWith<$Res> {
  _$BufferCopyWithImpl(Buffer _value, $Res Function(Buffer) _then) : super(_value, (v) => _then(v as Buffer));

  @override
  Buffer get _value => super._value as Buffer;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Buffer(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$Buffer implements Buffer {
  const _$Buffer(this.field0);

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
            other is Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $BufferCopyWith<Buffer> get copyWith => _$BufferCopyWithImpl<Buffer>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class Buffer implements KitchenSink {
  const factory Buffer(Uint8List field0) = _$Buffer;

  Uint8List get field0;
  @JsonKey(ignore: true)
  $BufferCopyWith<Buffer> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumsCopyWith<$Res> {
  factory $EnumsCopyWith(Enums value, $Res Function(Enums) then) = _$EnumsCopyWithImpl<$Res>;
  $Res call({Weekdays field0});
}

/// @nodoc
class _$EnumsCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $EnumsCopyWith<$Res> {
  _$EnumsCopyWithImpl(Enums _value, $Res Function(Enums) _then) : super(_value, (v) => _then(v as Enums));

  @override
  Enums get _value => super._value as Enums;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Enums(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$Enums implements Enums {
  const _$Enums(this.field0);

  @override
  final Weekdays field0;

  @override
  String toString() {
    return 'KitchenSink.enums(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Enums &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $EnumsCopyWith<Enums> get copyWith => _$EnumsCopyWithImpl<Enums>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class Enums implements KitchenSink {
  const factory Enums(Weekdays field0) = _$Enums;

  Weekdays get field0;
  @JsonKey(ignore: true)
  $EnumsCopyWith<Enums> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $StructlikeCopyWith<$Res> {
  factory $StructlikeCopyWith(Structlike value, $Res Function(Structlike) then) = _$StructlikeCopyWithImpl<$Res>;
  $Res call({Foobar foo, int? bar});

  $FoobarCopyWith<$Res> get foo;
}

/// @nodoc
class _$StructlikeCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $StructlikeCopyWith<$Res> {
  _$StructlikeCopyWithImpl(Structlike _value, $Res Function(Structlike) _then)
      : super(_value, (v) => _then(v as Structlike));

  @override
  Structlike get _value => super._value as Structlike;

  @override
  $Res call({
    Object? foo = freezed,
    Object? bar = freezed,
  }) {
    return _then(Structlike(
      foo: foo == freezed
          ? _value.foo
          : foo // ignore: cast_nullable_to_non_nullable
              as Foobar,
      bar: bar == freezed
          ? _value.bar
          : bar // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }

  @override
  $FoobarCopyWith<$Res> get foo {
    return $FoobarCopyWith<$Res>(_value.foo, (value) {
      return _then(_value.copyWith(foo: value));
    });
  }
}

/// @nodoc

class _$Structlike implements Structlike {
  const _$Structlike({required this.foo, this.bar});

  @override
  final Foobar foo;
  @override
  final int? bar;

  @override
  String toString() {
    return 'KitchenSink.structlike(foo: $foo, bar: $bar)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Structlike &&
            const DeepCollectionEquality().equals(other.foo, foo) &&
            const DeepCollectionEquality().equals(other.bar, bar));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(foo), const DeepCollectionEquality().hash(bar));

  @JsonKey(ignore: true)
  @override
  $StructlikeCopyWith<Structlike> get copyWith => _$StructlikeCopyWithImpl<Structlike>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int field0, int? field1) optional,
    required TResult Function(int field0) boxed,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
    required TResult Function(Foobar foo, int? bar) structlike,
  }) {
    return structlike(foo, bar);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
  }) {
    return structlike?.call(foo, bar);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int field0, int? field1)? optional,
    TResult Function(int field0)? boxed,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    TResult Function(Foobar foo, int? bar)? structlike,
    required TResult orElse(),
  }) {
    if (structlike != null) {
      return structlike(foo, bar);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Empty value) empty,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Boxed value) boxed,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
    required TResult Function(Structlike value) structlike,
  }) {
    return structlike(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
  }) {
    return structlike?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Boxed value)? boxed,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    TResult Function(Structlike value)? structlike,
    required TResult orElse(),
  }) {
    if (structlike != null) {
      return structlike(this);
    }
    return orElse();
  }
}

abstract class Structlike implements KitchenSink {
  const factory Structlike({required Foobar foo, int? bar}) = _$Structlike;

  Foobar get foo;
  int? get bar;
  @JsonKey(ignore: true)
  $StructlikeCopyWith<Structlike> get copyWith => throw _privateConstructorUsedError;
}
