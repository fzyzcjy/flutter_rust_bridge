// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sync_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSyncSseMoi {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSyncSseMoi);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  $EnumOpaqueTwinSyncSseMoiCopyWith(
      EnumOpaqueTwinSyncSseMoi _, $Res Function(EnumOpaqueTwinSyncSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSyncSseMoi].
extension EnumOpaqueTwinSyncSseMoiPatterns on EnumOpaqueTwinSyncSseMoi {
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
    TResult Function(EnumOpaqueTwinSyncSseMoi_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSyncSseMoi_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSyncSseMoi_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSyncSseMoi_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSyncSseMoi_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSyncSseMoi_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncSseMoi_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSyncSseMoi_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSyncSseMoi_Primitive value)
        primitive,
    required TResult Function(EnumOpaqueTwinSyncSseMoi_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSyncSseMoi_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSyncSseMoi_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSyncSseMoi_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct():
        return struct(_that);
      case EnumOpaqueTwinSyncSseMoi_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSyncSseMoi_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSyncSseMoi_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSyncSseMoi_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSyncSseMoi_Nothing():
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
    TResult? Function(EnumOpaqueTwinSyncSseMoi_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSyncSseMoi_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSyncSseMoi_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSyncSseMoi_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSyncSseMoi_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSyncSseMoi_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncSseMoi_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSyncSseMoi field0)? struct,
    TResult Function(I16 field0)? primitive,
    TResult Function(BoxDartDebugTwinSyncSseMoi field0)? traitObj,
    TResult Function(MutexHideDataTwinSyncSseMoi field0)? mutex,
    TResult Function(RwLockHideDataTwinSyncSseMoi field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSyncSseMoi field0) struct,
    required TResult Function(I16 field0) primitive,
    required TResult Function(BoxDartDebugTwinSyncSseMoi field0) traitObj,
    required TResult Function(MutexHideDataTwinSyncSseMoi field0) mutex,
    required TResult Function(RwLockHideDataTwinSyncSseMoi field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Nothing():
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
    TResult? Function(HideDataTwinSyncSseMoi field0)? struct,
    TResult? Function(I16 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSyncSseMoi field0)? traitObj,
    TResult? Function(MutexHideDataTwinSyncSseMoi field0)? mutex,
    TResult? Function(RwLockHideDataTwinSyncSseMoi field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSseMoi_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_Struct extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_Struct(this.field0) : super._();

  final HideDataTwinSyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSseMoi_StructCopyWith<EnumOpaqueTwinSyncSseMoi_Struct>
      get copyWith => _$EnumOpaqueTwinSyncSseMoi_StructCopyWithImpl<
          EnumOpaqueTwinSyncSseMoi_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSseMoi_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSseMoi_StructCopyWith(
          EnumOpaqueTwinSyncSseMoi_Struct value,
          $Res Function(EnumOpaqueTwinSyncSseMoi_Struct) _then) =
      _$EnumOpaqueTwinSyncSseMoi_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSseMoi_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSseMoi_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSseMoi_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSseMoi_Struct _self;
  final $Res Function(EnumOpaqueTwinSyncSseMoi_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSseMoi_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_Primitive extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_Primitive(this.field0) : super._();

  final I16 field0;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWith<
          EnumOpaqueTwinSyncSseMoi_Primitive>
      get copyWith => _$EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSyncSseMoi_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWith(
          EnumOpaqueTwinSyncSseMoi_Primitive value,
          $Res Function(EnumOpaqueTwinSyncSseMoi_Primitive) _then) =
      _$EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I16 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSseMoi_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSseMoi_Primitive _self;
  final $Res Function(EnumOpaqueTwinSyncSseMoi_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSseMoi_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I16,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_TraitObj extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSseMoi_TraitObjCopyWith<EnumOpaqueTwinSyncSseMoi_TraitObj>
      get copyWith => _$EnumOpaqueTwinSyncSseMoi_TraitObjCopyWithImpl<
          EnumOpaqueTwinSyncSseMoi_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSseMoi_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSseMoi_TraitObjCopyWith(
          EnumOpaqueTwinSyncSseMoi_TraitObj value,
          $Res Function(EnumOpaqueTwinSyncSseMoi_TraitObj) _then) =
      _$EnumOpaqueTwinSyncSseMoi_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSseMoi_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSseMoi_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSseMoi_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSseMoi_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSyncSseMoi_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSseMoi_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_Mutex extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_Mutex(this.field0) : super._();

  final MutexHideDataTwinSyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSseMoi_MutexCopyWith<EnumOpaqueTwinSyncSseMoi_Mutex>
      get copyWith => _$EnumOpaqueTwinSyncSseMoi_MutexCopyWithImpl<
          EnumOpaqueTwinSyncSseMoi_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSseMoi_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSseMoi_MutexCopyWith(
          EnumOpaqueTwinSyncSseMoi_Mutex value,
          $Res Function(EnumOpaqueTwinSyncSseMoi_Mutex) _then) =
      _$EnumOpaqueTwinSyncSseMoi_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSseMoi_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSseMoi_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSseMoi_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSseMoi_Mutex _self;
  final $Res Function(EnumOpaqueTwinSyncSseMoi_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSseMoi_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_RwLock extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSyncSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSseMoi_RwLockCopyWith<EnumOpaqueTwinSyncSseMoi_RwLock>
      get copyWith => _$EnumOpaqueTwinSyncSseMoi_RwLockCopyWithImpl<
          EnumOpaqueTwinSyncSseMoi_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSseMoi_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSseMoi_RwLockCopyWith(
          EnumOpaqueTwinSyncSseMoi_RwLock value,
          $Res Function(EnumOpaqueTwinSyncSseMoi_RwLock) _then) =
      _$EnumOpaqueTwinSyncSseMoi_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSyncSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSseMoi_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSseMoi_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSseMoi_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSseMoi_RwLock _self;
  final $Res Function(EnumOpaqueTwinSyncSseMoi_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSyncSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSseMoi_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSyncSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSseMoi_Nothing extends EnumOpaqueTwinSyncSseMoi {
  const EnumOpaqueTwinSyncSseMoi_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSseMoi_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSseMoi.nothing()';
  }
}

// dart format on
