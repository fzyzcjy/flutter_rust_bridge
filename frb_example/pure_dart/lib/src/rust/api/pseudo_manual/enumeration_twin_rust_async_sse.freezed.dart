// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

DistanceTwinRustAsyncSse _$DistanceTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'unknown':
      return DistanceTwinRustAsyncSse_Unknown.fromJson(json);
    case 'map':
      return DistanceTwinRustAsyncSse_Map.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'DistanceTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$DistanceTwinRustAsyncSse {
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
    required TResult Function(DistanceTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinRustAsyncSse_Map value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinRustAsyncSse_Map value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinRustAsyncSse_Map value)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DistanceTwinRustAsyncSseCopyWith<$Res> {
  factory $DistanceTwinRustAsyncSseCopyWith(DistanceTwinRustAsyncSse value,
          $Res Function(DistanceTwinRustAsyncSse) then) =
      _$DistanceTwinRustAsyncSseCopyWithImpl<$Res, DistanceTwinRustAsyncSse>;
}

/// @nodoc
class _$DistanceTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends DistanceTwinRustAsyncSse>
    implements $DistanceTwinRustAsyncSseCopyWith<$Res> {
  _$DistanceTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$DistanceTwinRustAsyncSse_UnknownImplCopyWith<$Res> {
  factory _$$DistanceTwinRustAsyncSse_UnknownImplCopyWith(
          _$DistanceTwinRustAsyncSse_UnknownImpl value,
          $Res Function(_$DistanceTwinRustAsyncSse_UnknownImpl) then) =
      __$$DistanceTwinRustAsyncSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DistanceTwinRustAsyncSse_UnknownImplCopyWithImpl<$Res>
    extends _$DistanceTwinRustAsyncSseCopyWithImpl<$Res,
        _$DistanceTwinRustAsyncSse_UnknownImpl>
    implements _$$DistanceTwinRustAsyncSse_UnknownImplCopyWith<$Res> {
  __$$DistanceTwinRustAsyncSse_UnknownImplCopyWithImpl(
      _$DistanceTwinRustAsyncSse_UnknownImpl _value,
      $Res Function(_$DistanceTwinRustAsyncSse_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$DistanceTwinRustAsyncSse_UnknownImpl
    extends DistanceTwinRustAsyncSse_Unknown {
  const _$DistanceTwinRustAsyncSse_UnknownImpl({final String? $type})
      : $type = $type ?? 'unknown',
        super._();

  factory _$DistanceTwinRustAsyncSse_UnknownImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$DistanceTwinRustAsyncSse_UnknownImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'DistanceTwinRustAsyncSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinRustAsyncSse_UnknownImpl);
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
    required TResult Function(DistanceTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinRustAsyncSse_Map value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinRustAsyncSse_Map value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinRustAsyncSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DistanceTwinRustAsyncSse_UnknownImplToJson(
      this,
    );
  }
}

abstract class DistanceTwinRustAsyncSse_Unknown
    extends DistanceTwinRustAsyncSse {
  const factory DistanceTwinRustAsyncSse_Unknown() =
      _$DistanceTwinRustAsyncSse_UnknownImpl;
  const DistanceTwinRustAsyncSse_Unknown._() : super._();

  factory DistanceTwinRustAsyncSse_Unknown.fromJson(Map<String, dynamic> json) =
      _$DistanceTwinRustAsyncSse_UnknownImpl.fromJson;
}

/// @nodoc
abstract class _$$DistanceTwinRustAsyncSse_MapImplCopyWith<$Res> {
  factory _$$DistanceTwinRustAsyncSse_MapImplCopyWith(
          _$DistanceTwinRustAsyncSse_MapImpl value,
          $Res Function(_$DistanceTwinRustAsyncSse_MapImpl) then) =
      __$$DistanceTwinRustAsyncSse_MapImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$DistanceTwinRustAsyncSse_MapImplCopyWithImpl<$Res>
    extends _$DistanceTwinRustAsyncSseCopyWithImpl<$Res,
        _$DistanceTwinRustAsyncSse_MapImpl>
    implements _$$DistanceTwinRustAsyncSse_MapImplCopyWith<$Res> {
  __$$DistanceTwinRustAsyncSse_MapImplCopyWithImpl(
      _$DistanceTwinRustAsyncSse_MapImpl _value,
      $Res Function(_$DistanceTwinRustAsyncSse_MapImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DistanceTwinRustAsyncSse_MapImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DistanceTwinRustAsyncSse_MapImpl extends DistanceTwinRustAsyncSse_Map {
  const _$DistanceTwinRustAsyncSse_MapImpl(this.field0, {final String? $type})
      : $type = $type ?? 'map',
        super._();

  factory _$DistanceTwinRustAsyncSse_MapImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$DistanceTwinRustAsyncSse_MapImplFromJson(json);

  @override
  final double field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'DistanceTwinRustAsyncSse.map(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinRustAsyncSse_MapImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DistanceTwinRustAsyncSse_MapImplCopyWith<
          _$DistanceTwinRustAsyncSse_MapImpl>
      get copyWith => __$$DistanceTwinRustAsyncSse_MapImplCopyWithImpl<
          _$DistanceTwinRustAsyncSse_MapImpl>(this, _$identity);

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
    required TResult Function(DistanceTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinRustAsyncSse_Map value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinRustAsyncSse_Map value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinRustAsyncSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$DistanceTwinRustAsyncSse_MapImplToJson(
      this,
    );
  }
}

abstract class DistanceTwinRustAsyncSse_Map extends DistanceTwinRustAsyncSse {
  const factory DistanceTwinRustAsyncSse_Map(final double field0) =
      _$DistanceTwinRustAsyncSse_MapImpl;
  const DistanceTwinRustAsyncSse_Map._() : super._();

  factory DistanceTwinRustAsyncSse_Map.fromJson(Map<String, dynamic> json) =
      _$DistanceTwinRustAsyncSse_MapImpl.fromJson;

  double get field0;
  @JsonKey(ignore: true)
  _$$DistanceTwinRustAsyncSse_MapImplCopyWith<
          _$DistanceTwinRustAsyncSse_MapImpl>
      get copyWith => throw _privateConstructorUsedError;
}

EnumWithItemMixedTwinRustAsyncSse _$EnumWithItemMixedTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemMixedTwinRustAsyncSse_A.fromJson(json);
    case 'b':
      return EnumWithItemMixedTwinRustAsyncSse_B.fromJson(json);
    case 'c':
      return EnumWithItemMixedTwinRustAsyncSse_C.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemMixedTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemMixedTwinRustAsyncSse {
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
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value) c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemMixedTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumWithItemMixedTwinRustAsyncSseCopyWith(
          EnumWithItemMixedTwinRustAsyncSse value,
          $Res Function(EnumWithItemMixedTwinRustAsyncSse) then) =
      _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl<$Res,
          EnumWithItemMixedTwinRustAsyncSse>;
}

/// @nodoc
class _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemMixedTwinRustAsyncSse>
    implements $EnumWithItemMixedTwinRustAsyncSseCopyWith<$Res> {
  _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWith(
          _$EnumWithItemMixedTwinRustAsyncSse_AImpl value,
          $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_AImpl) then) =
      __$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinRustAsyncSse_AImpl>
    implements _$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinRustAsyncSse_AImplCopyWithImpl(
      _$EnumWithItemMixedTwinRustAsyncSse_AImpl _value,
      $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_AImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinRustAsyncSse_AImpl
    extends EnumWithItemMixedTwinRustAsyncSse_A {
  const _$EnumWithItemMixedTwinRustAsyncSse_AImpl({final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemMixedTwinRustAsyncSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinRustAsyncSse_AImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinRustAsyncSse.a()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinRustAsyncSse_AImpl);
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
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value) c,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinRustAsyncSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinRustAsyncSse_A
    extends EnumWithItemMixedTwinRustAsyncSse {
  const factory EnumWithItemMixedTwinRustAsyncSse_A() =
      _$EnumWithItemMixedTwinRustAsyncSse_AImpl;
  const EnumWithItemMixedTwinRustAsyncSse_A._() : super._();

  factory EnumWithItemMixedTwinRustAsyncSse_A.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinRustAsyncSse_AImpl.fromJson;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWith(
          _$EnumWithItemMixedTwinRustAsyncSse_BImpl value,
          $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_BImpl) then) =
      __$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinRustAsyncSse_BImpl>
    implements _$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWithImpl(
      _$EnumWithItemMixedTwinRustAsyncSse_BImpl _value,
      $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemMixedTwinRustAsyncSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinRustAsyncSse_BImpl
    extends EnumWithItemMixedTwinRustAsyncSse_B {
  const _$EnumWithItemMixedTwinRustAsyncSse_BImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemMixedTwinRustAsyncSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinRustAsyncSse_BImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinRustAsyncSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinRustAsyncSse_BImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemMixedTwinRustAsyncSse_BImpl>
      get copyWith => __$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWithImpl<
          _$EnumWithItemMixedTwinRustAsyncSse_BImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value) c,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinRustAsyncSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinRustAsyncSse_B
    extends EnumWithItemMixedTwinRustAsyncSse {
  const factory EnumWithItemMixedTwinRustAsyncSse_B(final Uint8List field0) =
      _$EnumWithItemMixedTwinRustAsyncSse_BImpl;
  const EnumWithItemMixedTwinRustAsyncSse_B._() : super._();

  factory EnumWithItemMixedTwinRustAsyncSse_B.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinRustAsyncSse_BImpl.fromJson;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemMixedTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemMixedTwinRustAsyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWith(
          _$EnumWithItemMixedTwinRustAsyncSse_CImpl value,
          $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_CImpl) then) =
      __$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class __$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinRustAsyncSse_CImpl>
    implements _$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWithImpl(
      _$EnumWithItemMixedTwinRustAsyncSse_CImpl _value,
      $Res Function(_$EnumWithItemMixedTwinRustAsyncSse_CImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? cField = null,
  }) {
    return _then(_$EnumWithItemMixedTwinRustAsyncSse_CImpl(
      cField: null == cField
          ? _value.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemMixedTwinRustAsyncSse_CImpl
    extends EnumWithItemMixedTwinRustAsyncSse_C {
  const _$EnumWithItemMixedTwinRustAsyncSse_CImpl(
      {required this.cField, final String? $type})
      : $type = $type ?? 'c',
        super._();

  factory _$EnumWithItemMixedTwinRustAsyncSse_CImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemMixedTwinRustAsyncSse_CImplFromJson(json);

  @override
  final String cField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinRustAsyncSse.c(cField: $cField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinRustAsyncSse_CImpl &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, cField);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWith<
          _$EnumWithItemMixedTwinRustAsyncSse_CImpl>
      get copyWith => __$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWithImpl<
          _$EnumWithItemMixedTwinRustAsyncSse_CImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value) c,
  }) {
    return c(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
  }) {
    return c?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinRustAsyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemMixedTwinRustAsyncSse_CImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemMixedTwinRustAsyncSse_C
    extends EnumWithItemMixedTwinRustAsyncSse {
  const factory EnumWithItemMixedTwinRustAsyncSse_C(
          {required final String cField}) =
      _$EnumWithItemMixedTwinRustAsyncSse_CImpl;
  const EnumWithItemMixedTwinRustAsyncSse_C._() : super._();

  factory EnumWithItemMixedTwinRustAsyncSse_C.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemMixedTwinRustAsyncSse_CImpl.fromJson;

  String get cField;
  @JsonKey(ignore: true)
  _$$EnumWithItemMixedTwinRustAsyncSse_CImplCopyWith<
          _$EnumWithItemMixedTwinRustAsyncSse_CImpl>
      get copyWith => throw _privateConstructorUsedError;
}

EnumWithItemStructTwinRustAsyncSse _$EnumWithItemStructTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemStructTwinRustAsyncSse_A.fromJson(json);
    case 'b':
      return EnumWithItemStructTwinRustAsyncSse_B.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemStructTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemStructTwinRustAsyncSse {
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
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemStructTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumWithItemStructTwinRustAsyncSseCopyWith(
          EnumWithItemStructTwinRustAsyncSse value,
          $Res Function(EnumWithItemStructTwinRustAsyncSse) then) =
      _$EnumWithItemStructTwinRustAsyncSseCopyWithImpl<$Res,
          EnumWithItemStructTwinRustAsyncSse>;
}

/// @nodoc
class _$EnumWithItemStructTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemStructTwinRustAsyncSse>
    implements $EnumWithItemStructTwinRustAsyncSseCopyWith<$Res> {
  _$EnumWithItemStructTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWith(
          _$EnumWithItemStructTwinRustAsyncSse_AImpl value,
          $Res Function(_$EnumWithItemStructTwinRustAsyncSse_AImpl) then) =
      __$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class __$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinRustAsyncSse_AImpl>
    implements _$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWithImpl(
      _$EnumWithItemStructTwinRustAsyncSse_AImpl _value,
      $Res Function(_$EnumWithItemStructTwinRustAsyncSse_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? aField = null,
  }) {
    return _then(_$EnumWithItemStructTwinRustAsyncSse_AImpl(
      aField: null == aField
          ? _value.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemStructTwinRustAsyncSse_AImpl
    extends EnumWithItemStructTwinRustAsyncSse_A {
  const _$EnumWithItemStructTwinRustAsyncSse_AImpl(
      {required this.aField, final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemStructTwinRustAsyncSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemStructTwinRustAsyncSse_AImplFromJson(json);

  @override
  final Uint8List aField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemStructTwinRustAsyncSse.a(aField: $aField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinRustAsyncSse_AImpl &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWith<
          _$EnumWithItemStructTwinRustAsyncSse_AImpl>
      get copyWith => __$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWithImpl<
          _$EnumWithItemStructTwinRustAsyncSse_AImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemStructTwinRustAsyncSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemStructTwinRustAsyncSse_A
    extends EnumWithItemStructTwinRustAsyncSse {
  const factory EnumWithItemStructTwinRustAsyncSse_A(
          {required final Uint8List aField}) =
      _$EnumWithItemStructTwinRustAsyncSse_AImpl;
  const EnumWithItemStructTwinRustAsyncSse_A._() : super._();

  factory EnumWithItemStructTwinRustAsyncSse_A.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemStructTwinRustAsyncSse_AImpl.fromJson;

  Uint8List get aField;
  @JsonKey(ignore: true)
  _$$EnumWithItemStructTwinRustAsyncSse_AImplCopyWith<
          _$EnumWithItemStructTwinRustAsyncSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWith(
          _$EnumWithItemStructTwinRustAsyncSse_BImpl value,
          $Res Function(_$EnumWithItemStructTwinRustAsyncSse_BImpl) then) =
      __$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class __$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinRustAsyncSse_BImpl>
    implements _$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWithImpl(
      _$EnumWithItemStructTwinRustAsyncSse_BImpl _value,
      $Res Function(_$EnumWithItemStructTwinRustAsyncSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bField = null,
  }) {
    return _then(_$EnumWithItemStructTwinRustAsyncSse_BImpl(
      bField: null == bField
          ? _value.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemStructTwinRustAsyncSse_BImpl
    extends EnumWithItemStructTwinRustAsyncSse_B {
  const _$EnumWithItemStructTwinRustAsyncSse_BImpl(
      {required this.bField, final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemStructTwinRustAsyncSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemStructTwinRustAsyncSse_BImplFromJson(json);

  @override
  final Int32List bField;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemStructTwinRustAsyncSse.b(bField: $bField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinRustAsyncSse_BImpl &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemStructTwinRustAsyncSse_BImpl>
      get copyWith => __$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWithImpl<
          _$EnumWithItemStructTwinRustAsyncSse_BImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinRustAsyncSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemStructTwinRustAsyncSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemStructTwinRustAsyncSse_B
    extends EnumWithItemStructTwinRustAsyncSse {
  const factory EnumWithItemStructTwinRustAsyncSse_B(
          {required final Int32List bField}) =
      _$EnumWithItemStructTwinRustAsyncSse_BImpl;
  const EnumWithItemStructTwinRustAsyncSse_B._() : super._();

  factory EnumWithItemStructTwinRustAsyncSse_B.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemStructTwinRustAsyncSse_BImpl.fromJson;

  Int32List get bField;
  @JsonKey(ignore: true)
  _$$EnumWithItemStructTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemStructTwinRustAsyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

EnumWithItemTupleTwinRustAsyncSse _$EnumWithItemTupleTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'a':
      return EnumWithItemTupleTwinRustAsyncSse_A.fromJson(json);
    case 'b':
      return EnumWithItemTupleTwinRustAsyncSse_B.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'EnumWithItemTupleTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumWithItemTupleTwinRustAsyncSse {
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
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemTupleTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumWithItemTupleTwinRustAsyncSseCopyWith(
          EnumWithItemTupleTwinRustAsyncSse value,
          $Res Function(EnumWithItemTupleTwinRustAsyncSse) then) =
      _$EnumWithItemTupleTwinRustAsyncSseCopyWithImpl<$Res,
          EnumWithItemTupleTwinRustAsyncSse>;
}

/// @nodoc
class _$EnumWithItemTupleTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemTupleTwinRustAsyncSse>
    implements $EnumWithItemTupleTwinRustAsyncSseCopyWith<$Res> {
  _$EnumWithItemTupleTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWith(
          _$EnumWithItemTupleTwinRustAsyncSse_AImpl value,
          $Res Function(_$EnumWithItemTupleTwinRustAsyncSse_AImpl) then) =
      __$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinRustAsyncSse_AImpl>
    implements _$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWithImpl(
      _$EnumWithItemTupleTwinRustAsyncSse_AImpl _value,
      $Res Function(_$EnumWithItemTupleTwinRustAsyncSse_AImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinRustAsyncSse_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemTupleTwinRustAsyncSse_AImpl
    extends EnumWithItemTupleTwinRustAsyncSse_A {
  const _$EnumWithItemTupleTwinRustAsyncSse_AImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'a',
        super._();

  factory _$EnumWithItemTupleTwinRustAsyncSse_AImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemTupleTwinRustAsyncSse_AImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinRustAsyncSse.a(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinRustAsyncSse_AImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWith<
          _$EnumWithItemTupleTwinRustAsyncSse_AImpl>
      get copyWith => __$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWithImpl<
          _$EnumWithItemTupleTwinRustAsyncSse_AImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemTupleTwinRustAsyncSse_AImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemTupleTwinRustAsyncSse_A
    extends EnumWithItemTupleTwinRustAsyncSse {
  const factory EnumWithItemTupleTwinRustAsyncSse_A(final Uint8List field0) =
      _$EnumWithItemTupleTwinRustAsyncSse_AImpl;
  const EnumWithItemTupleTwinRustAsyncSse_A._() : super._();

  factory EnumWithItemTupleTwinRustAsyncSse_A.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemTupleTwinRustAsyncSse_AImpl.fromJson;

  @override
  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemTupleTwinRustAsyncSse_AImplCopyWith<
          _$EnumWithItemTupleTwinRustAsyncSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWith(
          _$EnumWithItemTupleTwinRustAsyncSse_BImpl value,
          $Res Function(_$EnumWithItemTupleTwinRustAsyncSse_BImpl) then) =
      __$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Int32List field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinRustAsyncSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinRustAsyncSse_BImpl>
    implements _$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWithImpl(
      _$EnumWithItemTupleTwinRustAsyncSse_BImpl _value,
      $Res Function(_$EnumWithItemTupleTwinRustAsyncSse_BImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinRustAsyncSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumWithItemTupleTwinRustAsyncSse_BImpl
    extends EnumWithItemTupleTwinRustAsyncSse_B {
  const _$EnumWithItemTupleTwinRustAsyncSse_BImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'b',
        super._();

  factory _$EnumWithItemTupleTwinRustAsyncSse_BImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumWithItemTupleTwinRustAsyncSse_BImplFromJson(json);

  @override
  final Int32List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinRustAsyncSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinRustAsyncSse_BImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemTupleTwinRustAsyncSse_BImpl>
      get copyWith => __$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWithImpl<
          _$EnumWithItemTupleTwinRustAsyncSse_BImpl>(this, _$identity);

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
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinRustAsyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumWithItemTupleTwinRustAsyncSse_BImplToJson(
      this,
    );
  }
}

abstract class EnumWithItemTupleTwinRustAsyncSse_B
    extends EnumWithItemTupleTwinRustAsyncSse {
  const factory EnumWithItemTupleTwinRustAsyncSse_B(final Int32List field0) =
      _$EnumWithItemTupleTwinRustAsyncSse_BImpl;
  const EnumWithItemTupleTwinRustAsyncSse_B._() : super._();

  factory EnumWithItemTupleTwinRustAsyncSse_B.fromJson(
          Map<String, dynamic> json) =
      _$EnumWithItemTupleTwinRustAsyncSse_BImpl.fromJson;

  @override
  Int32List get field0;
  @JsonKey(ignore: true)
  _$$EnumWithItemTupleTwinRustAsyncSse_BImplCopyWith<
          _$EnumWithItemTupleTwinRustAsyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

KitchenSinkTwinRustAsyncSse _$KitchenSinkTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'empty':
      return KitchenSinkTwinRustAsyncSse_Empty.fromJson(json);
    case 'primitives':
      return KitchenSinkTwinRustAsyncSse_Primitives.fromJson(json);
    case 'nested':
      return KitchenSinkTwinRustAsyncSse_Nested.fromJson(json);
    case 'optional':
      return KitchenSinkTwinRustAsyncSse_Optional.fromJson(json);
    case 'buffer':
      return KitchenSinkTwinRustAsyncSse_Buffer.fromJson(json);
    case 'enums':
      return KitchenSinkTwinRustAsyncSse_Enums.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'KitchenSinkTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$KitchenSinkTwinRustAsyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkTwinRustAsyncSseCopyWith<$Res> {
  factory $KitchenSinkTwinRustAsyncSseCopyWith(
          KitchenSinkTwinRustAsyncSse value,
          $Res Function(KitchenSinkTwinRustAsyncSse) then) =
      _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
          KitchenSinkTwinRustAsyncSse>;
}

/// @nodoc
class _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends KitchenSinkTwinRustAsyncSse>
    implements $KitchenSinkTwinRustAsyncSseCopyWith<$Res> {
  _$KitchenSinkTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_EmptyImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_EmptyImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_EmptyImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_EmptyImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_EmptyImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_EmptyImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinRustAsyncSse_EmptyImpl
    extends KitchenSinkTwinRustAsyncSse_Empty {
  const _$KitchenSinkTwinRustAsyncSse_EmptyImpl({final String? $type})
      : $type = $type ?? 'empty',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_EmptyImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_EmptyImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinRustAsyncSse.empty()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_EmptyImpl);
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
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_EmptyImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Empty
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Empty() =
      _$KitchenSinkTwinRustAsyncSse_EmptyImpl;
  const KitchenSinkTwinRustAsyncSse_Empty._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Empty.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_EmptyImpl.fromJson;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_PrimitivesImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_PrimitivesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(_$KitchenSinkTwinRustAsyncSse_PrimitivesImpl(
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
class _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl
    extends KitchenSinkTwinRustAsyncSse_Primitives {
  const _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl(
      {this.int32 = -1,
      required this.float64,
      required this.boolean,
      final String? $type})
      : $type = $type ?? 'primitives',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplFromJson(json);

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
    return 'KitchenSinkTwinRustAsyncSse.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl &&
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
  _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl>
      get copyWith =>
          __$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWithImpl<
              _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Primitives
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Primitives(
          {final int int32,
          required final double float64,
          required final bool boolean}) =
      _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl;
  const KitchenSinkTwinRustAsyncSse_Primitives._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Primitives.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl.fromJson;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinRustAsyncSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_PrimitivesImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_NestedImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_NestedImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, KitchenSinkTwinRustAsyncSse field1});

  $KitchenSinkTwinRustAsyncSseCopyWith<$Res> get field1;
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_NestedImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_NestedImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_NestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$KitchenSinkTwinRustAsyncSse_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinRustAsyncSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinRustAsyncSseCopyWith<$Res> get field1 {
    return $KitchenSinkTwinRustAsyncSseCopyWith<$Res>(_value.field1, (value) {
      return _then(_value.copyWith(field1: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinRustAsyncSse_NestedImpl
    extends KitchenSinkTwinRustAsyncSse_Nested {
  const _$KitchenSinkTwinRustAsyncSse_NestedImpl(this.field0,
      [this.field1 = const KitchenSinkTwinRustAsyncSse.empty(),
      final String? $type])
      : $type = $type ?? 'nested',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_NestedImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_NestedImplFromJson(json);

  @override
  final int field0;
  @override
  @JsonKey()
  final KitchenSinkTwinRustAsyncSse field1;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinRustAsyncSse.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_NestedImpl>
      get copyWith => __$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWithImpl<
          _$KitchenSinkTwinRustAsyncSse_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_NestedImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Nested
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Nested(final int field0,
          [final KitchenSinkTwinRustAsyncSse field1]) =
      _$KitchenSinkTwinRustAsyncSse_NestedImpl;
  const KitchenSinkTwinRustAsyncSse_Nested._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Nested.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_NestedImpl.fromJson;

  int get field0;
  KitchenSinkTwinRustAsyncSse get field1;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinRustAsyncSse_NestedImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_NestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_OptionalImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_OptionalImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_OptionalImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_OptionalImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_OptionalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSinkTwinRustAsyncSse_OptionalImpl(
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
class _$KitchenSinkTwinRustAsyncSse_OptionalImpl
    extends KitchenSinkTwinRustAsyncSse_Optional {
  const _$KitchenSinkTwinRustAsyncSse_OptionalImpl(
      [this.field0 = -1, this.field1, final String? $type])
      : $type = $type ?? 'optional',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_OptionalImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_OptionalImplFromJson(json);

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
    return 'KitchenSinkTwinRustAsyncSse.optional(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_OptionalImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_OptionalImpl>
      get copyWith => __$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWithImpl<
          _$KitchenSinkTwinRustAsyncSse_OptionalImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_OptionalImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Optional
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Optional(
      [final int? field0,
      final int? field1]) = _$KitchenSinkTwinRustAsyncSse_OptionalImpl;
  const KitchenSinkTwinRustAsyncSse_Optional._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Optional.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_OptionalImpl.fromJson;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinRustAsyncSse_OptionalImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_OptionalImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_BufferImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_BufferImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_BufferImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_BufferImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_BufferImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinRustAsyncSse_BufferImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinRustAsyncSse_BufferImpl
    extends KitchenSinkTwinRustAsyncSse_Buffer {
  const _$KitchenSinkTwinRustAsyncSse_BufferImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'buffer',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_BufferImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_BufferImplFromJson(json);

  @override
  final Uint8List field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinRustAsyncSse.buffer(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_BufferImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_BufferImpl>
      get copyWith => __$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWithImpl<
          _$KitchenSinkTwinRustAsyncSse_BufferImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_BufferImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Buffer
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Buffer(final Uint8List field0) =
      _$KitchenSinkTwinRustAsyncSse_BufferImpl;
  const KitchenSinkTwinRustAsyncSse_Buffer._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Buffer.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_BufferImpl.fromJson;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinRustAsyncSse_BufferImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_BufferImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWith(
          _$KitchenSinkTwinRustAsyncSse_EnumsImpl value,
          $Res Function(_$KitchenSinkTwinRustAsyncSse_EnumsImpl) then) =
      __$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({WeekdaysTwinRustAsyncSse field0});
}

/// @nodoc
class __$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinRustAsyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinRustAsyncSse_EnumsImpl>
    implements _$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWith<$Res> {
  __$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWithImpl(
      _$KitchenSinkTwinRustAsyncSse_EnumsImpl _value,
      $Res Function(_$KitchenSinkTwinRustAsyncSse_EnumsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinRustAsyncSse_EnumsImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinRustAsyncSse,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$KitchenSinkTwinRustAsyncSse_EnumsImpl
    extends KitchenSinkTwinRustAsyncSse_Enums {
  const _$KitchenSinkTwinRustAsyncSse_EnumsImpl(
      [this.field0 = WeekdaysTwinRustAsyncSse.sunday, final String? $type])
      : $type = $type ?? 'enums',
        super._();

  factory _$KitchenSinkTwinRustAsyncSse_EnumsImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$KitchenSinkTwinRustAsyncSse_EnumsImplFromJson(json);

  @override
  @JsonKey()
  final WeekdaysTwinRustAsyncSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'KitchenSinkTwinRustAsyncSse.enums(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinRustAsyncSse_EnumsImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_EnumsImpl>
      get copyWith => __$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWithImpl<
          _$KitchenSinkTwinRustAsyncSse_EnumsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)
        nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinRustAsyncSse field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinRustAsyncSse field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinRustAsyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinRustAsyncSse field0)? enums,
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
    required TResult Function(KitchenSinkTwinRustAsyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)
        optional,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinRustAsyncSse_Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinRustAsyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinRustAsyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinRustAsyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinRustAsyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinRustAsyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinRustAsyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$KitchenSinkTwinRustAsyncSse_EnumsImplToJson(
      this,
    );
  }
}

abstract class KitchenSinkTwinRustAsyncSse_Enums
    extends KitchenSinkTwinRustAsyncSse {
  const factory KitchenSinkTwinRustAsyncSse_Enums(
          [final WeekdaysTwinRustAsyncSse field0]) =
      _$KitchenSinkTwinRustAsyncSse_EnumsImpl;
  const KitchenSinkTwinRustAsyncSse_Enums._() : super._();

  factory KitchenSinkTwinRustAsyncSse_Enums.fromJson(
          Map<String, dynamic> json) =
      _$KitchenSinkTwinRustAsyncSse_EnumsImpl.fromJson;

  WeekdaysTwinRustAsyncSse get field0;
  @JsonKey(ignore: true)
  _$$KitchenSinkTwinRustAsyncSse_EnumsImplCopyWith<
          _$KitchenSinkTwinRustAsyncSse_EnumsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

MeasureTwinRustAsyncSse _$MeasureTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'speed':
      return MeasureTwinRustAsyncSse_Speed.fromJson(json);
    case 'distance':
      return MeasureTwinRustAsyncSse_Distance.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'MeasureTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$MeasureTwinRustAsyncSse {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinRustAsyncSse field0) speed,
    required TResult Function(DistanceTwinRustAsyncSse field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult? Function(DistanceTwinRustAsyncSse field0)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult Function(DistanceTwinRustAsyncSse field0)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinRustAsyncSse_Speed value) speed,
    required TResult Function(MeasureTwinRustAsyncSse_Distance value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinRustAsyncSse_Distance value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult Function(MeasureTwinRustAsyncSse_Distance value)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MeasureTwinRustAsyncSseCopyWith<$Res> {
  factory $MeasureTwinRustAsyncSseCopyWith(MeasureTwinRustAsyncSse value,
          $Res Function(MeasureTwinRustAsyncSse) then) =
      _$MeasureTwinRustAsyncSseCopyWithImpl<$Res, MeasureTwinRustAsyncSse>;
}

/// @nodoc
class _$MeasureTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends MeasureTwinRustAsyncSse>
    implements $MeasureTwinRustAsyncSseCopyWith<$Res> {
  _$MeasureTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$MeasureTwinRustAsyncSse_SpeedImplCopyWith<$Res> {
  factory _$$MeasureTwinRustAsyncSse_SpeedImplCopyWith(
          _$MeasureTwinRustAsyncSse_SpeedImpl value,
          $Res Function(_$MeasureTwinRustAsyncSse_SpeedImpl) then) =
      __$$MeasureTwinRustAsyncSse_SpeedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({SpeedTwinRustAsyncSse field0});

  $SpeedTwinRustAsyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinRustAsyncSse_SpeedImplCopyWithImpl<$Res>
    extends _$MeasureTwinRustAsyncSseCopyWithImpl<$Res,
        _$MeasureTwinRustAsyncSse_SpeedImpl>
    implements _$$MeasureTwinRustAsyncSse_SpeedImplCopyWith<$Res> {
  __$$MeasureTwinRustAsyncSse_SpeedImplCopyWithImpl(
      _$MeasureTwinRustAsyncSse_SpeedImpl _value,
      $Res Function(_$MeasureTwinRustAsyncSse_SpeedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinRustAsyncSse_SpeedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinRustAsyncSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinRustAsyncSseCopyWith<$Res> get field0 {
    return $SpeedTwinRustAsyncSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$MeasureTwinRustAsyncSse_SpeedImpl
    extends MeasureTwinRustAsyncSse_Speed {
  const _$MeasureTwinRustAsyncSse_SpeedImpl(this.field0, {final String? $type})
      : $type = $type ?? 'speed',
        super._();

  factory _$MeasureTwinRustAsyncSse_SpeedImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MeasureTwinRustAsyncSse_SpeedImplFromJson(json);

  @override
  final SpeedTwinRustAsyncSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MeasureTwinRustAsyncSse.speed(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinRustAsyncSse_SpeedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinRustAsyncSse_SpeedImplCopyWith<
          _$MeasureTwinRustAsyncSse_SpeedImpl>
      get copyWith => __$$MeasureTwinRustAsyncSse_SpeedImplCopyWithImpl<
          _$MeasureTwinRustAsyncSse_SpeedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinRustAsyncSse field0) speed,
    required TResult Function(DistanceTwinRustAsyncSse field0) distance,
  }) {
    return speed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult? Function(DistanceTwinRustAsyncSse field0)? distance,
  }) {
    return speed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult Function(DistanceTwinRustAsyncSse field0)? distance,
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
    required TResult Function(MeasureTwinRustAsyncSse_Speed value) speed,
    required TResult Function(MeasureTwinRustAsyncSse_Distance value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinRustAsyncSse_Distance value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult Function(MeasureTwinRustAsyncSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MeasureTwinRustAsyncSse_SpeedImplToJson(
      this,
    );
  }
}

abstract class MeasureTwinRustAsyncSse_Speed extends MeasureTwinRustAsyncSse {
  const factory MeasureTwinRustAsyncSse_Speed(
      final SpeedTwinRustAsyncSse field0) = _$MeasureTwinRustAsyncSse_SpeedImpl;
  const MeasureTwinRustAsyncSse_Speed._() : super._();

  factory MeasureTwinRustAsyncSse_Speed.fromJson(Map<String, dynamic> json) =
      _$MeasureTwinRustAsyncSse_SpeedImpl.fromJson;

  @override
  SpeedTwinRustAsyncSse get field0;
  @JsonKey(ignore: true)
  _$$MeasureTwinRustAsyncSse_SpeedImplCopyWith<
          _$MeasureTwinRustAsyncSse_SpeedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MeasureTwinRustAsyncSse_DistanceImplCopyWith<$Res> {
  factory _$$MeasureTwinRustAsyncSse_DistanceImplCopyWith(
          _$MeasureTwinRustAsyncSse_DistanceImpl value,
          $Res Function(_$MeasureTwinRustAsyncSse_DistanceImpl) then) =
      __$$MeasureTwinRustAsyncSse_DistanceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DistanceTwinRustAsyncSse field0});

  $DistanceTwinRustAsyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinRustAsyncSse_DistanceImplCopyWithImpl<$Res>
    extends _$MeasureTwinRustAsyncSseCopyWithImpl<$Res,
        _$MeasureTwinRustAsyncSse_DistanceImpl>
    implements _$$MeasureTwinRustAsyncSse_DistanceImplCopyWith<$Res> {
  __$$MeasureTwinRustAsyncSse_DistanceImplCopyWithImpl(
      _$MeasureTwinRustAsyncSse_DistanceImpl _value,
      $Res Function(_$MeasureTwinRustAsyncSse_DistanceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinRustAsyncSse_DistanceImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinRustAsyncSse,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinRustAsyncSseCopyWith<$Res> get field0 {
    return $DistanceTwinRustAsyncSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$MeasureTwinRustAsyncSse_DistanceImpl
    extends MeasureTwinRustAsyncSse_Distance {
  const _$MeasureTwinRustAsyncSse_DistanceImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'distance',
        super._();

  factory _$MeasureTwinRustAsyncSse_DistanceImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MeasureTwinRustAsyncSse_DistanceImplFromJson(json);

  @override
  final DistanceTwinRustAsyncSse field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MeasureTwinRustAsyncSse.distance(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinRustAsyncSse_DistanceImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinRustAsyncSse_DistanceImplCopyWith<
          _$MeasureTwinRustAsyncSse_DistanceImpl>
      get copyWith => __$$MeasureTwinRustAsyncSse_DistanceImplCopyWithImpl<
          _$MeasureTwinRustAsyncSse_DistanceImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinRustAsyncSse field0) speed,
    required TResult Function(DistanceTwinRustAsyncSse field0) distance,
  }) {
    return distance(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult? Function(DistanceTwinRustAsyncSse field0)? distance,
  }) {
    return distance?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse field0)? speed,
    TResult Function(DistanceTwinRustAsyncSse field0)? distance,
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
    required TResult Function(MeasureTwinRustAsyncSse_Speed value) speed,
    required TResult Function(MeasureTwinRustAsyncSse_Distance value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinRustAsyncSse_Distance value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinRustAsyncSse_Speed value)? speed,
    TResult Function(MeasureTwinRustAsyncSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MeasureTwinRustAsyncSse_DistanceImplToJson(
      this,
    );
  }
}

abstract class MeasureTwinRustAsyncSse_Distance
    extends MeasureTwinRustAsyncSse {
  const factory MeasureTwinRustAsyncSse_Distance(
          final DistanceTwinRustAsyncSse field0) =
      _$MeasureTwinRustAsyncSse_DistanceImpl;
  const MeasureTwinRustAsyncSse_Distance._() : super._();

  factory MeasureTwinRustAsyncSse_Distance.fromJson(Map<String, dynamic> json) =
      _$MeasureTwinRustAsyncSse_DistanceImpl.fromJson;

  @override
  DistanceTwinRustAsyncSse get field0;
  @JsonKey(ignore: true)
  _$$MeasureTwinRustAsyncSse_DistanceImplCopyWith<
          _$MeasureTwinRustAsyncSse_DistanceImpl>
      get copyWith => throw _privateConstructorUsedError;
}

SpeedTwinRustAsyncSse _$SpeedTwinRustAsyncSseFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'unknown':
      return SpeedTwinRustAsyncSse_Unknown.fromJson(json);
    case 'gps':
      return SpeedTwinRustAsyncSse_GPS.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'SpeedTwinRustAsyncSse',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$SpeedTwinRustAsyncSse {
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
    required TResult Function(SpeedTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinRustAsyncSse_GPS value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinRustAsyncSse_GPS value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinRustAsyncSse_GPS value)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpeedTwinRustAsyncSseCopyWith<$Res> {
  factory $SpeedTwinRustAsyncSseCopyWith(SpeedTwinRustAsyncSse value,
          $Res Function(SpeedTwinRustAsyncSse) then) =
      _$SpeedTwinRustAsyncSseCopyWithImpl<$Res, SpeedTwinRustAsyncSse>;
}

/// @nodoc
class _$SpeedTwinRustAsyncSseCopyWithImpl<$Res,
        $Val extends SpeedTwinRustAsyncSse>
    implements $SpeedTwinRustAsyncSseCopyWith<$Res> {
  _$SpeedTwinRustAsyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$SpeedTwinRustAsyncSse_UnknownImplCopyWith<$Res> {
  factory _$$SpeedTwinRustAsyncSse_UnknownImplCopyWith(
          _$SpeedTwinRustAsyncSse_UnknownImpl value,
          $Res Function(_$SpeedTwinRustAsyncSse_UnknownImpl) then) =
      __$$SpeedTwinRustAsyncSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SpeedTwinRustAsyncSse_UnknownImplCopyWithImpl<$Res>
    extends _$SpeedTwinRustAsyncSseCopyWithImpl<$Res,
        _$SpeedTwinRustAsyncSse_UnknownImpl>
    implements _$$SpeedTwinRustAsyncSse_UnknownImplCopyWith<$Res> {
  __$$SpeedTwinRustAsyncSse_UnknownImplCopyWithImpl(
      _$SpeedTwinRustAsyncSse_UnknownImpl _value,
      $Res Function(_$SpeedTwinRustAsyncSse_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$SpeedTwinRustAsyncSse_UnknownImpl
    extends SpeedTwinRustAsyncSse_Unknown {
  const _$SpeedTwinRustAsyncSse_UnknownImpl({final String? $type})
      : $type = $type ?? 'unknown',
        super._();

  factory _$SpeedTwinRustAsyncSse_UnknownImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$SpeedTwinRustAsyncSse_UnknownImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'SpeedTwinRustAsyncSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinRustAsyncSse_UnknownImpl);
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
    required TResult Function(SpeedTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinRustAsyncSse_GPS value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinRustAsyncSse_GPS value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinRustAsyncSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$SpeedTwinRustAsyncSse_UnknownImplToJson(
      this,
    );
  }
}

abstract class SpeedTwinRustAsyncSse_Unknown extends SpeedTwinRustAsyncSse {
  const factory SpeedTwinRustAsyncSse_Unknown() =
      _$SpeedTwinRustAsyncSse_UnknownImpl;
  const SpeedTwinRustAsyncSse_Unknown._() : super._();

  factory SpeedTwinRustAsyncSse_Unknown.fromJson(Map<String, dynamic> json) =
      _$SpeedTwinRustAsyncSse_UnknownImpl.fromJson;
}

/// @nodoc
abstract class _$$SpeedTwinRustAsyncSse_GPSImplCopyWith<$Res> {
  factory _$$SpeedTwinRustAsyncSse_GPSImplCopyWith(
          _$SpeedTwinRustAsyncSse_GPSImpl value,
          $Res Function(_$SpeedTwinRustAsyncSse_GPSImpl) then) =
      __$$SpeedTwinRustAsyncSse_GPSImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$SpeedTwinRustAsyncSse_GPSImplCopyWithImpl<$Res>
    extends _$SpeedTwinRustAsyncSseCopyWithImpl<$Res,
        _$SpeedTwinRustAsyncSse_GPSImpl>
    implements _$$SpeedTwinRustAsyncSse_GPSImplCopyWith<$Res> {
  __$$SpeedTwinRustAsyncSse_GPSImplCopyWithImpl(
      _$SpeedTwinRustAsyncSse_GPSImpl _value,
      $Res Function(_$SpeedTwinRustAsyncSse_GPSImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$SpeedTwinRustAsyncSse_GPSImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$SpeedTwinRustAsyncSse_GPSImpl extends SpeedTwinRustAsyncSse_GPS {
  const _$SpeedTwinRustAsyncSse_GPSImpl(this.field0, {final String? $type})
      : $type = $type ?? 'gps',
        super._();

  factory _$SpeedTwinRustAsyncSse_GPSImpl.fromJson(Map<String, dynamic> json) =>
      _$$SpeedTwinRustAsyncSse_GPSImplFromJson(json);

  @override
  final double field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'SpeedTwinRustAsyncSse.gps(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinRustAsyncSse_GPSImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SpeedTwinRustAsyncSse_GPSImplCopyWith<_$SpeedTwinRustAsyncSse_GPSImpl>
      get copyWith => __$$SpeedTwinRustAsyncSse_GPSImplCopyWithImpl<
          _$SpeedTwinRustAsyncSse_GPSImpl>(this, _$identity);

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
    required TResult Function(SpeedTwinRustAsyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinRustAsyncSse_GPS value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinRustAsyncSse_GPS value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinRustAsyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinRustAsyncSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$SpeedTwinRustAsyncSse_GPSImplToJson(
      this,
    );
  }
}

abstract class SpeedTwinRustAsyncSse_GPS extends SpeedTwinRustAsyncSse {
  const factory SpeedTwinRustAsyncSse_GPS(final double field0) =
      _$SpeedTwinRustAsyncSse_GPSImpl;
  const SpeedTwinRustAsyncSse_GPS._() : super._();

  factory SpeedTwinRustAsyncSse_GPS.fromJson(Map<String, dynamic> json) =
      _$SpeedTwinRustAsyncSse_GPSImpl.fromJson;

  double get field0;
  @JsonKey(ignore: true)
  _$$SpeedTwinRustAsyncSse_GPSImplCopyWith<_$SpeedTwinRustAsyncSse_GPSImpl>
      get copyWith => throw _privateConstructorUsedError;
}
