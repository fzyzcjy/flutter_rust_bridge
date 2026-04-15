// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dart_opaque.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumDartOpaqueTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $EnumDartOpaqueTwinNormalCopyWith<$Res> {
  $EnumDartOpaqueTwinNormalCopyWith(
      EnumDartOpaqueTwinNormal _, $Res Function(EnumDartOpaqueTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumDartOpaqueTwinNormal].
extension EnumDartOpaqueTwinNormalPatterns on EnumDartOpaqueTwinNormal {
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
    TResult Function(EnumDartOpaqueTwinNormal_Primitive value)? primitive,
    TResult Function(EnumDartOpaqueTwinNormal_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinNormal_Opaque() when opaque != null:
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
    required TResult Function(EnumDartOpaqueTwinNormal_Primitive value)
        primitive,
    required TResult Function(EnumDartOpaqueTwinNormal_Opaque value) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive():
        return primitive(_that);
      case EnumDartOpaqueTwinNormal_Opaque():
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
    TResult? Function(EnumDartOpaqueTwinNormal_Primitive value)? primitive,
    TResult? Function(EnumDartOpaqueTwinNormal_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinNormal_Opaque() when opaque != null:
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
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinNormal_Opaque() when opaque != null:
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
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive():
        return primitive(_that.field0);
      case EnumDartOpaqueTwinNormal_Opaque():
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
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinNormal_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumDartOpaqueTwinNormal_Primitive extends EnumDartOpaqueTwinNormal {
  const EnumDartOpaqueTwinNormal_Primitive(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumDartOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinNormal_PrimitiveCopyWith<
          EnumDartOpaqueTwinNormal_Primitive>
      get copyWith => _$EnumDartOpaqueTwinNormal_PrimitiveCopyWithImpl<
          EnumDartOpaqueTwinNormal_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinNormal_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumDartOpaqueTwinNormal.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinNormal_PrimitiveCopyWith<$Res>
    implements $EnumDartOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumDartOpaqueTwinNormal_PrimitiveCopyWith(
          EnumDartOpaqueTwinNormal_Primitive value,
          $Res Function(EnumDartOpaqueTwinNormal_Primitive) _then) =
      _$EnumDartOpaqueTwinNormal_PrimitiveCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinNormal_PrimitiveCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinNormal_PrimitiveCopyWith<$Res> {
  _$EnumDartOpaqueTwinNormal_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinNormal_Primitive _self;
  final $Res Function(EnumDartOpaqueTwinNormal_Primitive) _then;

  /// Create a copy of EnumDartOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinNormal_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class EnumDartOpaqueTwinNormal_Opaque extends EnumDartOpaqueTwinNormal {
  const EnumDartOpaqueTwinNormal_Opaque(this.field0) : super._();

  @override
  final Object field0;

  /// Create a copy of EnumDartOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinNormal_OpaqueCopyWith<EnumDartOpaqueTwinNormal_Opaque>
      get copyWith => _$EnumDartOpaqueTwinNormal_OpaqueCopyWithImpl<
          EnumDartOpaqueTwinNormal_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinNormal_Opaque &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinNormal.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinNormal_OpaqueCopyWith<$Res>
    implements $EnumDartOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumDartOpaqueTwinNormal_OpaqueCopyWith(
          EnumDartOpaqueTwinNormal_Opaque value,
          $Res Function(EnumDartOpaqueTwinNormal_Opaque) _then) =
      _$EnumDartOpaqueTwinNormal_OpaqueCopyWithImpl;
  @useResult
  $Res call({Object field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinNormal_OpaqueCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinNormal_OpaqueCopyWith<$Res> {
  _$EnumDartOpaqueTwinNormal_OpaqueCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinNormal_Opaque _self;
  final $Res Function(EnumDartOpaqueTwinNormal_Opaque) _then;

  /// Create a copy of EnumDartOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinNormal_Opaque(
      null == field0 ? _self.field0 : field0,
    ));
  }
}

// dart format on
