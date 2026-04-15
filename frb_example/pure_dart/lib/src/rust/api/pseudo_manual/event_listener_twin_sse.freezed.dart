// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EventTwinSse {
  String get address;
  String get payload;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EventTwinSseCopyWith<EventTwinSse> get copyWith =>
      _$EventTwinSseCopyWithImpl<EventTwinSse>(
          this as EventTwinSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EventTwinSse &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinSse(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class $EventTwinSseCopyWith<$Res> {
  factory $EventTwinSseCopyWith(
          EventTwinSse value, $Res Function(EventTwinSse) _then) =
      _$EventTwinSseCopyWithImpl;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinSseCopyWithImpl<$Res> implements $EventTwinSseCopyWith<$Res> {
  _$EventTwinSseCopyWithImpl(this._self, this._then);

  final EventTwinSse _self;
  final $Res Function(EventTwinSse) _then;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_self.copyWith(
      address: null == address
          ? _self.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _self.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// Adds pattern-matching-related methods to [EventTwinSse].
extension EventTwinSsePatterns on EventTwinSse {
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
    TResult Function(_EventTwinSse value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse() when $default != null:
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
    TResult Function(_EventTwinSse value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse():
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
    TResult? Function(_EventTwinSse value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse() when $default != null:
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
    TResult Function(String address, String payload)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse() when $default != null:
        return $default(_that.address, _that.payload);
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
    TResult Function(String address, String payload) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse():
        return $default(_that.address, _that.payload);
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
    TResult? Function(String address, String payload)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _EventTwinSse() when $default != null:
        return $default(_that.address, _that.payload);
      case _:
        return null;
    }
  }
}

/// @nodoc

class _EventTwinSse extends EventTwinSse {
  const _EventTwinSse({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$EventTwinSseCopyWith<_EventTwinSse> get copyWith =>
      __$EventTwinSseCopyWithImpl<_EventTwinSse>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _EventTwinSse &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinSse(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class _$EventTwinSseCopyWith<$Res>
    implements $EventTwinSseCopyWith<$Res> {
  factory _$EventTwinSseCopyWith(
          _EventTwinSse value, $Res Function(_EventTwinSse) _then) =
      __$EventTwinSseCopyWithImpl;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$EventTwinSseCopyWithImpl<$Res>
    implements _$EventTwinSseCopyWith<$Res> {
  __$EventTwinSseCopyWithImpl(this._self, this._then);

  final _EventTwinSse _self;
  final $Res Function(_EventTwinSse) _then;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_EventTwinSse(
      address: null == address
          ? _self.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _self.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
