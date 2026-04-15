// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$DistanceTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is DistanceTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinSync()';
  }
}

/// @nodoc
class $DistanceTwinSyncCopyWith<$Res> {
  $DistanceTwinSyncCopyWith(
      DistanceTwinSync _, $Res Function(DistanceTwinSync) __);
}

/// Adds pattern-matching-related methods to [DistanceTwinSync].
extension DistanceTwinSyncPatterns on DistanceTwinSync {
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
    TResult Function(DistanceTwinSync_Unknown value)? unknown,
    TResult Function(DistanceTwinSync_Map value)? map,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinSync_Map() when map != null:
        return map(_that);
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
    required TResult Function(DistanceTwinSync_Unknown value) unknown,
    required TResult Function(DistanceTwinSync_Map value) map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown():
        return unknown(_that);
      case DistanceTwinSync_Map():
        return map(_that);
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
    TResult? Function(DistanceTwinSync_Unknown value)? unknown,
    TResult? Function(DistanceTwinSync_Map value)? map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinSync_Map() when map != null:
        return map(_that);
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
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinSync_Map() when map != null:
        return map(_that.field0);
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
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown():
        return unknown();
      case DistanceTwinSync_Map():
        return map(_that.field0);
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSync_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinSync_Map() when map != null:
        return map(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class DistanceTwinSync_Unknown extends DistanceTwinSync {
  const DistanceTwinSync_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is DistanceTwinSync_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinSync.unknown()';
  }
}

/// @nodoc

class DistanceTwinSync_Map extends DistanceTwinSync {
  const DistanceTwinSync_Map(this.field0) : super._();

  final double field0;

