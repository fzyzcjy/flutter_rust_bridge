// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$EventTwinSse {
  String get address => throw _privateConstructorUsedError;
  String get payload => throw _privateConstructorUsedError;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $EventTwinSseCopyWith<EventTwinSse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EventTwinSseCopyWith<$Res> {
  factory $EventTwinSseCopyWith(
          EventTwinSse value, $Res Function(EventTwinSse) then) =
      _$EventTwinSseCopyWithImpl<$Res, EventTwinSse>;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinSseCopyWithImpl<$Res, $Val extends EventTwinSse>
    implements $EventTwinSseCopyWith<$Res> {
  _$EventTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_value.copyWith(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$EventTwinSseImplCopyWith<$Res>
    implements $EventTwinSseCopyWith<$Res> {
  factory _$$EventTwinSseImplCopyWith(
          _$EventTwinSseImpl value, $Res Function(_$EventTwinSseImpl) then) =
      __$$EventTwinSseImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$$EventTwinSseImplCopyWithImpl<$Res>
    extends _$EventTwinSseCopyWithImpl<$Res, _$EventTwinSseImpl>
    implements _$$EventTwinSseImplCopyWith<$Res> {
  __$$EventTwinSseImplCopyWithImpl(
      _$EventTwinSseImpl _value, $Res Function(_$EventTwinSseImpl) _then)
      : super(_value, _then);

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_$EventTwinSseImpl(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$EventTwinSseImpl extends _EventTwinSse {
  const _$EventTwinSseImpl({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  @override
  String toString() {
    return 'EventTwinSse(address: $address, payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EventTwinSseImpl &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EventTwinSseImplCopyWith<_$EventTwinSseImpl> get copyWith =>
      __$$EventTwinSseImplCopyWithImpl<_$EventTwinSseImpl>(this, _$identity);
}

abstract class _EventTwinSse extends EventTwinSse {
  const factory _EventTwinSse(
      {required final String address,
      required final String payload}) = _$EventTwinSseImpl;
  const _EventTwinSse._() : super._();

  @override
  String get address;
  @override
  String get payload;

  /// Create a copy of EventTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EventTwinSseImplCopyWith<_$EventTwinSseImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
