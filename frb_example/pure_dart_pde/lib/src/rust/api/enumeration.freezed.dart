// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$DistanceTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is DistanceTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinNormal()';
  }
}

/// @nodoc
class $DistanceTwinNormalCopyWith<$Res> {
  $DistanceTwinNormalCopyWith(
      DistanceTwinNormal _, $Res Function(DistanceTwinNormal) __);
}

/// Adds pattern-matching-related methods to [DistanceTwinNormal].
extension DistanceTwinNormalPatterns on DistanceTwinNormal {
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
    TResult Function(DistanceTwinNormal_Unknown value)? unknown,
    TResult Function(DistanceTwinNormal_Map value)? map,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinNormal_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinNormal_Map() when map != null:
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
    required TResult Function(DistanceTwinNormal_Unknown value) unknown,
    required TResult Function(DistanceTwinNormal_Map value) map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinNormal_Unknown():
        return unknown(_that);
      case DistanceTwinNormal_Map():
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
    TResult? Function(DistanceTwinNormal_Unknown value)? unknown,
    TResult? Function(DistanceTwinNormal_Map value)? map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinNormal_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinNormal_Map() when map != null:
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
      case DistanceTwinNormal_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinNormal_Map() when map != null:
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
      case DistanceTwinNormal_Unknown():
        return unknown();
      case DistanceTwinNormal_Map():
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
      case DistanceTwinNormal_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinNormal_Map() when map != null:
        return map(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class DistanceTwinNormal_Unknown extends DistanceTwinNormal {
  const DistanceTwinNormal_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DistanceTwinNormal_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinNormal.unknown()';
  }
}

/// @nodoc

class DistanceTwinNormal_Map extends DistanceTwinNormal {
  const DistanceTwinNormal_Map(this.field0) : super._();

  final double field0;

