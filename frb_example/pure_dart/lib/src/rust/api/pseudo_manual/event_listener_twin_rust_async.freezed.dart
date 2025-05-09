// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EventTwinRustAsync {
  String get address;
  String get payload;

  /// Create a copy of EventTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EventTwinRustAsyncCopyWith<EventTwinRustAsync> get copyWith =>
      _$EventTwinRustAsyncCopyWithImpl<EventTwinRustAsync>(
          this as EventTwinRustAsync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EventTwinRustAsync &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinRustAsync(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class $EventTwinRustAsyncCopyWith<$Res> {
  factory $EventTwinRustAsyncCopyWith(
          EventTwinRustAsync value, $Res Function(EventTwinRustAsync) _then) =
      _$EventTwinRustAsyncCopyWithImpl;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinRustAsyncCopyWithImpl<$Res>
    implements $EventTwinRustAsyncCopyWith<$Res> {
  _$EventTwinRustAsyncCopyWithImpl(this._self, this._then);

  final EventTwinRustAsync _self;
  final $Res Function(EventTwinRustAsync) _then;

  /// Create a copy of EventTwinRustAsync
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

/// @nodoc

class _EventTwinRustAsync extends EventTwinRustAsync {
  const _EventTwinRustAsync({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  /// Create a copy of EventTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$EventTwinRustAsyncCopyWith<_EventTwinRustAsync> get copyWith =>
      __$EventTwinRustAsyncCopyWithImpl<_EventTwinRustAsync>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _EventTwinRustAsync &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinRustAsync(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class _$EventTwinRustAsyncCopyWith<$Res>
    implements $EventTwinRustAsyncCopyWith<$Res> {
  factory _$EventTwinRustAsyncCopyWith(
          _EventTwinRustAsync value, $Res Function(_EventTwinRustAsync) _then) =
      __$EventTwinRustAsyncCopyWithImpl;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$EventTwinRustAsyncCopyWithImpl<$Res>
    implements _$EventTwinRustAsyncCopyWith<$Res> {
  __$EventTwinRustAsyncCopyWithImpl(this._self, this._then);

  final _EventTwinRustAsync _self;
  final $Res Function(_EventTwinRustAsync) _then;

  /// Create a copy of EventTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_EventTwinRustAsync(
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
