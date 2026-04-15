// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinNormalCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinNormalCopyWith(EnumWithGoodAndOpaqueTwinNormal _,
      $Res Function(EnumWithGoodAndOpaqueTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinNormal].
extension EnumWithGoodAndOpaqueTwinNormalPatterns
    on EnumWithGoodAndOpaqueTwinNormal {
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
    TResult Function(EnumWithGoodAndOpaqueTwinNormal_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinNormal_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinNormal_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinNormal_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinNormal_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinNormal_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinNormal field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinNormal field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque():
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
    TResult? Function(NonCloneSimpleTwinNormal field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinNormal_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinNormal_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinNormal_Good
    extends EnumWithGoodAndOpaqueTwinNormal {
  const EnumWithGoodAndOpaqueTwinNormal_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinNormal_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinNormal_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinNormal_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinNormal_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinNormal_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinNormal.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinNormal_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinNormal_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinNormal_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinNormal_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinNormal_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinNormal_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinNormal_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinNormal_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinNormal_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinNormal_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinNormal_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinNormal_Opaque
    extends EnumWithGoodAndOpaqueTwinNormal {
  const EnumWithGoodAndOpaqueTwinNormal_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinNormal field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinNormal_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinNormal_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinNormal_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinNormal.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinNormal_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinNormal_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinNormal field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinNormal_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinNormal_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinNormal_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinNormal_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinNormal,
    ));
  }
}

// dart format on
