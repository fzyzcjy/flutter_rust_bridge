// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSseCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSseCopyWith(EnumWithGoodAndOpaqueTwinSse _,
      $Res Function(EnumWithGoodAndOpaqueTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSse].
extension EnumWithGoodAndOpaqueTwinSsePatterns on EnumWithGoodAndOpaqueTwinSse {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSse_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSse_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSse_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSse_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSse_Opaque value) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSse_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSse_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSse_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSse_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSse field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSse_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSse field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSse_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSse field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSse_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSse_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSse_Good extends EnumWithGoodAndOpaqueTwinSse {
  const EnumWithGoodAndOpaqueTwinSse_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSse_GoodCopyWith<EnumWithGoodAndOpaqueTwinSse_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSse_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSse_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSse_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSse.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSse_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSse_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSse_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSse_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSse_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSse_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSse_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSse_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSse_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSse_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSse_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSse_Opaque extends EnumWithGoodAndOpaqueTwinSse {
  const EnumWithGoodAndOpaqueTwinSse_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSse field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSse_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSse_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSse_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSse.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSse_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSse_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSse field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSse_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSse_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSse_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSse_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSse,
    ));
  }
}

// dart format on
