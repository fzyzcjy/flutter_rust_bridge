// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSyncCopyWith<$Res> {
  $EnumOpaqueTwinSyncCopyWith(
      EnumOpaqueTwinSync _, $Res Function(EnumOpaqueTwinSync) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSync].
extension EnumOpaqueTwinSyncPatterns on EnumOpaqueTwinSync {
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
    TResult Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSync_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSync_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSync_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSync_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSync_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSync_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct():
        return struct(_that);
      case EnumOpaqueTwinSync_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSync_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSync_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSync_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSync_Nothing():
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
    TResult? Function(EnumOpaqueTwinSync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSync_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSync_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSync_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSync_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSync_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult Function(MutexHideDataTwinSync field0)? mutex,
    TResult Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSync_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSync_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSync_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSync_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSync field0) traitObj,
    required TResult Function(MutexHideDataTwinSync field0) mutex,
    required TResult Function(RwLockHideDataTwinSync field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSync_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSync_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSync_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSync_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSync_Nothing():
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
    TResult? Function(HideDataTwinSync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSync field0)? traitObj,
    TResult? Function(MutexHideDataTwinSync field0)? mutex,
    TResult? Function(RwLockHideDataTwinSync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSync_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSync_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSync_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSync_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSync_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSync_Struct extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_Struct(this.field0) : super._();

  final HideDataTwinSync field0;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSync_StructCopyWith<EnumOpaqueTwinSync_Struct> get copyWith =>
      _$EnumOpaqueTwinSync_StructCopyWithImpl<EnumOpaqueTwinSync_Struct>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSync_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSync_StructCopyWith(EnumOpaqueTwinSync_Struct value,
          $Res Function(EnumOpaqueTwinSync_Struct) _then) =
      _$EnumOpaqueTwinSync_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSync field0});
}

/// @nodoc
class _$EnumOpaqueTwinSync_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSync_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSync_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSync_Struct _self;
  final $Res Function(EnumOpaqueTwinSync_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSync_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSync_Primitive extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSync_PrimitiveCopyWith<EnumOpaqueTwinSync_Primitive>
      get copyWith => _$EnumOpaqueTwinSync_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSync_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSync_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSync_PrimitiveCopyWith(
          EnumOpaqueTwinSync_Primitive value,
          $Res Function(EnumOpaqueTwinSync_Primitive) _then) =
      _$EnumOpaqueTwinSync_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSync_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSync_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSync_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSync_Primitive _self;
  final $Res Function(EnumOpaqueTwinSync_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSync_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSync_TraitObj extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSync field0;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSync_TraitObjCopyWith<EnumOpaqueTwinSync_TraitObj>
      get copyWith => _$EnumOpaqueTwinSync_TraitObjCopyWithImpl<
          EnumOpaqueTwinSync_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSync_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSync_TraitObjCopyWith(
          EnumOpaqueTwinSync_TraitObj value,
          $Res Function(EnumOpaqueTwinSync_TraitObj) _then) =
      _$EnumOpaqueTwinSync_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSync field0});
}

/// @nodoc
class _$EnumOpaqueTwinSync_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSync_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSync_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSync_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSync_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSync_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSync_Mutex extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_Mutex(this.field0) : super._();

  final MutexHideDataTwinSync field0;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSync_MutexCopyWith<EnumOpaqueTwinSync_Mutex> get copyWith =>
      _$EnumOpaqueTwinSync_MutexCopyWithImpl<EnumOpaqueTwinSync_Mutex>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSync_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSync_MutexCopyWith(EnumOpaqueTwinSync_Mutex value,
          $Res Function(EnumOpaqueTwinSync_Mutex) _then) =
      _$EnumOpaqueTwinSync_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSync field0});
}

/// @nodoc
class _$EnumOpaqueTwinSync_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSync_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSync_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSync_Mutex _self;
  final $Res Function(EnumOpaqueTwinSync_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSync_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSync_RwLock extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSync field0;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSync_RwLockCopyWith<EnumOpaqueTwinSync_RwLock> get copyWith =>
      _$EnumOpaqueTwinSync_RwLockCopyWithImpl<EnumOpaqueTwinSync_RwLock>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSync_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSyncCopyWith<$Res> {
  factory $EnumOpaqueTwinSync_RwLockCopyWith(EnumOpaqueTwinSync_RwLock value,
          $Res Function(EnumOpaqueTwinSync_RwLock) _then) =
      _$EnumOpaqueTwinSync_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSync field0});
}

/// @nodoc
class _$EnumOpaqueTwinSync_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSync_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSync_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSync_RwLock _self;
  final $Res Function(EnumOpaqueTwinSync_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSync_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSync_Nothing extends EnumOpaqueTwinSync {
  const EnumOpaqueTwinSync_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSync_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSync.nothing()';
  }
}

// dart format on
