// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinRustAsyncSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsyncSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsyncSse()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinRustAsyncSseCopyWith<$Res> {
  $BasicGeneralEnumTwinRustAsyncSseCopyWith(BasicGeneralEnumTwinRustAsyncSse _,
      $Res Function(BasicGeneralEnumTwinRustAsyncSse) __);
}

/// Adds pattern-matching-related methods to [BasicGeneralEnumTwinRustAsyncSse].
extension BasicGeneralEnumTwinRustAsyncSsePatterns
    on BasicGeneralEnumTwinRustAsyncSse {
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
    TResult Function(BasicGeneralEnumTwinRustAsyncSse_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinRustAsyncSse_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinRustAsyncSse_Orange() when orange != null:
        return orange(_that);
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
    required TResult Function(BasicGeneralEnumTwinRustAsyncSse_Apple value)
        apple,
    required TResult Function(BasicGeneralEnumTwinRustAsyncSse_Orange value)
        orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple():
        return apple(_that);
      case BasicGeneralEnumTwinRustAsyncSse_Orange():
        return orange(_that);
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
    TResult? Function(BasicGeneralEnumTwinRustAsyncSse_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinRustAsyncSse_Orange value)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinRustAsyncSse_Orange() when orange != null:
        return orange(_that);
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
    TResult Function(String field)? apple,
    TResult Function()? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsyncSse_Orange() when orange != null:
        return orange();
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
    required TResult Function(String field) apple,
    required TResult Function() orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple():
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsyncSse_Orange():
        return orange();
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
    TResult? Function(String field)? apple,
    TResult? Function()? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsyncSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsyncSse_Orange() when orange != null:
        return orange();
      case _:
        return null;
    }
  }
}

/// @nodoc

class BasicGeneralEnumTwinRustAsyncSse_Apple
    extends BasicGeneralEnumTwinRustAsyncSse {
  const BasicGeneralEnumTwinRustAsyncSse_Apple({required this.field})
      : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinRustAsyncSse_AppleCopyWith<
          BasicGeneralEnumTwinRustAsyncSse_Apple>
      get copyWith => _$BasicGeneralEnumTwinRustAsyncSse_AppleCopyWithImpl<
          BasicGeneralEnumTwinRustAsyncSse_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsyncSse_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsyncSse.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinRustAsyncSse_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinRustAsyncSseCopyWith<$Res> {
  factory $BasicGeneralEnumTwinRustAsyncSse_AppleCopyWith(
          BasicGeneralEnumTwinRustAsyncSse_Apple value,
          $Res Function(BasicGeneralEnumTwinRustAsyncSse_Apple) _then) =
      _$BasicGeneralEnumTwinRustAsyncSse_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinRustAsyncSse_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinRustAsyncSse_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinRustAsyncSse_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinRustAsyncSse_Apple _self;
  final $Res Function(BasicGeneralEnumTwinRustAsyncSse_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinRustAsyncSse_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinRustAsyncSse_Orange
    extends BasicGeneralEnumTwinRustAsyncSse {
  const BasicGeneralEnumTwinRustAsyncSse_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsyncSse_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsyncSse.orange()';
  }
}

// dart format on
