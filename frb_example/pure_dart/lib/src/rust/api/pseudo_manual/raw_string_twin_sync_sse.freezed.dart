// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'raw_string_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$RawStringItemEnumTwinSyncSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSyncSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'RawStringItemEnumTwinSyncSse()';
  }
}

/// @nodoc
class $RawStringItemEnumTwinSyncSseCopyWith<$Res> {
  $RawStringItemEnumTwinSyncSseCopyWith(RawStringItemEnumTwinSyncSse _,
      $Res Function(RawStringItemEnumTwinSyncSse) __);
}

/// Adds pattern-matching-related methods to [RawStringItemEnumTwinSyncSse].
extension RawStringItemEnumTwinSyncSsePatterns on RawStringItemEnumTwinSyncSse {
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
    TResult Function(RawStringItemEnumTwinSyncSse_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSyncSse_Raw value)? raw,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSyncSse_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSyncSse_Raw() when raw != null:
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
    required TResult Function(RawStringItemEnumTwinSyncSse_Regular value)
        regular,
    required TResult Function(RawStringItemEnumTwinSyncSse_Raw value) raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSyncSse_Regular():
        return regular(_that);
      case RawStringItemEnumTwinSyncSse_Raw():
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
    TResult? Function(RawStringItemEnumTwinSyncSse_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSyncSse_Raw value)? raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSyncSse_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSyncSse_Raw() when raw != null:
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
      case RawStringItemEnumTwinSyncSse_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSyncSse_Raw() when raw != null:
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
      case RawStringItemEnumTwinSyncSse_Regular():
        return regular(_that.regular);
      case RawStringItemEnumTwinSyncSse_Raw():
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
      case RawStringItemEnumTwinSyncSse_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSyncSse_Raw() when raw != null:
        return raw(_that.type);
      case _:
        return null;
    }
  }
}

/// @nodoc

class RawStringItemEnumTwinSyncSse_Regular
    extends RawStringItemEnumTwinSyncSse {
  const RawStringItemEnumTwinSyncSse_Regular({required this.regular})
      : super._();

  final String regular;

  /// Create a copy of RawStringItemEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSyncSse_RegularCopyWith<
          RawStringItemEnumTwinSyncSse_Regular>
      get copyWith => _$RawStringItemEnumTwinSyncSse_RegularCopyWithImpl<
          RawStringItemEnumTwinSyncSse_Regular>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSyncSse_Regular &&
            (identical(other.regular, regular) || other.regular == regular));
  }

  @override
  int get hashCode => Object.hash(runtimeType, regular);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSyncSse.regular(regular: $regular)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSyncSse_RegularCopyWith<$Res>
    implements $RawStringItemEnumTwinSyncSseCopyWith<$Res> {
  factory $RawStringItemEnumTwinSyncSse_RegularCopyWith(
          RawStringItemEnumTwinSyncSse_Regular value,
          $Res Function(RawStringItemEnumTwinSyncSse_Regular) _then) =
      _$RawStringItemEnumTwinSyncSse_RegularCopyWithImpl;
  @useResult
  $Res call({String regular});
}

/// @nodoc
class _$RawStringItemEnumTwinSyncSse_RegularCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSyncSse_RegularCopyWith<$Res> {
  _$RawStringItemEnumTwinSyncSse_RegularCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSyncSse_Regular _self;
  final $Res Function(RawStringItemEnumTwinSyncSse_Regular) _then;

  /// Create a copy of RawStringItemEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? regular = null,
  }) {
    return _then(RawStringItemEnumTwinSyncSse_Regular(
      regular: null == regular
          ? _self.regular
          : regular // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class RawStringItemEnumTwinSyncSse_Raw extends RawStringItemEnumTwinSyncSse {
  const RawStringItemEnumTwinSyncSse_Raw({required this.type}) : super._();

  final String type;

  /// Create a copy of RawStringItemEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSyncSse_RawCopyWith<RawStringItemEnumTwinSyncSse_Raw>
      get copyWith => _$RawStringItemEnumTwinSyncSse_RawCopyWithImpl<
          RawStringItemEnumTwinSyncSse_Raw>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSyncSse_Raw &&
            (identical(other.type, type) || other.type == type));
  }

  @override
  int get hashCode => Object.hash(runtimeType, type);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSyncSse.raw(type: $type)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSyncSse_RawCopyWith<$Res>
    implements $RawStringItemEnumTwinSyncSseCopyWith<$Res> {
  factory $RawStringItemEnumTwinSyncSse_RawCopyWith(
          RawStringItemEnumTwinSyncSse_Raw value,
          $Res Function(RawStringItemEnumTwinSyncSse_Raw) _then) =
      _$RawStringItemEnumTwinSyncSse_RawCopyWithImpl;
  @useResult
  $Res call({String type});
}

/// @nodoc
class _$RawStringItemEnumTwinSyncSse_RawCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSyncSse_RawCopyWith<$Res> {
  _$RawStringItemEnumTwinSyncSse_RawCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSyncSse_Raw _self;
  final $Res Function(RawStringItemEnumTwinSyncSse_Raw) _then;

  /// Create a copy of RawStringItemEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? type = null,
  }) {
    return _then(RawStringItemEnumTwinSyncSse_Raw(
      type: null == type
          ? _self.type
          : type // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
