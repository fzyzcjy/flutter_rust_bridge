// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinRustAsyncCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinRustAsyncCopyWith(
      EnumWithGoodAndOpaqueTwinRustAsync _,
      $Res Function(EnumWithGoodAndOpaqueTwinRustAsync) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinRustAsync].
extension EnumWithGoodAndOpaqueTwinRustAsyncPatterns
    on EnumWithGoodAndOpaqueTwinRustAsync {
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
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsync_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsync_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque() when opaque != null:
        return opaque(_that);
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
    required TResult Function(EnumWithGoodAndOpaqueTwinRustAsync_Good value)
        good,
    required TResult Function(EnumWithGoodAndOpaqueTwinRustAsync_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque():
        return opaque(_that);
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
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsync_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsync_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque() when opaque != null:
        return opaque(_that);
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
    TResult Function(String field0)? good,
    TResult Function(NonCloneSimpleTwinRustAsync field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque() when opaque != null:
        return opaque(_that.field0);
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
    required TResult Function(String field0) good,
    required TResult Function(NonCloneSimpleTwinRustAsync field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque():
        return opaque(_that.field0);
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
    TResult? Function(String field0)? good,
    TResult? Function(NonCloneSimpleTwinRustAsync field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsync_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsync_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsync_Good
    extends EnumWithGoodAndOpaqueTwinRustAsync {
  const EnumWithGoodAndOpaqueTwinRustAsync_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsync_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinRustAsync_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsync_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsync.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsync_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsync_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsync_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsync_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsync_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsync_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsync_Opaque
    extends EnumWithGoodAndOpaqueTwinRustAsync {
  const EnumWithGoodAndOpaqueTwinRustAsync_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinRustAsync field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsync_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinRustAsync_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsync_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsync.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsync_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsync_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinRustAsync field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsync_OpaqueCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsync_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsync_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsync_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinRustAsync,
    ));
  }
}

// dart format on
