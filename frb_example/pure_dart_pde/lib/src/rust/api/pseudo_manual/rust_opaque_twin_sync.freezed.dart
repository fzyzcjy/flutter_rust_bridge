// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

EnumOpaqueTwinSync _$EnumOpaqueTwinSyncFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'struct':
      return EnumOpaqueTwinSync_Struct.fromJson(json);
    case 'primitive':
      return EnumOpaqueTwinSync_Primitive.fromJson(json);
    case 'traitObj':
      return EnumOpaqueTwinSync_TraitObj.fromJson(json);
    case 'mutex':
      return EnumOpaqueTwinSync_Mutex.fromJson(json);
    case 'rwLock':
      return EnumOpaqueTwinSync_RwLock.fromJson(json);
    case 'nothing':
      return EnumOpaqueTwinSync_Nothing.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'runtimeType', 'EnumOpaqueTwinSync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$EnumOpaqueTwinSync {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncCopyWith(
          EnumOpaqueTwinSync value, $Res Function(EnumOpaqueTwinSync) then) =
      _$EnumOpaqueTwinSyncCopyWithImpl<$Res, EnumOpaqueTwinSync>;
}

/// @nodoc
class _$EnumOpaqueTwinSyncCopyWithImpl<$Res, $Val extends EnumOpaqueTwinSync>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  _$EnumOpaqueTwinSyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_StructImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_StructImplCopyWith(
          _$EnumOpaqueTwinSync_StructImpl value,
          $Res Function(_$EnumOpaqueTwinSync_StructImpl) then) =
      __$$EnumOpaqueTwinSync_StructImplCopyWithImpl<$Res>;
  @useResult
  $Res call({HideDataTwinSync field0});
}

