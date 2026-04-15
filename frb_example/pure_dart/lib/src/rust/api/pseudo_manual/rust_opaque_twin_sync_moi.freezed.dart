// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sync_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSyncMoi {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSyncMoi);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  $EnumOpaqueTwinSyncMoiCopyWith(
      EnumOpaqueTwinSyncMoi _, $Res Function(EnumOpaqueTwinSyncMoi) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSyncMoi].
extension EnumOpaqueTwinSyncMoiPatterns on EnumOpaqueTwinSyncMoi {
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
    TResult Function(EnumOpaqueTwinSyncMoi_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSyncMoi_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSyncMoi_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSyncMoi_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSyncMoi_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSyncMoi_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncMoi_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSyncMoi_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSyncMoi_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSyncMoi_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSyncMoi_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSyncMoi_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSyncMoi_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct():
        return struct(_that);
      case EnumOpaqueTwinSyncMoi_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSyncMoi_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSyncMoi_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSyncMoi_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSyncMoi_Nothing():
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
    TResult? Function(EnumOpaqueTwinSyncMoi_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSyncMoi_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSyncMoi_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSyncMoi_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSyncMoi_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSyncMoi_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncMoi_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSyncMoi field0)? struct,
    TResult Function(I16 field0)? primitive,
    TResult Function(BoxDartDebugTwinSyncMoi field0)? traitObj,
    TResult Function(MutexHideDataTwinSyncMoi field0)? mutex,
    TResult Function(RwLockHideDataTwinSyncMoi field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncMoi_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSyncMoi field0) struct,
    required TResult Function(I16 field0) primitive,
    required TResult Function(BoxDartDebugTwinSyncMoi field0) traitObj,
    required TResult Function(MutexHideDataTwinSyncMoi field0) mutex,
    required TResult Function(RwLockHideDataTwinSyncMoi field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSyncMoi_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncMoi_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncMoi_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncMoi_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncMoi_Nothing():
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
    TResult? Function(HideDataTwinSyncMoi field0)? struct,
    TResult? Function(I16 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSyncMoi field0)? traitObj,
    TResult? Function(MutexHideDataTwinSyncMoi field0)? mutex,
    TResult? Function(RwLockHideDataTwinSyncMoi field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncMoi_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_Struct extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_Struct(this.field0) : super._();

  final HideDataTwinSyncMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncMoi_StructCopyWith<EnumOpaqueTwinSyncMoi_Struct>
      get copyWith => _$EnumOpaqueTwinSyncMoi_StructCopyWithImpl<
          EnumOpaqueTwinSyncMoi_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncMoi_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncMoi_StructCopyWith(
          EnumOpaqueTwinSyncMoi_Struct value,
          $Res Function(EnumOpaqueTwinSyncMoi_Struct) _then) =
      _$EnumOpaqueTwinSyncMoi_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSyncMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncMoi_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncMoi_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSyncMoi_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncMoi_Struct _self;
  final $Res Function(EnumOpaqueTwinSyncMoi_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncMoi_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSyncMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_Primitive extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_Primitive(this.field0) : super._();

  final I16 field0;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncMoi_PrimitiveCopyWith<EnumOpaqueTwinSyncMoi_Primitive>
      get copyWith => _$EnumOpaqueTwinSyncMoi_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSyncMoi_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncMoi_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncMoi_PrimitiveCopyWith(
          EnumOpaqueTwinSyncMoi_Primitive value,
          $Res Function(EnumOpaqueTwinSyncMoi_Primitive) _then) =
      _$EnumOpaqueTwinSyncMoi_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I16 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncMoi_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncMoi_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSyncMoi_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncMoi_Primitive _self;
  final $Res Function(EnumOpaqueTwinSyncMoi_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncMoi_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I16,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_TraitObj extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSyncMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncMoi_TraitObjCopyWith<EnumOpaqueTwinSyncMoi_TraitObj>
      get copyWith => _$EnumOpaqueTwinSyncMoi_TraitObjCopyWithImpl<
          EnumOpaqueTwinSyncMoi_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncMoi_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncMoi_TraitObjCopyWith(
          EnumOpaqueTwinSyncMoi_TraitObj value,
          $Res Function(EnumOpaqueTwinSyncMoi_TraitObj) _then) =
      _$EnumOpaqueTwinSyncMoi_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSyncMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncMoi_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncMoi_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSyncMoi_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncMoi_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSyncMoi_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncMoi_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSyncMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_Mutex extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_Mutex(this.field0) : super._();

  final MutexHideDataTwinSyncMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncMoi_MutexCopyWith<EnumOpaqueTwinSyncMoi_Mutex>
      get copyWith => _$EnumOpaqueTwinSyncMoi_MutexCopyWithImpl<
          EnumOpaqueTwinSyncMoi_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncMoi_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncMoi_MutexCopyWith(
          EnumOpaqueTwinSyncMoi_Mutex value,
          $Res Function(EnumOpaqueTwinSyncMoi_Mutex) _then) =
      _$EnumOpaqueTwinSyncMoi_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSyncMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncMoi_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncMoi_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSyncMoi_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncMoi_Mutex _self;
  final $Res Function(EnumOpaqueTwinSyncMoi_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncMoi_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSyncMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_RwLock extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSyncMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncMoi_RwLockCopyWith<EnumOpaqueTwinSyncMoi_RwLock>
      get copyWith => _$EnumOpaqueTwinSyncMoi_RwLockCopyWithImpl<
          EnumOpaqueTwinSyncMoi_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncMoi_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSyncMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncMoi_RwLockCopyWith(
          EnumOpaqueTwinSyncMoi_RwLock value,
          $Res Function(EnumOpaqueTwinSyncMoi_RwLock) _then) =
      _$EnumOpaqueTwinSyncMoi_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSyncMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncMoi_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncMoi_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSyncMoi_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncMoi_RwLock _self;
  final $Res Function(EnumOpaqueTwinSyncMoi_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSyncMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncMoi_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSyncMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncMoi_Nothing extends EnumOpaqueTwinSyncMoi {
  const EnumOpaqueTwinSyncMoi_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncMoi_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncMoi.nothing()';
  }
}

// dart format on
