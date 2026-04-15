// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinRustAsyncCopyWith<$Res> {
  $AbcTwinRustAsyncCopyWith(
      AbcTwinRustAsync _, $Res Function(AbcTwinRustAsync) __);
}

/// Adds pattern-matching-related methods to [AbcTwinRustAsync].
extension AbcTwinRustAsyncPatterns on AbcTwinRustAsync {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AbcTwinRustAsync_A value)? a,
    TResult Function(AbcTwinRustAsync_B value)? b,
    TResult Function(AbcTwinRustAsync_C value)? c,
    TResult Function(AbcTwinRustAsync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A() when a != null:
        return a(_that);
      case AbcTwinRustAsync_B() when b != null:
        return b(_that);
      case AbcTwinRustAsync_C() when c != null:
        return c(_that);
      case AbcTwinRustAsync_JustInt() when justInt != null:
        return justInt(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AbcTwinRustAsync_A value) a,
    required TResult Function(AbcTwinRustAsync_B value) b,
    required TResult Function(AbcTwinRustAsync_C value) c,
    required TResult Function(AbcTwinRustAsync_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A():
        return a(_that);
      case AbcTwinRustAsync_B():
        return b(_that);
      case AbcTwinRustAsync_C():
        return c(_that);
      case AbcTwinRustAsync_JustInt():
        return justInt(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AbcTwinRustAsync_A value)? a,
    TResult? Function(AbcTwinRustAsync_B value)? b,
    TResult? Function(AbcTwinRustAsync_C value)? c,
    TResult? Function(AbcTwinRustAsync_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A() when a != null:
        return a(_that);
      case AbcTwinRustAsync_B() when b != null:
        return b(_that);
      case AbcTwinRustAsync_C() when c != null:
        return c(_that);
      case AbcTwinRustAsync_JustInt() when justInt != null:
        return justInt(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ATwinRustAsync field0)? a,
    TResult Function(BTwinRustAsync field0)? b,
    TResult Function(CTwinRustAsync field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A() when a != null:
        return a(_that.field0);
      case AbcTwinRustAsync_B() when b != null:
        return b(_that.field0);
      case AbcTwinRustAsync_C() when c != null:
        return c(_that.field0);
      case AbcTwinRustAsync_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ATwinRustAsync field0) a,
    required TResult Function(BTwinRustAsync field0) b,
    required TResult Function(CTwinRustAsync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A():
        return a(_that.field0);
      case AbcTwinRustAsync_B():
        return b(_that.field0);
      case AbcTwinRustAsync_C():
        return c(_that.field0);
      case AbcTwinRustAsync_JustInt():
        return justInt(_that.field0);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ATwinRustAsync field0)? a,
    TResult? Function(BTwinRustAsync field0)? b,
    TResult? Function(CTwinRustAsync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsync_A() when a != null:
        return a(_that.field0);
      case AbcTwinRustAsync_B() when b != null:
        return b(_that.field0);
      case AbcTwinRustAsync_C() when c != null:
        return c(_that.field0);
      case AbcTwinRustAsync_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinRustAsync_A extends AbcTwinRustAsync {
  const AbcTwinRustAsync_A(this.field0) : super._();

  @override
  final ATwinRustAsync field0;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsync_ACopyWith<AbcTwinRustAsync_A> get copyWith =>
      _$AbcTwinRustAsync_ACopyWithImpl<AbcTwinRustAsync_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsync_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsync.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsync_ACopyWith<$Res>
    implements $AbcTwinRustAsyncCopyWith<$Res> {
  factory $AbcTwinRustAsync_ACopyWith(
          AbcTwinRustAsync_A value, $Res Function(AbcTwinRustAsync_A) _then) =
      _$AbcTwinRustAsync_ACopyWithImpl;
  @useResult
  $Res call({ATwinRustAsync field0});
}

/// @nodoc
class _$AbcTwinRustAsync_ACopyWithImpl<$Res>
    implements $AbcTwinRustAsync_ACopyWith<$Res> {
  _$AbcTwinRustAsync_ACopyWithImpl(this._self, this._then);

  final AbcTwinRustAsync_A _self;
  final $Res Function(AbcTwinRustAsync_A) _then;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsync_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinRustAsync,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsync_B extends AbcTwinRustAsync {
  const AbcTwinRustAsync_B(this.field0) : super._();

  @override
  final BTwinRustAsync field0;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsync_BCopyWith<AbcTwinRustAsync_B> get copyWith =>
      _$AbcTwinRustAsync_BCopyWithImpl<AbcTwinRustAsync_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsync_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsync.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsync_BCopyWith<$Res>
    implements $AbcTwinRustAsyncCopyWith<$Res> {
  factory $AbcTwinRustAsync_BCopyWith(
          AbcTwinRustAsync_B value, $Res Function(AbcTwinRustAsync_B) _then) =
      _$AbcTwinRustAsync_BCopyWithImpl;
  @useResult
  $Res call({BTwinRustAsync field0});
}

/// @nodoc
class _$AbcTwinRustAsync_BCopyWithImpl<$Res>
    implements $AbcTwinRustAsync_BCopyWith<$Res> {
  _$AbcTwinRustAsync_BCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsync_B _self;
  final $Res Function(AbcTwinRustAsync_B) _then;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsync_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinRustAsync,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsync_C extends AbcTwinRustAsync {
  const AbcTwinRustAsync_C(this.field0) : super._();

  @override
  final CTwinRustAsync field0;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsync_CCopyWith<AbcTwinRustAsync_C> get copyWith =>
      _$AbcTwinRustAsync_CCopyWithImpl<AbcTwinRustAsync_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsync_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsync.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsync_CCopyWith<$Res>
    implements $AbcTwinRustAsyncCopyWith<$Res> {
  factory $AbcTwinRustAsync_CCopyWith(
          AbcTwinRustAsync_C value, $Res Function(AbcTwinRustAsync_C) _then) =
      _$AbcTwinRustAsync_CCopyWithImpl;
  @useResult
  $Res call({CTwinRustAsync field0});
}

/// @nodoc
class _$AbcTwinRustAsync_CCopyWithImpl<$Res>
    implements $AbcTwinRustAsync_CCopyWith<$Res> {
  _$AbcTwinRustAsync_CCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsync_C _self;
  final $Res Function(AbcTwinRustAsync_C) _then;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsync_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinRustAsync,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsync_JustInt extends AbcTwinRustAsync {
  const AbcTwinRustAsync_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsync_JustIntCopyWith<AbcTwinRustAsync_JustInt> get copyWith =>
      _$AbcTwinRustAsync_JustIntCopyWithImpl<AbcTwinRustAsync_JustInt>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsync_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsync.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsync_JustIntCopyWith<$Res>
    implements $AbcTwinRustAsyncCopyWith<$Res> {
  factory $AbcTwinRustAsync_JustIntCopyWith(AbcTwinRustAsync_JustInt value,
          $Res Function(AbcTwinRustAsync_JustInt) _then) =
      _$AbcTwinRustAsync_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinRustAsync_JustIntCopyWithImpl<$Res>
    implements $AbcTwinRustAsync_JustIntCopyWith<$Res> {
  _$AbcTwinRustAsync_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsync_JustInt _self;
  final $Res Function(AbcTwinRustAsync_JustInt) _then;

  /// Create a copy of AbcTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsync_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