/// @nodoc
class __$$EnumOpaqueTwinSync_StructImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_StructImpl>
    implements _$$EnumOpaqueTwinSync_StructImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_StructImplCopyWithImpl(
      _$EnumOpaqueTwinSync_StructImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_StructImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaqueTwinSync_StructImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_StructImpl extends EnumOpaqueTwinSync_Struct {
  const _$EnumOpaqueTwinSync_StructImpl(this.field0, {final String? $type})
      : $type = $type ?? 'struct',
        super._();

  factory _$EnumOpaqueTwinSync_StructImpl.fromJson(Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_StructImplFromJson(json);

  @override
  final HideDataTwinSync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.struct(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_StructImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaqueTwinSync_StructImplCopyWith<_$EnumOpaqueTwinSync_StructImpl>
      get copyWith => __$$EnumOpaqueTwinSync_StructImplCopyWithImpl<
          _$EnumOpaqueTwinSync_StructImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return struct(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return struct?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return struct(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return struct?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_StructImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_Struct extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_Struct(final HideDataTwinSync field0) =
      _$EnumOpaqueTwinSync_StructImpl;
  const EnumOpaqueTwinSync_Struct._() : super._();

  factory EnumOpaqueTwinSync_Struct.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_StructImpl.fromJson;

  HideDataTwinSync get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaqueTwinSync_StructImplCopyWith<_$EnumOpaqueTwinSync_StructImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_PrimitiveImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_PrimitiveImplCopyWith(
          _$EnumOpaqueTwinSync_PrimitiveImpl value,
          $Res Function(_$EnumOpaqueTwinSync_PrimitiveImpl) then) =
      __$$EnumOpaqueTwinSync_PrimitiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class __$$EnumOpaqueTwinSync_PrimitiveImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_PrimitiveImpl>
    implements _$$EnumOpaqueTwinSync_PrimitiveImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_PrimitiveImplCopyWithImpl(
      _$EnumOpaqueTwinSync_PrimitiveImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_PrimitiveImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaqueTwinSync_PrimitiveImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_PrimitiveImpl extends EnumOpaqueTwinSync_Primitive {
  const _$EnumOpaqueTwinSync_PrimitiveImpl(this.field0, {final String? $type})
      : $type = $type ?? 'primitive',
        super._();

  factory _$EnumOpaqueTwinSync_PrimitiveImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_PrimitiveImplFromJson(json);

  @override
  final I32 field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.primitive(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_PrimitiveImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaqueTwinSync_PrimitiveImplCopyWith<
          _$EnumOpaqueTwinSync_PrimitiveImpl>
      get copyWith => __$$EnumOpaqueTwinSync_PrimitiveImplCopyWithImpl<
          _$EnumOpaqueTwinSync_PrimitiveImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_PrimitiveImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_Primitive extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_Primitive(final I32 field0) =
      _$EnumOpaqueTwinSync_PrimitiveImpl;
  const EnumOpaqueTwinSync_Primitive._() : super._();

  factory EnumOpaqueTwinSync_Primitive.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_PrimitiveImpl.fromJson;

  I32 get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaqueTwinSync_PrimitiveImplCopyWith<
          _$EnumOpaqueTwinSync_PrimitiveImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_TraitObjImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_TraitObjImplCopyWith(
          _$EnumOpaqueTwinSync_TraitObjImpl value,
          $Res Function(_$EnumOpaqueTwinSync_TraitObjImpl) then) =
      __$$EnumOpaqueTwinSync_TraitObjImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BoxDartDebugTwinSync field0});
}

/// @nodoc
class __$$EnumOpaqueTwinSync_TraitObjImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_TraitObjImpl>
    implements _$$EnumOpaqueTwinSync_TraitObjImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_TraitObjImplCopyWithImpl(
      _$EnumOpaqueTwinSync_TraitObjImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_TraitObjImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaqueTwinSync_TraitObjImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_TraitObjImpl extends EnumOpaqueTwinSync_TraitObj {
  const _$EnumOpaqueTwinSync_TraitObjImpl(this.field0, {final String? $type})
      : $type = $type ?? 'traitObj',
        super._();

  factory _$EnumOpaqueTwinSync_TraitObjImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_TraitObjImplFromJson(json);

  @override
  final BoxDartDebugTwinSync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.traitObj(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_TraitObjImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaqueTwinSync_TraitObjImplCopyWith<_$EnumOpaqueTwinSync_TraitObjImpl>
      get copyWith => __$$EnumOpaqueTwinSync_TraitObjImplCopyWithImpl<
          _$EnumOpaqueTwinSync_TraitObjImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return traitObj(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return traitObj?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return traitObj(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return traitObj?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_TraitObjImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_TraitObj extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_TraitObj(final BoxDartDebugTwinSync field0) =
      _$EnumOpaqueTwinSync_TraitObjImpl;
  const EnumOpaqueTwinSync_TraitObj._() : super._();

  factory EnumOpaqueTwinSync_TraitObj.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_TraitObjImpl.fromJson;

  BoxDartDebugTwinSync get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaqueTwinSync_TraitObjImplCopyWith<_$EnumOpaqueTwinSync_TraitObjImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_MutexImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_MutexImplCopyWith(
          _$EnumOpaqueTwinSync_MutexImpl value,
          $Res Function(_$EnumOpaqueTwinSync_MutexImpl) then) =
      __$$EnumOpaqueTwinSync_MutexImplCopyWithImpl<$Res>;
  @useResult
  $Res call({MutexHideDataTwinSync field0});
}

/// @nodoc
class __$$EnumOpaqueTwinSync_MutexImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_MutexImpl>
    implements _$$EnumOpaqueTwinSync_MutexImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_MutexImplCopyWithImpl(
      _$EnumOpaqueTwinSync_MutexImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_MutexImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaqueTwinSync_MutexImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_MutexImpl extends EnumOpaqueTwinSync_Mutex {
  const _$EnumOpaqueTwinSync_MutexImpl(this.field0, {final String? $type})
      : $type = $type ?? 'mutex',
        super._();

  factory _$EnumOpaqueTwinSync_MutexImpl.fromJson(Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_MutexImplFromJson(json);

  @override
  final MutexHideDataTwinSync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.mutex(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_MutexImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaqueTwinSync_MutexImplCopyWith<_$EnumOpaqueTwinSync_MutexImpl>
      get copyWith => __$$EnumOpaqueTwinSync_MutexImplCopyWithImpl<
          _$EnumOpaqueTwinSync_MutexImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return mutex(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return mutex?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return mutex(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return mutex?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_MutexImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_Mutex extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_Mutex(final MutexHideDataTwinSync field0) =
      _$EnumOpaqueTwinSync_MutexImpl;
  const EnumOpaqueTwinSync_Mutex._() : super._();

  factory EnumOpaqueTwinSync_Mutex.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_MutexImpl.fromJson;

  MutexHideDataTwinSync get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaqueTwinSync_MutexImplCopyWith<_$EnumOpaqueTwinSync_MutexImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_RwLockImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_RwLockImplCopyWith(
          _$EnumOpaqueTwinSync_RwLockImpl value,
          $Res Function(_$EnumOpaqueTwinSync_RwLockImpl) then) =
      __$$EnumOpaqueTwinSync_RwLockImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RwLockHideDataTwinSync field0});
}

/// @nodoc
class __$$EnumOpaqueTwinSync_RwLockImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_RwLockImpl>
    implements _$$EnumOpaqueTwinSync_RwLockImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_RwLockImplCopyWithImpl(
      _$EnumOpaqueTwinSync_RwLockImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_RwLockImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaqueTwinSync_RwLockImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSync,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_RwLockImpl extends EnumOpaqueTwinSync_RwLock {
  const _$EnumOpaqueTwinSync_RwLockImpl(this.field0, {final String? $type})
      : $type = $type ?? 'rwLock',
        super._();

  factory _$EnumOpaqueTwinSync_RwLockImpl.fromJson(Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_RwLockImplFromJson(json);

  @override
  final RwLockHideDataTwinSync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.rwLock(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_RwLockImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaqueTwinSync_RwLockImplCopyWith<_$EnumOpaqueTwinSync_RwLockImpl>
      get copyWith => __$$EnumOpaqueTwinSync_RwLockImplCopyWithImpl<
          _$EnumOpaqueTwinSync_RwLockImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return rwLock(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return rwLock?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return rwLock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return rwLock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_RwLockImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_RwLock extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_RwLock(final RwLockHideDataTwinSync field0) =
      _$EnumOpaqueTwinSync_RwLockImpl;
  const EnumOpaqueTwinSync_RwLock._() : super._();

  factory EnumOpaqueTwinSync_RwLock.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_RwLockImpl.fromJson;

  RwLockHideDataTwinSync get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaqueTwinSync_RwLockImplCopyWith<_$EnumOpaqueTwinSync_RwLockImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaqueTwinSync_NothingImplCopyWith<$Res> {
  factory _$$EnumOpaqueTwinSync_NothingImplCopyWith(
          _$EnumOpaqueTwinSync_NothingImpl value,
          $Res Function(_$EnumOpaqueTwinSync_NothingImpl) then) =
      __$$EnumOpaqueTwinSync_NothingImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$EnumOpaqueTwinSync_NothingImplCopyWithImpl<$Res>
    extends _$EnumOpaqueTwinSyncCopyWithImpl<$Res,
        _$EnumOpaqueTwinSync_NothingImpl>
    implements _$$EnumOpaqueTwinSync_NothingImplCopyWith<$Res> {
  __$$EnumOpaqueTwinSync_NothingImplCopyWithImpl(
      _$EnumOpaqueTwinSync_NothingImpl _value,
      $Res Function(_$EnumOpaqueTwinSync_NothingImpl) _then)
      : super(_value, _then);
}

/// @nodoc
@JsonSerializable()
class _$EnumOpaqueTwinSync_NothingImpl extends EnumOpaqueTwinSync_Nothing {
  const _$EnumOpaqueTwinSync_NothingImpl({final String? $type})
      : $type = $type ?? 'nothing',
        super._();

  factory _$EnumOpaqueTwinSync_NothingImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$EnumOpaqueTwinSync_NothingImplFromJson(json);

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.nothing()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaqueTwinSync_NothingImpl);
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    return nothing();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    return nothing?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    if (nothing != null) {
      return nothing();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    return nothing(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    return nothing?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    if (nothing != null) {
      return nothing(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$EnumOpaqueTwinSync_NothingImplToJson(
      this,
    );
  }
}

abstract class EnumOpaqueTwinSync_Nothing extends EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync_Nothing() = _$EnumOpaqueTwinSync_NothingImpl;
  const EnumOpaqueTwinSync_Nothing._() : super._();

  factory EnumOpaqueTwinSync_Nothing.fromJson(Map<String, dynamic> json) =
      _$EnumOpaqueTwinSync_NothingImpl.fromJson;
}