  /// Create a copy of DistanceTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DistanceTwinSync_MapCopyWith<DistanceTwinSync_Map> get copyWith =>
      _$DistanceTwinSync_MapCopyWithImpl<DistanceTwinSync_Map>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DistanceTwinSync_Map &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'DistanceTwinSync.map(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $DistanceTwinSync_MapCopyWith<$Res>
    implements $DistanceTwinSyncCopyWith<$Res> {
  factory $DistanceTwinSync_MapCopyWith(DistanceTwinSync_Map value,
          $Res Function(DistanceTwinSync_Map) _then) =
      _$DistanceTwinSync_MapCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$DistanceTwinSync_MapCopyWithImpl<$Res>
    implements $DistanceTwinSync_MapCopyWith<$Res> {
  _$DistanceTwinSync_MapCopyWithImpl(this._self, this._then);

  final DistanceTwinSync_Map _self;
  final $Res Function(DistanceTwinSync_Map) _then;

  /// Create a copy of DistanceTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(DistanceTwinSync_Map(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemMixedTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSync()';
  }
}

/// @nodoc
class $EnumWithItemMixedTwinSyncCopyWith<$Res> {
  $EnumWithItemMixedTwinSyncCopyWith(
      EnumWithItemMixedTwinSync _, $Res Function(EnumWithItemMixedTwinSync) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemMixedTwinSync].
extension EnumWithItemMixedTwinSyncPatterns on EnumWithItemMixedTwinSync {
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
    TResult Function(EnumWithItemMixedTwinSync_A value)? a,
    TResult Function(EnumWithItemMixedTwinSync_B value)? b,
    TResult Function(EnumWithItemMixedTwinSync_C value)? c,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinSync_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinSync_C() when c != null:
        return c(_that);
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
    required TResult Function(EnumWithItemMixedTwinSync_A value) a,
    required TResult Function(EnumWithItemMixedTwinSync_B value) b,
    required TResult Function(EnumWithItemMixedTwinSync_C value) c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A():
        return a(_that);
      case EnumWithItemMixedTwinSync_B():
        return b(_that);
      case EnumWithItemMixedTwinSync_C():
        return c(_that);
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
    TResult? Function(EnumWithItemMixedTwinSync_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSync_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSync_C value)? c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinSync_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinSync_C() when c != null:
        return c(_that);
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
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A() when a != null:
        return a();
      case EnumWithItemMixedTwinSync_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinSync_C() when c != null:
        return c(_that.cField);
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
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A():
        return a();
      case EnumWithItemMixedTwinSync_B():
        return b(_that.field0);
      case EnumWithItemMixedTwinSync_C():
        return c(_that.cField);
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
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSync_A() when a != null:
        return a();
      case EnumWithItemMixedTwinSync_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinSync_C() when c != null:
        return c(_that.cField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemMixedTwinSync_A extends EnumWithItemMixedTwinSync {
  const EnumWithItemMixedTwinSync_A() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSync_A);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSync.a()';
  }
}

/// @nodoc

class EnumWithItemMixedTwinSync_B extends EnumWithItemMixedTwinSync {
  const EnumWithItemMixedTwinSync_B(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of EnumWithItemMixedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinSync_BCopyWith<EnumWithItemMixedTwinSync_B>
      get copyWith => _$EnumWithItemMixedTwinSync_BCopyWithImpl<
          EnumWithItemMixedTwinSync_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSync_B &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSync.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinSync_BCopyWith<$Res>
    implements $EnumWithItemMixedTwinSyncCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSync_BCopyWith(
          EnumWithItemMixedTwinSync_B value,
          $Res Function(EnumWithItemMixedTwinSync_B) _then) =
      _$EnumWithItemMixedTwinSync_BCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemMixedTwinSync_BCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinSync_BCopyWith<$Res> {
  _$EnumWithItemMixedTwinSync_BCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinSync_B _self;
  final $Res Function(EnumWithItemMixedTwinSync_B) _then;

  /// Create a copy of EnumWithItemMixedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemMixedTwinSync_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemMixedTwinSync_C extends EnumWithItemMixedTwinSync {
  const EnumWithItemMixedTwinSync_C({required this.cField}) : super._();

  final String cField;

  /// Create a copy of EnumWithItemMixedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinSync_CCopyWith<EnumWithItemMixedTwinSync_C>
      get copyWith => _$EnumWithItemMixedTwinSync_CCopyWithImpl<
          EnumWithItemMixedTwinSync_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSync_C &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @override
  int get hashCode => Object.hash(runtimeType, cField);

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSync.c(cField: $cField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinSync_CCopyWith<$Res>
    implements $EnumWithItemMixedTwinSyncCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSync_CCopyWith(
          EnumWithItemMixedTwinSync_C value,
          $Res Function(EnumWithItemMixedTwinSync_C) _then) =
      _$EnumWithItemMixedTwinSync_CCopyWithImpl;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class _$EnumWithItemMixedTwinSync_CCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinSync_CCopyWith<$Res> {
  _$EnumWithItemMixedTwinSync_CCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinSync_C _self;
  final $Res Function(EnumWithItemMixedTwinSync_C) _then;

  /// Create a copy of EnumWithItemMixedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? cField = null,
  }) {
    return _then(EnumWithItemMixedTwinSync_C(
      cField: null == cField
          ? _self.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemStructTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSync()';
  }
}

/// @nodoc
class $EnumWithItemStructTwinSyncCopyWith<$Res> {
  $EnumWithItemStructTwinSyncCopyWith(EnumWithItemStructTwinSync _,
      $Res Function(EnumWithItemStructTwinSync) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemStructTwinSync].
extension EnumWithItemStructTwinSyncPatterns on EnumWithItemStructTwinSync {
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
    TResult Function(EnumWithItemStructTwinSync_A value)? a,
    TResult Function(EnumWithItemStructTwinSync_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinSync_B() when b != null:
        return b(_that);
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
    required TResult Function(EnumWithItemStructTwinSync_A value) a,
    required TResult Function(EnumWithItemStructTwinSync_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A():
        return a(_that);
      case EnumWithItemStructTwinSync_B():
        return b(_that);
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
    TResult? Function(EnumWithItemStructTwinSync_A value)? a,
    TResult? Function(EnumWithItemStructTwinSync_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinSync_B() when b != null:
        return b(_that);
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
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinSync_B() when b != null:
        return b(_that.bField);
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
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A():
        return a(_that.aField);
      case EnumWithItemStructTwinSync_B():
        return b(_that.bField);
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
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSync_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinSync_B() when b != null:
        return b(_that.bField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemStructTwinSync_A extends EnumWithItemStructTwinSync {
  const EnumWithItemStructTwinSync_A({required this.aField}) : super._();

  final Uint8List aField;

  /// Create a copy of EnumWithItemStructTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinSync_ACopyWith<EnumWithItemStructTwinSync_A>
      get copyWith => _$EnumWithItemStructTwinSync_ACopyWithImpl<
          EnumWithItemStructTwinSync_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSync_A &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinSync.a(aField: $aField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinSync_ACopyWith<$Res>
    implements $EnumWithItemStructTwinSyncCopyWith<$Res> {
  factory $EnumWithItemStructTwinSync_ACopyWith(
          EnumWithItemStructTwinSync_A value,
          $Res Function(EnumWithItemStructTwinSync_A) _then) =
      _$EnumWithItemStructTwinSync_ACopyWithImpl;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class _$EnumWithItemStructTwinSync_ACopyWithImpl<$Res>
    implements $EnumWithItemStructTwinSync_ACopyWith<$Res> {
  _$EnumWithItemStructTwinSync_ACopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinSync_A _self;
  final $Res Function(EnumWithItemStructTwinSync_A) _then;

  /// Create a copy of EnumWithItemStructTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? aField = null,
  }) {
    return _then(EnumWithItemStructTwinSync_A(
      aField: null == aField
          ? _self.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemStructTwinSync_B extends EnumWithItemStructTwinSync {
  const EnumWithItemStructTwinSync_B({required this.bField}) : super._();

  final Int32List bField;

  /// Create a copy of EnumWithItemStructTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinSync_BCopyWith<EnumWithItemStructTwinSync_B>
      get copyWith => _$EnumWithItemStructTwinSync_BCopyWithImpl<
          EnumWithItemStructTwinSync_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSync_B &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinSync.b(bField: $bField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinSync_BCopyWith<$Res>
    implements $EnumWithItemStructTwinSyncCopyWith<$Res> {
  factory $EnumWithItemStructTwinSync_BCopyWith(
          EnumWithItemStructTwinSync_B value,
          $Res Function(EnumWithItemStructTwinSync_B) _then) =
      _$EnumWithItemStructTwinSync_BCopyWithImpl;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class _$EnumWithItemStructTwinSync_BCopyWithImpl<$Res>
    implements $EnumWithItemStructTwinSync_BCopyWith<$Res> {
  _$EnumWithItemStructTwinSync_BCopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinSync_B _self;
  final $Res Function(EnumWithItemStructTwinSync_B) _then;

  /// Create a copy of EnumWithItemStructTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? bField = null,
  }) {
    return _then(EnumWithItemStructTwinSync_B(
      bField: null == bField
          ? _self.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemTupleTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithItemTupleTwinSyncCopyWith<$Res> {
  $EnumWithItemTupleTwinSyncCopyWith(
      EnumWithItemTupleTwinSync _, $Res Function(EnumWithItemTupleTwinSync) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemTupleTwinSync].
extension EnumWithItemTupleTwinSyncPatterns on EnumWithItemTupleTwinSync {
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
    TResult Function(EnumWithItemTupleTwinSync_A value)? a,
    TResult Function(EnumWithItemTupleTwinSync_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinSync_B() when b != null:
        return b(_that);
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
    required TResult Function(EnumWithItemTupleTwinSync_A value) a,
    required TResult Function(EnumWithItemTupleTwinSync_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A():
        return a(_that);
      case EnumWithItemTupleTwinSync_B():
        return b(_that);
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
    TResult? Function(EnumWithItemTupleTwinSync_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSync_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinSync_B() when b != null:
        return b(_that);
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
    TResult Function(Uint8List field0)? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinSync_B() when b != null:
        return b(_that.field0);
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
    required TResult Function(Uint8List field0) a,
    required TResult Function(int field0) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A():
        return a(_that.field0);
      case EnumWithItemTupleTwinSync_B():
        return b(_that.field0);
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
    TResult? Function(Uint8List field0)? a,
    TResult? Function(int field0)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSync_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinSync_B() when b != null:
        return b(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemTupleTwinSync_A extends EnumWithItemTupleTwinSync {
  const EnumWithItemTupleTwinSync_A(this.field0) : super._();

  @override
  final Uint8List field0;

  /// Create a copy of EnumWithItemTupleTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinSync_ACopyWith<EnumWithItemTupleTwinSync_A>
      get copyWith => _$EnumWithItemTupleTwinSync_ACopyWithImpl<
          EnumWithItemTupleTwinSync_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSync_A &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSync.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinSync_ACopyWith<$Res>
    implements $EnumWithItemTupleTwinSyncCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSync_ACopyWith(
          EnumWithItemTupleTwinSync_A value,
          $Res Function(EnumWithItemTupleTwinSync_A) _then) =
      _$EnumWithItemTupleTwinSync_ACopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinSync_ACopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinSync_ACopyWith<$Res> {
  _$EnumWithItemTupleTwinSync_ACopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinSync_A _self;
  final $Res Function(EnumWithItemTupleTwinSync_A) _then;

  /// Create a copy of EnumWithItemTupleTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinSync_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemTupleTwinSync_B extends EnumWithItemTupleTwinSync {
  const EnumWithItemTupleTwinSync_B(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumWithItemTupleTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinSync_BCopyWith<EnumWithItemTupleTwinSync_B>
      get copyWith => _$EnumWithItemTupleTwinSync_BCopyWithImpl<
          EnumWithItemTupleTwinSync_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSync_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSync.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinSync_BCopyWith<$Res>
    implements $EnumWithItemTupleTwinSyncCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSync_BCopyWith(
          EnumWithItemTupleTwinSync_B value,
          $Res Function(EnumWithItemTupleTwinSync_B) _then) =
      _$EnumWithItemTupleTwinSync_BCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinSync_BCopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinSync_BCopyWith<$Res> {
  _$EnumWithItemTupleTwinSync_BCopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinSync_B _self;
  final $Res Function(EnumWithItemTupleTwinSync_B) _then;

  /// Create a copy of EnumWithItemTupleTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinSync_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$KitchenSinkTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is KitchenSinkTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinSync()';
  }
}

/// @nodoc
class $KitchenSinkTwinSyncCopyWith<$Res> {
  $KitchenSinkTwinSyncCopyWith(
      KitchenSinkTwinSync _, $Res Function(KitchenSinkTwinSync) __);
}

/// Adds pattern-matching-related methods to [KitchenSinkTwinSync].
extension KitchenSinkTwinSyncPatterns on KitchenSinkTwinSync {
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
    TResult Function(KitchenSinkTwinSync_Empty value)? empty,
    TResult Function(KitchenSinkTwinSync_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSync_Nested value)? nested,
    TResult Function(KitchenSinkTwinSync_Optional value)? optional,
    TResult Function(KitchenSinkTwinSync_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSync_Enums value)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinSync_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinSync_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinSync_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinSync_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinSync_Enums() when enums != null:
        return enums(_that);
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
    required TResult Function(KitchenSinkTwinSync_Empty value) empty,
    required TResult Function(KitchenSinkTwinSync_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSync_Nested value) nested,
    required TResult Function(KitchenSinkTwinSync_Optional value) optional,
    required TResult Function(KitchenSinkTwinSync_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSync_Enums value) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty():
        return empty(_that);
      case KitchenSinkTwinSync_Primitives():
        return primitives(_that);
      case KitchenSinkTwinSync_Nested():
        return nested(_that);
      case KitchenSinkTwinSync_Optional():
        return optional(_that);
      case KitchenSinkTwinSync_Buffer():
        return buffer(_that);
      case KitchenSinkTwinSync_Enums():
        return enums(_that);
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
    TResult? Function(KitchenSinkTwinSync_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSync_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSync_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSync_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSync_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSync_Enums value)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinSync_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinSync_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinSync_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinSync_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinSync_Enums() when enums != null:
        return enums(_that);
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
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSync field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSync field0)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinSync_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSync_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinSync_Enums() when enums != null:
        return enums(_that.field0);
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
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSync field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSync field0) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty():
        return empty();
      case KitchenSinkTwinSync_Primitives():
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSync_Nested():
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Optional():
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Buffer():
        return buffer(_that.field0);
      case KitchenSinkTwinSync_Enums():
        return enums(_that.field0);
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
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSync field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSync field0)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSync_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinSync_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSync_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSync_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinSync_Enums() when enums != null:
        return enums(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class KitchenSinkTwinSync_Empty extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Empty() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinSync.empty()';
  }
}

/// @nodoc

class KitchenSinkTwinSync_Primitives extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Primitives(
      {this.int32 = -1, required this.float64, required this.boolean})
      : super._();

  /// Dart field comment
  @JsonKey()
  final int int32;
  final double float64;
  final bool boolean;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSync_PrimitivesCopyWith<KitchenSinkTwinSync_Primitives>
      get copyWith => _$KitchenSinkTwinSync_PrimitivesCopyWithImpl<
          KitchenSinkTwinSync_Primitives>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Primitives &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @override
  String toString() {
    return 'KitchenSinkTwinSync.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSync_PrimitivesCopyWith<$Res>
    implements $KitchenSinkTwinSyncCopyWith<$Res> {
  factory $KitchenSinkTwinSync_PrimitivesCopyWith(
          KitchenSinkTwinSync_Primitives value,
          $Res Function(KitchenSinkTwinSync_Primitives) _then) =
      _$KitchenSinkTwinSync_PrimitivesCopyWithImpl;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class _$KitchenSinkTwinSync_PrimitivesCopyWithImpl<$Res>
    implements $KitchenSinkTwinSync_PrimitivesCopyWith<$Res> {
  _$KitchenSinkTwinSync_PrimitivesCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSync_Primitives _self;
  final $Res Function(KitchenSinkTwinSync_Primitives) _then;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(KitchenSinkTwinSync_Primitives(
      int32: null == int32
          ? _self.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: null == float64
          ? _self.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: null == boolean
          ? _self.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class KitchenSinkTwinSync_Nested extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Nested(this.field0,
      [this.field1 = const KitchenSinkTwinSync.empty()])
      : super._();

  final int field0;
  @JsonKey()
  final KitchenSinkTwinSync field1;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSync_NestedCopyWith<KitchenSinkTwinSync_Nested>
      get copyWith =>
          _$KitchenSinkTwinSync_NestedCopyWithImpl<KitchenSinkTwinSync_Nested>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Nested &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinSync.nested(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSync_NestedCopyWith<$Res>
    implements $KitchenSinkTwinSyncCopyWith<$Res> {
  factory $KitchenSinkTwinSync_NestedCopyWith(KitchenSinkTwinSync_Nested value,
          $Res Function(KitchenSinkTwinSync_Nested) _then) =
      _$KitchenSinkTwinSync_NestedCopyWithImpl;
  @useResult
  $Res call({int field0, KitchenSinkTwinSync field1});

  $KitchenSinkTwinSyncCopyWith<$Res> get field1;
}

/// @nodoc
class _$KitchenSinkTwinSync_NestedCopyWithImpl<$Res>
    implements $KitchenSinkTwinSync_NestedCopyWith<$Res> {
  _$KitchenSinkTwinSync_NestedCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSync_Nested _self;
  final $Res Function(KitchenSinkTwinSync_Nested) _then;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(KitchenSinkTwinSync_Nested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _self.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinSync,
    ));
  }

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSyncCopyWith<$Res> get field1 {
    return $KitchenSinkTwinSyncCopyWith<$Res>(_self.field1, (value) {
      return _then(_self.copyWith(field1: value));
    });
  }
}

/// @nodoc

class KitchenSinkTwinSync_Optional extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Optional([this.field0 = -1, this.field1])
      : super._();

  /// Comment on anonymous field
  @JsonKey()
  final int? field0;
  final int? field1;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSync_OptionalCopyWith<KitchenSinkTwinSync_Optional>
      get copyWith => _$KitchenSinkTwinSync_OptionalCopyWithImpl<
          KitchenSinkTwinSync_Optional>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Optional &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinSync.optional(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSync_OptionalCopyWith<$Res>
    implements $KitchenSinkTwinSyncCopyWith<$Res> {
  factory $KitchenSinkTwinSync_OptionalCopyWith(
          KitchenSinkTwinSync_Optional value,
          $Res Function(KitchenSinkTwinSync_Optional) _then) =
      _$KitchenSinkTwinSync_OptionalCopyWithImpl;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class _$KitchenSinkTwinSync_OptionalCopyWithImpl<$Res>
    implements $KitchenSinkTwinSync_OptionalCopyWith<$Res> {
  _$KitchenSinkTwinSync_OptionalCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSync_Optional _self;
  final $Res Function(KitchenSinkTwinSync_Optional) _then;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(KitchenSinkTwinSync_Optional(
      freezed == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      freezed == field1
          ? _self.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class KitchenSinkTwinSync_Buffer extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Buffer(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSync_BufferCopyWith<KitchenSinkTwinSync_Buffer>
      get copyWith =>
          _$KitchenSinkTwinSync_BufferCopyWithImpl<KitchenSinkTwinSync_Buffer>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'KitchenSinkTwinSync.buffer(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSync_BufferCopyWith<$Res>
    implements $KitchenSinkTwinSyncCopyWith<$Res> {
  factory $KitchenSinkTwinSync_BufferCopyWith(KitchenSinkTwinSync_Buffer value,
          $Res Function(KitchenSinkTwinSync_Buffer) _then) =
      _$KitchenSinkTwinSync_BufferCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$KitchenSinkTwinSync_BufferCopyWithImpl<$Res>
    implements $KitchenSinkTwinSync_BufferCopyWith<$Res> {
  _$KitchenSinkTwinSync_BufferCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSync_Buffer _self;
  final $Res Function(KitchenSinkTwinSync_Buffer) _then;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinSync_Buffer(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class KitchenSinkTwinSync_Enums extends KitchenSinkTwinSync {
  const KitchenSinkTwinSync_Enums([this.field0 = WeekdaysTwinSync.sunday])
      : super._();

  @JsonKey()
  final WeekdaysTwinSync field0;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSync_EnumsCopyWith<KitchenSinkTwinSync_Enums> get copyWith =>
      _$KitchenSinkTwinSync_EnumsCopyWithImpl<KitchenSinkTwinSync_Enums>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSync_Enums &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'KitchenSinkTwinSync.enums(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSync_EnumsCopyWith<$Res>
    implements $KitchenSinkTwinSyncCopyWith<$Res> {
  factory $KitchenSinkTwinSync_EnumsCopyWith(KitchenSinkTwinSync_Enums value,
          $Res Function(KitchenSinkTwinSync_Enums) _then) =
      _$KitchenSinkTwinSync_EnumsCopyWithImpl;
  @useResult
  $Res call({WeekdaysTwinSync field0});
}

/// @nodoc
class _$KitchenSinkTwinSync_EnumsCopyWithImpl<$Res>
    implements $KitchenSinkTwinSync_EnumsCopyWith<$Res> {
  _$KitchenSinkTwinSync_EnumsCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSync_Enums _self;
  final $Res Function(KitchenSinkTwinSync_Enums) _then;

  /// Create a copy of KitchenSinkTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinSync_Enums(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinSync,
    ));
  }
}

/// @nodoc
mixin _$MeasureTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'MeasureTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $MeasureTwinSyncCopyWith<$Res> {
  $MeasureTwinSyncCopyWith(
      MeasureTwinSync _, $Res Function(MeasureTwinSync) __);
}

/// Adds pattern-matching-related methods to [MeasureTwinSync].
extension MeasureTwinSyncPatterns on MeasureTwinSync {
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
    TResult Function(MeasureTwinSync_Speed value)? speed,
    TResult Function(MeasureTwinSync_Distance value)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinSync_Distance() when distance != null:
        return distance(_that);
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
    required TResult Function(MeasureTwinSync_Speed value) speed,
    required TResult Function(MeasureTwinSync_Distance value) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed():
        return speed(_that);
      case MeasureTwinSync_Distance():
        return distance(_that);
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
    TResult? Function(MeasureTwinSync_Speed value)? speed,
    TResult? Function(MeasureTwinSync_Distance value)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinSync_Distance() when distance != null:
        return distance(_that);
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
    TResult Function(SpeedTwinSync field0)? speed,
    TResult Function(DistanceTwinSync field0)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinSync_Distance() when distance != null:
        return distance(_that.field0);
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
    required TResult Function(SpeedTwinSync field0) speed,
    required TResult Function(DistanceTwinSync field0) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed():
        return speed(_that.field0);
      case MeasureTwinSync_Distance():
        return distance(_that.field0);
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
    TResult? Function(SpeedTwinSync field0)? speed,
    TResult? Function(DistanceTwinSync field0)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSync_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinSync_Distance() when distance != null:
        return distance(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class MeasureTwinSync_Speed extends MeasureTwinSync {
  const MeasureTwinSync_Speed(this.field0) : super._();

  @override
  final SpeedTwinSync field0;

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinSync_SpeedCopyWith<MeasureTwinSync_Speed> get copyWith =>
      _$MeasureTwinSync_SpeedCopyWithImpl<MeasureTwinSync_Speed>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSync_Speed &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinSync.speed(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinSync_SpeedCopyWith<$Res>
    implements $MeasureTwinSyncCopyWith<$Res> {
  factory $MeasureTwinSync_SpeedCopyWith(MeasureTwinSync_Speed value,
          $Res Function(MeasureTwinSync_Speed) _then) =
      _$MeasureTwinSync_SpeedCopyWithImpl;
  @useResult
  $Res call({SpeedTwinSync field0});

  $SpeedTwinSyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinSync_SpeedCopyWithImpl<$Res>
    implements $MeasureTwinSync_SpeedCopyWith<$Res> {
  _$MeasureTwinSync_SpeedCopyWithImpl(this._self, this._then);

  final MeasureTwinSync_Speed _self;
  final $Res Function(MeasureTwinSync_Speed) _then;

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinSync_Speed(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinSync,
    ));
  }

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinSyncCopyWith<$Res> get field0 {
    return $SpeedTwinSyncCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc

class MeasureTwinSync_Distance extends MeasureTwinSync {
  const MeasureTwinSync_Distance(this.field0) : super._();

  @override
  final DistanceTwinSync field0;

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinSync_DistanceCopyWith<MeasureTwinSync_Distance> get copyWith =>
      _$MeasureTwinSync_DistanceCopyWithImpl<MeasureTwinSync_Distance>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSync_Distance &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinSync.distance(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinSync_DistanceCopyWith<$Res>
    implements $MeasureTwinSyncCopyWith<$Res> {
  factory $MeasureTwinSync_DistanceCopyWith(MeasureTwinSync_Distance value,
          $Res Function(MeasureTwinSync_Distance) _then) =
      _$MeasureTwinSync_DistanceCopyWithImpl;
  @useResult
  $Res call({DistanceTwinSync field0});

  $DistanceTwinSyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinSync_DistanceCopyWithImpl<$Res>
    implements $MeasureTwinSync_DistanceCopyWith<$Res> {
  _$MeasureTwinSync_DistanceCopyWithImpl(this._self, this._then);

  final MeasureTwinSync_Distance _self;
  final $Res Function(MeasureTwinSync_Distance) _then;

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinSync_Distance(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinSync,
    ));
  }

  /// Create a copy of MeasureTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinSyncCopyWith<$Res> get field0 {
    return $DistanceTwinSyncCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$SpeedTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinSync()';
  }
}

/// @nodoc
class $SpeedTwinSyncCopyWith<$Res> {
  $SpeedTwinSyncCopyWith(SpeedTwinSync _, $Res Function(SpeedTwinSync) __);
}

/// Adds pattern-matching-related methods to [SpeedTwinSync].
extension SpeedTwinSyncPatterns on SpeedTwinSync {
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
    TResult Function(SpeedTwinSync_Unknown value)? unknown,
    TResult Function(SpeedTwinSync_GPS value)? gps,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinSync_GPS() when gps != null:
        return gps(_that);
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
    required TResult Function(SpeedTwinSync_Unknown value) unknown,
    required TResult Function(SpeedTwinSync_GPS value) gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown():
        return unknown(_that);
      case SpeedTwinSync_GPS():
        return gps(_that);
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
    TResult? Function(SpeedTwinSync_Unknown value)? unknown,
    TResult? Function(SpeedTwinSync_GPS value)? gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinSync_GPS() when gps != null:
        return gps(_that);
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
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinSync_GPS() when gps != null:
        return gps(_that.field0);
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
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown():
        return unknown();
      case SpeedTwinSync_GPS():
        return gps(_that.field0);
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSync_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinSync_GPS() when gps != null:
        return gps(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class SpeedTwinSync_Unknown extends SpeedTwinSync {
  const SpeedTwinSync_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinSync_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinSync.unknown()';
  }
}

/// @nodoc

class SpeedTwinSync_GPS extends SpeedTwinSync {
  const SpeedTwinSync_GPS(this.field0) : super._();

  final double field0;

  /// Create a copy of SpeedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SpeedTwinSync_GPSCopyWith<SpeedTwinSync_GPS> get copyWith =>
      _$SpeedTwinSync_GPSCopyWithImpl<SpeedTwinSync_GPS>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SpeedTwinSync_GPS &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SpeedTwinSync.gps(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SpeedTwinSync_GPSCopyWith<$Res>
    implements $SpeedTwinSyncCopyWith<$Res> {
  factory $SpeedTwinSync_GPSCopyWith(
          SpeedTwinSync_GPS value, $Res Function(SpeedTwinSync_GPS) _then) =
      _$SpeedTwinSync_GPSCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$SpeedTwinSync_GPSCopyWithImpl<$Res>
    implements $SpeedTwinSync_GPSCopyWith<$Res> {
  _$SpeedTwinSync_GPSCopyWithImpl(this._self, this._then);

  final SpeedTwinSync_GPS _self;
  final $Res Function(SpeedTwinSync_GPS) _then;

  /// Create a copy of SpeedTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SpeedTwinSync_GPS(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

// dart format on
