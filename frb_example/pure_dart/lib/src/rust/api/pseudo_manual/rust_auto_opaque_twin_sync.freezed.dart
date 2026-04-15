// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSyncCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSyncCopyWith(EnumWithGoodAndOpaqueTwinSync _,
      $Res Function(EnumWithGoodAndOpaqueTwinSync) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSync].
extension EnumWithGoodAndOpaqueTwinSyncPatterns
    on EnumWithGoodAndOpaqueTwinSync {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSync_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSync_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSync_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSync_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSync_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSync_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSync_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSync_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSync_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSync field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSync_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSync field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSync_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSync field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSync_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSync_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSync_Good extends EnumWithGoodAndOpaqueTwinSync {
  const EnumWithGoodAndOpaqueTwinSync_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSync_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinSync_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSync_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSync_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSync_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSync.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSync_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSync_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSync_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSync_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSync_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSync_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSync_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSync_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSync_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSync_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSync_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSync_Opaque
    extends EnumWithGoodAndOpaqueTwinSync {
  const EnumWithGoodAndOpaqueTwinSync_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSync field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSync_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSync_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSync_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSync.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSync_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSync_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSync field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSync_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSync_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSync_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSync_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSync,
    ));
  }
}

// dart format on
