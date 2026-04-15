// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'method_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$SimpleEnumTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinSync()';
  }
}

/// @nodoc
class $SimpleEnumTwinSyncCopyWith<$Res> {
  $SimpleEnumTwinSyncCopyWith(
      SimpleEnumTwinSync _, $Res Function(SimpleEnumTwinSync) __);
}

/// Adds pattern-matching-related methods to [SimpleEnumTwinSync].
extension SimpleEnumTwinSyncPatterns on SimpleEnumTwinSync {
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
    TResult Function(SimpleEnumTwinSync_First value)? first,
    TResult Function(SimpleEnumTwinSync_Second value)? second,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First() when first != null:
        return first(_that);
      case SimpleEnumTwinSync_Second() when second != null:
        return second(_that);
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
    required TResult Function(SimpleEnumTwinSync_First value) first,
    required TResult Function(SimpleEnumTwinSync_Second value) second,
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First():
        return first(_that);
      case SimpleEnumTwinSync_Second():
        return second(_that);
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
    TResult? Function(SimpleEnumTwinSync_First value)? first,
    TResult? Function(SimpleEnumTwinSync_Second value)? second,
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First() when first != null:
        return first(_that);
      case SimpleEnumTwinSync_Second() when second != null:
        return second(_that);
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
    TResult Function()? first,
    TResult Function(String field0)? second,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First() when first != null:
        return first();
      case SimpleEnumTwinSync_Second() when second != null:
        return second(_that.field0);
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
    required TResult Function() first,
    required TResult Function(String field0) second,
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First():
        return first();
      case SimpleEnumTwinSync_Second():
        return second(_that.field0);
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
    TResult? Function()? first,
    TResult? Function(String field0)? second,
  }) {
    final _that = this;
    switch (_that) {
      case SimpleEnumTwinSync_First() when first != null:
        return first();
      case SimpleEnumTwinSync_Second() when second != null:
        return second(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class SimpleEnumTwinSync_First extends SimpleEnumTwinSync {
  const SimpleEnumTwinSync_First() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinSync_First);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinSync.first()';
  }
}

/// @nodoc

class SimpleEnumTwinSync_Second extends SimpleEnumTwinSync {
  const SimpleEnumTwinSync_Second(this.field0) : super._();

  final String field0;

  /// Create a copy of SimpleEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SimpleEnumTwinSync_SecondCopyWith<SimpleEnumTwinSync_Second> get copyWith =>
      _$SimpleEnumTwinSync_SecondCopyWithImpl<SimpleEnumTwinSync_Second>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinSync_Second &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SimpleEnumTwinSync.second(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SimpleEnumTwinSync_SecondCopyWith<$Res>
    implements $SimpleEnumTwinSyncCopyWith<$Res> {
  factory $SimpleEnumTwinSync_SecondCopyWith(SimpleEnumTwinSync_Second value,
          $Res Function(SimpleEnumTwinSync_Second) _then) =
      _$SimpleEnumTwinSync_SecondCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$SimpleEnumTwinSync_SecondCopyWithImpl<$Res>
    implements $SimpleEnumTwinSync_SecondCopyWith<$Res> {
  _$SimpleEnumTwinSync_SecondCopyWithImpl(this._self, this._then);

  final SimpleEnumTwinSync_Second _self;
  final $Res Function(SimpleEnumTwinSync_Second) _then;

  /// Create a copy of SimpleEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SimpleEnumTwinSync_Second(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
