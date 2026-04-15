// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSyncSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSyncSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  $EnumOpaqueTwinSyncSseCopyWith(
      EnumOpaqueTwinSyncSse _, $Res Function(EnumOpaqueTwinSyncSse) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSyncSse].
extension EnumOpaqueTwinSyncSsePatterns on EnumOpaqueTwinSyncSse {
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
    TResult Function(EnumOpaqueTwinSyncSse_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSyncSse_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSyncSse_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSyncSse_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSyncSse_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSyncSse_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncSse_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSyncSse_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSyncSse_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSyncSse_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSyncSse_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSyncSse_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSyncSse_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct():
        return struct(_that);
      case EnumOpaqueTwinSyncSse_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSyncSse_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSyncSse_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSyncSse_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSyncSse_Nothing():
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
    TResult? Function(EnumOpaqueTwinSyncSse_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSyncSse_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSyncSse_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSyncSse_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSyncSse_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSyncSse_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSyncSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSyncSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSyncSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSyncSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSyncSse_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSyncSse field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSyncSse field0)? traitObj,
    TResult Function(MutexHideDataTwinSyncSse field0)? mutex,
    TResult Function(RwLockHideDataTwinSyncSse field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSse_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSyncSse field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSyncSse field0) traitObj,
    required TResult Function(MutexHideDataTwinSyncSse field0) mutex,
    required TResult Function(RwLockHideDataTwinSyncSse field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSse_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSse_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSse_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSse_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSse_Nothing():
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
    TResult? Function(HideDataTwinSyncSse field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSyncSse field0)? traitObj,
    TResult? Function(MutexHideDataTwinSyncSse field0)? mutex,
    TResult? Function(RwLockHideDataTwinSyncSse field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSyncSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSyncSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSyncSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSyncSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSyncSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSyncSse_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_Struct extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_Struct(this.field0) : super._();

  final HideDataTwinSyncSse field0;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSse_StructCopyWith<EnumOpaqueTwinSyncSse_Struct>
      get copyWith => _$EnumOpaqueTwinSyncSse_StructCopyWithImpl<
          EnumOpaqueTwinSyncSse_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSse_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSse_StructCopyWith(
          EnumOpaqueTwinSyncSse_Struct value,
          $Res Function(EnumOpaqueTwinSyncSse_Struct) _then) =
      _$EnumOpaqueTwinSyncSse_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSse_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSse_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSse_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSse_Struct _self;
  final $Res Function(EnumOpaqueTwinSyncSse_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSse_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_Primitive extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSse_PrimitiveCopyWith<EnumOpaqueTwinSyncSse_Primitive>
      get copyWith => _$EnumOpaqueTwinSyncSse_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSyncSse_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSse_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSse_PrimitiveCopyWith(
          EnumOpaqueTwinSyncSse_Primitive value,
          $Res Function(EnumOpaqueTwinSyncSse_Primitive) _then) =
      _$EnumOpaqueTwinSyncSse_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSse_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSse_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSse_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSse_Primitive _self;
  final $Res Function(EnumOpaqueTwinSyncSse_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSse_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_TraitObj extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSyncSse field0;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSse_TraitObjCopyWith<EnumOpaqueTwinSyncSse_TraitObj>
      get copyWith => _$EnumOpaqueTwinSyncSse_TraitObjCopyWithImpl<
          EnumOpaqueTwinSyncSse_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSse_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSse_TraitObjCopyWith(
          EnumOpaqueTwinSyncSse_TraitObj value,
          $Res Function(EnumOpaqueTwinSyncSse_TraitObj) _then) =
      _$EnumOpaqueTwinSyncSse_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSse_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSse_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSse_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSse_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSyncSse_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSse_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_Mutex extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_Mutex(this.field0) : super._();

  final MutexHideDataTwinSyncSse field0;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSse_MutexCopyWith<EnumOpaqueTwinSyncSse_Mutex>
      get copyWith => _$EnumOpaqueTwinSyncSse_MutexCopyWithImpl<
          EnumOpaqueTwinSyncSse_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSse_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSse_MutexCopyWith(
          EnumOpaqueTwinSyncSse_Mutex value,
          $Res Function(EnumOpaqueTwinSyncSse_Mutex) _then) =
      _$EnumOpaqueTwinSyncSse_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSse_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSse_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSse_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSse_Mutex _self;
  final $Res Function(EnumOpaqueTwinSyncSse_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSse_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_RwLock extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSyncSse field0;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSyncSse_RwLockCopyWith<EnumOpaqueTwinSyncSse_RwLock>
      get copyWith => _$EnumOpaqueTwinSyncSse_RwLockCopyWithImpl<
          EnumOpaqueTwinSyncSse_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSyncSse_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSyncSse_RwLockCopyWith(
          EnumOpaqueTwinSyncSse_RwLock value,
          $Res Function(EnumOpaqueTwinSyncSse_RwLock) _then) =
      _$EnumOpaqueTwinSyncSse_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSyncSse_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSyncSse_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSyncSse_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSyncSse_RwLock _self;
  final $Res Function(EnumOpaqueTwinSyncSse_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSyncSse_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSyncSse_Nothing extends EnumOpaqueTwinSyncSse {
  const EnumOpaqueTwinSyncSse_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSyncSse_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSyncSse.nothing()';
  }
}

// dart format on
