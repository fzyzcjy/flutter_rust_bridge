// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sync_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSyncMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSyncMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSyncMoiCopyWith(EnumWithGoodAndOpaqueTwinSyncMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinSyncMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSyncMoi].
extension EnumWithGoodAndOpaqueTwinSyncMoiPatterns
    on EnumWithGoodAndOpaqueTwinSyncMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSyncMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncMoi_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncMoi_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSyncMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSyncMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSyncMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSyncMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSyncMoi_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncMoi_Good
    extends EnumWithGoodAndOpaqueTwinSyncMoi {
  const EnumWithGoodAndOpaqueTwinSyncMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinSyncMoi_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSyncMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncMoi_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSyncMoi_Opaque
    extends EnumWithGoodAndOpaqueTwinSyncMoi {
  const EnumWithGoodAndOpaqueTwinSyncMoi_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSyncMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSyncMoi_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSyncMoi_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSyncMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSyncMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSyncMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSyncMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSyncMoi_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSyncMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSyncMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSyncMoi,
    ));
  }
}

// dart format on
