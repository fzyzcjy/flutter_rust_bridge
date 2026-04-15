// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSse()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSseCopyWith<$Res> {
  $EnumOpaqueTwinSseCopyWith(
      EnumOpaqueTwinSse _, $Res Function(EnumOpaqueTwinSse) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSse].
extension EnumOpaqueTwinSsePatterns on EnumOpaqueTwinSse {
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
    TResult Function(EnumOpaqueTwinSse_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSse_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSse_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSse_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSse_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSse_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSse_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSse_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSse_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSse_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSse_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSse_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSse_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct():
        return struct(_that);
      case EnumOpaqueTwinSse_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSse_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSse_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSse_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSse_Nothing():
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
    TResult? Function(EnumOpaqueTwinSse_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSse_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSse_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSse_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSse_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSse_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSse_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSse_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSse_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSse_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSse field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebugTwinSse field0)? traitObj,
    TResult Function(MutexHideDataTwinSse field0)? mutex,
    TResult Function(RwLockHideDataTwinSse field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSse_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSse field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebugTwinSse field0) traitObj,
    required TResult Function(MutexHideDataTwinSse field0) mutex,
    required TResult Function(RwLockHideDataTwinSse field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSse_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSse_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSse_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSse_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSse_Nothing():
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
    TResult? Function(HideDataTwinSse field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSse field0)? traitObj,
    TResult? Function(MutexHideDataTwinSse field0)? mutex,
    TResult? Function(RwLockHideDataTwinSse field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSse_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSse_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSse_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSse_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSse_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSse_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSse_Struct extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_Struct(this.field0) : super._();

  final HideDataTwinSse field0;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSse_StructCopyWith<EnumOpaqueTwinSse_Struct> get copyWith =>
      _$EnumOpaqueTwinSse_StructCopyWithImpl<EnumOpaqueTwinSse_Struct>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSse_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSse_StructCopyWith(EnumOpaqueTwinSse_Struct value,
          $Res Function(EnumOpaqueTwinSse_Struct) _then) =
      _$EnumOpaqueTwinSse_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSse_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSse_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSse_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSse_Struct _self;
  final $Res Function(EnumOpaqueTwinSse_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSse_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSse_Primitive extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_Primitive(this.field0) : super._();

  final I32 field0;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSse_PrimitiveCopyWith<EnumOpaqueTwinSse_Primitive>
      get copyWith => _$EnumOpaqueTwinSse_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSse_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSse_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSse_PrimitiveCopyWith(
          EnumOpaqueTwinSse_Primitive value,
          $Res Function(EnumOpaqueTwinSse_Primitive) _then) =
      _$EnumOpaqueTwinSse_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSse_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSse_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSse_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSse_Primitive _self;
  final $Res Function(EnumOpaqueTwinSse_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSse_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSse_TraitObj extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSse field0;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSse_TraitObjCopyWith<EnumOpaqueTwinSse_TraitObj>
      get copyWith =>
          _$EnumOpaqueTwinSse_TraitObjCopyWithImpl<EnumOpaqueTwinSse_TraitObj>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSse_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSse_TraitObjCopyWith(EnumOpaqueTwinSse_TraitObj value,
          $Res Function(EnumOpaqueTwinSse_TraitObj) _then) =
      _$EnumOpaqueTwinSse_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSse_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSse_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSse_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSse_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSse_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSse_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSse_Mutex extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_Mutex(this.field0) : super._();

  final MutexHideDataTwinSse field0;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSse_MutexCopyWith<EnumOpaqueTwinSse_Mutex> get copyWith =>
      _$EnumOpaqueTwinSse_MutexCopyWithImpl<EnumOpaqueTwinSse_Mutex>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSse_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSse_MutexCopyWith(EnumOpaqueTwinSse_Mutex value,
          $Res Function(EnumOpaqueTwinSse_Mutex) _then) =
      _$EnumOpaqueTwinSse_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSse_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSse_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSse_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSse_Mutex _self;
  final $Res Function(EnumOpaqueTwinSse_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSse_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSse_RwLock extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSse field0;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSse_RwLockCopyWith<EnumOpaqueTwinSse_RwLock> get copyWith =>
      _$EnumOpaqueTwinSse_RwLockCopyWithImpl<EnumOpaqueTwinSse_RwLock>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSse_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSseCopyWith<$Res> {
  factory $EnumOpaqueTwinSse_RwLockCopyWith(EnumOpaqueTwinSse_RwLock value,
          $Res Function(EnumOpaqueTwinSse_RwLock) _then) =
      _$EnumOpaqueTwinSse_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSse field0});
}

/// @nodoc
class _$EnumOpaqueTwinSse_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSse_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSse_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSse_RwLock _self;
  final $Res Function(EnumOpaqueTwinSse_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSse_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSse,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSse_Nothing extends EnumOpaqueTwinSse {
  const EnumOpaqueTwinSse_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSse_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSse.nothing()';
  }
}

// dart format on
