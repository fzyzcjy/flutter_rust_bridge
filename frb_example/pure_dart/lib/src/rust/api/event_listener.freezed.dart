// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'event_listener.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$EventTwinNormal {
  String get address => throw _privateConstructorUsedError;
  String get payload => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $EventTwinNormalCopyWith<EventTwinNormal> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EventTwinNormalCopyWith<$Res> {
  factory $EventTwinNormalCopyWith(
          EventTwinNormal value, $Res Function(EventTwinNormal) then) =
      _$EventTwinNormalCopyWithImpl<$Res, EventTwinNormal>;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventTwinNormalCopyWithImpl<$Res, $Val extends EventTwinNormal>
    implements $EventTwinNormalCopyWith<$Res> {
  _$EventTwinNormalCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

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
abstract class _$$EventTwinNormalImplCopyWith<$Res>
    implements $EventTwinNormalCopyWith<$Res> {
  factory _$$EventTwinNormalImplCopyWith(_$EventTwinNormalImpl value,
          $Res Function(_$EventTwinNormalImpl) then) =
      __$$EventTwinNormalImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$$EventTwinNormalImplCopyWithImpl<$Res>
    extends _$EventTwinNormalCopyWithImpl<$Res, _$EventTwinNormalImpl>
    implements _$$EventTwinNormalImplCopyWith<$Res> {
  __$$EventTwinNormalImplCopyWithImpl(
      _$EventTwinNormalImpl _value, $Res Function(_$EventTwinNormalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_$EventTwinNormalImpl(
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

class _$EventTwinNormalImpl extends _EventTwinNormal {
  const _$EventTwinNormalImpl({required this.address, required this.payload})
      : super._();

  @override
  final String address;
  @override
  final String payload;

  @override
  String toString() {
    return 'EventTwinNormal(address: $address, payload: $payload)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EventTwinNormalImpl &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EventTwinNormalImplCopyWith<_$EventTwinNormalImpl> get copyWith =>
      __$$EventTwinNormalImplCopyWithImpl<_$EventTwinNormalImpl>(
          this, _$identity);
}

abstract class _EventTwinNormal extends EventTwinNormal {
  const factory _EventTwinNormal(
      {required final String address,
      required final String payload}) = _$EventTwinNormalImpl;
  const _EventTwinNormal._() : super._();

  @override
  String get address;
  @override
  String get payload;
  @override
  @JsonKey(ignore: true)
  _$$EventTwinNormalImplCopyWith<_$EventTwinNormalImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
