// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dart_opaque_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumDartOpaqueTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $EnumDartOpaqueTwinSseCopyWith<$Res> {
  $EnumDartOpaqueTwinSseCopyWith(
      EnumDartOpaqueTwinSse _, $Res Function(EnumDartOpaqueTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumDartOpaqueTwinSse].
extension EnumDartOpaqueTwinSsePatterns on EnumDartOpaqueTwinSse {
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
    TResult Function(EnumDartOpaqueTwinSse_Primitive value)? primitive,
    TResult Function(EnumDartOpaqueTwinSse_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinSse_Opaque() when opaque != null:
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
    required TResult Function(EnumDartOpaqueTwinSse_Primitive value) primitive,
    required TResult Function(EnumDartOpaqueTwinSse_Opaque value) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinSse_Primitive():
        return primitive(_that);
      case EnumDartOpaqueTwinSse_Opaque():
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
    TResult? Function(EnumDartOpaqueTwinSse_Primitive value)? primitive,
    TResult? Function(EnumDartOpaqueTwinSse_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinSse_Opaque() when opaque != null:
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
      case EnumDartOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinSse_Opaque() when opaque != null:
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
      case EnumDartOpaqueTwinSse_Primitive():
        return primitive(_that.field0);
      case EnumDartOpaqueTwinSse_Opaque():
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
      case EnumDartOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinSse_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumDartOpaqueTwinSse_Primitive extends EnumDartOpaqueTwinSse {
  const EnumDartOpaqueTwinSse_Primitive(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumDartOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinSse_PrimitiveCopyWith<EnumDartOpaqueTwinSse_Primitive>
      get copyWith => _$EnumDartOpaqueTwinSse_PrimitiveCopyWithImpl<
          EnumDartOpaqueTwinSse_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinSse_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumDartOpaqueTwinSse.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinSse_PrimitiveCopyWith<$Res>
    implements $EnumDartOpaqueTwinSseCopyWith<$Res> {
  factory $EnumDartOpaqueTwinSse_PrimitiveCopyWith(
          EnumDartOpaqueTwinSse_Primitive value,
          $Res Function(EnumDartOpaqueTwinSse_Primitive) _then) =
      _$EnumDartOpaqueTwinSse_PrimitiveCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinSse_PrimitiveCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinSse_PrimitiveCopyWith<$Res> {
  _$EnumDartOpaqueTwinSse_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinSse_Primitive _self;
  final $Res Function(EnumDartOpaqueTwinSse_Primitive) _then;

  /// Create a copy of EnumDartOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinSse_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class EnumDartOpaqueTwinSse_Opaque extends EnumDartOpaqueTwinSse {
  const EnumDartOpaqueTwinSse_Opaque(this.field0) : super._();

  @override
  final Object field0;

  /// Create a copy of EnumDartOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinSse_OpaqueCopyWith<EnumDartOpaqueTwinSse_Opaque>
      get copyWith => _$EnumDartOpaqueTwinSse_OpaqueCopyWithImpl<
          EnumDartOpaqueTwinSse_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinSse_Opaque &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinSse.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinSse_OpaqueCopyWith<$Res>
    implements $EnumDartOpaqueTwinSseCopyWith<$Res> {
  factory $EnumDartOpaqueTwinSse_OpaqueCopyWith(
          EnumDartOpaqueTwinSse_Opaque value,
          $Res Function(EnumDartOpaqueTwinSse_Opaque) _then) =
      _$EnumDartOpaqueTwinSse_OpaqueCopyWithImpl;
  @useResult
  $Res call({Object field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinSse_OpaqueCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinSse_OpaqueCopyWith<$Res> {
  _$EnumDartOpaqueTwinSse_OpaqueCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinSse_Opaque _self;
  final $Res Function(EnumDartOpaqueTwinSse_Opaque) _then;

  /// Create a copy of EnumDartOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinSse_Opaque(
      null == field0 ? _self.field0 : field0,
    ));
  }
}

// dart format on
