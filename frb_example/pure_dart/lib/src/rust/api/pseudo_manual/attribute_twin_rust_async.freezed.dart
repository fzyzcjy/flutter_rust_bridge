// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'attribute_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$UserIdTwinRustAsync {
  int get value;

  /// Create a copy of UserIdTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $UserIdTwinRustAsyncCopyWith<UserIdTwinRustAsync> get copyWith =>
      _$UserIdTwinRustAsyncCopyWithImpl<UserIdTwinRustAsync>(
          this as UserIdTwinRustAsync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is UserIdTwinRustAsync &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinRustAsync(value: $value)';
  }
}

/// @nodoc
abstract mixin class $UserIdTwinRustAsyncCopyWith<$Res> {
  factory $UserIdTwinRustAsyncCopyWith(
          UserIdTwinRustAsync value, $Res Function(UserIdTwinRustAsync) _then) =
      _$UserIdTwinRustAsyncCopyWithImpl;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdTwinRustAsyncCopyWithImpl<$Res>
    implements $UserIdTwinRustAsyncCopyWith<$Res> {
  _$UserIdTwinRustAsyncCopyWithImpl(this._self, this._then);

  final UserIdTwinRustAsync _self;
  final $Res Function(UserIdTwinRustAsync) _then;

  /// Create a copy of UserIdTwinRustAsync
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

/// Adds pattern-matching-related methods to [UserIdTwinRustAsync].
extension UserIdTwinRustAsyncPatterns on UserIdTwinRustAsync {
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
    TResult Function(_UserIdTwinRustAsync value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsync() when $default != null:
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
    TResult Function(_UserIdTwinRustAsync value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsync():
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
    TResult? Function(_UserIdTwinRustAsync value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _UserIdTwinRustAsync() when $default != null:
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
      case _UserIdTwinRustAsync() when $default != null:
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
      case _UserIdTwinRustAsync():
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
      case _UserIdTwinRustAsync() when $default != null:
        return $default(_that.value);
      case _:
        return null;
    }
  }
}

/// @nodoc

class _UserIdTwinRustAsync implements UserIdTwinRustAsync {
  const _UserIdTwinRustAsync({this.value = 0});

  @override
  @JsonKey()
  final int value;

  /// Create a copy of UserIdTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$UserIdTwinRustAsyncCopyWith<_UserIdTwinRustAsync> get copyWith =>
      __$UserIdTwinRustAsyncCopyWithImpl<_UserIdTwinRustAsync>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _UserIdTwinRustAsync &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @override
  String toString() {
    return 'UserIdTwinRustAsync(value: $value)';
  }
}

/// @nodoc
abstract mixin class _$UserIdTwinRustAsyncCopyWith<$Res>
    implements $UserIdTwinRustAsyncCopyWith<$Res> {
  factory _$UserIdTwinRustAsyncCopyWith(_UserIdTwinRustAsync value,
          $Res Function(_UserIdTwinRustAsync) _then) =
      __$UserIdTwinRustAsyncCopyWithImpl;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$UserIdTwinRustAsyncCopyWithImpl<$Res>
    implements _$UserIdTwinRustAsyncCopyWith<$Res> {
  __$UserIdTwinRustAsyncCopyWithImpl(this._self, this._then);

  final _UserIdTwinRustAsync _self;
  final $Res Function(_UserIdTwinRustAsync) _then;

  /// Create a copy of UserIdTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? value = null,
  }) {
    return _then(_UserIdTwinRustAsync(
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

// dart format on
