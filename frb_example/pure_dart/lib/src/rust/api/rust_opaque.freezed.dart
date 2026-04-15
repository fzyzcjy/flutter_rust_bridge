// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal()';
  }
}

/// @nodoc
class $EnumOpaqueTwinNormalCopyWith<$Res> {
  $EnumOpaqueTwinNormalCopyWith(
      EnumOpaqueTwinNormal _, $Res Function(EnumOpaqueTwinNormal) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinNormal].
extension EnumOpaqueTwinNormalPatterns on EnumOpaqueTwinNormal {
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
    TResult Function(EnumOpaqueTwinNormal_Struct value)? struct,
    TResult Function(EnumOpaqueTwinNormal_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinNormal_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinNormal_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinNormal_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinNormal_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinNormal_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinNormal_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinNormal_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinNormal_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinNormal_Struct value) struct,
    required TResult Function(EnumOpaqueTwinNormal_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinNormal_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinNormal_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinNormal_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinNormal_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct():
        return struct(_that);
      case EnumOpaqueTwinNormal_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinNormal_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinNormal_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinNormal_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinNormal_Nothing():
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
    TResult? Function(EnumOpaqueTwinNormal_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinNormal_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinNormal_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinNormal_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinNormal_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinNormal_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinNormal_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinNormal_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinNormal_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinNormal_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinNormal field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinNormal field0)? traitObj,
    TResult Function(MutexHideDataTwinNormal field0)? mutex,
    TResult Function(RwLockHideDataTwinNormal field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinNormal_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinNormal_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinNormal_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinNormal_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinNormal field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinNormal field0) traitObj,
    required TResult Function(MutexHideDataTwinNormal field0) mutex,
    required TResult Function(RwLockHideDataTwinNormal field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinNormal_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinNormal_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinNormal_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinNormal_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinNormal_Nothing():
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
    TResult? Function(HideDataTwinNormal field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinNormal field0)? traitObj,
    TResult? Function(MutexHideDataTwinNormal field0)? mutex,
    TResult? Function(RwLockHideDataTwinNormal field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinNormal_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinNormal_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinNormal_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinNormal_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinNormal_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinNormal_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_Struct extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_Struct(this.field0) : super._();

  final HideDataTwinNormal field0;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinNormal_StructCopyWith<EnumOpaqueTwinNormal_Struct>
      get copyWith => _$EnumOpaqueTwinNormal_StructCopyWithImpl<
          EnumOpaqueTwinNormal_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinNormal_StructCopyWith<$Res>
    implements $EnumOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumOpaqueTwinNormal_StructCopyWith(
          EnumOpaqueTwinNormal_Struct value,
          $Res Function(EnumOpaqueTwinNormal_Struct) _then) =
      _$EnumOpaqueTwinNormal_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinNormal field0});
}

/// @nodoc
class _$EnumOpaqueTwinNormal_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinNormal_StructCopyWith<$Res> {
  _$EnumOpaqueTwinNormal_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinNormal_Struct _self;
  final $Res Function(EnumOpaqueTwinNormal_Struct) _then;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinNormal_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinNormal,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_Primitive extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinNormal_PrimitiveCopyWith<EnumOpaqueTwinNormal_Primitive>
      get copyWith => _$EnumOpaqueTwinNormal_PrimitiveCopyWithImpl<
          EnumOpaqueTwinNormal_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinNormal_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumOpaqueTwinNormal_PrimitiveCopyWith(
          EnumOpaqueTwinNormal_Primitive value,
          $Res Function(EnumOpaqueTwinNormal_Primitive) _then) =
      _$EnumOpaqueTwinNormal_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinNormal_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinNormal_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinNormal_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinNormal_Primitive _self;
  final $Res Function(EnumOpaqueTwinNormal_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinNormal_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_TraitObj extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinNormal field0;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinNormal_TraitObjCopyWith<EnumOpaqueTwinNormal_TraitObj>
      get copyWith => _$EnumOpaqueTwinNormal_TraitObjCopyWithImpl<
          EnumOpaqueTwinNormal_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinNormal_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumOpaqueTwinNormal_TraitObjCopyWith(
          EnumOpaqueTwinNormal_TraitObj value,
          $Res Function(EnumOpaqueTwinNormal_TraitObj) _then) =
      _$EnumOpaqueTwinNormal_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinNormal field0});
}

/// @nodoc
class _$EnumOpaqueTwinNormal_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinNormal_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinNormal_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinNormal_TraitObj _self;
  final $Res Function(EnumOpaqueTwinNormal_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinNormal_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinNormal,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_Mutex extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_Mutex(this.field0) : super._();

  final MutexHideDataTwinNormal field0;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinNormal_MutexCopyWith<EnumOpaqueTwinNormal_Mutex>
      get copyWith =>
          _$EnumOpaqueTwinNormal_MutexCopyWithImpl<EnumOpaqueTwinNormal_Mutex>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinNormal_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumOpaqueTwinNormal_MutexCopyWith(EnumOpaqueTwinNormal_Mutex value,
          $Res Function(EnumOpaqueTwinNormal_Mutex) _then) =
      _$EnumOpaqueTwinNormal_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinNormal field0});
}

/// @nodoc
class _$EnumOpaqueTwinNormal_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinNormal_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinNormal_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinNormal_Mutex _self;
  final $Res Function(EnumOpaqueTwinNormal_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinNormal_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinNormal,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_RwLock extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_RwLock(this.field0) : super._();

  final RwLockHideDataTwinNormal field0;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinNormal_RwLockCopyWith<EnumOpaqueTwinNormal_RwLock>
      get copyWith => _$EnumOpaqueTwinNormal_RwLockCopyWithImpl<
          EnumOpaqueTwinNormal_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinNormal_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinNormalCopyWith<$Res> {
  factory $EnumOpaqueTwinNormal_RwLockCopyWith(
          EnumOpaqueTwinNormal_RwLock value,
          $Res Function(EnumOpaqueTwinNormal_RwLock) _then) =
      _$EnumOpaqueTwinNormal_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinNormal field0});
}

/// @nodoc
class _$EnumOpaqueTwinNormal_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinNormal_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinNormal_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinNormal_RwLock _self;
  final $Res Function(EnumOpaqueTwinNormal_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinNormal_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinNormal,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinNormal_Nothing extends EnumOpaqueTwinNormal {
  const EnumOpaqueTwinNormal_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinNormal_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinNormal.nothing()';
  }
}

// dart format on
