// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'raw_string_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$RawStringItemEnumTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'RawStringItemEnumTwinSync()';
  }
}

/// @nodoc
class $RawStringItemEnumTwinSyncCopyWith<$Res> {
  $RawStringItemEnumTwinSyncCopyWith(
      RawStringItemEnumTwinSync _, $Res Function(RawStringItemEnumTwinSync) __);
}

/// Adds pattern-matching-related methods to [RawStringItemEnumTwinSync].
extension RawStringItemEnumTwinSyncPatterns on RawStringItemEnumTwinSync {
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
    TResult Function(RawStringItemEnumTwinSync_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSync_Raw value)? raw,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSync_Raw() when raw != null:
        return raw(_that);
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
    required TResult Function(RawStringItemEnumTwinSync_Regular value) regular,
    required TResult Function(RawStringItemEnumTwinSync_Raw value) raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular():
        return regular(_that);
      case RawStringItemEnumTwinSync_Raw():
        return raw(_that);
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
    TResult? Function(RawStringItemEnumTwinSync_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSync_Raw value)? raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSync_Raw() when raw != null:
        return raw(_that);
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
    TResult Function(String regular)? regular,
    TResult Function(String type)? raw,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSync_Raw() when raw != null:
        return raw(_that.type);
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
    required TResult Function(String regular) regular,
    required TResult Function(String type) raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular():
        return regular(_that.regular);
      case RawStringItemEnumTwinSync_Raw():
        return raw(_that.type);
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
    TResult? Function(String regular)? regular,
    TResult? Function(String type)? raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSync_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSync_Raw() when raw != null:
        return raw(_that.type);
      case _:
        return null;
    }
  }
}

/// @nodoc

class RawStringItemEnumTwinSync_Regular extends RawStringItemEnumTwinSync {
  const RawStringItemEnumTwinSync_Regular({required this.regular}) : super._();

  final String regular;

  /// Create a copy of RawStringItemEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSync_RegularCopyWith<RawStringItemEnumTwinSync_Regular>
      get copyWith => _$RawStringItemEnumTwinSync_RegularCopyWithImpl<
          RawStringItemEnumTwinSync_Regular>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSync_Regular &&
            (identical(other.regular, regular) || other.regular == regular));
  }

  @override
  int get hashCode => Object.hash(runtimeType, regular);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSync.regular(regular: $regular)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSync_RegularCopyWith<$Res>
    implements $RawStringItemEnumTwinSyncCopyWith<$Res> {
  factory $RawStringItemEnumTwinSync_RegularCopyWith(
          RawStringItemEnumTwinSync_Regular value,
          $Res Function(RawStringItemEnumTwinSync_Regular) _then) =
      _$RawStringItemEnumTwinSync_RegularCopyWithImpl;
  @useResult
  $Res call({String regular});
}

/// @nodoc
class _$RawStringItemEnumTwinSync_RegularCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSync_RegularCopyWith<$Res> {
  _$RawStringItemEnumTwinSync_RegularCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSync_Regular _self;
  final $Res Function(RawStringItemEnumTwinSync_Regular) _then;

  /// Create a copy of RawStringItemEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? regular = null,
  }) {
    return _then(RawStringItemEnumTwinSync_Regular(
      regular: null == regular
          ? _self.regular
          : regular // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class RawStringItemEnumTwinSync_Raw extends RawStringItemEnumTwinSync {
  const RawStringItemEnumTwinSync_Raw({required this.type}) : super._();

  final String type;

  /// Create a copy of RawStringItemEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSync_RawCopyWith<RawStringItemEnumTwinSync_Raw>
      get copyWith => _$RawStringItemEnumTwinSync_RawCopyWithImpl<
          RawStringItemEnumTwinSync_Raw>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSync_Raw &&
            (identical(other.type, type) || other.type == type));
  }

  @override
  int get hashCode => Object.hash(runtimeType, type);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSync.raw(type: $type)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSync_RawCopyWith<$Res>
    implements $RawStringItemEnumTwinSyncCopyWith<$Res> {
  factory $RawStringItemEnumTwinSync_RawCopyWith(
          RawStringItemEnumTwinSync_Raw value,
          $Res Function(RawStringItemEnumTwinSync_Raw) _then) =
      _$RawStringItemEnumTwinSync_RawCopyWithImpl;
  @useResult
  $Res call({String type});
}

/// @nodoc
class _$RawStringItemEnumTwinSync_RawCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSync_RawCopyWith<$Res> {
  _$RawStringItemEnumTwinSync_RawCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSync_Raw _self;
  final $Res Function(RawStringItemEnumTwinSync_Raw) _then;

  /// Create a copy of RawStringItemEnumTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? type = null,
  }) {
    return _then(RawStringItemEnumTwinSync_Raw(
      type: null == type
          ? _self.type
          : type // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
