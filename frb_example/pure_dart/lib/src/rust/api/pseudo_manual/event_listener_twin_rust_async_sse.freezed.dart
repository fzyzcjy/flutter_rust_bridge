// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EventTwinRustAsyncSse {
  String get address;
  String get payload;

  /// Create a copy of EventTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EventTwinRustAsyncSseCopyWith<EventTwinRustAsyncSse> get copyWith =>
      _$EventTwinRustAsyncSseCopyWithImpl<EventTwinRustAsyncSse>(
          this as EventTwinRustAsyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EventTwinRustAsyncSse &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinRustAsyncSse(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class $EventTwinRustAsyncSseCopyWith<$Res> {
  factory $EventTwinRustAsyncSseCopyWith(EventTwinRustAsyncSse value,
          $Res Function(EventTwinRustAsyncSse) _then) =
      _$EventTwinRustAsyncSseCopyWithImpl;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinRustAsyncSseCopyWithImpl<$Res>
    implements $EventTwinRustAsyncSseCopyWith<$Res> {
  _$EventTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final EventTwinRustAsyncSse _self;
  final $Res Function(EventTwinRustAsyncSse) _then;

  /// Create a copy of EventTwinRustAsyncSse
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

class _EventTwinRustAsyncSse extends EventTwinRustAsyncSse {
  const _EventTwinRustAsyncSse({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  /// Create a copy of EventTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$EventTwinRustAsyncSseCopyWith<_EventTwinRustAsyncSse> get copyWith =>
      __$EventTwinRustAsyncSseCopyWithImpl<_EventTwinRustAsyncSse>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _EventTwinRustAsyncSse &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinRustAsyncSse(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class _$EventTwinRustAsyncSseCopyWith<$Res>
    implements $EventTwinRustAsyncSseCopyWith<$Res> {
  factory _$EventTwinRustAsyncSseCopyWith(_EventTwinRustAsyncSse value,
          $Res Function(_EventTwinRustAsyncSse) _then) =
      __$EventTwinRustAsyncSseCopyWithImpl;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$EventTwinRustAsyncSseCopyWithImpl<$Res>
    implements _$EventTwinRustAsyncSseCopyWith<$Res> {
  __$EventTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final _EventTwinRustAsyncSse _self;
  final $Res Function(_EventTwinRustAsyncSse) _then;

  /// Create a copy of EventTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_EventTwinRustAsyncSse(
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
