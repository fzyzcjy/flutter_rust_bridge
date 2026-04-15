// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$DistanceTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is DistanceTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinSse()';
  }
}

/// @nodoc
class $DistanceTwinSseCopyWith<$Res> {
  $DistanceTwinSseCopyWith(
      DistanceTwinSse _, $Res Function(DistanceTwinSse) __);
}

/// Adds pattern-matching-related methods to [DistanceTwinSse].
extension DistanceTwinSsePatterns on DistanceTwinSse {
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
    TResult Function(DistanceTwinSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSse_Map value)? map,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSse_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinSse_Map() when map != null:
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
    required TResult Function(DistanceTwinSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSse_Map value) map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSse_Unknown():
        return unknown(_that);
      case DistanceTwinSse_Map():
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
    TResult? Function(DistanceTwinSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSse_Map value)? map,
  }) {
    final _that = this;
    switch (_that) {
      case DistanceTwinSse_Unknown() when unknown != null:
        return unknown(_that);
      case DistanceTwinSse_Map() when map != null:
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
      case DistanceTwinSse_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinSse_Map() when map != null:
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
      case DistanceTwinSse_Unknown():
        return unknown();
      case DistanceTwinSse_Map():
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
      case DistanceTwinSse_Unknown() when unknown != null:
        return unknown();
      case DistanceTwinSse_Map() when map != null:
        return map(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class DistanceTwinSse_Unknown extends DistanceTwinSse {
  const DistanceTwinSse_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is DistanceTwinSse_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'DistanceTwinSse.unknown()';
  }
}

/// @nodoc

class DistanceTwinSse_Map extends DistanceTwinSse {
  const DistanceTwinSse_Map(this.field0) : super._();

  final double field0;

  /// Create a copy of DistanceTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $DistanceTwinSse_MapCopyWith<DistanceTwinSse_Map> get copyWith =>
      _$DistanceTwinSse_MapCopyWithImpl<DistanceTwinSse_Map>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is DistanceTwinSse_Map &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'DistanceTwinSse.map(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $DistanceTwinSse_MapCopyWith<$Res>
    implements $DistanceTwinSseCopyWith<$Res> {
  factory $DistanceTwinSse_MapCopyWith(
          DistanceTwinSse_Map value, $Res Function(DistanceTwinSse_Map) _then) =
      _$DistanceTwinSse_MapCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$DistanceTwinSse_MapCopyWithImpl<$Res>
    implements $DistanceTwinSse_MapCopyWith<$Res> {
  _$DistanceTwinSse_MapCopyWithImpl(this._self, this._then);

  final DistanceTwinSse_Map _self;
  final $Res Function(DistanceTwinSse_Map) _then;

  /// Create a copy of DistanceTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(DistanceTwinSse_Map(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemMixedTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumWithItemMixedTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse()';
  }
}

/// @nodoc
class $EnumWithItemMixedTwinSseCopyWith<$Res> {
  $EnumWithItemMixedTwinSseCopyWith(
      EnumWithItemMixedTwinSse _, $Res Function(EnumWithItemMixedTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemMixedTwinSse].
extension EnumWithItemMixedTwinSsePatterns on EnumWithItemMixedTwinSse {
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
    TResult Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSse_C value)? c,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinSse_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinSse_C() when c != null:
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
    required TResult Function(EnumWithItemMixedTwinSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSse_C value) c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSse_A():
        return a(_that);
      case EnumWithItemMixedTwinSse_B():
        return b(_that);
      case EnumWithItemMixedTwinSse_C():
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
    TResult? Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSse_C value)? c,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemMixedTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemMixedTwinSse_B() when b != null:
        return b(_that);
      case EnumWithItemMixedTwinSse_C() when c != null:
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
      case EnumWithItemMixedTwinSse_A() when a != null:
        return a();
      case EnumWithItemMixedTwinSse_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinSse_C() when c != null:
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
      case EnumWithItemMixedTwinSse_A():
        return a();
      case EnumWithItemMixedTwinSse_B():
        return b(_that.field0);
      case EnumWithItemMixedTwinSse_C():
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
      case EnumWithItemMixedTwinSse_A() when a != null:
        return a();
      case EnumWithItemMixedTwinSse_B() when b != null:
        return b(_that.field0);
      case EnumWithItemMixedTwinSse_C() when c != null:
        return c(_that.cField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemMixedTwinSse_A extends EnumWithItemMixedTwinSse {
  const EnumWithItemMixedTwinSse_A() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSse_A);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.a()';
  }
}

/// @nodoc

class EnumWithItemMixedTwinSse_B extends EnumWithItemMixedTwinSse {
  const EnumWithItemMixedTwinSse_B(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of EnumWithItemMixedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinSse_BCopyWith<EnumWithItemMixedTwinSse_B>
      get copyWith =>
          _$EnumWithItemMixedTwinSse_BCopyWithImpl<EnumWithItemMixedTwinSse_B>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSse_B &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinSse_BCopyWith<$Res>
    implements $EnumWithItemMixedTwinSseCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSse_BCopyWith(EnumWithItemMixedTwinSse_B value,
          $Res Function(EnumWithItemMixedTwinSse_B) _then) =
      _$EnumWithItemMixedTwinSse_BCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemMixedTwinSse_BCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinSse_BCopyWith<$Res> {
  _$EnumWithItemMixedTwinSse_BCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinSse_B _self;
  final $Res Function(EnumWithItemMixedTwinSse_B) _then;

  /// Create a copy of EnumWithItemMixedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemMixedTwinSse_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemMixedTwinSse_C extends EnumWithItemMixedTwinSse {
  const EnumWithItemMixedTwinSse_C({required this.cField}) : super._();

  final String cField;

  /// Create a copy of EnumWithItemMixedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemMixedTwinSse_CCopyWith<EnumWithItemMixedTwinSse_C>
      get copyWith =>
          _$EnumWithItemMixedTwinSse_CCopyWithImpl<EnumWithItemMixedTwinSse_C>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemMixedTwinSse_C &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @override
  int get hashCode => Object.hash(runtimeType, cField);

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.c(cField: $cField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemMixedTwinSse_CCopyWith<$Res>
    implements $EnumWithItemMixedTwinSseCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSse_CCopyWith(EnumWithItemMixedTwinSse_C value,
          $Res Function(EnumWithItemMixedTwinSse_C) _then) =
      _$EnumWithItemMixedTwinSse_CCopyWithImpl;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class _$EnumWithItemMixedTwinSse_CCopyWithImpl<$Res>
    implements $EnumWithItemMixedTwinSse_CCopyWith<$Res> {
  _$EnumWithItemMixedTwinSse_CCopyWithImpl(this._self, this._then);

  final EnumWithItemMixedTwinSse_C _self;
  final $Res Function(EnumWithItemMixedTwinSse_C) _then;

  /// Create a copy of EnumWithItemMixedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? cField = null,
  }) {
    return _then(EnumWithItemMixedTwinSse_C(
      cField: null == cField
          ? _self.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemStructTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSse()';
  }
}

/// @nodoc
class $EnumWithItemStructTwinSseCopyWith<$Res> {
  $EnumWithItemStructTwinSseCopyWith(
      EnumWithItemStructTwinSse _, $Res Function(EnumWithItemStructTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemStructTwinSse].
extension EnumWithItemStructTwinSsePatterns on EnumWithItemStructTwinSse {
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
    TResult Function(EnumWithItemStructTwinSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinSse_B() when b != null:
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
    required TResult Function(EnumWithItemStructTwinSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSse_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSse_A():
        return a(_that);
      case EnumWithItemStructTwinSse_B():
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
    TResult? Function(EnumWithItemStructTwinSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSse_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemStructTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemStructTwinSse_B() when b != null:
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
      case EnumWithItemStructTwinSse_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinSse_B() when b != null:
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
      case EnumWithItemStructTwinSse_A():
        return a(_that.aField);
      case EnumWithItemStructTwinSse_B():
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
      case EnumWithItemStructTwinSse_A() when a != null:
        return a(_that.aField);
      case EnumWithItemStructTwinSse_B() when b != null:
        return b(_that.bField);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemStructTwinSse_A extends EnumWithItemStructTwinSse {
  const EnumWithItemStructTwinSse_A({required this.aField}) : super._();

  final Uint8List aField;

  /// Create a copy of EnumWithItemStructTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinSse_ACopyWith<EnumWithItemStructTwinSse_A>
      get copyWith => _$EnumWithItemStructTwinSse_ACopyWithImpl<
          EnumWithItemStructTwinSse_A>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSse_A &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinSse.a(aField: $aField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinSse_ACopyWith<$Res>
    implements $EnumWithItemStructTwinSseCopyWith<$Res> {
  factory $EnumWithItemStructTwinSse_ACopyWith(
          EnumWithItemStructTwinSse_A value,
          $Res Function(EnumWithItemStructTwinSse_A) _then) =
      _$EnumWithItemStructTwinSse_ACopyWithImpl;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class _$EnumWithItemStructTwinSse_ACopyWithImpl<$Res>
    implements $EnumWithItemStructTwinSse_ACopyWith<$Res> {
  _$EnumWithItemStructTwinSse_ACopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinSse_A _self;
  final $Res Function(EnumWithItemStructTwinSse_A) _then;

  /// Create a copy of EnumWithItemStructTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? aField = null,
  }) {
    return _then(EnumWithItemStructTwinSse_A(
      aField: null == aField
          ? _self.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemStructTwinSse_B extends EnumWithItemStructTwinSse {
  const EnumWithItemStructTwinSse_B({required this.bField}) : super._();

  final Int32List bField;

  /// Create a copy of EnumWithItemStructTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemStructTwinSse_BCopyWith<EnumWithItemStructTwinSse_B>
      get copyWith => _$EnumWithItemStructTwinSse_BCopyWithImpl<
          EnumWithItemStructTwinSse_B>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemStructTwinSse_B &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  @override
  String toString() {
    return 'EnumWithItemStructTwinSse.b(bField: $bField)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemStructTwinSse_BCopyWith<$Res>
    implements $EnumWithItemStructTwinSseCopyWith<$Res> {
  factory $EnumWithItemStructTwinSse_BCopyWith(
          EnumWithItemStructTwinSse_B value,
          $Res Function(EnumWithItemStructTwinSse_B) _then) =
      _$EnumWithItemStructTwinSse_BCopyWithImpl;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class _$EnumWithItemStructTwinSse_BCopyWithImpl<$Res>
    implements $EnumWithItemStructTwinSse_BCopyWith<$Res> {
  _$EnumWithItemStructTwinSse_BCopyWithImpl(this._self, this._then);

  final EnumWithItemStructTwinSse_B _self;
  final $Res Function(EnumWithItemStructTwinSse_B) _then;

  /// Create a copy of EnumWithItemStructTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? bField = null,
  }) {
    return _then(EnumWithItemStructTwinSse_B(
      bField: null == bField
          ? _self.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
mixin _$EnumWithItemTupleTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithItemTupleTwinSseCopyWith<$Res> {
  $EnumWithItemTupleTwinSseCopyWith(
      EnumWithItemTupleTwinSse _, $Res Function(EnumWithItemTupleTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithItemTupleTwinSse].
extension EnumWithItemTupleTwinSsePatterns on EnumWithItemTupleTwinSse {
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
    TResult Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinSse_B() when b != null:
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
    required TResult Function(EnumWithItemTupleTwinSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSse_B value) b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSse_A():
        return a(_that);
      case EnumWithItemTupleTwinSse_B():
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
    TResult? Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSse_B value)? b,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithItemTupleTwinSse_A() when a != null:
        return a(_that);
      case EnumWithItemTupleTwinSse_B() when b != null:
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
      case EnumWithItemTupleTwinSse_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinSse_B() when b != null:
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
      case EnumWithItemTupleTwinSse_A():
        return a(_that.field0);
      case EnumWithItemTupleTwinSse_B():
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
      case EnumWithItemTupleTwinSse_A() when a != null:
        return a(_that.field0);
      case EnumWithItemTupleTwinSse_B() when b != null:
        return b(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithItemTupleTwinSse_A extends EnumWithItemTupleTwinSse {
  const EnumWithItemTupleTwinSse_A(this.field0) : super._();

  @override
  final Uint8List field0;

  /// Create a copy of EnumWithItemTupleTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinSse_ACopyWith<EnumWithItemTupleTwinSse_A>
      get copyWith =>
          _$EnumWithItemTupleTwinSse_ACopyWithImpl<EnumWithItemTupleTwinSse_A>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSse_A &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSse.a(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinSse_ACopyWith<$Res>
    implements $EnumWithItemTupleTwinSseCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSse_ACopyWith(EnumWithItemTupleTwinSse_A value,
          $Res Function(EnumWithItemTupleTwinSse_A) _then) =
      _$EnumWithItemTupleTwinSse_ACopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinSse_ACopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinSse_ACopyWith<$Res> {
  _$EnumWithItemTupleTwinSse_ACopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinSse_A _self;
  final $Res Function(EnumWithItemTupleTwinSse_A) _then;

  /// Create a copy of EnumWithItemTupleTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinSse_A(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class EnumWithItemTupleTwinSse_B extends EnumWithItemTupleTwinSse {
  const EnumWithItemTupleTwinSse_B(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumWithItemTupleTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithItemTupleTwinSse_BCopyWith<EnumWithItemTupleTwinSse_B>
      get copyWith =>
          _$EnumWithItemTupleTwinSse_BCopyWithImpl<EnumWithItemTupleTwinSse_B>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithItemTupleTwinSse_B &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSse.b(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithItemTupleTwinSse_BCopyWith<$Res>
    implements $EnumWithItemTupleTwinSseCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSse_BCopyWith(EnumWithItemTupleTwinSse_B value,
          $Res Function(EnumWithItemTupleTwinSse_B) _then) =
      _$EnumWithItemTupleTwinSse_BCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumWithItemTupleTwinSse_BCopyWithImpl<$Res>
    implements $EnumWithItemTupleTwinSse_BCopyWith<$Res> {
  _$EnumWithItemTupleTwinSse_BCopyWithImpl(this._self, this._then);

  final EnumWithItemTupleTwinSse_B _self;
  final $Res Function(EnumWithItemTupleTwinSse_B) _then;

  /// Create a copy of EnumWithItemTupleTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithItemTupleTwinSse_B(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$KitchenSinkTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is KitchenSinkTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinSse()';
  }
}

/// @nodoc
class $KitchenSinkTwinSseCopyWith<$Res> {
  $KitchenSinkTwinSseCopyWith(
      KitchenSinkTwinSse _, $Res Function(KitchenSinkTwinSse) __);
}

/// Adds pattern-matching-related methods to [KitchenSinkTwinSse].
extension KitchenSinkTwinSsePatterns on KitchenSinkTwinSse {
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
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinSse_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinSse_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinSse_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinSse_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinSse_Enums() when enums != null:
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
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty():
        return empty(_that);
      case KitchenSinkTwinSse_Primitives():
        return primitives(_that);
      case KitchenSinkTwinSse_Nested():
        return nested(_that);
      case KitchenSinkTwinSse_Optional():
        return optional(_that);
      case KitchenSinkTwinSse_Buffer():
        return buffer(_that);
      case KitchenSinkTwinSse_Enums():
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
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty() when empty != null:
        return empty(_that);
      case KitchenSinkTwinSse_Primitives() when primitives != null:
        return primitives(_that);
      case KitchenSinkTwinSse_Nested() when nested != null:
        return nested(_that);
      case KitchenSinkTwinSse_Optional() when optional != null:
        return optional(_that);
      case KitchenSinkTwinSse_Buffer() when buffer != null:
        return buffer(_that);
      case KitchenSinkTwinSse_Enums() when enums != null:
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
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinSse_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSse_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinSse_Enums() when enums != null:
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
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty():
        return empty();
      case KitchenSinkTwinSse_Primitives():
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSse_Nested():
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Optional():
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Buffer():
        return buffer(_that.field0);
      case KitchenSinkTwinSse_Enums():
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
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    final _that = this;
    switch (_that) {
      case KitchenSinkTwinSse_Empty() when empty != null:
        return empty();
      case KitchenSinkTwinSse_Primitives() when primitives != null:
        return primitives(_that.int32, _that.float64, _that.boolean);
      case KitchenSinkTwinSse_Nested() when nested != null:
        return nested(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Optional() when optional != null:
        return optional(_that.field0, _that.field1);
      case KitchenSinkTwinSse_Buffer() when buffer != null:
        return buffer(_that.field0);
      case KitchenSinkTwinSse_Enums() when enums != null:
        return enums(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class KitchenSinkTwinSse_Empty extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Empty() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is KitchenSinkTwinSse_Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.empty()';
  }
}

/// @nodoc

class KitchenSinkTwinSse_Primitives extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Primitives(
      {this.int32 = -1, required this.float64, required this.boolean})
      : super._();

  /// Dart field comment
  @JsonKey()
  final int int32;
  final double float64;
  final bool boolean;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSse_PrimitivesCopyWith<KitchenSinkTwinSse_Primitives>
      get copyWith => _$KitchenSinkTwinSse_PrimitivesCopyWithImpl<
          KitchenSinkTwinSse_Primitives>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSse_Primitives &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @override
  String toString() {
    return 'KitchenSinkTwinSse.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSse_PrimitivesCopyWith<$Res>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSse_PrimitivesCopyWith(
          KitchenSinkTwinSse_Primitives value,
          $Res Function(KitchenSinkTwinSse_Primitives) _then) =
      _$KitchenSinkTwinSse_PrimitivesCopyWithImpl;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class _$KitchenSinkTwinSse_PrimitivesCopyWithImpl<$Res>
    implements $KitchenSinkTwinSse_PrimitivesCopyWith<$Res> {
  _$KitchenSinkTwinSse_PrimitivesCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSse_Primitives _self;
  final $Res Function(KitchenSinkTwinSse_Primitives) _then;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(KitchenSinkTwinSse_Primitives(
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

class KitchenSinkTwinSse_Nested extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Nested(this.field0,
      [this.field1 = const KitchenSinkTwinSse.empty()])
      : super._();

  final int field0;
  @JsonKey()
  final KitchenSinkTwinSse field1;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSse_NestedCopyWith<KitchenSinkTwinSse_Nested> get copyWith =>
      _$KitchenSinkTwinSse_NestedCopyWithImpl<KitchenSinkTwinSse_Nested>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSse_Nested &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinSse.nested(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSse_NestedCopyWith<$Res>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSse_NestedCopyWith(KitchenSinkTwinSse_Nested value,
          $Res Function(KitchenSinkTwinSse_Nested) _then) =
      _$KitchenSinkTwinSse_NestedCopyWithImpl;
  @useResult
  $Res call({int field0, KitchenSinkTwinSse field1});

  $KitchenSinkTwinSseCopyWith<$Res> get field1;
}

/// @nodoc
class _$KitchenSinkTwinSse_NestedCopyWithImpl<$Res>
    implements $KitchenSinkTwinSse_NestedCopyWith<$Res> {
  _$KitchenSinkTwinSse_NestedCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSse_Nested _self;
  final $Res Function(KitchenSinkTwinSse_Nested) _then;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(KitchenSinkTwinSse_Nested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _self.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinSse,
    ));
  }

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSseCopyWith<$Res> get field1 {
    return $KitchenSinkTwinSseCopyWith<$Res>(_self.field1, (value) {
      return _then(_self.copyWith(field1: value));
    });
  }
}

/// @nodoc

class KitchenSinkTwinSse_Optional extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Optional([this.field0 = -1, this.field1])
      : super._();

  /// Comment on anonymous field
  @JsonKey()
  final int? field0;
  final int? field1;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSse_OptionalCopyWith<KitchenSinkTwinSse_Optional>
      get copyWith => _$KitchenSinkTwinSse_OptionalCopyWithImpl<
          KitchenSinkTwinSse_Optional>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSse_Optional &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @override
  String toString() {
    return 'KitchenSinkTwinSse.optional(field0: $field0, field1: $field1)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSse_OptionalCopyWith<$Res>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSse_OptionalCopyWith(
          KitchenSinkTwinSse_Optional value,
          $Res Function(KitchenSinkTwinSse_Optional) _then) =
      _$KitchenSinkTwinSse_OptionalCopyWithImpl;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class _$KitchenSinkTwinSse_OptionalCopyWithImpl<$Res>
    implements $KitchenSinkTwinSse_OptionalCopyWith<$Res> {
  _$KitchenSinkTwinSse_OptionalCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSse_Optional _self;
  final $Res Function(KitchenSinkTwinSse_Optional) _then;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(KitchenSinkTwinSse_Optional(
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

class KitchenSinkTwinSse_Buffer extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Buffer(this.field0) : super._();

  final Uint8List field0;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSse_BufferCopyWith<KitchenSinkTwinSse_Buffer> get copyWith =>
      _$KitchenSinkTwinSse_BufferCopyWithImpl<KitchenSinkTwinSse_Buffer>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSse_Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'KitchenSinkTwinSse.buffer(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSse_BufferCopyWith<$Res>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSse_BufferCopyWith(KitchenSinkTwinSse_Buffer value,
          $Res Function(KitchenSinkTwinSse_Buffer) _then) =
      _$KitchenSinkTwinSse_BufferCopyWithImpl;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class _$KitchenSinkTwinSse_BufferCopyWithImpl<$Res>
    implements $KitchenSinkTwinSse_BufferCopyWith<$Res> {
  _$KitchenSinkTwinSse_BufferCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSse_Buffer _self;
  final $Res Function(KitchenSinkTwinSse_Buffer) _then;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinSse_Buffer(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class KitchenSinkTwinSse_Enums extends KitchenSinkTwinSse {
  const KitchenSinkTwinSse_Enums([this.field0 = WeekdaysTwinSse.sunday])
      : super._();

  @JsonKey()
  final WeekdaysTwinSse field0;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSse_EnumsCopyWith<KitchenSinkTwinSse_Enums> get copyWith =>
      _$KitchenSinkTwinSse_EnumsCopyWithImpl<KitchenSinkTwinSse_Enums>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is KitchenSinkTwinSse_Enums &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'KitchenSinkTwinSse.enums(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $KitchenSinkTwinSse_EnumsCopyWith<$Res>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSse_EnumsCopyWith(KitchenSinkTwinSse_Enums value,
          $Res Function(KitchenSinkTwinSse_Enums) _then) =
      _$KitchenSinkTwinSse_EnumsCopyWithImpl;
  @useResult
  $Res call({WeekdaysTwinSse field0});
}

/// @nodoc
class _$KitchenSinkTwinSse_EnumsCopyWithImpl<$Res>
    implements $KitchenSinkTwinSse_EnumsCopyWith<$Res> {
  _$KitchenSinkTwinSse_EnumsCopyWithImpl(this._self, this._then);

  final KitchenSinkTwinSse_Enums _self;
  final $Res Function(KitchenSinkTwinSse_Enums) _then;

  /// Create a copy of KitchenSinkTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(KitchenSinkTwinSse_Enums(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinSse,
    ));
  }
}

/// @nodoc
mixin _$MeasureTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'MeasureTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $MeasureTwinSseCopyWith<$Res> {
  $MeasureTwinSseCopyWith(MeasureTwinSse _, $Res Function(MeasureTwinSse) __);
}

/// Adds pattern-matching-related methods to [MeasureTwinSse].
extension MeasureTwinSsePatterns on MeasureTwinSse {
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
    TResult Function(MeasureTwinSse_Speed value)? speed,
    TResult Function(MeasureTwinSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinSse_Distance() when distance != null:
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
    required TResult Function(MeasureTwinSse_Speed value) speed,
    required TResult Function(MeasureTwinSse_Distance value) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed():
        return speed(_that);
      case MeasureTwinSse_Distance():
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
    TResult? Function(MeasureTwinSse_Speed value)? speed,
    TResult? Function(MeasureTwinSse_Distance value)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed() when speed != null:
        return speed(_that);
      case MeasureTwinSse_Distance() when distance != null:
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
    TResult Function(SpeedTwinSse field0)? speed,
    TResult Function(DistanceTwinSse field0)? distance,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinSse_Distance() when distance != null:
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
    required TResult Function(SpeedTwinSse field0) speed,
    required TResult Function(DistanceTwinSse field0) distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed():
        return speed(_that.field0);
      case MeasureTwinSse_Distance():
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
    TResult? Function(SpeedTwinSse field0)? speed,
    TResult? Function(DistanceTwinSse field0)? distance,
  }) {
    final _that = this;
    switch (_that) {
      case MeasureTwinSse_Speed() when speed != null:
        return speed(_that.field0);
      case MeasureTwinSse_Distance() when distance != null:
        return distance(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class MeasureTwinSse_Speed extends MeasureTwinSse {
  const MeasureTwinSse_Speed(this.field0) : super._();

  @override
  final SpeedTwinSse field0;

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinSse_SpeedCopyWith<MeasureTwinSse_Speed> get copyWith =>
      _$MeasureTwinSse_SpeedCopyWithImpl<MeasureTwinSse_Speed>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSse_Speed &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinSse.speed(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinSse_SpeedCopyWith<$Res>
    implements $MeasureTwinSseCopyWith<$Res> {
  factory $MeasureTwinSse_SpeedCopyWith(MeasureTwinSse_Speed value,
          $Res Function(MeasureTwinSse_Speed) _then) =
      _$MeasureTwinSse_SpeedCopyWithImpl;
  @useResult
  $Res call({SpeedTwinSse field0});

  $SpeedTwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinSse_SpeedCopyWithImpl<$Res>
    implements $MeasureTwinSse_SpeedCopyWith<$Res> {
  _$MeasureTwinSse_SpeedCopyWithImpl(this._self, this._then);

  final MeasureTwinSse_Speed _self;
  final $Res Function(MeasureTwinSse_Speed) _then;

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinSse_Speed(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinSse,
    ));
  }

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinSseCopyWith<$Res> get field0 {
    return $SpeedTwinSseCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc

class MeasureTwinSse_Distance extends MeasureTwinSse {
  const MeasureTwinSse_Distance(this.field0) : super._();

  @override
  final DistanceTwinSse field0;

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MeasureTwinSse_DistanceCopyWith<MeasureTwinSse_Distance> get copyWith =>
      _$MeasureTwinSse_DistanceCopyWithImpl<MeasureTwinSse_Distance>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MeasureTwinSse_Distance &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MeasureTwinSse.distance(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MeasureTwinSse_DistanceCopyWith<$Res>
    implements $MeasureTwinSseCopyWith<$Res> {
  factory $MeasureTwinSse_DistanceCopyWith(MeasureTwinSse_Distance value,
          $Res Function(MeasureTwinSse_Distance) _then) =
      _$MeasureTwinSse_DistanceCopyWithImpl;
  @useResult
  $Res call({DistanceTwinSse field0});

  $DistanceTwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$MeasureTwinSse_DistanceCopyWithImpl<$Res>
    implements $MeasureTwinSse_DistanceCopyWith<$Res> {
  _$MeasureTwinSse_DistanceCopyWithImpl(this._self, this._then);

  final MeasureTwinSse_Distance _self;
  final $Res Function(MeasureTwinSse_Distance) _then;

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MeasureTwinSse_Distance(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinSse,
    ));
  }

  /// Create a copy of MeasureTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinSseCopyWith<$Res> get field0 {
    return $DistanceTwinSseCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$SpeedTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinSse()';
  }
}

/// @nodoc
class $SpeedTwinSseCopyWith<$Res> {
  $SpeedTwinSseCopyWith(SpeedTwinSse _, $Res Function(SpeedTwinSse) __);
}

/// Adds pattern-matching-related methods to [SpeedTwinSse].
extension SpeedTwinSsePatterns on SpeedTwinSse {
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
    TResult Function(SpeedTwinSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSse_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinSse_GPS() when gps != null:
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
    required TResult Function(SpeedTwinSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSse_GPS value) gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSse_Unknown():
        return unknown(_that);
      case SpeedTwinSse_GPS():
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
    TResult? Function(SpeedTwinSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSse_GPS value)? gps,
  }) {
    final _that = this;
    switch (_that) {
      case SpeedTwinSse_Unknown() when unknown != null:
        return unknown(_that);
      case SpeedTwinSse_GPS() when gps != null:
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
      case SpeedTwinSse_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinSse_GPS() when gps != null:
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
      case SpeedTwinSse_Unknown():
        return unknown();
      case SpeedTwinSse_GPS():
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
      case SpeedTwinSse_Unknown() when unknown != null:
        return unknown();
      case SpeedTwinSse_GPS() when gps != null:
        return gps(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class SpeedTwinSse_Unknown extends SpeedTwinSse {
  const SpeedTwinSse_Unknown() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SpeedTwinSse_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SpeedTwinSse.unknown()';
  }
}

/// @nodoc

class SpeedTwinSse_GPS extends SpeedTwinSse {
  const SpeedTwinSse_GPS(this.field0) : super._();

  final double field0;

  /// Create a copy of SpeedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SpeedTwinSse_GPSCopyWith<SpeedTwinSse_GPS> get copyWith =>
      _$SpeedTwinSse_GPSCopyWithImpl<SpeedTwinSse_GPS>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SpeedTwinSse_GPS &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SpeedTwinSse.gps(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SpeedTwinSse_GPSCopyWith<$Res>
    implements $SpeedTwinSseCopyWith<$Res> {
  factory $SpeedTwinSse_GPSCopyWith(
          SpeedTwinSse_GPS value, $Res Function(SpeedTwinSse_GPS) _then) =
      _$SpeedTwinSse_GPSCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$SpeedTwinSse_GPSCopyWithImpl<$Res>
    implements $SpeedTwinSse_GPSCopyWith<$Res> {
  _$SpeedTwinSse_GPSCopyWithImpl(this._self, this._then);

  final SpeedTwinSse_GPS _self;
  final $Res Function(SpeedTwinSse_GPS) _then;

  /// Create a copy of SpeedTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SpeedTwinSse_GPS(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

// dart format on
