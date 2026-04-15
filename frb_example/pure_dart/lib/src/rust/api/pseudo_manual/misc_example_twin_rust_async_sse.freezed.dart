// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinRustAsyncSseCopyWith<$Res> {
  $AbcTwinRustAsyncSseCopyWith(
      AbcTwinRustAsyncSse _, $Res Function(AbcTwinRustAsyncSse) __);
}

/// Adds pattern-matching-related methods to [AbcTwinRustAsyncSse].
extension AbcTwinRustAsyncSsePatterns on AbcTwinRustAsyncSse {
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
    TResult Function(AbcTwinRustAsyncSse_A value)? a,
    TResult Function(AbcTwinRustAsyncSse_B value)? b,
    TResult Function(AbcTwinRustAsyncSse_C value)? c,
    TResult Function(AbcTwinRustAsyncSse_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A() when a != null:
        return a(_that);
      case AbcTwinRustAsyncSse_B() when b != null:
        return b(_that);
      case AbcTwinRustAsyncSse_C() when c != null:
        return c(_that);
      case AbcTwinRustAsyncSse_JustInt() when justInt != null:
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
    required TResult Function(AbcTwinRustAsyncSse_A value) a,
    required TResult Function(AbcTwinRustAsyncSse_B value) b,
    required TResult Function(AbcTwinRustAsyncSse_C value) c,
    required TResult Function(AbcTwinRustAsyncSse_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A():
        return a(_that);
      case AbcTwinRustAsyncSse_B():
        return b(_that);
      case AbcTwinRustAsyncSse_C():
        return c(_that);
      case AbcTwinRustAsyncSse_JustInt():
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
    TResult? Function(AbcTwinRustAsyncSse_A value)? a,
    TResult? Function(AbcTwinRustAsyncSse_B value)? b,
    TResult? Function(AbcTwinRustAsyncSse_C value)? c,
    TResult? Function(AbcTwinRustAsyncSse_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A() when a != null:
        return a(_that);
      case AbcTwinRustAsyncSse_B() when b != null:
        return b(_that);
      case AbcTwinRustAsyncSse_C() when c != null:
        return c(_that);
      case AbcTwinRustAsyncSse_JustInt() when justInt != null:
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
    TResult Function(ATwinRustAsyncSse field0)? a,
    TResult Function(BTwinRustAsyncSse field0)? b,
    TResult Function(CTwinRustAsyncSse field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinRustAsyncSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinRustAsyncSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinRustAsyncSse_JustInt() when justInt != null:
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
    required TResult Function(ATwinRustAsyncSse field0) a,
    required TResult Function(BTwinRustAsyncSse field0) b,
    required TResult Function(CTwinRustAsyncSse field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A():
        return a(_that.field0);
      case AbcTwinRustAsyncSse_B():
        return b(_that.field0);
      case AbcTwinRustAsyncSse_C():
        return c(_that.field0);
      case AbcTwinRustAsyncSse_JustInt():
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
    TResult? Function(ATwinRustAsyncSse field0)? a,
    TResult? Function(BTwinRustAsyncSse field0)? b,
    TResult? Function(CTwinRustAsyncSse field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinRustAsyncSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinRustAsyncSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinRustAsyncSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinRustAsyncSse_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinRustAsyncSse_A extends AbcTwinRustAsyncSse {
  const AbcTwinRustAsyncSse_A(this.field0) : super._();

  @override
  final ATwinRustAsyncSse field0;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsyncSse_ACopyWith<AbcTwinRustAsyncSse_A> get copyWith =>
      _$AbcTwinRustAsyncSse_ACopyWithImpl<AbcTwinRustAsyncSse_A>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsyncSse_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsyncSse.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsyncSse_ACopyWith<$Res>
    implements $AbcTwinRustAsyncSseCopyWith<$Res> {
  factory $AbcTwinRustAsyncSse_ACopyWith(AbcTwinRustAsyncSse_A value,
          $Res Function(AbcTwinRustAsyncSse_A) _then) =
      _$AbcTwinRustAsyncSse_ACopyWithImpl;
  @useResult
  $Res call({ATwinRustAsyncSse field0});
}

/// @nodoc
class _$AbcTwinRustAsyncSse_ACopyWithImpl<$Res>
    implements $AbcTwinRustAsyncSse_ACopyWith<$Res> {
  _$AbcTwinRustAsyncSse_ACopyWithImpl(this._self, this._then);

  final AbcTwinRustAsyncSse_A _self;
  final $Res Function(AbcTwinRustAsyncSse_A) _then;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsyncSse_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsyncSse_B extends AbcTwinRustAsyncSse {
  const AbcTwinRustAsyncSse_B(this.field0) : super._();

  @override
  final BTwinRustAsyncSse field0;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsyncSse_BCopyWith<AbcTwinRustAsyncSse_B> get copyWith =>
      _$AbcTwinRustAsyncSse_BCopyWithImpl<AbcTwinRustAsyncSse_B>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsyncSse_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsyncSse.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsyncSse_BCopyWith<$Res>
    implements $AbcTwinRustAsyncSseCopyWith<$Res> {
  factory $AbcTwinRustAsyncSse_BCopyWith(AbcTwinRustAsyncSse_B value,
          $Res Function(AbcTwinRustAsyncSse_B) _then) =
      _$AbcTwinRustAsyncSse_BCopyWithImpl;
  @useResult
  $Res call({BTwinRustAsyncSse field0});
}

/// @nodoc
class _$AbcTwinRustAsyncSse_BCopyWithImpl<$Res>
    implements $AbcTwinRustAsyncSse_BCopyWith<$Res> {
  _$AbcTwinRustAsyncSse_BCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsyncSse_B _self;
  final $Res Function(AbcTwinRustAsyncSse_B) _then;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsyncSse_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsyncSse_C extends AbcTwinRustAsyncSse {
  const AbcTwinRustAsyncSse_C(this.field0) : super._();

  @override
  final CTwinRustAsyncSse field0;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsyncSse_CCopyWith<AbcTwinRustAsyncSse_C> get copyWith =>
      _$AbcTwinRustAsyncSse_CCopyWithImpl<AbcTwinRustAsyncSse_C>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsyncSse_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsyncSse.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsyncSse_CCopyWith<$Res>
    implements $AbcTwinRustAsyncSseCopyWith<$Res> {
  factory $AbcTwinRustAsyncSse_CCopyWith(AbcTwinRustAsyncSse_C value,
          $Res Function(AbcTwinRustAsyncSse_C) _then) =
      _$AbcTwinRustAsyncSse_CCopyWithImpl;
  @useResult
  $Res call({CTwinRustAsyncSse field0});
}

/// @nodoc
class _$AbcTwinRustAsyncSse_CCopyWithImpl<$Res>
    implements $AbcTwinRustAsyncSse_CCopyWith<$Res> {
  _$AbcTwinRustAsyncSse_CCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsyncSse_C _self;
  final $Res Function(AbcTwinRustAsyncSse_C) _then;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsyncSse_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinRustAsyncSse_JustInt extends AbcTwinRustAsyncSse {
  const AbcTwinRustAsyncSse_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinRustAsyncSse_JustIntCopyWith<AbcTwinRustAsyncSse_JustInt>
      get copyWith => _$AbcTwinRustAsyncSse_JustIntCopyWithImpl<
          AbcTwinRustAsyncSse_JustInt>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinRustAsyncSse_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinRustAsyncSse.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinRustAsyncSse_JustIntCopyWith<$Res>
    implements $AbcTwinRustAsyncSseCopyWith<$Res> {
  factory $AbcTwinRustAsyncSse_JustIntCopyWith(
          AbcTwinRustAsyncSse_JustInt value,
          $Res Function(AbcTwinRustAsyncSse_JustInt) _then) =
      _$AbcTwinRustAsyncSse_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinRustAsyncSse_JustIntCopyWithImpl<$Res>
    implements $AbcTwinRustAsyncSse_JustIntCopyWith<$Res> {
  _$AbcTwinRustAsyncSse_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinRustAsyncSse_JustInt _self;
  final $Res Function(AbcTwinRustAsyncSse_JustInt) _then;

  /// Create a copy of AbcTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinRustAsyncSse_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
