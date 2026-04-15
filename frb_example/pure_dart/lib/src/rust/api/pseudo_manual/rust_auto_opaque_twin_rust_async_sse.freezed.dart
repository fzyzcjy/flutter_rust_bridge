// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinRustAsyncSseCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinRustAsyncSseCopyWith(
      EnumWithGoodAndOpaqueTwinRustAsyncSse _,
      $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinRustAsyncSse].
extension EnumWithGoodAndOpaqueTwinRustAsyncSsePatterns
    on EnumWithGoodAndOpaqueTwinRustAsyncSse {
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
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque value)?
        opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good value)
        good,
    required TResult Function(
            EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque value)?
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinRustAsyncSse field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinRustAsyncSse field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque():
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
    TResult? Function(NonCloneSimpleTwinRustAsyncSse field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncSse_Good
    extends EnumWithGoodAndOpaqueTwinRustAsyncSse {
  const EnumWithGoodAndOpaqueTwinRustAsyncSse_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncSse_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinRustAsyncSse_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSse_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSse.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncSse_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncSse_GoodCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncSse_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncSse_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque
    extends EnumWithGoodAndOpaqueTwinRustAsyncSse {
  const EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinRustAsyncSse field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque>
      get copyWith =>
          _$EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWithImpl<
              EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSse.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinRustAsyncSse field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncSse_OpaqueCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncSse_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinRustAsyncSse,
    ));
  }
}

// dart format on
