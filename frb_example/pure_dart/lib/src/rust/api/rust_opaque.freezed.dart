// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$EnumOpaque {
  FrbOpaque get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumOpaqueCopyWith<$Res> {
  factory $EnumOpaqueCopyWith(
          EnumOpaque value, $Res Function(EnumOpaque) then) =
      _$EnumOpaqueCopyWithImpl<$Res, EnumOpaque>;
}

/// @nodoc
class _$EnumOpaqueCopyWithImpl<$Res, $Val extends EnumOpaque>
    implements $EnumOpaqueCopyWith<$Res> {
  _$EnumOpaqueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumOpaque_StructImplCopyWith<$Res> {
  factory _$$EnumOpaque_StructImplCopyWith(_$EnumOpaque_StructImpl value,
          $Res Function(_$EnumOpaque_StructImpl) then) =
      __$$EnumOpaque_StructImplCopyWithImpl<$Res>;
  @useResult
  $Res call({HideData field0});
}

/// @nodoc
class __$$EnumOpaque_StructImplCopyWithImpl<$Res>
    extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_StructImpl>
    implements _$$EnumOpaque_StructImplCopyWith<$Res> {
  __$$EnumOpaque_StructImplCopyWithImpl(_$EnumOpaque_StructImpl _value,
      $Res Function(_$EnumOpaque_StructImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_StructImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_StructImpl implements EnumOpaque_Struct {
  const _$EnumOpaque_StructImpl(this.field0);

  @override
  final HideData field0;

  @override
  String toString() {
    return 'EnumOpaque.struct(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_StructImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_StructImplCopyWith<_$EnumOpaque_StructImpl> get copyWith =>
      __$$EnumOpaque_StructImplCopyWithImpl<_$EnumOpaque_StructImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return struct(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return struct?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
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
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return struct(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return struct?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Struct implements EnumOpaque {
  const factory EnumOpaque_Struct(final HideData field0) =
      _$EnumOpaque_StructImpl;

  @override
  HideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_StructImplCopyWith<_$EnumOpaque_StructImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_PrimitiveImplCopyWith<$Res> {
  factory _$$EnumOpaque_PrimitiveImplCopyWith(_$EnumOpaque_PrimitiveImpl value,
          $Res Function(_$EnumOpaque_PrimitiveImpl) then) =
      __$$EnumOpaque_PrimitiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class __$$EnumOpaque_PrimitiveImplCopyWithImpl<$Res>
    extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_PrimitiveImpl>
    implements _$$EnumOpaque_PrimitiveImplCopyWith<$Res> {
  __$$EnumOpaque_PrimitiveImplCopyWithImpl(_$EnumOpaque_PrimitiveImpl _value,
      $Res Function(_$EnumOpaque_PrimitiveImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_PrimitiveImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_PrimitiveImpl implements EnumOpaque_Primitive {
  const _$EnumOpaque_PrimitiveImpl(this.field0);

  @override
  final I32 field0;

  @override
  String toString() {
    return 'EnumOpaque.primitive(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_PrimitiveImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_PrimitiveImplCopyWith<_$EnumOpaque_PrimitiveImpl>
      get copyWith =>
          __$$EnumOpaque_PrimitiveImplCopyWithImpl<_$EnumOpaque_PrimitiveImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
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
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Primitive implements EnumOpaque {
  const factory EnumOpaque_Primitive(final I32 field0) =
      _$EnumOpaque_PrimitiveImpl;

  @override
  I32 get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_PrimitiveImplCopyWith<_$EnumOpaque_PrimitiveImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_TraitObjImplCopyWith<$Res> {
  factory _$$EnumOpaque_TraitObjImplCopyWith(_$EnumOpaque_TraitObjImpl value,
          $Res Function(_$EnumOpaque_TraitObjImpl) then) =
      __$$EnumOpaque_TraitObjImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BoxDartDebug field0});
}

/// @nodoc
class __$$EnumOpaque_TraitObjImplCopyWithImpl<$Res>
    extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_TraitObjImpl>
    implements _$$EnumOpaque_TraitObjImplCopyWith<$Res> {
  __$$EnumOpaque_TraitObjImplCopyWithImpl(_$EnumOpaque_TraitObjImpl _value,
      $Res Function(_$EnumOpaque_TraitObjImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_TraitObjImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebug,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_TraitObjImpl implements EnumOpaque_TraitObj {
  const _$EnumOpaque_TraitObjImpl(this.field0);

  @override
  final BoxDartDebug field0;

  @override
  String toString() {
    return 'EnumOpaque.traitObj(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_TraitObjImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_TraitObjImplCopyWith<_$EnumOpaque_TraitObjImpl> get copyWith =>
      __$$EnumOpaque_TraitObjImplCopyWithImpl<_$EnumOpaque_TraitObjImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return traitObj(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return traitObj?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
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
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return traitObj(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return traitObj?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_TraitObj implements EnumOpaque {
  const factory EnumOpaque_TraitObj(final BoxDartDebug field0) =
      _$EnumOpaque_TraitObjImpl;

  @override
  BoxDartDebug get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_TraitObjImplCopyWith<_$EnumOpaque_TraitObjImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_MutexImplCopyWith<$Res> {
  factory _$$EnumOpaque_MutexImplCopyWith(_$EnumOpaque_MutexImpl value,
          $Res Function(_$EnumOpaque_MutexImpl) then) =
      __$$EnumOpaque_MutexImplCopyWithImpl<$Res>;
  @useResult
  $Res call({MutexHideData field0});
}

/// @nodoc
class __$$EnumOpaque_MutexImplCopyWithImpl<$Res>
    extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_MutexImpl>
    implements _$$EnumOpaque_MutexImplCopyWith<$Res> {
  __$$EnumOpaque_MutexImplCopyWithImpl(_$EnumOpaque_MutexImpl _value,
      $Res Function(_$EnumOpaque_MutexImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_MutexImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_MutexImpl implements EnumOpaque_Mutex {
  const _$EnumOpaque_MutexImpl(this.field0);

  @override
  final MutexHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.mutex(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_MutexImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_MutexImplCopyWith<_$EnumOpaque_MutexImpl> get copyWith =>
      __$$EnumOpaque_MutexImplCopyWithImpl<_$EnumOpaque_MutexImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return mutex(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return mutex?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
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
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return mutex(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return mutex?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Mutex implements EnumOpaque {
  const factory EnumOpaque_Mutex(final MutexHideData field0) =
      _$EnumOpaque_MutexImpl;

  @override
  MutexHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_MutexImplCopyWith<_$EnumOpaque_MutexImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_RwLockImplCopyWith<$Res> {
  factory _$$EnumOpaque_RwLockImplCopyWith(_$EnumOpaque_RwLockImpl value,
          $Res Function(_$EnumOpaque_RwLockImpl) then) =
      __$$EnumOpaque_RwLockImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RwLockHideData field0});
}

/// @nodoc
class __$$EnumOpaque_RwLockImplCopyWithImpl<$Res>
    extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_RwLockImpl>
    implements _$$EnumOpaque_RwLockImplCopyWith<$Res> {
  __$$EnumOpaque_RwLockImplCopyWithImpl(_$EnumOpaque_RwLockImpl _value,
      $Res Function(_$EnumOpaque_RwLockImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_RwLockImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_RwLockImpl implements EnumOpaque_RwLock {
  const _$EnumOpaque_RwLockImpl(this.field0);

  @override
  final RwLockHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.rwLock(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_RwLockImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_RwLockImplCopyWith<_$EnumOpaque_RwLockImpl> get copyWith =>
      __$$EnumOpaque_RwLockImplCopyWithImpl<_$EnumOpaque_RwLockImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return rwLock(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return rwLock?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
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
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return rwLock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return rwLock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_RwLock implements EnumOpaque {
  const factory EnumOpaque_RwLock(final RwLockHideData field0) =
      _$EnumOpaque_RwLockImpl;

  @override
  RwLockHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_RwLockImplCopyWith<_$EnumOpaque_RwLockImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