  /// Create a copy of DistanceTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DistanceTwinNormal_MapCopyWith<DistanceTwinNormal_Map> get copyWith =>
      _$DistanceTwinNormal_MapCopyWithImpl<DistanceTwinNormal_Map>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DistanceTwinNormal_Map &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'DistanceTwinNormal.map(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $DistanceTwinNormal_MapCopyWith<$Res>
    implements $DistanceTwinNormalCopyWith<$Res> {
  factory $DistanceTwinNormal_MapCopyWith(DistanceTwinNormal_Map value,
          $Res Function(DistanceTwinNormal_Map) _then) =
      _$DistanceTwinNormal_MapCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$DistanceTwinNormal_MapCopyWithImpl<$Res>
    implements $DistanceTwinNormal_MapCopyWith<$Res> {
  _$DistanceTwinNormal_MapCopyWithImpl(this._self, this._then);

  final DistanceTwinNormal_Map _self;
  final $Res Function(DistanceTwinNormal_Map) _then;

  /// Create a copy of DistanceTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(DistanceTwinNormal_Map(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemMixedTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinNormal()';
  }
}

/// @nodoc
class $EnumWithItemMixedTwinNormalCopyWith<$Res> {
  $EnumWithItemMixedTwinNormalCopyWith(EnumWithItemMixedTwinNormal _,
      $Res Function(EnumWithItemMixedTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemMixedTwinNormal].
extension EnumWithItemMixedTwinNormalPatterns on EnumWithItemMixedTwinNormal {
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
    TResult Function(EnumWithItemMixedTwinNormal_A value)? a,
    TResult Function(EnumWithItemMixedTwinNormal_B value)? b,
    TResult Function(EnumWithItemMixedTwinNormal_C value)? c,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinNormal_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinNormal_C() when c != null:
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
    required TResult Function(EnumWithItemMixedTwinNormal_A value) a,
    required TResult Function(EnumWithItemMixedTwinNormal_B value) b,
    required TResult Function(EnumWithItemMixedTwinNormal_C value) c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinNormal_A():
        return a(_that);
      case EnumWithItemMixedTwinNormal_B():
        return b(_that);
      case EnumWithItemMixedTwinNormal_C():
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
    TResult? Function(EnumWithItemMixedTwinNormal_A value)? a,
    TResult? Function(EnumWithItemMixedTwinNormal_B value)? b,
    TResult? Function(EnumWithItemMixedTwinNormal_C value)? c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinNormal_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinNormal_C() when c != null:
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
      case EnumWithItemMixedTwinNormal_A() when a != null:
        return a();
      case EnumWithItemMixedTwinNormal_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinNormal_C() when c != null:
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
      case EnumWithItemMixedTwinNormal_A():
        return a();
      case EnumWithItemMixedTwinNormal_B():
        return b(_that.field0);
      case EnumWithItemMixedTwinNormal_C():
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
      case EnumWithItemMixedTwinNormal_A() when a != null:
        return a();
      case EnumWithItemMixedTwinNormal_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinNormal_C() when c != null:
        return c(_that.cField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemMixedTwinNormal_A extends EnumWithItemMixedTwinNormal {
  const EnumWithItemMixedTwinNormal_A() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinNormal_A);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinNormal.a()';
  }
}

/// @nodoc

class EnumWithItemMixedTwinNormal_B extends EnumWithItemMixedTwinNormal {
  const EnumWithItemMixedTwinNormal_B(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of EnumWithItemMixedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinNormal_BCopyWith<EnumWithItemMixedTwinNormal_B>
      get copyWith => _$EnumWithItemMixedTwinNormal_BCopyWithImpl<
          EnumWithItemMixedTwinNormal_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinNormal_B &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemMixedTwinNormal.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinNormal_BCopyWith<$Res>
    implements $EnumWithItemMixedTwinNormalCopyWith<$Res> {
  factory $EnumWithItemMixedTwinNormal_BCopyWith(
          EnumWithItemMixedTwinNormal_B value,
          $Res Function(EnumWithItemMixedTwinNormal_B) _then) =
      _$EnumWithItemMixedTwinNormal_BCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemMixedTwinNormal_BCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinNormal_BCopyWith<$Res> {
  _$EnumWithItemMixedTwinNormal_BCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinNormal_B _self;
  final $Res Function(EnumWithItemMixedTwinNormal_B) _then;

  /// Create a copy of EnumWithItemMixedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemMixedTwinNormal_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemMixedTwinNormal_C extends EnumWithItemMixedTwinNormal {
  const EnumWithItemMixedTwinNormal_C({required this.cField}) : super._();

  final String cField;

  /// Create a copy of EnumWithItemMixedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinNormal_CCopyWith<EnumWithItemMixedTwinNormal_C>
      get copyWith => _$EnumWithItemMixedTwinNormal_CCopyWithImpl<
          EnumWithItemMixedTwinNormal_C>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinNormal_C &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @override
  int get hashCode => Object.hash(runtimeType, cField);

  @override
  String toString() {
    return 'EnumWithItemMixedTwinNormal.c(cField: $cField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinNormal_CCopyWith<$Res>
    implements $EnumWithItemMixedTwinNormalCopyWith<$Res> {
  factory $EnumWithItemMixedTwinNormal_CCopyWith(
          EnumWithItemMixedTwinNormal_C value,
          $Res Function(EnumWithItemMixedTwinNormal_C) _then) =
      _$EnumWithItemMixedTwinNormal_CCopyWithImpl;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class _$EnumWithItemMixedTwinNormal_CCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinNormal_CCopyWith<$Res> {
  _$EnumWithItemMixedTwinNormal_CCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinNormal_C _self;
  final $Res Function(EnumWithItemMixedTwinNormal_C) _then;

  /// Create a copy of EnumWithItemMixedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? cField = null,
  }) {
    return _then(EnumWithItemMixedTwinNormal_C(
      cField: null == cField
          ? _self.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemStructTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemStructTwinNormal()';
  }
}

/// @nodoc
class $EnumWithItemStructTwinNormalCopyWith<$Res> {
  $EnumWithItemStructTwinNormalCopyWith(EnumWithItemStructTwinNormal _,
      $Res Function(EnumWithItemStructTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemStructTwinNormal].
extension EnumWithItemStructTwinNormalPatterns on EnumWithItemStructTwinNormal {
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
    TResult Function(EnumWithItemStructTwinNormal_A value)? a,
    TResult Function(EnumWithItemStructTwinNormal_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinNormal_B() when b != null:
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
    required TResult Function(EnumWithItemStructTwinNormal_A value) a,
    required TResult Function(EnumWithItemStructTwinNormal_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinNormal_A():
        return a(_that);
      case EnumWithItemStructTwinNormal_B():
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
    TResult? Function(EnumWithItemStructTwinNormal_A value)? a,
    TResult? Function(EnumWithItemStructTwinNormal_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinNormal_B() when b != null:
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
      case EnumWithItemStructTwinNormal_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinNormal_B() when b != null:
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
      case EnumWithItemStructTwinNormal_A():
        return a(_that.aField);
      case EnumWithItemStructTwinNormal_B():
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
      case EnumWithItemStructTwinNormal_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinNormal_B() when b != null:
        return b(_that.bField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemStructTwinNormal_A extends EnumWithItemStructTwinNormal {
  const EnumWithItemStructTwinNormal_A({required this.aField}) : super._();

  final Uint8List aField;

  /// Create a copy of EnumWithItemStructTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinNormal_ACopyWith<EnumWithItemStructTwinNormal_A>
      get copyWith => _$EnumWithItemStructTwinNormal_ACopyWithImpl<
          EnumWithItemStructTwinNormal_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinNormal_A &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinNormal.a(aField: $aField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinNormal_ACopyWith<$Res>
    implements $EnumWithItemStructTwinNormalCopyWith<$Res> {
  factory $EnumWithItemStructTwinNormal_ACopyWith(
          EnumWithItemStructTwinNormal_A value,
          $Res Function(EnumWithItemStructTwinNormal_A) _then) =
      _$EnumWithItemStructTwinNormal_ACopyWithImpl;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class _$EnumWithItemStructTwinNormal_ACopyWithImpl<$Res>
    implements $EnumWithItemStructTwinNormal_ACopyWith<$Res> {
  _$EnumWithItemStructTwinNormal_ACopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinNormal_A _self;
  final $Res Function(EnumWithItemStructTwinNormal_A) _then;

  /// Create a copy of EnumWithItemStructTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? aField = null,
  }) {
    return _then(EnumWithItemStructTwinNormal_A(
      aField: null == aField
          ? _self.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemStructTwinNormal_B extends EnumWithItemStructTwinNormal {
  const EnumWithItemStructTwinNormal_B({required this.bField}) : super._();

  final Int32List bField;

  /// Create a copy of EnumWithItemStructTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinNormal_BCopyWith<EnumWithItemStructTwinNormal_B>
      get copyWith => _$EnumWithItemStructTwinNormal_BCopyWithImpl<
          EnumWithItemStructTwinNormal_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinNormal_B &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinNormal.b(bField: $bField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinNormal_BCopyWith<$Res>
    implements $EnumWithItemStructTwinNormalCopyWith<$Res> {
  factory $EnumWithItemStructTwinNormal_BCopyWith(
          EnumWithItemStructTwinNormal_B value,
          $Res Function(EnumWithItemStructTwinNormal_B) _then) =
      _$EnumWithItemStructTwinNormal_BCopyWithImpl;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class _$EnumWithItemStructTwinNormal_BCopyWithImpl<$Res>
    implements $EnumWithItemStructTwinNormal_BCopyWith<$Res> {
  _$EnumWithItemStructTwinNormal_BCopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinNormal_B _self;
  final $Res Function(EnumWithItemStructTwinNormal_B) _then;

  /// Create a copy of EnumWithItemStructTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? bField = null,
  }) {
    return _then(EnumWithItemStructTwinNormal_B(
      bField: null == bField
          ? _self.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemTupleTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithItemTupleTwinNormalCopyWith<$Res> {
  $EnumWithItemTupleTwinNormalCopyWith(EnumWithItemTupleTwinNormal _,
      $Res Function(EnumWithItemTupleTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemTupleTwinNormal].
extension EnumWithItemTupleTwinNormalPatterns on EnumWithItemTupleTwinNormal {
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
    TResult Function(EnumWithItemTupleTwinNormal_A value)? a,
    TResult Function(EnumWithItemTupleTwinNormal_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinNormal_B() when b != null:
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
    required TResult Function(EnumWithItemTupleTwinNormal_A value) a,
    required TResult Function(EnumWithItemTupleTwinNormal_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinNormal_A():
        return a(_that);
      case EnumWithItemTupleTwinNormal_B():
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
    TResult? Function(EnumWithItemTupleTwinNormal_A value)? a,
    TResult? Function(EnumWithItemTupleTwinNormal_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinNormal_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinNormal_B() when b != null:
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
      case EnumWithItemTupleTwinNormal_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinNormal_B() when b != null:
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
      case EnumWithItemTupleTwinNormal_A():
        return a(_that.field0);
      case EnumWithItemTupleTwinNormal_B():
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
      case EnumWithItemTupleTwinNormal_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinNormal_B() when b != null:
        return b(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemTupleTwinNormal_A extends EnumWithItemTupleTwinNormal {
  const EnumWithItemTupleTwinNormal_A(this.field0) : super._();

  @override
  final Uint8List field0;

  /// Create a copy of EnumWithItemTupleTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinNormal_ACopyWith<EnumWithItemTupleTwinNormal_A>
      get copyWith => _$EnumWithItemTupleTwinNormal_ACopyWithImpl<
          EnumWithItemTupleTwinNormal_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinNormal_A &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinNormal.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinNormal_ACopyWith<$Res>
    implements $EnumWithItemTupleTwinNormalCopyWith<$Res> {
  factory $EnumWithItemTupleTwinNormal_ACopyWith(
          EnumWithItemTupleTwinNormal_A value,
          $Res Function(EnumWithItemTupleTwinNormal_A) _then) =
      _$EnumWithItemTupleTwinNormal_ACopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinNormal_ACopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinNormal_ACopyWith<$Res> {
  _$EnumWithItemTupleTwinNormal_ACopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinNormal_A _self;
  final $Res Function(EnumWithItemTupleTwinNormal_A) _then;

  /// Create a copy of EnumWithItemTupleTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinNormal_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemTupleTwinNormal_B extends EnumWithItemTupleTwinNormal {
  const EnumWithItemTupleTwinNormal_B(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumWithItemTupleTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinNormal_BCopyWith<EnumWithItemTupleTwinNormal_B>
      get copyWith => _$EnumWithItemTupleTwinNormal_BCopyWithImpl<
          EnumWithItemTupleTwinNormal_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinNormal_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithItemTupleTwinNormal.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinNormal_BCopyWith<$Res>
    implements $EnumWithItemTupleTwinNormalCopyWith<$Res> {
  factory $EnumWithItemTupleTwinNormal_BCopyWith(
          EnumWithItemTupleTwinNormal_B value,
          $Res Function(EnumWithItemTupleTwinNormal_B) _then) =
      _$EnumWithItemTupleTwinNormal_BCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinNormal_BCopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinNormal_BCopyWith<$Res> {
  _$EnumWithItemTupleTwinNormal_BCopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinNormal_B _self;
  final $Res Function(EnumWithItemTupleTwinNormal_B) _then;

  /// Create a copy of EnumWithItemTupleTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinNormal_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$KitchenSinkTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is KitchenSinkTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinNormal()';
  }
}

/// @nodoc
class $KitchenSinkTwinNormalCopyWith<$Res> {
  $KitchenSinkTwinNormalCopyWith(
      KitchenSinkTwinNormal _, $Res Function(KitchenSinkTwinNormal) __);
}

/// Adds pattern-matching-related methods to [KitchenSinkTwinNormal].
extension KitchenSinkTwinNormalPatterns on KitchenSinkTwinNormal {
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
    TResult Function(KitchenSinkTwinNormal_Empty value)? empty,
    TResult Function(KitchenSinkTwinNormal_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinNormal_Nested value)? nested,
    TResult Function(KitchenSinkTwinNormal_Optional value)? optional,
    TResult Function(KitchenSinkTwinNormal_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinNormal_Enums value)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinNormal_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinNormal_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinNormal_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinNormal_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinNormal_Enums() when enums != null:
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
    required TResult Function(KitchenSinkTwinNormal_Empty value) empty,
    required TResult Function(KitchenSinkTwinNormal_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinNormal_Nested value) nested,
    required TResult Function(KitchenSinkTwinNormal_Optional value) optional,
    required TResult Function(KitchenSinkTwinNormal_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinNormal_Enums value) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty():
        return empty(_that);
      case KitchenSinkTwinNormal_Primitives():
        return primitives(_that);
      case KitchenSinkTwinNormal_Nested():
        return nested(_that);
      case KitchenSinkTwinNormal_Optional():
        return optional(_that);
      case KitchenSinkTwinNormal_Buffer():
        return buffer(_that);
      case KitchenSinkTwinNormal_Enums():
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
    TResult? Function(KitchenSinkTwinNormal_Empty value)? empty,
    TResult? Function(KitchenSinkTwinNormal_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinNormal_Nested value)? nested,
    TResult? Function(KitchenSinkTwinNormal_Optional value)? optional,
    TResult? Function(KitchenSinkTwinNormal_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinNormal_Enums value)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinNormal_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinNormal_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinNormal_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinNormal_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinNormal_Enums() when enums != null:
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
    TResult Function(int field0, KitchenSinkTwinNormal field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinNormal field0)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinNormal_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinNormal_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinNormal_Enums() when enums != null:
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
    required TResult Function(int field0, KitchenSinkTwinNormal field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinNormal field0) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty():
        return empty();
      case KitchenSinkTwinNormal_Primitives():
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinNormal_Nested():
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Optional():
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Buffer():
        return buffer(_that.field0);
      case KitchenSinkTwinNormal_Enums():
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
    TResult? Function(int field0, KitchenSinkTwinNormal field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinNormal field0)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinNormal_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinNormal_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinNormal_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinNormal_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinNormal_Enums() when enums != null:
        return enums(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class KitchenSinkTwinNormal_Empty extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Empty() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.empty()';
  }
}

/// @nodoc

class KitchenSinkTwinNormal_Primitives extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Primitives(
      {this.int32 = -1, required this.float64, required this.boolean})
      : super._();

  /// Dart field comment
  @JsonKey()
  final int int32;
  final double float64;
  final bool boolean;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormal_PrimitivesCopyWith<KitchenSinkTwinNormal_Primitives>
      get copyWith => _$KitchenSinkTwinNormal_PrimitivesCopyWithImpl<
          KitchenSinkTwinNormal_Primitives>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Primitives &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinNormal_PrimitivesCopyWith<$Res>
    implements $KitchenSinkTwinNormalCopyWith<$Res> {
  factory $KitchenSinkTwinNormal_PrimitivesCopyWith(
          KitchenSinkTwinNormal_Primitives value,
          $Res Function(KitchenSinkTwinNormal_Primitives) _then) =
      _$KitchenSinkTwinNormal_PrimitivesCopyWithImpl;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class _$KitchenSinkTwinNormal_PrimitivesCopyWithImpl<$Res>
    implements $KitchenSinkTwinNormal_PrimitivesCopyWith<$Res> {
  _$KitchenSinkTwinNormal_PrimitivesCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinNormal_Primitives _self;
  final $Res Function(KitchenSinkTwinNormal_Primitives) _then;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(KitchenSinkTwinNormal_Primitives(
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

class KitchenSinkTwinNormal_Nested extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Nested(this.field0,
      [this.field1 = const KitchenSinkTwinNormal.empty()])
      : super._();

  final int field0;
  @JsonKey()
  final KitchenSinkTwinNormal field1;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormal_NestedCopyWith<KitchenSinkTwinNormal_Nested>
      get copyWith => _$KitchenSinkTwinNormal_NestedCopyWithImpl<
          KitchenSinkTwinNormal_Nested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Nested &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.nested(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinNormal_NestedCopyWith<$Res>
    implements $KitchenSinkTwinNormalCopyWith<$Res> {
  factory $KitchenSinkTwinNormal_NestedCopyWith(
          KitchenSinkTwinNormal_Nested value,
          $Res Function(KitchenSinkTwinNormal_Nested) _then) =
      _$KitchenSinkTwinNormal_NestedCopyWithImpl;
  @useResult
  $Res call({int field0, KitchenSinkTwinNormal field1});

  $KitchenSinkTwinNormalCopyWith<$Res> get field1;
}

/// @nodoc
class _$KitchenSinkTwinNormal_NestedCopyWithImpl<$Res>
    implements $KitchenSinkTwinNormal_NestedCopyWith<$Res> {
  _$KitchenSinkTwinNormal_NestedCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinNormal_Nested _self;
  final $Res Function(KitchenSinkTwinNormal_Nested) _then;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(KitchenSinkTwinNormal_Nested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _self.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinNormal,
    ));
  }

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormalCopyWith<$Res> get field1 {
    return $KitchenSinkTwinNormalCopyWith<$Res>(_self.field1, (value) {
      return _then(_self.copyWith(field1: value));
    });
  }
}

/// @nodoc

class KitchenSinkTwinNormal_Optional extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Optional([this.field0 = -1, this.field1])
      : super._();

  /// Comment on anonymous field
  @JsonKey()
  final int? field0;
  final int? field1;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormal_OptionalCopyWith<KitchenSinkTwinNormal_Optional>
      get copyWith => _$KitchenSinkTwinNormal_OptionalCopyWithImpl<
          KitchenSinkTwinNormal_Optional>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Optional &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.optional(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinNormal_OptionalCopyWith<$Res>
    implements $KitchenSinkTwinNormalCopyWith<$Res> {
  factory $KitchenSinkTwinNormal_OptionalCopyWith(
          KitchenSinkTwinNormal_Optional value,
          $Res Function(KitchenSinkTwinNormal_Optional) _then) =
      _$KitchenSinkTwinNormal_OptionalCopyWithImpl;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class _$KitchenSinkTwinNormal_OptionalCopyWithImpl<$Res>
    implements $KitchenSinkTwinNormal_OptionalCopyWith<$Res> {
  _$KitchenSinkTwinNormal_OptionalCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinNormal_Optional _self;
  final $Res Function(KitchenSinkTwinNormal_Optional) _then;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(KitchenSinkTwinNormal_Optional(
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

class KitchenSinkTwinNormal_Buffer extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Buffer(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormal_BufferCopyWith<KitchenSinkTwinNormal_Buffer>
      get copyWith => _$KitchenSinkTwinNormal_BufferCopyWithImpl<
          KitchenSinkTwinNormal_Buffer>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.buffer(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinNormal_BufferCopyWith<$Res>
    implements $KitchenSinkTwinNormalCopyWith<$Res> {
  factory $KitchenSinkTwinNormal_BufferCopyWith(
          KitchenSinkTwinNormal_Buffer value,
          $Res Function(KitchenSinkTwinNormal_Buffer) _then) =
      _$KitchenSinkTwinNormal_BufferCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$KitchenSinkTwinNormal_BufferCopyWithImpl<$Res>
    implements $KitchenSinkTwinNormal_BufferCopyWith<$Res> {
  _$KitchenSinkTwinNormal_BufferCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinNormal_Buffer _self;
  final $Res Function(KitchenSinkTwinNormal_Buffer) _then;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinNormal_Buffer(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class KitchenSinkTwinNormal_Enums extends KitchenSinkTwinNormal {
  const KitchenSinkTwinNormal_Enums([this.field0 = WeekdaysTwinNormal.sunday])
      : super._();

  @JsonKey()
  final WeekdaysTwinNormal field0;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinNormal_EnumsCopyWith<KitchenSinkTwinNormal_Enums>
      get copyWith => _$KitchenSinkTwinNormal_EnumsCopyWithImpl<
          KitchenSinkTwinNormal_Enums>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinNormal_Enums &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'KitchenSinkTwinNormal.enums(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinNormal_EnumsCopyWith<$Res>
    implements $KitchenSinkTwinNormalCopyWith<$Res> {
  factory $KitchenSinkTwinNormal_EnumsCopyWith(
          KitchenSinkTwinNormal_Enums value,
          $Res Function(KitchenSinkTwinNormal_Enums) _then) =
      _$KitchenSinkTwinNormal_EnumsCopyWithImpl;
  @useResult
  $Res call({WeekdaysTwinNormal field0});
}

/// @nodoc
class _$KitchenSinkTwinNormal_EnumsCopyWithImpl<$Res>
    implements $KitchenSinkTwinNormal_EnumsCopyWith<$Res> {
  _$KitchenSinkTwinNormal_EnumsCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinNormal_Enums _self;
  final $Res Function(KitchenSinkTwinNormal_Enums) _then;

  /// Create a copy of KitchenSinkTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinNormal_Enums(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinNormal,
    ));
  }
}

/// @nodoc
mixin _$MeasureTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'MeasureTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $MeasureTwinNormalCopyWith<$Res> {
  $MeasureTwinNormalCopyWith(
      MeasureTwinNormal _, $Res Function(MeasureTwinNormal) __);
}

/// Adds pattern-matching-related methods to [MeasureTwinNormal].
extension MeasureTwinNormalPatterns on MeasureTwinNormal {
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
    TResult Function(MeasureTwinNormal_Speed value)? speed,
    TResult Function(MeasureTwinNormal_Distance value)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinNormal_Distance() when distance != null:
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
    required TResult Function(MeasureTwinNormal_Speed value) speed,
    required TResult Function(MeasureTwinNormal_Distance value) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed():
        return speed(_that);
      case MeasureTwinNormal_Distance():
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
    TResult? Function(MeasureTwinNormal_Speed value)? speed,
    TResult? Function(MeasureTwinNormal_Distance value)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinNormal_Distance() when distance != null:
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
    TResult Function(SpeedTwinNormal field0)? speed,
    TResult Function(DistanceTwinNormal field0)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinNormal_Distance() when distance != null:
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
    required TResult Function(SpeedTwinNormal field0) speed,
    required TResult Function(DistanceTwinNormal field0) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed():
        return speed(_that.field0);
      case MeasureTwinNormal_Distance():
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
    TResult? Function(SpeedTwinNormal field0)? speed,
    TResult? Function(DistanceTwinNormal field0)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinNormal_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinNormal_Distance() when distance != null:
        return distance(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class MeasureTwinNormal_Speed extends MeasureTwinNormal {
  const MeasureTwinNormal_Speed(this.field0) : super._();

  @override
  final SpeedTwinNormal field0;

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinNormal_SpeedCopyWith<MeasureTwinNormal_Speed> get copyWith =>
      _$MeasureTwinNormal_SpeedCopyWithImpl<MeasureTwinNormal_Speed>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinNormal_Speed &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinNormal.speed(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinNormal_SpeedCopyWith<$Res>
    implements $MeasureTwinNormalCopyWith<$Res> {
  factory $MeasureTwinNormal_SpeedCopyWith(MeasureTwinNormal_Speed value,
          $Res Function(MeasureTwinNormal_Speed) _then) =
      _$MeasureTwinNormal_SpeedCopyWithImpl;
  @useResult
  $Res call({SpeedTwinNormal field0});

  $SpeedTwinNormalCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinNormal_SpeedCopyWithImpl<$Res>
    implements $MeasureTwinNormal_SpeedCopyWith<$Res> {
  _$MeasureTwinNormal_SpeedCopyWithImpl(this._self, this._then);

  final MeasureTwinNormal_Speed _self;
  final $Res Function(MeasureTwinNormal_Speed) _then;

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinNormal_Speed(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinNormal,
    ));
  }

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinNormalCopyWith<$Res> get field0 {
    return $SpeedTwinNormalCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc

class MeasureTwinNormal_Distance extends MeasureTwinNormal {
  const MeasureTwinNormal_Distance(this.field0) : super._();

  @override
  final DistanceTwinNormal field0;

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinNormal_DistanceCopyWith<MeasureTwinNormal_Distance>
      get copyWith =>
          _$MeasureTwinNormal_DistanceCopyWithImpl<MeasureTwinNormal_Distance>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinNormal_Distance &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinNormal.distance(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinNormal_DistanceCopyWith<$Res>
    implements $MeasureTwinNormalCopyWith<$Res> {
  factory $MeasureTwinNormal_DistanceCopyWith(MeasureTwinNormal_Distance value,
          $Res Function(MeasureTwinNormal_Distance) _then) =
      _$MeasureTwinNormal_DistanceCopyWithImpl;
  @useResult
  $Res call({DistanceTwinNormal field0});

  $DistanceTwinNormalCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinNormal_DistanceCopyWithImpl<$Res>
    implements $MeasureTwinNormal_DistanceCopyWith<$Res> {
  _$MeasureTwinNormal_DistanceCopyWithImpl(this._self, this._then);

  final MeasureTwinNormal_Distance _self;
  final $Res Function(MeasureTwinNormal_Distance) _then;

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinNormal_Distance(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinNormal,
    ));
  }

  /// Create a copy of MeasureTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinNormalCopyWith<$Res> get field0 {
    return $DistanceTwinNormalCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$SpeedTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinNormal()';
  }
}

/// @nodoc
class $SpeedTwinNormalCopyWith<$Res> {
  $SpeedTwinNormalCopyWith(
      SpeedTwinNormal _, $Res Function(SpeedTwinNormal) __);
}

/// Adds pattern-matching-related methods to [SpeedTwinNormal].
extension SpeedTwinNormalPatterns on SpeedTwinNormal {
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
    TResult Function(SpeedTwinNormal_Unknown value)? unknown,
    TResult Function(SpeedTwinNormal_GPS value)? gps,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinNormal_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinNormal_GPS() when gps != null:
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
    required TResult Function(SpeedTwinNormal_Unknown value) unknown,
    required TResult Function(SpeedTwinNormal_GPS value) gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinNormal_Unknown():
        return unknown(_that);
      case SpeedTwinNormal_GPS():
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
    TResult? Function(SpeedTwinNormal_Unknown value)? unknown,
    TResult? Function(SpeedTwinNormal_GPS value)? gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinNormal_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinNormal_GPS() when gps != null:
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
      case SpeedTwinNormal_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinNormal_GPS() when gps != null:
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
      case SpeedTwinNormal_Unknown():
        return unknown();
      case SpeedTwinNormal_GPS():
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
      case SpeedTwinNormal_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinNormal_GPS() when gps != null:
        return gps(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class SpeedTwinNormal_Unknown extends SpeedTwinNormal {
  const SpeedTwinNormal_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinNormal_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinNormal.unknown()';
  }
}

/// @nodoc

class SpeedTwinNormal_GPS extends SpeedTwinNormal {
  const SpeedTwinNormal_GPS(this.field0) : super._();

  final double field0;

  /// Create a copy of SpeedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SpeedTwinNormal_GPSCopyWith<SpeedTwinNormal_GPS> get copyWith =>
      _$SpeedTwinNormal_GPSCopyWithImpl<SpeedTwinNormal_GPS>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SpeedTwinNormal_GPS &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SpeedTwinNormal.gps(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SpeedTwinNormal_GPSCopyWith<$Res>
    implements $SpeedTwinNormalCopyWith<$Res> {
  factory $SpeedTwinNormal_GPSCopyWith(
          SpeedTwinNormal_GPS value, $Res Function(SpeedTwinNormal_GPS) _then) =
      _$SpeedTwinNormal_GPSCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$SpeedTwinNormal_GPSCopyWithImpl<$Res>
    implements $SpeedTwinNormal_GPSCopyWith<$Res> {
  _$SpeedTwinNormal_GPSCopyWithImpl(this._self, this._then);

  final SpeedTwinNormal_GPS _self;
  final $Res Function(SpeedTwinNormal_GPS) _then;

  /// Create a copy of SpeedTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SpeedTwinNormal_GPS(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

// dart format on
