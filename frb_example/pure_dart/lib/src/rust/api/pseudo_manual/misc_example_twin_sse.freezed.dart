// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinSseCopyWith<$Res> {
  $AbcTwinSseCopyWith(AbcTwinSse _, $Res Function(AbcTwinSse) __);
}

/// Adds pattern-matching-related methods to [AbcTwinSse].
extension AbcTwinSsePatterns on AbcTwinSse {
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
    TResult Function(AbcTwinSse_A value)? a,
    TResult Function(AbcTwinSse_B value)? b,
    TResult Function(AbcTwinSse_C value)? c,
    TResult Function(AbcTwinSse_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A() when a != null:
        return a(_that);
      case AbcTwinSse_B() when b != null:
        return b(_that);
      case AbcTwinSse_C() when c != null:
        return c(_that);
      case AbcTwinSse_JustInt() when justInt != null:
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
    required TResult Function(AbcTwinSse_A value) a,
    required TResult Function(AbcTwinSse_B value) b,
    required TResult Function(AbcTwinSse_C value) c,
    required TResult Function(AbcTwinSse_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A():
        return a(_that);
      case AbcTwinSse_B():
        return b(_that);
      case AbcTwinSse_C():
        return c(_that);
      case AbcTwinSse_JustInt():
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
    TResult? Function(AbcTwinSse_A value)? a,
    TResult? Function(AbcTwinSse_B value)? b,
    TResult? Function(AbcTwinSse_C value)? c,
    TResult? Function(AbcTwinSse_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A() when a != null:
        return a(_that);
      case AbcTwinSse_B() when b != null:
        return b(_that);
      case AbcTwinSse_C() when c != null:
        return c(_that);
      case AbcTwinSse_JustInt() when justInt != null:
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
    TResult Function(ATwinSse field0)? a,
    TResult Function(BTwinSse field0)? b,
    TResult Function(CTwinSse field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinSse_JustInt() when justInt != null:
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
    required TResult Function(ATwinSse field0) a,
    required TResult Function(BTwinSse field0) b,
    required TResult Function(CTwinSse field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A():
        return a(_that.field0);
      case AbcTwinSse_B():
        return b(_that.field0);
      case AbcTwinSse_C():
        return c(_that.field0);
      case AbcTwinSse_JustInt():
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
    TResult? Function(ATwinSse field0)? a,
    TResult? Function(BTwinSse field0)? b,
    TResult? Function(CTwinSse field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinSse_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinSse_A extends AbcTwinSse {
  const AbcTwinSse_A(this.field0) : super._();

  @override
  final ATwinSse field0;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSse_ACopyWith<AbcTwinSse_A> get copyWith =>
      _$AbcTwinSse_ACopyWithImpl<AbcTwinSse_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSse_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSse.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSse_ACopyWith<$Res>
    implements $AbcTwinSseCopyWith<$Res> {
  factory $AbcTwinSse_ACopyWith(
          AbcTwinSse_A value, $Res Function(AbcTwinSse_A) _then) =
      _$AbcTwinSse_ACopyWithImpl;
  @useResult
  $Res call({ATwinSse field0});
}

/// @nodoc
class _$AbcTwinSse_ACopyWithImpl<$Res> implements $AbcTwinSse_ACopyWith<$Res> {
  _$AbcTwinSse_ACopyWithImpl(this._self, this._then);

  final AbcTwinSse_A _self;
  final $Res Function(AbcTwinSse_A) _then;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSse_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinSse,
    ));
  }
}

/// @nodoc

class AbcTwinSse_B extends AbcTwinSse {
  const AbcTwinSse_B(this.field0) : super._();

  @override
  final BTwinSse field0;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSse_BCopyWith<AbcTwinSse_B> get copyWith =>
      _$AbcTwinSse_BCopyWithImpl<AbcTwinSse_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSse_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSse.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSse_BCopyWith<$Res>
    implements $AbcTwinSseCopyWith<$Res> {
  factory $AbcTwinSse_BCopyWith(
          AbcTwinSse_B value, $Res Function(AbcTwinSse_B) _then) =
      _$AbcTwinSse_BCopyWithImpl;
  @useResult
  $Res call({BTwinSse field0});
}

/// @nodoc
class _$AbcTwinSse_BCopyWithImpl<$Res> implements $AbcTwinSse_BCopyWith<$Res> {
  _$AbcTwinSse_BCopyWithImpl(this._self, this._then);

  final AbcTwinSse_B _self;
  final $Res Function(AbcTwinSse_B) _then;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSse_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinSse,
    ));
  }
}

/// @nodoc

class AbcTwinSse_C extends AbcTwinSse {
  const AbcTwinSse_C(this.field0) : super._();

  @override
  final CTwinSse field0;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSse_CCopyWith<AbcTwinSse_C> get copyWith =>
      _$AbcTwinSse_CCopyWithImpl<AbcTwinSse_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSse_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSse.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSse_CCopyWith<$Res>
    implements $AbcTwinSseCopyWith<$Res> {
  factory $AbcTwinSse_CCopyWith(
          AbcTwinSse_C value, $Res Function(AbcTwinSse_C) _then) =
      _$AbcTwinSse_CCopyWithImpl;
  @useResult
  $Res call({CTwinSse field0});
}

/// @nodoc
class _$AbcTwinSse_CCopyWithImpl<$Res> implements $AbcTwinSse_CCopyWith<$Res> {
  _$AbcTwinSse_CCopyWithImpl(this._self, this._then);

  final AbcTwinSse_C _self;
  final $Res Function(AbcTwinSse_C) _then;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSse_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinSse,
    ));
  }
}

/// @nodoc

class AbcTwinSse_JustInt extends AbcTwinSse {
  const AbcTwinSse_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSse_JustIntCopyWith<AbcTwinSse_JustInt> get copyWith =>
      _$AbcTwinSse_JustIntCopyWithImpl<AbcTwinSse_JustInt>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSse_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSse.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSse_JustIntCopyWith<$Res>
    implements $AbcTwinSseCopyWith<$Res> {
  factory $AbcTwinSse_JustIntCopyWith(
          AbcTwinSse_JustInt value, $Res Function(AbcTwinSse_JustInt) _then) =
      _$AbcTwinSse_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinSse_JustIntCopyWithImpl<$Res>
    implements $AbcTwinSse_JustIntCopyWith<$Res> {
  _$AbcTwinSse_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinSse_JustInt _self;
  final $Res Function(AbcTwinSse_JustInt) _then;

  /// Create a copy of AbcTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSse_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
