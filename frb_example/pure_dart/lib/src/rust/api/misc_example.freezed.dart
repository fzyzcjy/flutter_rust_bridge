// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinNormalCopyWith<$Res> {
  $AbcTwinNormalCopyWith(AbcTwinNormal _, $Res Function(AbcTwinNormal) __);
}

/// Adds pattern-matching-related methods to [AbcTwinNormal].
extension AbcTwinNormalPatterns on AbcTwinNormal {
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
    TResult Function(AbcTwinNormal_A value)? a,
    TResult Function(AbcTwinNormal_B value)? b,
    TResult Function(AbcTwinNormal_C value)? c,
    TResult Function(AbcTwinNormal_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A() when a != null:
        return a(_that);
      case AbcTwinNormal_B() when b != null:
        return b(_that);
      case AbcTwinNormal_C() when c != null:
        return c(_that);
      case AbcTwinNormal_JustInt() when justInt != null:
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
    required TResult Function(AbcTwinNormal_A value) a,
    required TResult Function(AbcTwinNormal_B value) b,
    required TResult Function(AbcTwinNormal_C value) c,
    required TResult Function(AbcTwinNormal_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A():
        return a(_that);
      case AbcTwinNormal_B():
        return b(_that);
      case AbcTwinNormal_C():
        return c(_that);
      case AbcTwinNormal_JustInt():
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
    TResult? Function(AbcTwinNormal_A value)? a,
    TResult? Function(AbcTwinNormal_B value)? b,
    TResult? Function(AbcTwinNormal_C value)? c,
    TResult? Function(AbcTwinNormal_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A() when a != null:
        return a(_that);
      case AbcTwinNormal_B() when b != null:
        return b(_that);
      case AbcTwinNormal_C() when c != null:
        return c(_that);
      case AbcTwinNormal_JustInt() when justInt != null:
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
    TResult Function(ATwinNormal field0)? a,
    TResult Function(BTwinNormal field0)? b,
    TResult Function(CTwinNormal field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A() when a != null:
        return a(_that.field0);
      case AbcTwinNormal_B() when b != null:
        return b(_that.field0);
      case AbcTwinNormal_C() when c != null:
        return c(_that.field0);
      case AbcTwinNormal_JustInt() when justInt != null:
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
    required TResult Function(ATwinNormal field0) a,
    required TResult Function(BTwinNormal field0) b,
    required TResult Function(CTwinNormal field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A():
        return a(_that.field0);
      case AbcTwinNormal_B():
        return b(_that.field0);
      case AbcTwinNormal_C():
        return c(_that.field0);
      case AbcTwinNormal_JustInt():
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
    TResult? Function(ATwinNormal field0)? a,
    TResult? Function(BTwinNormal field0)? b,
    TResult? Function(CTwinNormal field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinNormal_A() when a != null:
        return a(_that.field0);
      case AbcTwinNormal_B() when b != null:
        return b(_that.field0);
      case AbcTwinNormal_C() when c != null:
        return c(_that.field0);
      case AbcTwinNormal_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinNormal_A extends AbcTwinNormal {
  const AbcTwinNormal_A(this.field0) : super._();

  @override
  final ATwinNormal field0;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinNormal_ACopyWith<AbcTwinNormal_A> get copyWith =>
      _$AbcTwinNormal_ACopyWithImpl<AbcTwinNormal_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinNormal_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinNormal.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinNormal_ACopyWith<$Res>
    implements $AbcTwinNormalCopyWith<$Res> {
  factory $AbcTwinNormal_ACopyWith(
          AbcTwinNormal_A value, $Res Function(AbcTwinNormal_A) _then) =
      _$AbcTwinNormal_ACopyWithImpl;
  @useResult
  $Res call({ATwinNormal field0});
}

/// @nodoc
class _$AbcTwinNormal_ACopyWithImpl<$Res>
    implements $AbcTwinNormal_ACopyWith<$Res> {
  _$AbcTwinNormal_ACopyWithImpl(this._self, this._then);

  final AbcTwinNormal_A _self;
  final $Res Function(AbcTwinNormal_A) _then;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinNormal_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinNormal,
    ));
  }
}

/// @nodoc

class AbcTwinNormal_B extends AbcTwinNormal {
  const AbcTwinNormal_B(this.field0) : super._();

  @override
  final BTwinNormal field0;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinNormal_BCopyWith<AbcTwinNormal_B> get copyWith =>
      _$AbcTwinNormal_BCopyWithImpl<AbcTwinNormal_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinNormal_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinNormal.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinNormal_BCopyWith<$Res>
    implements $AbcTwinNormalCopyWith<$Res> {
  factory $AbcTwinNormal_BCopyWith(
          AbcTwinNormal_B value, $Res Function(AbcTwinNormal_B) _then) =
      _$AbcTwinNormal_BCopyWithImpl;
  @useResult
  $Res call({BTwinNormal field0});
}

/// @nodoc
class _$AbcTwinNormal_BCopyWithImpl<$Res>
    implements $AbcTwinNormal_BCopyWith<$Res> {
  _$AbcTwinNormal_BCopyWithImpl(this._self, this._then);

  final AbcTwinNormal_B _self;
  final $Res Function(AbcTwinNormal_B) _then;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinNormal_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinNormal,
    ));
  }
}

/// @nodoc

class AbcTwinNormal_C extends AbcTwinNormal {
  const AbcTwinNormal_C(this.field0) : super._();

  @override
  final CTwinNormal field0;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinNormal_CCopyWith<AbcTwinNormal_C> get copyWith =>
      _$AbcTwinNormal_CCopyWithImpl<AbcTwinNormal_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinNormal_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinNormal.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinNormal_CCopyWith<$Res>
    implements $AbcTwinNormalCopyWith<$Res> {
  factory $AbcTwinNormal_CCopyWith(
          AbcTwinNormal_C value, $Res Function(AbcTwinNormal_C) _then) =
      _$AbcTwinNormal_CCopyWithImpl;
  @useResult
  $Res call({CTwinNormal field0});
}

/// @nodoc
class _$AbcTwinNormal_CCopyWithImpl<$Res>
    implements $AbcTwinNormal_CCopyWith<$Res> {
  _$AbcTwinNormal_CCopyWithImpl(this._self, this._then);

  final AbcTwinNormal_C _self;
  final $Res Function(AbcTwinNormal_C) _then;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinNormal_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinNormal,
    ));
  }
}

/// @nodoc

class AbcTwinNormal_JustInt extends AbcTwinNormal {
  const AbcTwinNormal_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinNormal_JustIntCopyWith<AbcTwinNormal_JustInt> get copyWith =>
      _$AbcTwinNormal_JustIntCopyWithImpl<AbcTwinNormal_JustInt>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinNormal_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinNormal.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinNormal_JustIntCopyWith<$Res>
    implements $AbcTwinNormalCopyWith<$Res> {
  factory $AbcTwinNormal_JustIntCopyWith(AbcTwinNormal_JustInt value,
          $Res Function(AbcTwinNormal_JustInt) _then) =
      _$AbcTwinNormal_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinNormal_JustIntCopyWithImpl<$Res>
    implements $AbcTwinNormal_JustIntCopyWith<$Res> {
  _$AbcTwinNormal_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinNormal_JustInt _self;
  final $Res Function(AbcTwinNormal_JustInt) _then;

  /// Create a copy of AbcTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinNormal_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
