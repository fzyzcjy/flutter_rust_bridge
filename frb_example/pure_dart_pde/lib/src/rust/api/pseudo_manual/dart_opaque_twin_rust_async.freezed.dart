// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dart_opaque_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumDartOpaqueTwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $EnumDartOpaqueTwinRustAsyncCopyWith<$Res> {
  $EnumDartOpaqueTwinRustAsyncCopyWith(EnumDartOpaqueTwinRustAsync _,
      $Res Function(EnumDartOpaqueTwinRustAsync) __);
}

/// Adds pattern-matching-related methods to [EnumDartOpaqueTwinRustAsync].
extension EnumDartOpaqueTwinRustAsyncPatterns on EnumDartOpaqueTwinRustAsync {
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
    TResult Function(EnumDartOpaqueTwinRustAsync_Primitive value)? primitive,
    TResult Function(EnumDartOpaqueTwinRustAsync_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinRustAsync_Opaque() when opaque != null:
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
    required TResult Function(EnumDartOpaqueTwinRustAsync_Primitive value)
        primitive,
    required TResult Function(EnumDartOpaqueTwinRustAsync_Opaque value) opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinRustAsync_Primitive():
        return primitive(_that);
      case EnumDartOpaqueTwinRustAsync_Opaque():
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
    TResult? Function(EnumDartOpaqueTwinRustAsync_Primitive value)? primitive,
    TResult? Function(EnumDartOpaqueTwinRustAsync_Opaque value)? opaque,
  }) {
    final _that = this;
    switch (_that) {
      case EnumDartOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumDartOpaqueTwinRustAsync_Opaque() when opaque != null:
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
      case EnumDartOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinRustAsync_Opaque() when opaque != null:
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
      case EnumDartOpaqueTwinRustAsync_Primitive():
        return primitive(_that.field0);
      case EnumDartOpaqueTwinRustAsync_Opaque():
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
      case EnumDartOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumDartOpaqueTwinRustAsync_Opaque() when opaque != null:
        return opaque(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumDartOpaqueTwinRustAsync_Primitive
    extends EnumDartOpaqueTwinRustAsync {
  const EnumDartOpaqueTwinRustAsync_Primitive(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of EnumDartOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinRustAsync_PrimitiveCopyWith<
          EnumDartOpaqueTwinRustAsync_Primitive>
      get copyWith => _$EnumDartOpaqueTwinRustAsync_PrimitiveCopyWithImpl<
          EnumDartOpaqueTwinRustAsync_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinRustAsync_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumDartOpaqueTwinRustAsync.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinRustAsync_PrimitiveCopyWith<$Res>
    implements $EnumDartOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumDartOpaqueTwinRustAsync_PrimitiveCopyWith(
          EnumDartOpaqueTwinRustAsync_Primitive value,
          $Res Function(EnumDartOpaqueTwinRustAsync_Primitive) _then) =
      _$EnumDartOpaqueTwinRustAsync_PrimitiveCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinRustAsync_PrimitiveCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinRustAsync_PrimitiveCopyWith<$Res> {
  _$EnumDartOpaqueTwinRustAsync_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinRustAsync_Primitive _self;
  final $Res Function(EnumDartOpaqueTwinRustAsync_Primitive) _then;

  /// Create a copy of EnumDartOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinRustAsync_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class EnumDartOpaqueTwinRustAsync_Opaque extends EnumDartOpaqueTwinRustAsync {
  const EnumDartOpaqueTwinRustAsync_Opaque(this.field0) : super._();

  @override
  final Object field0;

  /// Create a copy of EnumDartOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumDartOpaqueTwinRustAsync_OpaqueCopyWith<
          EnumDartOpaqueTwinRustAsync_Opaque>
      get copyWith => _$EnumDartOpaqueTwinRustAsync_OpaqueCopyWithImpl<
          EnumDartOpaqueTwinRustAsync_Opaque>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumDartOpaqueTwinRustAsync_Opaque &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'EnumDartOpaqueTwinRustAsync.opaque(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumDartOpaqueTwinRustAsync_OpaqueCopyWith<$Res>
    implements $EnumDartOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumDartOpaqueTwinRustAsync_OpaqueCopyWith(
          EnumDartOpaqueTwinRustAsync_Opaque value,
          $Res Function(EnumDartOpaqueTwinRustAsync_Opaque) _then) =
      _$EnumDartOpaqueTwinRustAsync_OpaqueCopyWithImpl;
  @useResult
  $Res call({Object field0});
}

/// @nodoc
class _$EnumDartOpaqueTwinRustAsync_OpaqueCopyWithImpl<$Res>
    implements $EnumDartOpaqueTwinRustAsync_OpaqueCopyWith<$Res> {
  _$EnumDartOpaqueTwinRustAsync_OpaqueCopyWithImpl(this._self, this._then);

  final EnumDartOpaqueTwinRustAsync_Opaque _self;
  final $Res Function(EnumDartOpaqueTwinRustAsync_Opaque) _then;

  /// Create a copy of EnumDartOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumDartOpaqueTwinRustAsync_Opaque(
      null == field0 ? _self.field0 : field0,
    ));
  }
}

// dart format on
