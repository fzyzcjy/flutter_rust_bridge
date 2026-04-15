// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinSyncSseCopyWith<$Res> {
  $AbcTwinSyncSseCopyWith(AbcTwinSyncSse _, $Res Function(AbcTwinSyncSse) __);
}

/// Adds pattern-matching-related methods to [AbcTwinSyncSse].
extension AbcTwinSyncSsePatterns on AbcTwinSyncSse {
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
    TResult Function(AbcTwinSyncSse_A value)? a,
    TResult Function(AbcTwinSyncSse_B value)? b,
    TResult Function(AbcTwinSyncSse_C value)? c,
    TResult Function(AbcTwinSyncSse_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A() when a != null:
        return a(_that);
      case AbcTwinSyncSse_B() when b != null:
        return b(_that);
      case AbcTwinSyncSse_C() when c != null:
        return c(_that);
      case AbcTwinSyncSse_JustInt() when justInt != null:
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
    required TResult Function(AbcTwinSyncSse_A value) a,
    required TResult Function(AbcTwinSyncSse_B value) b,
    required TResult Function(AbcTwinSyncSse_C value) c,
    required TResult Function(AbcTwinSyncSse_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A():
        return a(_that);
      case AbcTwinSyncSse_B():
        return b(_that);
      case AbcTwinSyncSse_C():
        return c(_that);
      case AbcTwinSyncSse_JustInt():
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
    TResult? Function(AbcTwinSyncSse_A value)? a,
    TResult? Function(AbcTwinSyncSse_B value)? b,
    TResult? Function(AbcTwinSyncSse_C value)? c,
    TResult? Function(AbcTwinSyncSse_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A() when a != null:
        return a(_that);
      case AbcTwinSyncSse_B() when b != null:
        return b(_that);
      case AbcTwinSyncSse_C() when c != null:
        return c(_that);
      case AbcTwinSyncSse_JustInt() when justInt != null:
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
    TResult Function(ATwinSyncSse field0)? a,
    TResult Function(BTwinSyncSse field0)? b,
    TResult Function(CTwinSyncSse field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinSyncSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinSyncSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinSyncSse_JustInt() when justInt != null:
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
    required TResult Function(ATwinSyncSse field0) a,
    required TResult Function(BTwinSyncSse field0) b,
    required TResult Function(CTwinSyncSse field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A():
        return a(_that.field0);
      case AbcTwinSyncSse_B():
        return b(_that.field0);
      case AbcTwinSyncSse_C():
        return c(_that.field0);
      case AbcTwinSyncSse_JustInt():
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
    TResult? Function(ATwinSyncSse field0)? a,
    TResult? Function(BTwinSyncSse field0)? b,
    TResult? Function(CTwinSyncSse field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSyncSse_A() when a != null:
        return a(_that.field0);
      case AbcTwinSyncSse_B() when b != null:
        return b(_that.field0);
      case AbcTwinSyncSse_C() when c != null:
        return c(_that.field0);
      case AbcTwinSyncSse_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinSyncSse_A extends AbcTwinSyncSse {
  const AbcTwinSyncSse_A(this.field0) : super._();

  @override
  final ATwinSyncSse field0;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSyncSse_ACopyWith<AbcTwinSyncSse_A> get copyWith =>
      _$AbcTwinSyncSse_ACopyWithImpl<AbcTwinSyncSse_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSyncSse_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSyncSse.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSyncSse_ACopyWith<$Res>
    implements $AbcTwinSyncSseCopyWith<$Res> {
  factory $AbcTwinSyncSse_ACopyWith(
          AbcTwinSyncSse_A value, $Res Function(AbcTwinSyncSse_A) _then) =
      _$AbcTwinSyncSse_ACopyWithImpl;
  @useResult
  $Res call({ATwinSyncSse field0});
}

/// @nodoc
class _$AbcTwinSyncSse_ACopyWithImpl<$Res>
    implements $AbcTwinSyncSse_ACopyWith<$Res> {
  _$AbcTwinSyncSse_ACopyWithImpl(this._self, this._then);

  final AbcTwinSyncSse_A _self;
  final $Res Function(AbcTwinSyncSse_A) _then;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSyncSse_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinSyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinSyncSse_B extends AbcTwinSyncSse {
  const AbcTwinSyncSse_B(this.field0) : super._();

  @override
  final BTwinSyncSse field0;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSyncSse_BCopyWith<AbcTwinSyncSse_B> get copyWith =>
      _$AbcTwinSyncSse_BCopyWithImpl<AbcTwinSyncSse_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSyncSse_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSyncSse.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSyncSse_BCopyWith<$Res>
    implements $AbcTwinSyncSseCopyWith<$Res> {
  factory $AbcTwinSyncSse_BCopyWith(
          AbcTwinSyncSse_B value, $Res Function(AbcTwinSyncSse_B) _then) =
      _$AbcTwinSyncSse_BCopyWithImpl;
  @useResult
  $Res call({BTwinSyncSse field0});
}

/// @nodoc
class _$AbcTwinSyncSse_BCopyWithImpl<$Res>
    implements $AbcTwinSyncSse_BCopyWith<$Res> {
  _$AbcTwinSyncSse_BCopyWithImpl(this._self, this._then);

  final AbcTwinSyncSse_B _self;
  final $Res Function(AbcTwinSyncSse_B) _then;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSyncSse_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinSyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinSyncSse_C extends AbcTwinSyncSse {
  const AbcTwinSyncSse_C(this.field0) : super._();

  @override
  final CTwinSyncSse field0;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSyncSse_CCopyWith<AbcTwinSyncSse_C> get copyWith =>
      _$AbcTwinSyncSse_CCopyWithImpl<AbcTwinSyncSse_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSyncSse_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSyncSse.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSyncSse_CCopyWith<$Res>
    implements $AbcTwinSyncSseCopyWith<$Res> {
  factory $AbcTwinSyncSse_CCopyWith(
          AbcTwinSyncSse_C value, $Res Function(AbcTwinSyncSse_C) _then) =
      _$AbcTwinSyncSse_CCopyWithImpl;
  @useResult
  $Res call({CTwinSyncSse field0});
}

/// @nodoc
class _$AbcTwinSyncSse_CCopyWithImpl<$Res>
    implements $AbcTwinSyncSse_CCopyWith<$Res> {
  _$AbcTwinSyncSse_CCopyWithImpl(this._self, this._then);

  final AbcTwinSyncSse_C _self;
  final $Res Function(AbcTwinSyncSse_C) _then;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSyncSse_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinSyncSse,
    ));
  }
}

/// @nodoc

class AbcTwinSyncSse_JustInt extends AbcTwinSyncSse {
  const AbcTwinSyncSse_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSyncSse_JustIntCopyWith<AbcTwinSyncSse_JustInt> get copyWith =>
      _$AbcTwinSyncSse_JustIntCopyWithImpl<AbcTwinSyncSse_JustInt>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSyncSse_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSyncSse.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSyncSse_JustIntCopyWith<$Res>
    implements $AbcTwinSyncSseCopyWith<$Res> {
  factory $AbcTwinSyncSse_JustIntCopyWith(AbcTwinSyncSse_JustInt value,
          $Res Function(AbcTwinSyncSse_JustInt) _then) =
      _$AbcTwinSyncSse_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinSyncSse_JustIntCopyWithImpl<$Res>
    implements $AbcTwinSyncSse_JustIntCopyWith<$Res> {
  _$AbcTwinSyncSse_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinSyncSse_JustInt _self;
  final $Res Function(AbcTwinSyncSse_JustInt) _then;

  /// Create a copy of AbcTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSyncSse_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
