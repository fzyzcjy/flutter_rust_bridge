// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

DistanceTwinSse _$DistanceTwinSseFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'unknown':
      return DistanceTwinSse_Unknown.fromJson(json);
    case 'map':
      return DistanceTwinSse_Map.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'DistanceTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$DistanceTwinSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSse_Map value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSse_Map value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSse_Map value)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DistanceTwinSseCopyWith<$Res> {
  factory $DistanceTwinSseCopyWith(
          DistanceTwinSse value, $Res Function(DistanceTwinSse) then) =
      _$DistanceTwinSseCopyWithImpl<$Res, DistanceTwinSse>;
}

/// @nodoc
class _$DistanceTwinSseCopyWithImpl<$Res, $Val extends DistanceTwinSse>
    implements $DistanceTwinSseCopyWith<$Res> {
  _$DistanceTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$DistanceTwinSse_UnknownImplCopyWith<$Res> {
  factory _$$DistanceTwinSse_UnknownImplCopyWith(
          _$DistanceTwinSse_UnknownImpl value,
          $Res Function(_$DistanceTwinSse_UnknownImpl) then) =
      __$$DistanceTwinSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DistanceTwinSse_UnknownImplCopyWithImpl<$Res>
    extends _$DistanceTwinSseCopyWithImpl<$Res, _$DistanceTwinSse_UnknownImpl>
    implements _$$DistanceTwinSse_UnknownImplCopyWith<$Res> {
  __$$DistanceTwinSse_UnknownImplCopyWithImpl(
      _$DistanceTwinSse_UnknownImpl _value,
      $Res Function(_$DistanceTwinSse_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$DistanceTwinSse_UnknownImpl extends DistanceTwinSse_Unknown {
  const _$DistanceTwinSse_UnknownImpl({final String? $type})
      : $type = $type ?? 'unknown',
        super._();

  factory _$DistanceTwinSse_UnknownImpl.fromJson(Map<String, dynamic> json) =>
      _$$DistanceTwinSse_UnknownImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'DistanceTwinSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinSse_UnknownImpl);
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSse_Map value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSse_Map value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DistanceTwinSse_UnknownImplToJson(
      this,
    );
  }
}

abstract class DistanceTwinSse_Unknown extends DistanceTwinSse {
  const factory DistanceTwinSse_Unknown() = _$DistanceTwinSse_UnknownImpl;
  const DistanceTwinSse_Unknown._() : super._();

  factory DistanceTwinSse_Unknown.fromJson(Map<String, dynamic> json) =
      _$DistanceTwinSse_UnknownImpl.fromJson;
}

/// @nodoc
abstract class _$$DistanceTwinSse_MapImplCopyWith<$Res> {
  factory _$$DistanceTwinSse_MapImplCopyWith(_$DistanceTwinSse_MapImpl value,
          $Res Function(_$DistanceTwinSse_MapImpl) then) =
      __$$DistanceTwinSse_MapImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$DistanceTwinSse_MapImplCopyWithImpl<$Res>
    extends _$DistanceTwinSseCopyWithImpl<$Res, _$DistanceTwinSse_MapImpl>
    implements _$$DistanceTwinSse_MapImplCopyWith<$Res> {
  __$$DistanceTwinSse_MapImplCopyWithImpl(_$DistanceTwinSse_MapImpl _value,
      $Res Function(_$DistanceTwinSse_MapImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DistanceTwinSse_MapImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DistanceTwinSse_MapImpl extends DistanceTwinSse_Map {
  const _$DistanceTwinSse_MapImpl(this.field0, {final String? $type})
      : $type = $type ?? 'map',
        super._();

  factory _$DistanceTwinSse_MapImpl.fromJson(Map<String, dynamic> json) =>
      _$$DistanceTwinSse_MapImplFromJson(json);

  @override
  final double field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'DistanceTwinSse.map(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinSse_MapImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DistanceTwinSse_MapImplCopyWith<_$DistanceTwinSse_MapImpl> get copyWith =>
      __$$DistanceTwinSse_MapImplCopyWithImpl<_$DistanceTwinSse_MapImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return map(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return map?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSse_Map value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSse_Map value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DistanceTwinSse_MapImplToJson(
      this,
    );
  }
}

abstract class DistanceTwinSse_Map extends DistanceTwinSse {
  const factory DistanceTwinSse_Map(final double field0) =
      _$DistanceTwinSse_MapImpl;
  const DistanceTwinSse_Map._() : super._();

  factory DistanceTwinSse_Map.fromJson(Map<String, dynamic> json) =
      _$DistanceTwinSse_MapImpl.fromJson;

  double get field0;
  @JsonKey(ignore: true)
  _$$DistanceTwinSse_MapImplCopyWith<_$DistanceTwinSse_MapImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

EnumWithItemMixedTwinSse _$EnumWithItemMixedTwinSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemMixedTwinSse_A.fromJson(json);
    case 'b':
      return EnumWithItemMixedTwinSse_B.fromJson(json);
    case 'c':
      return EnumWithItemMixedTwinSse_C.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemMixedTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemMixedTwinSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSse_C value) c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSse_C value)? c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSse_C value)? c,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemMixedTwinSseCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSseCopyWith(EnumWithItemMixedTwinSse value,
          $Res Function(EnumWithItemMixedTwinSse) then) =
      _$EnumWithItemMixedTwinSseCopyWithImpl<$Res, EnumWithItemMixedTwinSse>;
}

/// @nodoc
class _$EnumWithItemMixedTwinSseCopyWithImpl<$Res,
        $Val extends EnumWithItemMixedTwinSse>
    implements $EnumWithItemMixedTwinSseCopyWith<$Res> {
  _$EnumWithItemMixedTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSse_AImplCopyWith(
          _$EnumWithItemMixedTwinSse_AImpl value,
          $Res Function(_$EnumWithItemMixedTwinSse_AImpl) then) =
      __$$EnumWithItemMixedTwinSse_AImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$EnumWithItemMixedTwinSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSse_AImpl>
    implements _$$EnumWithItemMixedTwinSse_AImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSse_AImplCopyWithImpl(
      _$EnumWithItemMixedTwinSse_AImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSse_AImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinSse_AImpl extends EnumWithItemMixedTwinSse_A {
  const _$EnumWithItemMixedTwinSse_AImpl({final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemMixedTwinSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinSse_AImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.a()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSse_AImpl);
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return a();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return a?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSse_C value) c,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSse_C value)? c,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSse_C value)? c,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinSse_A extends EnumWithItemMixedTwinSse {
  const factory EnumWithItemMixedTwinSse_A() = _$EnumWithItemMixedTwinSse_AImpl;
  const EnumWithItemMixedTwinSse_A._() : super._();

  factory EnumWithItemMixedTwinSse_A.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinSse_AImpl.fromJson;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSse_BImplCopyWith(
          _$EnumWithItemMixedTwinSse_BImpl value,
          $Res Function(_$EnumWithItemMixedTwinSse_BImpl) then) =
      __$$EnumWithItemMixedTwinSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemMixedTwinSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSse_BImpl>
    implements _$$EnumWithItemMixedTwinSse_BImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSse_BImplCopyWithImpl(
      _$EnumWithItemMixedTwinSse_BImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemMixedTwinSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinSse_BImpl extends EnumWithItemMixedTwinSse_B {
  const _$EnumWithItemMixedTwinSse_BImpl(this.field0, {final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemMixedTwinSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinSse_BImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSse_BImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinSse_BImplCopyWith<_$EnumWithItemMixedTwinSse_BImpl>
      get copyWith => __$$EnumWithItemMixedTwinSse_BImplCopyWithImpl<
          _$EnumWithItemMixedTwinSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSse_C value) c,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSse_C value)? c,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSse_C value)? c,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinSse_B extends EnumWithItemMixedTwinSse {
  const factory EnumWithItemMixedTwinSse_B(final Uint8List field0) =
      _$EnumWithItemMixedTwinSse_BImpl;
  const EnumWithItemMixedTwinSse_B._() : super._();

  factory EnumWithItemMixedTwinSse_B.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinSse_BImpl.fromJson;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemMixedTwinSse_BImplCopyWith<_$EnumWithItemMixedTwinSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSse_CImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSse_CImplCopyWith(
          _$EnumWithItemMixedTwinSse_CImpl value,
          $Res Function(_$EnumWithItemMixedTwinSse_CImpl) then) =
      __$$EnumWithItemMixedTwinSse_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class __$$EnumWithItemMixedTwinSse_CImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSse_CImpl>
    implements _$$EnumWithItemMixedTwinSse_CImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSse_CImplCopyWithImpl(
      _$EnumWithItemMixedTwinSse_CImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSse_CImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? cField = null,
  }) {
    return _then(_$EnumWithItemMixedTwinSse_CImpl(
      cField: null == cField
          ? _value.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinSse_CImpl extends EnumWithItemMixedTwinSse_C {
  const _$EnumWithItemMixedTwinSse_CImpl(
      {required this.cField, final String? $type})
      : $type = $type ?? 'c',
        super._();

  factory _$EnumWithItemMixedTwinSse_CImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinSse_CImplFromJson(json);

  @override
  final String cField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSse.c(cField: $cField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSse_CImpl &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, cField);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinSse_CImplCopyWith<_$EnumWithItemMixedTwinSse_CImpl>
      get copyWith => __$$EnumWithItemMixedTwinSse_CImplCopyWithImpl<
          _$EnumWithItemMixedTwinSse_CImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return c(cField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return c?.call(cField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(cField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSse_C value) c,
  }) {
    return c(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSse_C value)? c,
  }) {
    return c?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSse_C value)? c,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinSse_CImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinSse_C extends EnumWithItemMixedTwinSse {
  const factory EnumWithItemMixedTwinSse_C({required final String cField}) =
      _$EnumWithItemMixedTwinSse_CImpl;
  const EnumWithItemMixedTwinSse_C._() : super._();

  factory EnumWithItemMixedTwinSse_C.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinSse_CImpl.fromJson;

  String get cField;
  @JsonKey(ignore: true)
  _$$EnumWithItemMixedTwinSse_CImplCopyWith<_$EnumWithItemMixedTwinSse_CImpl>
      get copyWith => throw _privateConstructorUsedError;
}

EnumWithItemStructTwinSse _$EnumWithItemStructTwinSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemStructTwinSse_A.fromJson(json);
    case 'b':
      return EnumWithItemStructTwinSse_B.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemStructTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemStructTwinSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemStructTwinSseCopyWith<$Res> {
  factory $EnumWithItemStructTwinSseCopyWith(EnumWithItemStructTwinSse value,
          $Res Function(EnumWithItemStructTwinSse) then) =
      _$EnumWithItemStructTwinSseCopyWithImpl<$Res, EnumWithItemStructTwinSse>;
}

/// @nodoc
class _$EnumWithItemStructTwinSseCopyWithImpl<$Res,
        $Val extends EnumWithItemStructTwinSse>
    implements $EnumWithItemStructTwinSseCopyWith<$Res> {
  _$EnumWithItemStructTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinSse_AImplCopyWith(
          _$EnumWithItemStructTwinSse_AImpl value,
          $Res Function(_$EnumWithItemStructTwinSse_AImpl) then) =
      __$$EnumWithItemStructTwinSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class __$$EnumWithItemStructTwinSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinSse_AImpl>
    implements _$$EnumWithItemStructTwinSse_AImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinSse_AImplCopyWithImpl(
      _$EnumWithItemStructTwinSse_AImpl _value,
      $Res Function(_$EnumWithItemStructTwinSse_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? aField = null,
  }) {
    return _then(_$EnumWithItemStructTwinSse_AImpl(
      aField: null == aField
          ? _value.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemStructTwinSse_AImpl extends EnumWithItemStructTwinSse_A {
  const _$EnumWithItemStructTwinSse_AImpl(
      {required this.aField, final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemStructTwinSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemStructTwinSse_AImplFromJson(json);

  @override
  final Uint8List aField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSse.a(aField: $aField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinSse_AImpl &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinSse_AImplCopyWith<_$EnumWithItemStructTwinSse_AImpl>
      get copyWith => __$$EnumWithItemStructTwinSse_AImplCopyWithImpl<
          _$EnumWithItemStructTwinSse_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) {
    return a(aField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) {
    return a?.call(aField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(aField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemStructTwinSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemStructTwinSse_A extends EnumWithItemStructTwinSse {
  const factory EnumWithItemStructTwinSse_A({required final Uint8List aField}) =
      _$EnumWithItemStructTwinSse_AImpl;
  const EnumWithItemStructTwinSse_A._() : super._();

  factory EnumWithItemStructTwinSse_A.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemStructTwinSse_AImpl.fromJson;

  Uint8List get aField;
  @JsonKey(ignore: true)
  _$$EnumWithItemStructTwinSse_AImplCopyWith<_$EnumWithItemStructTwinSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinSse_BImplCopyWith(
          _$EnumWithItemStructTwinSse_BImpl value,
          $Res Function(_$EnumWithItemStructTwinSse_BImpl) then) =
      __$$EnumWithItemStructTwinSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class __$$EnumWithItemStructTwinSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinSse_BImpl>
    implements _$$EnumWithItemStructTwinSse_BImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinSse_BImplCopyWithImpl(
      _$EnumWithItemStructTwinSse_BImpl _value,
      $Res Function(_$EnumWithItemStructTwinSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bField = null,
  }) {
    return _then(_$EnumWithItemStructTwinSse_BImpl(
      bField: null == bField
          ? _value.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemStructTwinSse_BImpl extends EnumWithItemStructTwinSse_B {
  const _$EnumWithItemStructTwinSse_BImpl(
      {required this.bField, final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemStructTwinSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemStructTwinSse_BImplFromJson(json);

  @override
  final Int32List bField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSse.b(bField: $bField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinSse_BImpl &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinSse_BImplCopyWith<_$EnumWithItemStructTwinSse_BImpl>
      get copyWith => __$$EnumWithItemStructTwinSse_BImplCopyWithImpl<
          _$EnumWithItemStructTwinSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) {
    return b(bField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) {
    return b?.call(bField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(bField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemStructTwinSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemStructTwinSse_B extends EnumWithItemStructTwinSse {
  const factory EnumWithItemStructTwinSse_B({required final Int32List bField}) =
      _$EnumWithItemStructTwinSse_BImpl;
  const EnumWithItemStructTwinSse_B._() : super._();

  factory EnumWithItemStructTwinSse_B.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemStructTwinSse_BImpl.fromJson;

  Int32List get bField;
  @JsonKey(ignore: true)
  _$$EnumWithItemStructTwinSse_BImplCopyWith<_$EnumWithItemStructTwinSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

EnumWithItemTupleTwinSse _$EnumWithItemTupleTwinSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemTupleTwinSse_A.fromJson(json);
    case 'b':
      return EnumWithItemTupleTwinSse_B.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemTupleTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemTupleTwinSse {
  List<int> get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(Int32List field0) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(Int32List field0)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(Int32List field0)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemTupleTwinSseCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSseCopyWith(EnumWithItemTupleTwinSse value,
          $Res Function(EnumWithItemTupleTwinSse) then) =
      _$EnumWithItemTupleTwinSseCopyWithImpl<$Res, EnumWithItemTupleTwinSse>;
}

/// @nodoc
class _$EnumWithItemTupleTwinSseCopyWithImpl<$Res,
        $Val extends EnumWithItemTupleTwinSse>
    implements $EnumWithItemTupleTwinSseCopyWith<$Res> {
  _$EnumWithItemTupleTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinSse_AImplCopyWith(
          _$EnumWithItemTupleTwinSse_AImpl value,
          $Res Function(_$EnumWithItemTupleTwinSse_AImpl) then) =
      __$$EnumWithItemTupleTwinSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinSse_AImpl>
    implements _$$EnumWithItemTupleTwinSse_AImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinSse_AImplCopyWithImpl(
      _$EnumWithItemTupleTwinSse_AImpl _value,
      $Res Function(_$EnumWithItemTupleTwinSse_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinSse_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemTupleTwinSse_AImpl extends EnumWithItemTupleTwinSse_A {
  const _$EnumWithItemTupleTwinSse_AImpl(this.field0, {final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemTupleTwinSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemTupleTwinSse_AImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSse.a(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinSse_AImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinSse_AImplCopyWith<_$EnumWithItemTupleTwinSse_AImpl>
      get copyWith => __$$EnumWithItemTupleTwinSse_AImplCopyWithImpl<
          _$EnumWithItemTupleTwinSse_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(Int32List field0) b,
  }) {
    return a(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(Int32List field0)? b,
  }) {
    return a?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(Int32List field0)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemTupleTwinSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemTupleTwinSse_A extends EnumWithItemTupleTwinSse {
  const factory EnumWithItemTupleTwinSse_A(final Uint8List field0) =
      _$EnumWithItemTupleTwinSse_AImpl;
  const EnumWithItemTupleTwinSse_A._() : super._();

  factory EnumWithItemTupleTwinSse_A.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemTupleTwinSse_AImpl.fromJson;

  @override
  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemTupleTwinSse_AImplCopyWith<_$EnumWithItemTupleTwinSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinSse_BImplCopyWith(
          _$EnumWithItemTupleTwinSse_BImpl value,
          $Res Function(_$EnumWithItemTupleTwinSse_BImpl) then) =
      __$$EnumWithItemTupleTwinSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Int32List field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinSse_BImpl>
    implements _$$EnumWithItemTupleTwinSse_BImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinSse_BImplCopyWithImpl(
      _$EnumWithItemTupleTwinSse_BImpl _value,
      $Res Function(_$EnumWithItemTupleTwinSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemTupleTwinSse_BImpl extends EnumWithItemTupleTwinSse_B {
  const _$EnumWithItemTupleTwinSse_BImpl(this.field0, {final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemTupleTwinSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemTupleTwinSse_BImplFromJson(json);

  @override
  final Int32List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinSse_BImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinSse_BImplCopyWith<_$EnumWithItemTupleTwinSse_BImpl>
      get copyWith => __$$EnumWithItemTupleTwinSse_BImplCopyWithImpl<
          _$EnumWithItemTupleTwinSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(Int32List field0) b,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(Int32List field0)? b,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(Int32List field0)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemTupleTwinSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemTupleTwinSse_B extends EnumWithItemTupleTwinSse {
  const factory EnumWithItemTupleTwinSse_B(final Int32List field0) =
      _$EnumWithItemTupleTwinSse_BImpl;
  const EnumWithItemTupleTwinSse_B._() : super._();

  factory EnumWithItemTupleTwinSse_B.fromJson(Map<String, dynamic> json) =
      _$EnumWithItemTupleTwinSse_BImpl.fromJson;

  @override
  Int32List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemTupleTwinSse_BImplCopyWith<_$EnumWithItemTupleTwinSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

KitchenSinkTwinSse _$KitchenSinkTwinSseFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'empty':
      return KitchenSinkTwinSse_Empty.fromJson(json);
    case 'primitives':
      return KitchenSinkTwinSse_Primitives.fromJson(json);
    case 'nested':
      return KitchenSinkTwinSse_Nested.fromJson(json);
    case 'optional':
      return KitchenSinkTwinSse_Optional.fromJson(json);
    case 'buffer':
      return KitchenSinkTwinSse_Buffer.fromJson(json);
    case 'enums':
      return KitchenSinkTwinSse_Enums.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'KitchenSinkTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$KitchenSinkTwinSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkTwinSseCopyWith<$Res> {
  factory $KitchenSinkTwinSseCopyWith(
          KitchenSinkTwinSse value, $Res Function(KitchenSinkTwinSse) then) =
      _$KitchenSinkTwinSseCopyWithImpl<$Res, KitchenSinkTwinSse>;
}

/// @nodoc
class _$KitchenSinkTwinSseCopyWithImpl<$Res, $Val extends KitchenSinkTwinSse>
    implements $KitchenSinkTwinSseCopyWith<$Res> {
  _$KitchenSinkTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_EmptyImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_EmptyImplCopyWith(
          _$KitchenSinkTwinSse_EmptyImpl value,
          $Res Function(_$KitchenSinkTwinSse_EmptyImpl) then) =
      __$$KitchenSinkTwinSse_EmptyImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSinkTwinSse_EmptyImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_EmptyImpl>
    implements _$$KitchenSinkTwinSse_EmptyImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_EmptyImplCopyWithImpl(
      _$KitchenSinkTwinSse_EmptyImpl _value,
      $Res Function(_$KitchenSinkTwinSse_EmptyImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_EmptyImpl extends KitchenSinkTwinSse_Empty {
  const _$KitchenSinkTwinSse_EmptyImpl({final String? $type})
      : $type = $type ?? 'empty',
        super._();

  factory _$KitchenSinkTwinSse_EmptyImpl.fromJson(Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_EmptyImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.empty()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_EmptyImpl);
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_EmptyImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Empty extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Empty() = _$KitchenSinkTwinSse_EmptyImpl;
  const KitchenSinkTwinSse_Empty._() : super._();

  factory KitchenSinkTwinSse_Empty.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_EmptyImpl.fromJson;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_PrimitivesImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_PrimitivesImplCopyWith(
          _$KitchenSinkTwinSse_PrimitivesImpl value,
          $Res Function(_$KitchenSinkTwinSse_PrimitivesImpl) then) =
      __$$KitchenSinkTwinSse_PrimitivesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSinkTwinSse_PrimitivesImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_PrimitivesImpl>
    implements _$$KitchenSinkTwinSse_PrimitivesImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_PrimitivesImplCopyWithImpl(
      _$KitchenSinkTwinSse_PrimitivesImpl _value,
      $Res Function(_$KitchenSinkTwinSse_PrimitivesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(_$KitchenSinkTwinSse_PrimitivesImpl(
      int32: null == int32
          ? _value.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: null == float64
          ? _value.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: null == boolean
          ? _value.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_PrimitivesImpl
    extends KitchenSinkTwinSse_Primitives {
  const _$KitchenSinkTwinSse_PrimitivesImpl(
      {this.int32 = -1,
      required this.float64,
      required this.boolean,
      final String? $type})
      : $type = $type ?? 'primitives',
        super._();

  factory _$KitchenSinkTwinSse_PrimitivesImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_PrimitivesImplFromJson(json);

  /// Dart field comment
  @override
  @JsonKey()
  final int int32;
  @override
  final double float64;
  @override
  final bool boolean;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_PrimitivesImpl &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinSse_PrimitivesImpl>
      get copyWith => __$$KitchenSinkTwinSse_PrimitivesImplCopyWithImpl<
          _$KitchenSinkTwinSse_PrimitivesImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(int32, float64, boolean);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_PrimitivesImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Primitives extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Primitives(
      {final int int32,
      required final double float64,
      required final bool boolean}) = _$KitchenSinkTwinSse_PrimitivesImpl;
  const KitchenSinkTwinSse_Primitives._() : super._();

  factory KitchenSinkTwinSse_Primitives.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_PrimitivesImpl.fromJson;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinSse_PrimitivesImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_NestedImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_NestedImplCopyWith(
          _$KitchenSinkTwinSse_NestedImpl value,
          $Res Function(_$KitchenSinkTwinSse_NestedImpl) then) =
      __$$KitchenSinkTwinSse_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, KitchenSinkTwinSse field1});

  $KitchenSinkTwinSseCopyWith<$Res> get field1;
}

/// @nodoc
class __$$KitchenSinkTwinSse_NestedImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_NestedImpl>
    implements _$$KitchenSinkTwinSse_NestedImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_NestedImplCopyWithImpl(
      _$KitchenSinkTwinSse_NestedImpl _value,
      $Res Function(_$KitchenSinkTwinSse_NestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$KitchenSinkTwinSse_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSseCopyWith<$Res> get field1 {
    return $KitchenSinkTwinSseCopyWith<$Res>(_value.field1, (value) {
      return _then(_value.copyWith(field1: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_NestedImpl extends KitchenSinkTwinSse_Nested {
  const _$KitchenSinkTwinSse_NestedImpl(this.field0,
      [this.field1 = const KitchenSinkTwinSse.empty(), final String? $type])
      : $type = $type ?? 'nested',
        super._();

  factory _$KitchenSinkTwinSse_NestedImpl.fromJson(Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_NestedImplFromJson(json);

  @override
  final int field0;
  @override
  @JsonKey()
  final KitchenSinkTwinSse field1;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSse_NestedImplCopyWith<_$KitchenSinkTwinSse_NestedImpl>
      get copyWith => __$$KitchenSinkTwinSse_NestedImplCopyWithImpl<
          _$KitchenSinkTwinSse_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_NestedImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Nested extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Nested(final int field0,
      [final KitchenSinkTwinSse field1]) = _$KitchenSinkTwinSse_NestedImpl;
  const KitchenSinkTwinSse_Nested._() : super._();

  factory KitchenSinkTwinSse_Nested.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_NestedImpl.fromJson;

  int get field0;
  KitchenSinkTwinSse get field1;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinSse_NestedImplCopyWith<_$KitchenSinkTwinSse_NestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_OptionalImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_OptionalImplCopyWith(
          _$KitchenSinkTwinSse_OptionalImpl value,
          $Res Function(_$KitchenSinkTwinSse_OptionalImpl) then) =
      __$$KitchenSinkTwinSse_OptionalImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSinkTwinSse_OptionalImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_OptionalImpl>
    implements _$$KitchenSinkTwinSse_OptionalImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_OptionalImplCopyWithImpl(
      _$KitchenSinkTwinSse_OptionalImpl _value,
      $Res Function(_$KitchenSinkTwinSse_OptionalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSinkTwinSse_OptionalImpl(
      freezed == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      freezed == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_OptionalImpl extends KitchenSinkTwinSse_Optional {
  const _$KitchenSinkTwinSse_OptionalImpl(
      [this.field0 = -1, this.field1, final String? $type])
      : $type = $type ?? 'optional',
        super._();

  factory _$KitchenSinkTwinSse_OptionalImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_OptionalImplFromJson(json);

  /// Comment on anonymous field
  @override
  @JsonKey()
  final int? field0;
  @override
  final int? field1;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.optional(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_OptionalImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSse_OptionalImplCopyWith<_$KitchenSinkTwinSse_OptionalImpl>
      get copyWith => __$$KitchenSinkTwinSse_OptionalImplCopyWithImpl<
          _$KitchenSinkTwinSse_OptionalImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_OptionalImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Optional extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Optional(
      [final int? field0,
      final int? field1]) = _$KitchenSinkTwinSse_OptionalImpl;
  const KitchenSinkTwinSse_Optional._() : super._();

  factory KitchenSinkTwinSse_Optional.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_OptionalImpl.fromJson;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinSse_OptionalImplCopyWith<_$KitchenSinkTwinSse_OptionalImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_BufferImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_BufferImplCopyWith(
          _$KitchenSinkTwinSse_BufferImpl value,
          $Res Function(_$KitchenSinkTwinSse_BufferImpl) then) =
      __$$KitchenSinkTwinSse_BufferImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSinkTwinSse_BufferImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_BufferImpl>
    implements _$$KitchenSinkTwinSse_BufferImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_BufferImplCopyWithImpl(
      _$KitchenSinkTwinSse_BufferImpl _value,
      $Res Function(_$KitchenSinkTwinSse_BufferImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinSse_BufferImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_BufferImpl extends KitchenSinkTwinSse_Buffer {
  const _$KitchenSinkTwinSse_BufferImpl(this.field0, {final String? $type})
      : $type = $type ?? 'buffer',
        super._();

  factory _$KitchenSinkTwinSse_BufferImpl.fromJson(Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_BufferImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.buffer(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_BufferImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSse_BufferImplCopyWith<_$KitchenSinkTwinSse_BufferImpl>
      get copyWith => __$$KitchenSinkTwinSse_BufferImplCopyWithImpl<
          _$KitchenSinkTwinSse_BufferImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_BufferImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Buffer extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Buffer(final Uint8List field0) =
      _$KitchenSinkTwinSse_BufferImpl;
  const KitchenSinkTwinSse_Buffer._() : super._();

  factory KitchenSinkTwinSse_Buffer.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_BufferImpl.fromJson;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinSse_BufferImplCopyWith<_$KitchenSinkTwinSse_BufferImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSse_EnumsImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSse_EnumsImplCopyWith(
          _$KitchenSinkTwinSse_EnumsImpl value,
          $Res Function(_$KitchenSinkTwinSse_EnumsImpl) then) =
      __$$KitchenSinkTwinSse_EnumsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({WeekdaysTwinSse field0});
}

/// @nodoc
class __$$KitchenSinkTwinSse_EnumsImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSse_EnumsImpl>
    implements _$$KitchenSinkTwinSse_EnumsImplCopyWith<$Res> {
  __$$KitchenSinkTwinSse_EnumsImplCopyWithImpl(
      _$KitchenSinkTwinSse_EnumsImpl _value,
      $Res Function(_$KitchenSinkTwinSse_EnumsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinSse_EnumsImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinSse,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinSse_EnumsImpl extends KitchenSinkTwinSse_Enums {
  const _$KitchenSinkTwinSse_EnumsImpl(
      [this.field0 = WeekdaysTwinSse.sunday, final String? $type])
      : $type = $type ?? 'enums',
        super._();

  factory _$KitchenSinkTwinSse_EnumsImpl.fromJson(Map<String, dynamic> json) =>
      _$$KitchenSinkTwinSse_EnumsImplFromJson(json);

  @override
  @JsonKey()
  final WeekdaysTwinSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinSse.enums(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSse_EnumsImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSse_EnumsImplCopyWith<_$KitchenSinkTwinSse_EnumsImpl>
      get copyWith => __$$KitchenSinkTwinSse_EnumsImplCopyWithImpl<
          _$KitchenSinkTwinSse_EnumsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSse field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSse field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSse field0)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSse_Primitives value) primitives,
    required TResult Function(KitchenSinkTwinSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSse_Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSse_Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinSse_EnumsImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinSse_Enums extends KitchenSinkTwinSse {
  const factory KitchenSinkTwinSse_Enums([final WeekdaysTwinSse field0]) =
      _$KitchenSinkTwinSse_EnumsImpl;
  const KitchenSinkTwinSse_Enums._() : super._();

  factory KitchenSinkTwinSse_Enums.fromJson(Map<String, dynamic> json) =
      _$KitchenSinkTwinSse_EnumsImpl.fromJson;

  WeekdaysTwinSse get field0;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinSse_EnumsImplCopyWith<_$KitchenSinkTwinSse_EnumsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

MeasureTwinSse _$MeasureTwinSseFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'speed':
      return MeasureTwinSse_Speed.fromJson(json);
    case 'distance':
      return MeasureTwinSse_Distance.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'MeasureTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$MeasureTwinSse {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSse field0) speed,
    required TResult Function(DistanceTwinSse field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse field0)? speed,
    TResult? Function(DistanceTwinSse field0)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSse field0)? speed,
    TResult Function(DistanceTwinSse field0)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSse_Speed value) speed,
    required TResult Function(MeasureTwinSse_Distance value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSse_Speed value)? speed,
    TResult? Function(MeasureTwinSse_Distance value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSse_Speed value)? speed,
    TResult Function(MeasureTwinSse_Distance value)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MeasureTwinSseCopyWith<$Res> {
  factory $MeasureTwinSseCopyWith(
          MeasureTwinSse value, $Res Function(MeasureTwinSse) then) =
      _$MeasureTwinSseCopyWithImpl<$Res, MeasureTwinSse>;
}

/// @nodoc
class _$MeasureTwinSseCopyWithImpl<$Res, $Val extends MeasureTwinSse>
    implements $MeasureTwinSseCopyWith<$Res> {
  _$MeasureTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$MeasureTwinSse_SpeedImplCopyWith<$Res> {
  factory _$$MeasureTwinSse_SpeedImplCopyWith(_$MeasureTwinSse_SpeedImpl value,
          $Res Function(_$MeasureTwinSse_SpeedImpl) then) =
      __$$MeasureTwinSse_SpeedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({SpeedTwinSse field0});

  $SpeedTwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinSse_SpeedImplCopyWithImpl<$Res>
    extends _$MeasureTwinSseCopyWithImpl<$Res, _$MeasureTwinSse_SpeedImpl>
    implements _$$MeasureTwinSse_SpeedImplCopyWith<$Res> {
  __$$MeasureTwinSse_SpeedImplCopyWithImpl(_$MeasureTwinSse_SpeedImpl _value,
      $Res Function(_$MeasureTwinSse_SpeedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinSse_SpeedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinSseCopyWith<$Res> get field0 {
    return $SpeedTwinSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$MeasureTwinSse_SpeedImpl extends MeasureTwinSse_Speed {
  const _$MeasureTwinSse_SpeedImpl(this.field0, {final String? $type})
      : $type = $type ?? 'speed',
        super._();

  factory _$MeasureTwinSse_SpeedImpl.fromJson(Map<String, dynamic> json) =>
      _$$MeasureTwinSse_SpeedImplFromJson(json);

  @override
  final SpeedTwinSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MeasureTwinSse.speed(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinSse_SpeedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinSse_SpeedImplCopyWith<_$MeasureTwinSse_SpeedImpl>
      get copyWith =>
          __$$MeasureTwinSse_SpeedImplCopyWithImpl<_$MeasureTwinSse_SpeedImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSse field0) speed,
    required TResult Function(DistanceTwinSse field0) distance,
  }) {
    return speed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse field0)? speed,
    TResult? Function(DistanceTwinSse field0)? distance,
  }) {
    return speed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSse field0)? speed,
    TResult Function(DistanceTwinSse field0)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSse_Speed value) speed,
    required TResult Function(MeasureTwinSse_Distance value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSse_Speed value)? speed,
    TResult? Function(MeasureTwinSse_Distance value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSse_Speed value)? speed,
    TResult Function(MeasureTwinSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MeasureTwinSse_SpeedImplToJson(
      this,
    );
  }
}

abstract class MeasureTwinSse_Speed extends MeasureTwinSse {
  const factory MeasureTwinSse_Speed(final SpeedTwinSse field0) =
      _$MeasureTwinSse_SpeedImpl;
  const MeasureTwinSse_Speed._() : super._();

  factory MeasureTwinSse_Speed.fromJson(Map<String, dynamic> json) =
      _$MeasureTwinSse_SpeedImpl.fromJson;

  @override
  SpeedTwinSse get field0;
  @JsonKey(ignore: true)
  _$$MeasureTwinSse_SpeedImplCopyWith<_$MeasureTwinSse_SpeedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MeasureTwinSse_DistanceImplCopyWith<$Res> {
  factory _$$MeasureTwinSse_DistanceImplCopyWith(
          _$MeasureTwinSse_DistanceImpl value,
          $Res Function(_$MeasureTwinSse_DistanceImpl) then) =
      __$$MeasureTwinSse_DistanceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DistanceTwinSse field0});

  $DistanceTwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinSse_DistanceImplCopyWithImpl<$Res>
    extends _$MeasureTwinSseCopyWithImpl<$Res, _$MeasureTwinSse_DistanceImpl>
    implements _$$MeasureTwinSse_DistanceImplCopyWith<$Res> {
  __$$MeasureTwinSse_DistanceImplCopyWithImpl(
      _$MeasureTwinSse_DistanceImpl _value,
      $Res Function(_$MeasureTwinSse_DistanceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinSse_DistanceImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinSseCopyWith<$Res> get field0 {
    return $DistanceTwinSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$MeasureTwinSse_DistanceImpl extends MeasureTwinSse_Distance {
  const _$MeasureTwinSse_DistanceImpl(this.field0, {final String? $type})
      : $type = $type ?? 'distance',
        super._();

  factory _$MeasureTwinSse_DistanceImpl.fromJson(Map<String, dynamic> json) =>
      _$$MeasureTwinSse_DistanceImplFromJson(json);

  @override
  final DistanceTwinSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MeasureTwinSse.distance(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinSse_DistanceImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinSse_DistanceImplCopyWith<_$MeasureTwinSse_DistanceImpl>
      get copyWith => __$$MeasureTwinSse_DistanceImplCopyWithImpl<
          _$MeasureTwinSse_DistanceImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSse field0) speed,
    required TResult Function(DistanceTwinSse field0) distance,
  }) {
    return distance(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse field0)? speed,
    TResult? Function(DistanceTwinSse field0)? distance,
  }) {
    return distance?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSse field0)? speed,
    TResult Function(DistanceTwinSse field0)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSse_Speed value) speed,
    required TResult Function(MeasureTwinSse_Distance value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSse_Speed value)? speed,
    TResult? Function(MeasureTwinSse_Distance value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSse_Speed value)? speed,
    TResult Function(MeasureTwinSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MeasureTwinSse_DistanceImplToJson(
      this,
    );
  }
}

abstract class MeasureTwinSse_Distance extends MeasureTwinSse {
  const factory MeasureTwinSse_Distance(final DistanceTwinSse field0) =
      _$MeasureTwinSse_DistanceImpl;
  const MeasureTwinSse_Distance._() : super._();

  factory MeasureTwinSse_Distance.fromJson(Map<String, dynamic> json) =
      _$MeasureTwinSse_DistanceImpl.fromJson;

  @override
  DistanceTwinSse get field0;
  @JsonKey(ignore: true)
  _$$MeasureTwinSse_DistanceImplCopyWith<_$MeasureTwinSse_DistanceImpl>
      get copyWith => throw _privateConstructorUsedError;
}

SpeedTwinSse _$SpeedTwinSseFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'unknown':
      return SpeedTwinSse_Unknown.fromJson(json);
    case 'gps':
      return SpeedTwinSse_GPS.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'SpeedTwinSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$SpeedTwinSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSse_GPS value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSse_GPS value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSse_GPS value)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpeedTwinSseCopyWith<$Res> {
  factory $SpeedTwinSseCopyWith(
          SpeedTwinSse value, $Res Function(SpeedTwinSse) then) =
      _$SpeedTwinSseCopyWithImpl<$Res, SpeedTwinSse>;
}

/// @nodoc
class _$SpeedTwinSseCopyWithImpl<$Res, $Val extends SpeedTwinSse>
    implements $SpeedTwinSseCopyWith<$Res> {
  _$SpeedTwinSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$SpeedTwinSse_UnknownImplCopyWith<$Res> {
  factory _$$SpeedTwinSse_UnknownImplCopyWith(_$SpeedTwinSse_UnknownImpl value,
          $Res Function(_$SpeedTwinSse_UnknownImpl) then) =
      __$$SpeedTwinSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SpeedTwinSse_UnknownImplCopyWithImpl<$Res>
    extends _$SpeedTwinSseCopyWithImpl<$Res, _$SpeedTwinSse_UnknownImpl>
    implements _$$SpeedTwinSse_UnknownImplCopyWith<$Res> {
  __$$SpeedTwinSse_UnknownImplCopyWithImpl(_$SpeedTwinSse_UnknownImpl _value,
      $Res Function(_$SpeedTwinSse_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$SpeedTwinSse_UnknownImpl extends SpeedTwinSse_Unknown {
  const _$SpeedTwinSse_UnknownImpl({final String? $type})
      : $type = $type ?? 'unknown',
        super._();

  factory _$SpeedTwinSse_UnknownImpl.fromJson(Map<String, dynamic> json) =>
      _$$SpeedTwinSse_UnknownImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'SpeedTwinSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinSse_UnknownImpl);
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSse_GPS value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSse_GPS value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$SpeedTwinSse_UnknownImplToJson(
      this,
    );
  }
}

abstract class SpeedTwinSse_Unknown extends SpeedTwinSse {
  const factory SpeedTwinSse_Unknown() = _$SpeedTwinSse_UnknownImpl;
  const SpeedTwinSse_Unknown._() : super._();

  factory SpeedTwinSse_Unknown.fromJson(Map<String, dynamic> json) =
      _$SpeedTwinSse_UnknownImpl.fromJson;
}

/// @nodoc
abstract class _$$SpeedTwinSse_GPSImplCopyWith<$Res> {
  factory _$$SpeedTwinSse_GPSImplCopyWith(_$SpeedTwinSse_GPSImpl value,
          $Res Function(_$SpeedTwinSse_GPSImpl) then) =
      __$$SpeedTwinSse_GPSImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$SpeedTwinSse_GPSImplCopyWithImpl<$Res>
    extends _$SpeedTwinSseCopyWithImpl<$Res, _$SpeedTwinSse_GPSImpl>
    implements _$$SpeedTwinSse_GPSImplCopyWith<$Res> {
  __$$SpeedTwinSse_GPSImplCopyWithImpl(_$SpeedTwinSse_GPSImpl _value,
      $Res Function(_$SpeedTwinSse_GPSImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$SpeedTwinSse_GPSImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$SpeedTwinSse_GPSImpl extends SpeedTwinSse_GPS {
  const _$SpeedTwinSse_GPSImpl(this.field0, {final String? $type})
      : $type = $type ?? 'gps',
        super._();

  factory _$SpeedTwinSse_GPSImpl.fromJson(Map<String, dynamic> json) =>
      _$$SpeedTwinSse_GPSImplFromJson(json);

  @override
  final double field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'SpeedTwinSse.gps(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinSse_GPSImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SpeedTwinSse_GPSImplCopyWith<_$SpeedTwinSse_GPSImpl> get copyWith =>
      __$$SpeedTwinSse_GPSImplCopyWithImpl<_$SpeedTwinSse_GPSImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return gps(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return gps?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSse_GPS value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSse_GPS value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$SpeedTwinSse_GPSImplToJson(
      this,
    );
  }
}

abstract class SpeedTwinSse_GPS extends SpeedTwinSse {
  const factory SpeedTwinSse_GPS(final double field0) = _$SpeedTwinSse_GPSImpl;
  const SpeedTwinSse_GPS._() : super._();

  factory SpeedTwinSse_GPS.fromJson(Map<String, dynamic> json) =
      _$SpeedTwinSse_GPSImpl.fromJson;

  double get field0;
  @JsonKey(ignore: true)
  _$$SpeedTwinSse_GPSImplCopyWith<_$SpeedTwinSse_GPSImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
