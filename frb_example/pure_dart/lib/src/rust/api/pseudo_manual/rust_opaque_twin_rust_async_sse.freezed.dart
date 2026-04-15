// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinRustAsyncSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse()';
  }
}

/// @nodoc
class $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  $EnumOpaqueTwinRustAsyncSseCopyWith(EnumOpaqueTwinRustAsyncSse _,
      $Res Function(EnumOpaqueTwinRustAsyncSse) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinRustAsyncSse].
extension EnumOpaqueTwinRustAsyncSsePatterns on EnumOpaqueTwinRustAsyncSse {
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
    TResult Function(EnumOpaqueTwinRustAsyncSse_Struct value)? struct,
    TResult Function(EnumOpaqueTwinRustAsyncSse_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinRustAsyncSse_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinRustAsyncSse_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinRustAsyncSse_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinRustAsyncSse_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSse_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinRustAsyncSse_Struct value) struct,
    required TResult Function(EnumOpaqueTwinRustAsyncSse_Primitive value)
        primitive,
    required TResult Function(EnumOpaqueTwinRustAsyncSse_TraitObj value)
        traitObj,
    required TResult Function(EnumOpaqueTwinRustAsyncSse_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinRustAsyncSse_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinRustAsyncSse_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct():
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSse_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSse_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSse_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSse_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSse_Nothing():
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
    TResult? Function(EnumOpaqueTwinRustAsyncSse_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinRustAsyncSse_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinRustAsyncSse_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinRustAsyncSse_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinRustAsyncSse_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinRustAsyncSse_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsyncSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsyncSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsyncSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsyncSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsyncSse_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinRustAsyncSse field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinRustAsyncSse field0)? traitObj,
    TResult Function(MutexHideDataTwinRustAsyncSse field0)? mutex,
    TResult Function(RwLockHideDataTwinRustAsyncSse field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinRustAsyncSse field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinRustAsyncSse field0) traitObj,
    required TResult Function(MutexHideDataTwinRustAsyncSse field0) mutex,
    required TResult Function(RwLockHideDataTwinRustAsyncSse field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Nothing():
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
    TResult? Function(HideDataTwinRustAsyncSse field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinRustAsyncSse field0)? traitObj,
    TResult? Function(MutexHideDataTwinRustAsyncSse field0)? mutex,
    TResult? Function(RwLockHideDataTwinRustAsyncSse field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsyncSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsyncSse_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_Struct extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_Struct(this.field0) : super._();

  final HideDataTwinRustAsyncSse field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSse_StructCopyWith<EnumOpaqueTwinRustAsyncSse_Struct>
      get copyWith => _$EnumOpaqueTwinRustAsyncSse_StructCopyWithImpl<
          EnumOpaqueTwinRustAsyncSse_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSse_StructCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSse_StructCopyWith(
          EnumOpaqueTwinRustAsyncSse_Struct value,
          $Res Function(EnumOpaqueTwinRustAsyncSse_Struct) _then) =
      _$EnumOpaqueTwinRustAsyncSse_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinRustAsyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSse_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSse_StructCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSse_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSse_Struct _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSse_Struct) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSse_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_Primitive extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWith<
          EnumOpaqueTwinRustAsyncSse_Primitive>
      get copyWith => _$EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWithImpl<
          EnumOpaqueTwinRustAsyncSse_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWith(
          EnumOpaqueTwinRustAsyncSse_Primitive value,
          $Res Function(EnumOpaqueTwinRustAsyncSse_Primitive) _then) =
      _$EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSse_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSse_Primitive _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSse_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSse_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_TraitObj extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinRustAsyncSse field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSse_TraitObjCopyWith<
          EnumOpaqueTwinRustAsyncSse_TraitObj>
      get copyWith => _$EnumOpaqueTwinRustAsyncSse_TraitObjCopyWithImpl<
          EnumOpaqueTwinRustAsyncSse_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSse_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSse_TraitObjCopyWith(
          EnumOpaqueTwinRustAsyncSse_TraitObj value,
          $Res Function(EnumOpaqueTwinRustAsyncSse_TraitObj) _then) =
      _$EnumOpaqueTwinRustAsyncSse_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinRustAsyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSse_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSse_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSse_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSse_TraitObj _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSse_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSse_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_Mutex extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_Mutex(this.field0) : super._();

  final MutexHideDataTwinRustAsyncSse field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSse_MutexCopyWith<EnumOpaqueTwinRustAsyncSse_Mutex>
      get copyWith => _$EnumOpaqueTwinRustAsyncSse_MutexCopyWithImpl<
          EnumOpaqueTwinRustAsyncSse_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSse_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSse_MutexCopyWith(
          EnumOpaqueTwinRustAsyncSse_Mutex value,
          $Res Function(EnumOpaqueTwinRustAsyncSse_Mutex) _then) =
      _$EnumOpaqueTwinRustAsyncSse_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinRustAsyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSse_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSse_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSse_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSse_Mutex _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSse_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSse_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_RwLock extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_RwLock(this.field0) : super._();

  final RwLockHideDataTwinRustAsyncSse field0;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsyncSse_RwLockCopyWith<EnumOpaqueTwinRustAsyncSse_RwLock>
      get copyWith => _$EnumOpaqueTwinRustAsyncSse_RwLockCopyWithImpl<
          EnumOpaqueTwinRustAsyncSse_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsyncSse_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncSseCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsyncSse_RwLockCopyWith(
          EnumOpaqueTwinRustAsyncSse_RwLock value,
          $Res Function(EnumOpaqueTwinRustAsyncSse_RwLock) _then) =
      _$EnumOpaqueTwinRustAsyncSse_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinRustAsyncSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsyncSse_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsyncSse_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsyncSse_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsyncSse_RwLock _self;
  final $Res Function(EnumOpaqueTwinRustAsyncSse_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsyncSse_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinRustAsyncSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsyncSse_Nothing extends EnumOpaqueTwinRustAsyncSse {
  const EnumOpaqueTwinRustAsyncSse_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsyncSse_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsyncSse.nothing()';
  }
}

// dart format on
