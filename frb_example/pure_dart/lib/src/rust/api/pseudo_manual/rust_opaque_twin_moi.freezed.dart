// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinMoi {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinMoi);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi()';
  }
}

/// @nodoc
class $EnumOpaqueTwinMoiCopyWith<$Res> {
  $EnumOpaqueTwinMoiCopyWith(
      EnumOpaqueTwinMoi _, $Res Function(EnumOpaqueTwinMoi) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinMoi].
extension EnumOpaqueTwinMoiPatterns on EnumOpaqueTwinMoi {
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
    TResult Function(EnumOpaqueTwinMoi_Struct value)? struct,
    TResult Function(EnumOpaqueTwinMoi_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinMoi_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinMoi_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinMoi_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinMoi_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinMoi_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinMoi_Struct value) struct,
    required TResult Function(EnumOpaqueTwinMoi_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinMoi_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinMoi_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinMoi_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinMoi_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct():
        return struct(_that);
      case EnumOpaqueTwinMoi_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinMoi_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinMoi_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinMoi_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinMoi_Nothing():
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
    TResult? Function(EnumOpaqueTwinMoi_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinMoi_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinMoi_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinMoi_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinMoi_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinMoi_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinMoi_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinMoi field0)? struct,
    TResult Function(I16 field0)? primitive,
    TResult Function(BoxDartDebugTwinMoi field0)? traitObj,
    TResult Function(MutexHideDataTwinMoi field0)? mutex,
    TResult Function(RwLockHideDataTwinMoi field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinMoi_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinMoi field0) struct,
    required TResult Function(I16 field0) primitive,
    required TResult Function(BoxDartDebugTwinMoi field0) traitObj,
    required TResult Function(MutexHideDataTwinMoi field0) mutex,
    required TResult Function(RwLockHideDataTwinMoi field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinMoi_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinMoi_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinMoi_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinMoi_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinMoi_Nothing():
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
    TResult? Function(HideDataTwinMoi field0)? struct,
    TResult? Function(I16 field0)? primitive,
    TResult? Function(BoxDartDebugTwinMoi field0)? traitObj,
    TResult? Function(MutexHideDataTwinMoi field0)? mutex,
    TResult? Function(RwLockHideDataTwinMoi field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinMoi_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_Struct extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_Struct(this.field0) : super._();

  final HideDataTwinMoi field0;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinMoi_StructCopyWith<EnumOpaqueTwinMoi_Struct> get copyWith =>
      _$EnumOpaqueTwinMoi_StructCopyWithImpl<EnumOpaqueTwinMoi_Struct>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinMoi_StructCopyWith<$Res>
    implements $EnumOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinMoi_StructCopyWith(EnumOpaqueTwinMoi_Struct value,
          $Res Function(EnumOpaqueTwinMoi_Struct) _then) =
      _$EnumOpaqueTwinMoi_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinMoi_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinMoi_StructCopyWith<$Res> {
  _$EnumOpaqueTwinMoi_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinMoi_Struct _self;
  final $Res Function(EnumOpaqueTwinMoi_Struct) _then;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinMoi_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_Primitive extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_Primitive(this.field0) : super._();

  final I16 field0;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinMoi_PrimitiveCopyWith<EnumOpaqueTwinMoi_Primitive>
      get copyWith => _$EnumOpaqueTwinMoi_PrimitiveCopyWithImpl<
          EnumOpaqueTwinMoi_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinMoi_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinMoi_PrimitiveCopyWith(
          EnumOpaqueTwinMoi_Primitive value,
          $Res Function(EnumOpaqueTwinMoi_Primitive) _then) =
      _$EnumOpaqueTwinMoi_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I16 field0});
}

/// @nodoc
class _$EnumOpaqueTwinMoi_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinMoi_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinMoi_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinMoi_Primitive _self;
  final $Res Function(EnumOpaqueTwinMoi_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinMoi_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I16,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_TraitObj extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinMoi field0;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinMoi_TraitObjCopyWith<EnumOpaqueTwinMoi_TraitObj>
      get copyWith =>
          _$EnumOpaqueTwinMoi_TraitObjCopyWithImpl<EnumOpaqueTwinMoi_TraitObj>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinMoi_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinMoi_TraitObjCopyWith(EnumOpaqueTwinMoi_TraitObj value,
          $Res Function(EnumOpaqueTwinMoi_TraitObj) _then) =
      _$EnumOpaqueTwinMoi_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinMoi_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinMoi_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinMoi_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinMoi_TraitObj _self;
  final $Res Function(EnumOpaqueTwinMoi_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinMoi_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_Mutex extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_Mutex(this.field0) : super._();

  final MutexHideDataTwinMoi field0;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinMoi_MutexCopyWith<EnumOpaqueTwinMoi_Mutex> get copyWith =>
      _$EnumOpaqueTwinMoi_MutexCopyWithImpl<EnumOpaqueTwinMoi_Mutex>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinMoi_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinMoi_MutexCopyWith(EnumOpaqueTwinMoi_Mutex value,
          $Res Function(EnumOpaqueTwinMoi_Mutex) _then) =
      _$EnumOpaqueTwinMoi_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinMoi_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinMoi_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinMoi_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinMoi_Mutex _self;
  final $Res Function(EnumOpaqueTwinMoi_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinMoi_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_RwLock extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_RwLock(this.field0) : super._();

  final RwLockHideDataTwinMoi field0;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinMoi_RwLockCopyWith<EnumOpaqueTwinMoi_RwLock> get copyWith =>
      _$EnumOpaqueTwinMoi_RwLockCopyWithImpl<EnumOpaqueTwinMoi_RwLock>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinMoi_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinMoi_RwLockCopyWith(EnumOpaqueTwinMoi_RwLock value,
          $Res Function(EnumOpaqueTwinMoi_RwLock) _then) =
      _$EnumOpaqueTwinMoi_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinMoi_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinMoi_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinMoi_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinMoi_RwLock _self;
  final $Res Function(EnumOpaqueTwinMoi_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinMoi_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinMoi_Nothing extends EnumOpaqueTwinMoi {
  const EnumOpaqueTwinMoi_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinMoi_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinMoi.nothing()';
  }
}

// dart format on
