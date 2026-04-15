// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinRustAsync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsync()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinRustAsyncCopyWith<$Res> {
  $BasicGeneralEnumTwinRustAsyncCopyWith(BasicGeneralEnumTwinRustAsync _,
      $Res Function(BasicGeneralEnumTwinRustAsync) __);
}

/// Adds pattern-matching-related methods to [BasicGeneralEnumTwinRustAsync].
extension BasicGeneralEnumTwinRustAsyncPatterns
    on BasicGeneralEnumTwinRustAsync {
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
    TResult Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsync_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinRustAsync_Orange() when orange != null:
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
    required TResult Function(BasicGeneralEnumTwinRustAsync_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinRustAsync_Orange value)
        orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsync_Apple():
        return apple(_that);
      case BasicGeneralEnumTwinRustAsync_Orange():
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
    TResult? Function(BasicGeneralEnumTwinRustAsync_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinRustAsync_Orange value)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinRustAsync_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinRustAsync_Orange() when orange != null:
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
      case BasicGeneralEnumTwinRustAsync_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsync_Orange() when orange != null:
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
      case BasicGeneralEnumTwinRustAsync_Apple():
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsync_Orange():
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
      case BasicGeneralEnumTwinRustAsync_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinRustAsync_Orange() when orange != null:
        return orange();
      case _:
        return null;
    }
  }
}

/// @nodoc

class BasicGeneralEnumTwinRustAsync_Apple
    extends BasicGeneralEnumTwinRustAsync {
  const BasicGeneralEnumTwinRustAsync_Apple({required this.field}) : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinRustAsync_AppleCopyWith<
          BasicGeneralEnumTwinRustAsync_Apple>
      get copyWith => _$BasicGeneralEnumTwinRustAsync_AppleCopyWithImpl<
          BasicGeneralEnumTwinRustAsync_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsync_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsync.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinRustAsync_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinRustAsyncCopyWith<$Res> {
  factory $BasicGeneralEnumTwinRustAsync_AppleCopyWith(
          BasicGeneralEnumTwinRustAsync_Apple value,
          $Res Function(BasicGeneralEnumTwinRustAsync_Apple) _then) =
      _$BasicGeneralEnumTwinRustAsync_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinRustAsync_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinRustAsync_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinRustAsync_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinRustAsync_Apple _self;
  final $Res Function(BasicGeneralEnumTwinRustAsync_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinRustAsync_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinRustAsync_Orange
    extends BasicGeneralEnumTwinRustAsync {
  const BasicGeneralEnumTwinRustAsync_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinRustAsync_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinRustAsync.orange()';
  }
}

// dart format on
