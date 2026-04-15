// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSyncSseCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSyncSseCopyWith(EnumWithGoodAndOpaqueTwinSyncSse _,
      $Res Function(EnumWithGoodAndOpaqueTwinSyncSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSyncSse].
extension EnumWithGoodAndOpaqueTwinSyncSsePatterns
    on EnumWithGoodAndOpaqueTwinSyncSse {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSyncSse_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSyncSse_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncSse_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncSse_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncSse_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncSse_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSyncSse field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSyncSse field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSyncSse field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSse_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncSse_Good
    extends EnumWithGoodAndOpaqueTwinSyncSse {
  const EnumWithGoodAndOpaqueTwinSyncSse_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinSyncSse_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncSse_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSse_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSse.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSyncSse_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncSse_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncSse_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncSse_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncSse_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncSse_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncSse_Opaque
    extends EnumWithGoodAndOpaqueTwinSyncSse {
  const EnumWithGoodAndOpaqueTwinSyncSse_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSyncSse field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSyncSse_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncSse_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSse_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSse.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSyncSse_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncSse_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSyncSse field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncSse_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncSse_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncSse_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncSse_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSyncSse,
    ));
  }
}

// dart format on
