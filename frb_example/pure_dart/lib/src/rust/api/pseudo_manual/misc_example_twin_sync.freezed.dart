// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_example_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AbcTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'AbcTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $AbcTwinSyncCopyWith<$Res> {
  $AbcTwinSyncCopyWith(AbcTwinSync _, $Res Function(AbcTwinSync) __);
}

/// Adds pattern-matching-related methods to [AbcTwinSync].
extension AbcTwinSyncPatterns on AbcTwinSync {
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
    TResult Function(AbcTwinSync_A value)? a,
    TResult Function(AbcTwinSync_B value)? b,
    TResult Function(AbcTwinSync_C value)? c,
    TResult Function(AbcTwinSync_JustInt value)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A() when a != null:
        return a(_that);
      case AbcTwinSync_B() when b != null:
        return b(_that);
      case AbcTwinSync_C() when c != null:
        return c(_that);
      case AbcTwinSync_JustInt() when justInt != null:
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
    required TResult Function(AbcTwinSync_A value) a,
    required TResult Function(AbcTwinSync_B value) b,
    required TResult Function(AbcTwinSync_C value) c,
    required TResult Function(AbcTwinSync_JustInt value) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A():
        return a(_that);
      case AbcTwinSync_B():
        return b(_that);
      case AbcTwinSync_C():
        return c(_that);
      case AbcTwinSync_JustInt():
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
    TResult? Function(AbcTwinSync_A value)? a,
    TResult? Function(AbcTwinSync_B value)? b,
    TResult? Function(AbcTwinSync_C value)? c,
    TResult? Function(AbcTwinSync_JustInt value)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A() when a != null:
        return a(_that);
      case AbcTwinSync_B() when b != null:
        return b(_that);
      case AbcTwinSync_C() when c != null:
        return c(_that);
      case AbcTwinSync_JustInt() when justInt != null:
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
    TResult Function(ATwinSync field0)? a,
    TResult Function(BTwinSync field0)? b,
    TResult Function(CTwinSync field0)? c,
    TResult Function(int field0)? justInt,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A() when a != null:
        return a(_that.field0);
      case AbcTwinSync_B() when b != null:
        return b(_that.field0);
      case AbcTwinSync_C() when c != null:
        return c(_that.field0);
      case AbcTwinSync_JustInt() when justInt != null:
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
    required TResult Function(ATwinSync field0) a,
    required TResult Function(BTwinSync field0) b,
    required TResult Function(CTwinSync field0) c,
    required TResult Function(int field0) justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A():
        return a(_that.field0);
      case AbcTwinSync_B():
        return b(_that.field0);
      case AbcTwinSync_C():
        return c(_that.field0);
      case AbcTwinSync_JustInt():
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
    TResult? Function(ATwinSync field0)? a,
    TResult? Function(BTwinSync field0)? b,
    TResult? Function(CTwinSync field0)? c,
    TResult? Function(int field0)? justInt,
  }) {
    final _that = this;
    switch (_that) {
      case AbcTwinSync_A() when a != null:
        return a(_that.field0);
      case AbcTwinSync_B() when b != null:
        return b(_that.field0);
      case AbcTwinSync_C() when c != null:
        return c(_that.field0);
      case AbcTwinSync_JustInt() when justInt != null:
        return justInt(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AbcTwinSync_A extends AbcTwinSync {
  const AbcTwinSync_A(this.field0) : super._();

  @override
  final ATwinSync field0;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSync_ACopyWith<AbcTwinSync_A> get copyWith =>
      _$AbcTwinSync_ACopyWithImpl<AbcTwinSync_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSync_A &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSync.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSync_ACopyWith<$Res>
    implements $AbcTwinSyncCopyWith<$Res> {
  factory $AbcTwinSync_ACopyWith(
          AbcTwinSync_A value, $Res Function(AbcTwinSync_A) _then) =
      _$AbcTwinSync_ACopyWithImpl;
  @useResult
  $Res call({ATwinSync field0});
}

/// @nodoc
class _$AbcTwinSync_ACopyWithImpl<$Res>
    implements $AbcTwinSync_ACopyWith<$Res> {
  _$AbcTwinSync_ACopyWithImpl(this._self, this._then);

  final AbcTwinSync_A _self;
  final $Res Function(AbcTwinSync_A) _then;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSync_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ATwinSync,
    ));
  }
}

/// @nodoc

class AbcTwinSync_B extends AbcTwinSync {
  const AbcTwinSync_B(this.field0) : super._();

  @override
  final BTwinSync field0;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSync_BCopyWith<AbcTwinSync_B> get copyWith =>
      _$AbcTwinSync_BCopyWithImpl<AbcTwinSync_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSync_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSync.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSync_BCopyWith<$Res>
    implements $AbcTwinSyncCopyWith<$Res> {
  factory $AbcTwinSync_BCopyWith(
          AbcTwinSync_B value, $Res Function(AbcTwinSync_B) _then) =
      _$AbcTwinSync_BCopyWithImpl;
  @useResult
  $Res call({BTwinSync field0});
}

/// @nodoc
class _$AbcTwinSync_BCopyWithImpl<$Res>
    implements $AbcTwinSync_BCopyWith<$Res> {
  _$AbcTwinSync_BCopyWithImpl(this._self, this._then);

  final AbcTwinSync_B _self;
  final $Res Function(AbcTwinSync_B) _then;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSync_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BTwinSync,
    ));
  }
}

/// @nodoc

class AbcTwinSync_C extends AbcTwinSync {
  const AbcTwinSync_C(this.field0) : super._();

  @override
  final CTwinSync field0;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSync_CCopyWith<AbcTwinSync_C> get copyWith =>
      _$AbcTwinSync_CCopyWithImpl<AbcTwinSync_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSync_C &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSync.c(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSync_CCopyWith<$Res>
    implements $AbcTwinSyncCopyWith<$Res> {
  factory $AbcTwinSync_CCopyWith(
          AbcTwinSync_C value, $Res Function(AbcTwinSync_C) _then) =
      _$AbcTwinSync_CCopyWithImpl;
  @useResult
  $Res call({CTwinSync field0});
}

/// @nodoc
class _$AbcTwinSync_CCopyWithImpl<$Res>
    implements $AbcTwinSync_CCopyWith<$Res> {
  _$AbcTwinSync_CCopyWithImpl(this._self, this._then);

  final AbcTwinSync_C _self;
  final $Res Function(AbcTwinSync_C) _then;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSync_C(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CTwinSync,
    ));
  }
}

/// @nodoc

class AbcTwinSync_JustInt extends AbcTwinSync {
  const AbcTwinSync_JustInt(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AbcTwinSync_JustIntCopyWith<AbcTwinSync_JustInt> get copyWith =>
      _$AbcTwinSync_JustIntCopyWithImpl<AbcTwinSync_JustInt>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AbcTwinSync_JustInt &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AbcTwinSync.justInt(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AbcTwinSync_JustIntCopyWith<$Res>
    implements $AbcTwinSyncCopyWith<$Res> {
  factory $AbcTwinSync_JustIntCopyWith(
          AbcTwinSync_JustInt value, $Res Function(AbcTwinSync_JustInt) _then) =
      _$AbcTwinSync_JustIntCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$AbcTwinSync_JustIntCopyWithImpl<$Res>
    implements $AbcTwinSync_JustIntCopyWith<$Res> {
  _$AbcTwinSync_JustIntCopyWithImpl(this._self, this._then);

  final AbcTwinSync_JustInt _self;
  final $Res Function(AbcTwinSync_JustInt) _then;

  /// Create a copy of AbcTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AbcTwinSync_JustInt(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
