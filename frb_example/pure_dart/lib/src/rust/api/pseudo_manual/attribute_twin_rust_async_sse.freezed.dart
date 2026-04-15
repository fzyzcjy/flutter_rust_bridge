// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinRustAsyncSse {
  int get value;

  /// Create a copy of UserIdTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinRustAsyncSseCopyWith<UserIdTwinRustAsyncSse> get copyWith =>
      _$UserIdTwinRustAsyncSseCopyWithImpl<UserIdTwinRustAsyncSse>(
          this as UserIdTwinRustAsyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinRustAsyncSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinRustAsyncSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinRustAsyncSseCopyWith<$Res> {
  factory $UserIdTwinRustAsyncSseCopyWith(UserIdTwinRustAsyncSse value,
          $Res Function(UserIdTwinRustAsyncSse) _then) =
      _$UserIdTwinRustAsyncSseCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinRustAsyncSseCopyWithImpl<$Res>
    implements $UserIdTwinRustAsyncSseCopyWith<$Res> {
  _$UserIdTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final UserIdTwinRustAsyncSse _self;
  final $Res Function(UserIdTwinRustAsyncSse) _then;

  /// Create a copy of UserIdTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_self.copyWith(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// Adds pattern-matching-related methods to [UserIdTwinRustAsyncSse].
extension UserIdTwinRustAsyncSsePatterns on UserIdTwinRustAsyncSse {
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
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_UserIdTwinRustAsyncSse value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse() when $default != null:
        return $default(_that);
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
  TResult map<TResult extends Object?>(
    TResult Function(_UserIdTwinRustAsyncSse value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse():
        return $default(_that);
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
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_UserIdTwinRustAsyncSse value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse() when $default != null:
        return $default(_that);
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
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(int value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse() when $default != null:
        return $default(_that.value);
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
  TResult when<TResult extends Object?>(
    TResult Function(int value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse():
        return $default(_that.value);
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
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(int value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsyncSse() when $default != null:
        return $default(_that.value);
      case _:
        return null;
    }
  }
}

/// @nodoc

class _UserIdTwinRustAsyncSse implements UserIdTwinRustAsyncSse {
  const _UserIdTwinRustAsyncSse({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinRustAsyncSseCopyWith<_UserIdTwinRustAsyncSse> get copyWith =>
      __$UserIdTwinRustAsyncSseCopyWithImpl<_UserIdTwinRustAsyncSse>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinRustAsyncSse &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinRustAsyncSse(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinRustAsyncSseCopyWith<$Res>
    implements $UserIdTwinRustAsyncSseCopyWith<$Res> {
  factory _$UserIdTwinRustAsyncSseCopyWith(_UserIdTwinRustAsyncSse value,
          $Res Function(_UserIdTwinRustAsyncSse) _then) =
      __$UserIdTwinRustAsyncSseCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinRustAsyncSseCopyWithImpl<$Res>
    implements _$UserIdTwinRustAsyncSseCopyWith<$Res> {
  __$UserIdTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final _UserIdTwinRustAsyncSse _self;
  final $Res Function(_UserIdTwinRustAsyncSse) _then;

  /// Create a copy of UserIdTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinRustAsyncSse(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
