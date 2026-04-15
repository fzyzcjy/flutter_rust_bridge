// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sync_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSyncSseMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSseMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSseMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSyncSseMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSyncSseMoiCopyWith(
      EnumWithGoodAndOpaqueTwinSyncSseMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinSyncSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSyncSseMoi].
extension EnumWithGoodAndOpaqueTwinSyncSseMoiPatterns
    on EnumWithGoodAndOpaqueTwinSyncSseMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good value)
        good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSyncSseMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSyncSseMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSyncSseMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncSseMoi_Good
    extends EnumWithGoodAndOpaqueTwinSyncSseMoi {
  const EnumWithGoodAndOpaqueTwinSyncSseMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSseMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSseMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncSseMoi_GoodCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncSseMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncSseMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque
    extends EnumWithGoodAndOpaqueTwinSyncSseMoi {
  const EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSyncSseMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncSseMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSyncSseMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncSseMoi_OpaqueCopyWithImpl(
      this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncSseMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSyncSseMoi,
    ));
  }
}

// dart format on
