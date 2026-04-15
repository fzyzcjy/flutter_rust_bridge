// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_rust_async_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinRustAsyncMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinRustAsyncMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinRustAsyncMoiCopyWith(
      EnumWithGoodAndOpaqueTwinRustAsyncMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinRustAsyncMoi].
extension EnumWithGoodAndOpaqueTwinRustAsyncMoiPatterns
    on EnumWithGoodAndOpaqueTwinRustAsyncMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque value)?
        opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good value)
        good,
    required TResult Function(
            EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque value)?
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinRustAsyncMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinRustAsyncMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinRustAsyncMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good
    extends EnumWithGoodAndOpaqueTwinRustAsyncMoi {
  const EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_GoodCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque
    extends EnumWithGoodAndOpaqueTwinRustAsyncMoi {
  const EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinRustAsyncMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque>
      get copyWith =>
          _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWithImpl<
              EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinRustAsyncMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinRustAsyncMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinRustAsyncMoi_OpaqueCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinRustAsyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinRustAsyncMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinRustAsyncMoi,
    ));
  }
}

// dart format on
