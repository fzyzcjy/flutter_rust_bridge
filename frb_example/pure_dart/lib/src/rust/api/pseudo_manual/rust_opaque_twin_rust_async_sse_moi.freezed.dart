// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_rust_async_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinRustAsyncSseMoi {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi()';
  }
}

/// @nodoc
class $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  $EnumOpaqueTwinRustAsyncSseMoiCopyWith(EnumOpaqueTwinRustAsyncSseMoi _,
      $Res Function(EnumOpaqueTwinRustAsyncSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinRustAsyncSseMoi].
extension EnumOpaqueTwinRustAsyncSseMoiPatterns
    on EnumOpaqueTwinRustAsyncSseMoi {
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
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Struct value)? struct,
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing() when nothing != null:
        return nothing(_that);
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
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Struct value)
        struct,
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Primitive value)
        primitive,
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_TraitObj value)
        traitObj,
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_RwLock value)
        rwLock,
    required TResult Function(EnumOpaqueTwinRustAsyncSseMoi_Nothing value)
        nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct():
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing():
        return nothing(_that);
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
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinRustAsyncSseMoi_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing() when nothing != null:
        return nothing(_that);
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
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideDataTwinRustAsyncSseMoi field0)? struct,
    TResult Function(I16 field0)? primitive,
    TResult Function(BoxDartDebugTwinRustAsyncSseMoi field0)? traitObj,
    TResult Function(MutexHideDataTwinRustAsyncSseMoi field0)? mutex,
    TResult Function(RwLockHideDataTwinRustAsyncSseMoi field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing() when nothing != null:
        return nothing();
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
  TResult when<TResult extends Object?>({
    required TResult Function(HideDataTwinRustAsyncSseMoi field0) struct,
    required TResult Function(I16 field0) primitive,
    required TResult Function(BoxDartDebugTwinRustAsyncSseMoi field0) traitObj,
    required TResult Function(MutexHideDataTwinRustAsyncSseMoi field0) mutex,
    required TResult Function(RwLockHideDataTwinRustAsyncSseMoi field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing():
        return nothing();
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
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideDataTwinRustAsyncSseMoi field0)? struct,
    TResult? Function(I16 field0)? primitive,
    TResult? Function(BoxDartDebugTwinRustAsyncSseMoi field0)? traitObj,
    TResult? Function(MutexHideDataTwinRustAsyncSseMoi field0)? mutex,
    TResult? Function(RwLockHideDataTwinRustAsyncSseMoi field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSseMoi_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_Struct
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_Struct(this.field0) : super._();

  final HideDataTwinRustAsyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSseMoi_StructCopyWith<
          EnumOpaqueTwinRustAsyncSseMoi_Struct>
      get copyWith => _$EnumOpaqueTwinRustAsyncSseMoi_StructCopyWithImpl<
          EnumOpaqueTwinRustAsyncSseMoi_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSseMoi_StructCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSseMoi_StructCopyWith(
          EnumOpaqueTwinRustAsyncSseMoi_Struct value,
          $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Struct) _then) =
      _$EnumOpaqueTwinRustAsyncSseMoi_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinRustAsyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSseMoi_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoi_StructCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSseMoi_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSseMoi_Struct _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Struct) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSseMoi_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinRustAsyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_Primitive
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_Primitive(this.field0) : super._();

  final I16 field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWith<
          EnumOpaqueTwinRustAsyncSseMoi_Primitive>
      get copyWith => _$EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWithImpl<
          EnumOpaqueTwinRustAsyncSseMoi_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWith(
          EnumOpaqueTwinRustAsyncSseMoi_Primitive value,
          $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Primitive) _then) =
      _$EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I16 field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSseMoi_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSseMoi_Primitive _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSseMoi_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I16,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_TraitObj
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinRustAsyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWith<
          EnumOpaqueTwinRustAsyncSseMoi_TraitObj>
      get copyWith => _$EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWithImpl<
          EnumOpaqueTwinRustAsyncSseMoi_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWith(
          EnumOpaqueTwinRustAsyncSseMoi_TraitObj value,
          $Res Function(EnumOpaqueTwinRustAsyncSseMoi_TraitObj) _then) =
      _$EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinRustAsyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSseMoi_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSseMoi_TraitObj _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSseMoi_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSseMoi_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinRustAsyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_Mutex
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_Mutex(this.field0) : super._();

  final MutexHideDataTwinRustAsyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWith<
          EnumOpaqueTwinRustAsyncSseMoi_Mutex>
      get copyWith => _$EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWithImpl<
          EnumOpaqueTwinRustAsyncSseMoi_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWith(
          EnumOpaqueTwinRustAsyncSseMoi_Mutex value,
          $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Mutex) _then) =
      _$EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinRustAsyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSseMoi_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSseMoi_Mutex _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSseMoi_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSseMoi_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinRustAsyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_RwLock
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_RwLock(this.field0) : super._();

  final RwLockHideDataTwinRustAsyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWith<
          EnumOpaqueTwinRustAsyncSseMoi_RwLock>
      get copyWith => _$EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWithImpl<
          EnumOpaqueTwinRustAsyncSseMoi_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWith(
          EnumOpaqueTwinRustAsyncSseMoi_RwLock value,
          $Res Function(EnumOpaqueTwinRustAsyncSseMoi_RwLock) _then) =
      _$EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinRustAsyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSseMoi_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSseMoi_RwLock _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSseMoi_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSseMoi_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinRustAsyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSseMoi_Nothing
    extends EnumOpaqueTwinRustAsyncSseMoi {
  const EnumOpaqueTwinRustAsyncSseMoi_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSseMoi_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSseMoi.nothing()';
  }
}

// dart format on
