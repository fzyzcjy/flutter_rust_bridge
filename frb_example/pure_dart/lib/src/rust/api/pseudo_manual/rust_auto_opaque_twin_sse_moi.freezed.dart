// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_auto_opaque_twin_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumWithGoodAndOpaqueTwinSseMoi {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSseMoi &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSseMoi(field0: $field0)';
  }
}

/// @nodoc
class $EnumWithGoodAndOpaqueTwinSseMoiCopyWith<$Res> {
  $EnumWithGoodAndOpaqueTwinSseMoiCopyWith(EnumWithGoodAndOpaqueTwinSseMoi _,
      $Res Function(EnumWithGoodAndOpaqueTwinSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumWithGoodAndOpaqueTwinSseMoi].
extension EnumWithGoodAndOpaqueTwinSseMoiPatterns
    on EnumWithGoodAndOpaqueTwinSseMoi {
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
    TResult Function(EnumWithGoodAndOpaqueTwinSseMoi_Good value)? good,
    TResult Function(EnumWithGoodAndOpaqueTwinSseMoi_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque() when opaque != null:
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
    required TResult Function(EnumWithGoodAndOpaqueTwinSseMoi_Good value) good,
    required TResult Function(EnumWithGoodAndOpaqueTwinSseMoi_Opaque value)
        opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good():
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque():
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
    TResult? Function(EnumWithGoodAndOpaqueTwinSseMoi_Good value)? good,
    TResult? Function(EnumWithGoodAndOpaqueTwinSseMoi_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good() when good != null:
        return good(_that);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque() when opaque != null:
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
    TResult Function(NonCloneSimpleTwinSseMoi field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque() when opaque != null:
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
    required TResult Function(NonCloneSimpleTwinSseMoi field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good():
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque():
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
    TResult? Function(NonCloneSimpleTwinSseMoi field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumWithGoodAndOpaqueTwinSseMoi_Good() when good != null:
        return good(_that.field0);
      case EnumWithGoodAndOpaqueTwinSseMoi_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSseMoi_Good
    extends EnumWithGoodAndOpaqueTwinSseMoi {
  const EnumWithGoodAndOpaqueTwinSseMoi_Good(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWith<
          EnumWithGoodAndOpaqueTwinSseMoi_Good>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSseMoi_Good>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSseMoi_Good &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSseMoi.good(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWith(
          EnumWithGoodAndOpaqueTwinSseMoi_Good value,
          $Res Function(EnumWithGoodAndOpaqueTwinSseMoi_Good) _then) =
      _$EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSseMoi_GoodCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSseMoi_Good _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSseMoi_Good) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSseMoi_Good(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class EnumWithGoodAndOpaqueTwinSseMoi_Opaque
    extends EnumWithGoodAndOpaqueTwinSseMoi {
  const EnumWithGoodAndOpaqueTwinSseMoi_Opaque(this.field0) : super._();

  @override
  final NonCloneSimpleTwinSseMoi field0;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWith<
          EnumWithGoodAndOpaqueTwinSseMoi_Opaque>
      get copyWith => _$EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWithImpl<
          EnumWithGoodAndOpaqueTwinSseMoi_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumWithGoodAndOpaqueTwinSseMoi_Opaque &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumWithGoodAndOpaqueTwinSseMoi.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWith<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWith(
          EnumWithGoodAndOpaqueTwinSseMoi_Opaque value,
          $Res Function(EnumWithGoodAndOpaqueTwinSseMoi_Opaque) _then) =
      _$EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWithImpl;
  @useResult
  $Res call({NonCloneSimpleTwinSseMoi field0});
}

/// @nodoc
class _$EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWithImpl<$Res>
    implements $EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWith<$Res> {
  _$EnumWithGoodAndOpaqueTwinSseMoi_OpaqueCopyWithImpl(this._self, this._then);

  final EnumWithGoodAndOpaqueTwinSseMoi_Opaque _self;
  final $Res Function(EnumWithGoodAndOpaqueTwinSseMoi_Opaque) _then;

  /// Create a copy of EnumWithGoodAndOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumWithGoodAndOpaqueTwinSseMoi_Opaque(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NonCloneSimpleTwinSseMoi,
    ));
  }
}

// dart format on
