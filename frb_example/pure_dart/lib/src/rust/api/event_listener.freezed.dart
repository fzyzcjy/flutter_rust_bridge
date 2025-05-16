// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EventTwinNormal {
  String get address;
  String get payload;

  /// Create a copy of EventTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EventTwinNormalCopyWith<EventTwinNormal> get copyWith =>
      _$EventTwinNormalCopyWithImpl<EventTwinNormal>(
          this as EventTwinNormal, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EventTwinNormal &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinNormal(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class $EventTwinNormalCopyWith<$Res> {
  factory $EventTwinNormalCopyWith(
          EventTwinNormal value, $Res Function(EventTwinNormal) _then) =
      _$EventTwinNormalCopyWithImpl;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinNormalCopyWithImpl<$Res>
    implements $EventTwinNormalCopyWith<$Res> {
  _$EventTwinNormalCopyWithImpl(this._self, this._then);

  final EventTwinNormal _self;
  final $Res Function(EventTwinNormal) _then;

  /// Create a copy of EventTwinNormal
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

class _EventTwinNormal extends EventTwinNormal {
  const _EventTwinNormal({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  /// Create a copy of EventTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$EventTwinNormalCopyWith<_EventTwinNormal> get copyWith =>
      __$EventTwinNormalCopyWithImpl<_EventTwinNormal>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _EventTwinNormal &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @override
  String toString() {
    return 'EventTwinNormal(address: $address, payload: $payload)';
  }
}

/// @nodoc
abstract mixin class _$EventTwinNormalCopyWith<$Res>
    implements $EventTwinNormalCopyWith<$Res> {
  factory _$EventTwinNormalCopyWith(
          _EventTwinNormal value, $Res Function(_EventTwinNormal) _then) =
      __$EventTwinNormalCopyWithImpl;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$EventTwinNormalCopyWithImpl<$Res>
    implements _$EventTwinNormalCopyWith<$Res> {
  __$EventTwinNormalCopyWithImpl(this._self, this._then);

  final _EventTwinNormal _self;
  final $Res Function(_EventTwinNormal) _then;

  /// Create a copy of EventTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_EventTwinNormal(
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
