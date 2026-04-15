// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'raw_string_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$RawStringItemEnumTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is RawStringItemEnumTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'RawStringItemEnumTwinSse()';
  }
}

/// @nodoc
class $RawStringItemEnumTwinSseCopyWith<$Res> {
  $RawStringItemEnumTwinSseCopyWith(
      RawStringItemEnumTwinSse _, $Res Function(RawStringItemEnumTwinSse) __);
}

/// Adds pattern-matching-related methods to [RawStringItemEnumTwinSse].
extension RawStringItemEnumTwinSsePatterns on RawStringItemEnumTwinSse {
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
    TResult Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult Function(RawStringItemEnumTwinSse_Raw value)? raw,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSse_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSse_Raw() when raw != null:
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
    required TResult Function(RawStringItemEnumTwinSse_Regular value) regular,
    required TResult Function(RawStringItemEnumTwinSse_Raw value) raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSse_Regular():
        return regular(_that);
      case RawStringItemEnumTwinSse_Raw():
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
    TResult? Function(RawStringItemEnumTwinSse_Regular value)? regular,
    TResult? Function(RawStringItemEnumTwinSse_Raw value)? raw,
  }) {
    final _that = this;
    switch (_that) {
      case RawStringItemEnumTwinSse_Regular() when regular != null:
        return regular(_that);
      case RawStringItemEnumTwinSse_Raw() when raw != null:
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
      case RawStringItemEnumTwinSse_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSse_Raw() when raw != null:
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
      case RawStringItemEnumTwinSse_Regular():
        return regular(_that.regular);
      case RawStringItemEnumTwinSse_Raw():
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
      case RawStringItemEnumTwinSse_Regular() when regular != null:
        return regular(_that.regular);
      case RawStringItemEnumTwinSse_Raw() when raw != null:
        return raw(_that.type);
      case _:
        return null;
    }
  }
}

/// @nodoc

class RawStringItemEnumTwinSse_Regular extends RawStringItemEnumTwinSse {
  const RawStringItemEnumTwinSse_Regular({required this.regular}) : super._();

  final String regular;

  /// Create a copy of RawStringItemEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSse_RegularCopyWith<RawStringItemEnumTwinSse_Regular>
      get copyWith => _$RawStringItemEnumTwinSse_RegularCopyWithImpl<
          RawStringItemEnumTwinSse_Regular>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSse_Regular &&
            (identical(other.regular, regular) || other.regular == regular));
  }

  @override
  int get hashCode => Object.hash(runtimeType, regular);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSse.regular(regular: $regular)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSse_RegularCopyWith<$Res>
    implements $RawStringItemEnumTwinSseCopyWith<$Res> {
  factory $RawStringItemEnumTwinSse_RegularCopyWith(
          RawStringItemEnumTwinSse_Regular value,
          $Res Function(RawStringItemEnumTwinSse_Regular) _then) =
      _$RawStringItemEnumTwinSse_RegularCopyWithImpl;
  @useResult
  $Res call({String regular});
}

/// @nodoc
class _$RawStringItemEnumTwinSse_RegularCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSse_RegularCopyWith<$Res> {
  _$RawStringItemEnumTwinSse_RegularCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSse_Regular _self;
  final $Res Function(RawStringItemEnumTwinSse_Regular) _then;

  /// Create a copy of RawStringItemEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? regular = null,
  }) {
    return _then(RawStringItemEnumTwinSse_Regular(
      regular: null == regular
          ? _self.regular
          : regular // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class RawStringItemEnumTwinSse_Raw extends RawStringItemEnumTwinSse {
  const RawStringItemEnumTwinSse_Raw({required this.type}) : super._();

  final String type;

  /// Create a copy of RawStringItemEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringItemEnumTwinSse_RawCopyWith<RawStringItemEnumTwinSse_Raw>
      get copyWith => _$RawStringItemEnumTwinSse_RawCopyWithImpl<
          RawStringItemEnumTwinSse_Raw>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringItemEnumTwinSse_Raw &&
            (identical(other.type, type) || other.type == type));
  }

  @override
  int get hashCode => Object.hash(runtimeType, type);

  @override
  String toString() {
    return 'RawStringItemEnumTwinSse.raw(type: $type)';
  }
}

/// @nodoc
abstract mixin class $RawStringItemEnumTwinSse_RawCopyWith<$Res>
    implements $RawStringItemEnumTwinSseCopyWith<$Res> {
  factory $RawStringItemEnumTwinSse_RawCopyWith(
          RawStringItemEnumTwinSse_Raw value,
          $Res Function(RawStringItemEnumTwinSse_Raw) _then) =
      _$RawStringItemEnumTwinSse_RawCopyWithImpl;
  @useResult
  $Res call({String type});
}

/// @nodoc
class _$RawStringItemEnumTwinSse_RawCopyWithImpl<$Res>
    implements $RawStringItemEnumTwinSse_RawCopyWith<$Res> {
  _$RawStringItemEnumTwinSse_RawCopyWithImpl(this._self, this._then);

  final RawStringItemEnumTwinSse_Raw _self;
  final $Res Function(RawStringItemEnumTwinSse_Raw) _then;

  /// Create a copy of RawStringItemEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? type = null,
  }) {
    return _then(RawStringItemEnumTwinSse_Raw(
      type: null == type
          ? _self.type
          : type // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
