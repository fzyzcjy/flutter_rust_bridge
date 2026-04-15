// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinMoiCopyWith(EnumWithGoodAndOpaqueTwinMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinMoi].
extension EnumWithGoodAndOpaqueTwinMoiPatterns on EnumWithGoodAndOpaqueTwinMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinMoi_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinMoi_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinMoi_Opaque value) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinMoi_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinMoi_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinMoi_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinMoi_Good extends EnumWithGoodAndOpaqueTwinMoi {
  const EnumWithGoodAndOpaqueTwinMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinMoi_GoodCopyWith<EnumWithGoodAndOpaqueTwinMoi_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinMoi_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinMoi_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinMoi_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinMoi_Opaque extends EnumWithGoodAndOpaqueTwinMoi {
  const EnumWithGoodAndOpaqueTwinMoi_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinMoi_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinMoi_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinMoi_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinMoi_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinMoi,
    ));
  }
}

// dart format on
