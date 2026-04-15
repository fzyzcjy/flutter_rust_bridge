// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_rust_async_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSseMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSseMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinRustAsyncSseMoiCopyWith(
      EnumWithGoodAndOpaqueTwinRustAsyncSseMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinRustAsyncSseMoi].
extension EnumWithGoodAndOpaqueTwinRustAsyncSseMoiPatterns
    on EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque value)?
        opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque()
          when opaque != null:
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
    required TResult Function(
            EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good value)
        good,
    required TResult Function(
            EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good value)?
        good,
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque value)?
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque()
          when opaque != null:
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
    TResult Function(NonCloneSimpleTwinRustAsyncSseMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque()
          when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinRustAsyncSseMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinRustAsyncSseMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque()
          when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good
    extends EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
  const EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good>
      get copyWith =>
          _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWithImpl<
              EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSseMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWith<
    $Res> implements $EnumWithGoodAndOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_GoodCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque
    extends EnumWithGoodAndOpaqueTwinRustAsyncSseMoi {
  const EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque(this.field0)
      : super._();

  @override
  final NonCloneSimpleTwinRustAsyncSseMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque>
      get copyWith =>
          _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWithImpl<
                  EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncSseMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWith<
    $Res> implements $EnumWithGoodAndOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque)
              _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinRustAsyncSseMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_OpaqueCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncSseMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinRustAsyncSseMoi,
    ));
  }
}

// dart format on
