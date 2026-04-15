// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinRustAsync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinRustAsync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync()';
  }
}

/// @nodoc
class $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  $EnumOpaqueTwinRustAsyncCopyWith(
      EnumOpaqueTwinRustAsync _, $Res Function(EnumOpaqueTwinRustAsync) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinRustAsync].
extension EnumOpaqueTwinRustAsyncPatterns on EnumOpaqueTwinRustAsync {
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
    TResult Function(EnumOpaqueTwinRustAsync_Struct value)? struct,
    TResult Function(EnumOpaqueTwinRustAsync_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinRustAsync_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinRustAsync_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinRustAsync_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinRustAsync_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsync_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsync_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsync_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsync_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinRustAsync_Struct value) struct,
    required TResult Function(EnumOpaqueTwinRustAsync_Primitive value)
        primitive,
    required TResult Function(EnumOpaqueTwinRustAsync_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinRustAsync_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinRustAsync_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinRustAsync_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct():
        return struct(_that);
      case EnumOpaqueTwinRustAsync_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinRustAsync_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinRustAsync_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinRustAsync_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinRustAsync_Nothing():
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
    TResult? Function(EnumOpaqueTwinRustAsync_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinRustAsync_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinRustAsync_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinRustAsync_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinRustAsync_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinRustAsync_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinRustAsync_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinRustAsync_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinRustAsync_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinRustAsync_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinRustAsync field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinRustAsync field0)? traitObj,
    TResult Function(MutexHideDataTwinRustAsync field0)? mutex,
    TResult Function(RwLockHideDataTwinRustAsync field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsync_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsync_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsync_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsync_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinRustAsync field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinRustAsync field0) traitObj,
    required TResult Function(MutexHideDataTwinRustAsync field0) mutex,
    required TResult Function(RwLockHideDataTwinRustAsync field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsync_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsync_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsync_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsync_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsync_Nothing():
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
    TResult? Function(HideDataTwinRustAsync field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinRustAsync field0)? traitObj,
    TResult? Function(MutexHideDataTwinRustAsync field0)? mutex,
    TResult? Function(RwLockHideDataTwinRustAsync field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinRustAsync_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinRustAsync_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinRustAsync_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinRustAsync_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinRustAsync_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinRustAsync_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_Struct extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_Struct(this.field0) : super._();

  final HideDataTwinRustAsync field0;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsync_StructCopyWith<EnumOpaqueTwinRustAsync_Struct>
      get copyWith => _$EnumOpaqueTwinRustAsync_StructCopyWithImpl<
          EnumOpaqueTwinRustAsync_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsync_StructCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsync_StructCopyWith(
          EnumOpaqueTwinRustAsync_Struct value,
          $Res Function(EnumOpaqueTwinRustAsync_Struct) _then) =
      _$EnumOpaqueTwinRustAsync_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinRustAsync field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsync_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsync_StructCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsync_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsync_Struct _self;
  final $Res Function(EnumOpaqueTwinRustAsync_Struct) _then;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsync_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinRustAsync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_Primitive extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsync_PrimitiveCopyWith<EnumOpaqueTwinRustAsync_Primitive>
      get copyWith => _$EnumOpaqueTwinRustAsync_PrimitiveCopyWithImpl<
          EnumOpaqueTwinRustAsync_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsync_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsync_PrimitiveCopyWith(
          EnumOpaqueTwinRustAsync_Primitive value,
          $Res Function(EnumOpaqueTwinRustAsync_Primitive) _then) =
      _$EnumOpaqueTwinRustAsync_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsync_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsync_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsync_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsync_Primitive _self;
  final $Res Function(EnumOpaqueTwinRustAsync_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsync_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_TraitObj extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinRustAsync field0;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsync_TraitObjCopyWith<EnumOpaqueTwinRustAsync_TraitObj>
      get copyWith => _$EnumOpaqueTwinRustAsync_TraitObjCopyWithImpl<
          EnumOpaqueTwinRustAsync_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsync_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsync_TraitObjCopyWith(
          EnumOpaqueTwinRustAsync_TraitObj value,
          $Res Function(EnumOpaqueTwinRustAsync_TraitObj) _then) =
      _$EnumOpaqueTwinRustAsync_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinRustAsync field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsync_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsync_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsync_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsync_TraitObj _self;
  final $Res Function(EnumOpaqueTwinRustAsync_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsync_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinRustAsync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_Mutex extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_Mutex(this.field0) : super._();

  final MutexHideDataTwinRustAsync field0;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsync_MutexCopyWith<EnumOpaqueTwinRustAsync_Mutex>
      get copyWith => _$EnumOpaqueTwinRustAsync_MutexCopyWithImpl<
          EnumOpaqueTwinRustAsync_Mutex>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsync_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsync_MutexCopyWith(
          EnumOpaqueTwinRustAsync_Mutex value,
          $Res Function(EnumOpaqueTwinRustAsync_Mutex) _then) =
      _$EnumOpaqueTwinRustAsync_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinRustAsync field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsync_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsync_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsync_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsync_Mutex _self;
  final $Res Function(EnumOpaqueTwinRustAsync_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsync_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinRustAsync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_RwLock extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_RwLock(this.field0) : super._();

  final RwLockHideDataTwinRustAsync field0;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinRustAsync_RwLockCopyWith<EnumOpaqueTwinRustAsync_RwLock>
      get copyWith => _$EnumOpaqueTwinRustAsync_RwLockCopyWithImpl<
          EnumOpaqueTwinRustAsync_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinRustAsync_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinRustAsyncCopyWith<$Res> {
  factory $EnumOpaqueTwinRustAsync_RwLockCopyWith(
          EnumOpaqueTwinRustAsync_RwLock value,
          $Res Function(EnumOpaqueTwinRustAsync_RwLock) _then) =
      _$EnumOpaqueTwinRustAsync_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinRustAsync field0});
}

/// @nodoc
class _$EnumOpaqueTwinRustAsync_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinRustAsync_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinRustAsync_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinRustAsync_RwLock _self;
  final $Res Function(EnumOpaqueTwinRustAsync_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinRustAsync_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinRustAsync,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinRustAsync_Nothing extends EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinRustAsync_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinRustAsync.nothing()';
  }
}

// dart format on
