// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'rust_opaque_twin_sse_moi.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$EnumOpaqueTwinSseMoi {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is EnumOpaqueTwinSseMoi);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi()';
  }
}

/// @nodoc
class $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  $EnumOpaqueTwinSseMoiCopyWith(
      EnumOpaqueTwinSseMoi _, $Res Function(EnumOpaqueTwinSseMoi) __);
}

/// Adds pattern-matching-related methods to [EnumOpaqueTwinSseMoi].
extension EnumOpaqueTwinSseMoiPatterns on EnumOpaqueTwinSseMoi {
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
    TResult Function(EnumOpaqueTwinSseMoi_Struct value)? struct,
    TResult Function(EnumOpaqueTwinSseMoi_Primitive value)? primitive,
    TResult Function(EnumOpaqueTwinSseMoi_TraitObj value)? traitObj,
    TResult Function(EnumOpaqueTwinSseMoi_Mutex value)? mutex,
    TResult Function(EnumOpaqueTwinSseMoi_RwLock value)? rwLock,
    TResult Function(EnumOpaqueTwinSseMoi_Nothing value)? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSseMoi_Nothing() when nothing != null:
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
    required TResult Function(EnumOpaqueTwinSseMoi_Struct value) struct,
    required TResult Function(EnumOpaqueTwinSseMoi_Primitive value) primitive,
    required TResult Function(EnumOpaqueTwinSseMoi_TraitObj value) traitObj,
    required TResult Function(EnumOpaqueTwinSseMoi_Mutex value) mutex,
    required TResult Function(EnumOpaqueTwinSseMoi_RwLock value) rwLock,
    required TResult Function(EnumOpaqueTwinSseMoi_Nothing value) nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct():
        return struct(_that);
      case EnumOpaqueTwinSseMoi_Primitive():
        return primitive(_that);
      case EnumOpaqueTwinSseMoi_TraitObj():
        return traitObj(_that);
      case EnumOpaqueTwinSseMoi_Mutex():
        return mutex(_that);
      case EnumOpaqueTwinSseMoi_RwLock():
        return rwLock(_that);
      case EnumOpaqueTwinSseMoi_Nothing():
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
    TResult? Function(EnumOpaqueTwinSseMoi_Struct value)? struct,
    TResult? Function(EnumOpaqueTwinSseMoi_Primitive value)? primitive,
    TResult? Function(EnumOpaqueTwinSseMoi_TraitObj value)? traitObj,
    TResult? Function(EnumOpaqueTwinSseMoi_Mutex value)? mutex,
    TResult? Function(EnumOpaqueTwinSseMoi_RwLock value)? rwLock,
    TResult? Function(EnumOpaqueTwinSseMoi_Nothing value)? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct() when struct != null:
        return struct(_that);
      case EnumOpaqueTwinSseMoi_Primitive() when primitive != null:
        return primitive(_that);
      case EnumOpaqueTwinSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that);
      case EnumOpaqueTwinSseMoi_Mutex() when mutex != null:
        return mutex(_that);
      case EnumOpaqueTwinSseMoi_RwLock() when rwLock != null:
        return rwLock(_that);
      case EnumOpaqueTwinSseMoi_Nothing() when nothing != null:
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
    TResult Function(HideDataTwinSseMoi field0)? struct,
    TResult Function(I16 field0)? primitive,
    TResult Function(BoxDartDebugTwinSseMoi field0)? traitObj,
    TResult Function(MutexHideDataTwinSseMoi field0)? mutex,
    TResult Function(RwLockHideDataTwinSseMoi field0)? rwLock,
    TResult Function()? nothing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSseMoi_Nothing() when nothing != null:
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
    required TResult Function(HideDataTwinSseMoi field0) struct,
    required TResult Function(I16 field0) primitive,
    required TResult Function(BoxDartDebugTwinSseMoi field0) traitObj,
    required TResult Function(MutexHideDataTwinSseMoi field0) mutex,
    required TResult Function(RwLockHideDataTwinSseMoi field0) rwLock,
    required TResult Function() nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct():
        return struct(_that.field0);
      case EnumOpaqueTwinSseMoi_Primitive():
        return primitive(_that.field0);
      case EnumOpaqueTwinSseMoi_TraitObj():
        return traitObj(_that.field0);
      case EnumOpaqueTwinSseMoi_Mutex():
        return mutex(_that.field0);
      case EnumOpaqueTwinSseMoi_RwLock():
        return rwLock(_that.field0);
      case EnumOpaqueTwinSseMoi_Nothing():
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
    TResult? Function(HideDataTwinSseMoi field0)? struct,
    TResult? Function(I16 field0)? primitive,
    TResult? Function(BoxDartDebugTwinSseMoi field0)? traitObj,
    TResult? Function(MutexHideDataTwinSseMoi field0)? mutex,
    TResult? Function(RwLockHideDataTwinSseMoi field0)? rwLock,
    TResult? Function()? nothing,
  }) {
    final _that = this;
    switch (_that) {
      case EnumOpaqueTwinSseMoi_Struct() when struct != null:
        return struct(_that.field0);
      case EnumOpaqueTwinSseMoi_Primitive() when primitive != null:
        return primitive(_that.field0);
      case EnumOpaqueTwinSseMoi_TraitObj() when traitObj != null:
        return traitObj(_that.field0);
      case EnumOpaqueTwinSseMoi_Mutex() when mutex != null:
        return mutex(_that.field0);
      case EnumOpaqueTwinSseMoi_RwLock() when rwLock != null:
        return rwLock(_that.field0);
      case EnumOpaqueTwinSseMoi_Nothing() when nothing != null:
        return nothing();
      case _:
        return null;
    }
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_Struct extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_Struct(this.field0) : super._();

  final HideDataTwinSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSseMoi_StructCopyWith<EnumOpaqueTwinSseMoi_Struct>
      get copyWith => _$EnumOpaqueTwinSseMoi_StructCopyWithImpl<
          EnumOpaqueTwinSseMoi_Struct>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.struct(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSseMoi_StructCopyWith<$Res>
    implements $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSseMoi_StructCopyWith(
          EnumOpaqueTwinSseMoi_Struct value,
          $Res Function(EnumOpaqueTwinSseMoi_Struct) _then) =
      _$EnumOpaqueTwinSseMoi_StructCopyWithImpl;
  @useResult
  $Res call({HideDataTwinSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSseMoi_StructCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSseMoi_StructCopyWith<$Res> {
  _$EnumOpaqueTwinSseMoi_StructCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSseMoi_Struct _self;
  final $Res Function(EnumOpaqueTwinSseMoi_Struct) _then;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSseMoi_Struct(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideDataTwinSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_Primitive extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_Primitive(this.field0) : super._();

  final I16 field0;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSseMoi_PrimitiveCopyWith<EnumOpaqueTwinSseMoi_Primitive>
      get copyWith => _$EnumOpaqueTwinSseMoi_PrimitiveCopyWithImpl<
          EnumOpaqueTwinSseMoi_Primitive>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.primitive(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSseMoi_PrimitiveCopyWith<$Res>
    implements $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSseMoi_PrimitiveCopyWith(
          EnumOpaqueTwinSseMoi_Primitive value,
          $Res Function(EnumOpaqueTwinSseMoi_Primitive) _then) =
      _$EnumOpaqueTwinSseMoi_PrimitiveCopyWithImpl;
  @useResult
  $Res call({I16 field0});
}

/// @nodoc
class _$EnumOpaqueTwinSseMoi_PrimitiveCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSseMoi_PrimitiveCopyWith<$Res> {
  _$EnumOpaqueTwinSseMoi_PrimitiveCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSseMoi_Primitive _self;
  final $Res Function(EnumOpaqueTwinSseMoi_Primitive) _then;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSseMoi_Primitive(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I16,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_TraitObj extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_TraitObj(this.field0) : super._();

  final BoxDartDebugTwinSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSseMoi_TraitObjCopyWith<EnumOpaqueTwinSseMoi_TraitObj>
      get copyWith => _$EnumOpaqueTwinSseMoi_TraitObjCopyWithImpl<
          EnumOpaqueTwinSseMoi_TraitObj>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.traitObj(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSseMoi_TraitObjCopyWith<$Res>
    implements $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSseMoi_TraitObjCopyWith(
          EnumOpaqueTwinSseMoi_TraitObj value,
          $Res Function(EnumOpaqueTwinSseMoi_TraitObj) _then) =
      _$EnumOpaqueTwinSseMoi_TraitObjCopyWithImpl;
  @useResult
  $Res call({BoxDartDebugTwinSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSseMoi_TraitObjCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSseMoi_TraitObjCopyWith<$Res> {
  _$EnumOpaqueTwinSseMoi_TraitObjCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSseMoi_TraitObj _self;
  final $Res Function(EnumOpaqueTwinSseMoi_TraitObj) _then;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSseMoi_TraitObj(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebugTwinSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_Mutex extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_Mutex(this.field0) : super._();

  final MutexHideDataTwinSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSseMoi_MutexCopyWith<EnumOpaqueTwinSseMoi_Mutex>
      get copyWith =>
          _$EnumOpaqueTwinSseMoi_MutexCopyWithImpl<EnumOpaqueTwinSseMoi_Mutex>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.mutex(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSseMoi_MutexCopyWith<$Res>
    implements $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSseMoi_MutexCopyWith(EnumOpaqueTwinSseMoi_Mutex value,
          $Res Function(EnumOpaqueTwinSseMoi_Mutex) _then) =
      _$EnumOpaqueTwinSseMoi_MutexCopyWithImpl;
  @useResult
  $Res call({MutexHideDataTwinSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSseMoi_MutexCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSseMoi_MutexCopyWith<$Res> {
  _$EnumOpaqueTwinSseMoi_MutexCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSseMoi_Mutex _self;
  final $Res Function(EnumOpaqueTwinSseMoi_Mutex) _then;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSseMoi_Mutex(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideDataTwinSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_RwLock extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_RwLock(this.field0) : super._();

  final RwLockHideDataTwinSseMoi field0;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $EnumOpaqueTwinSseMoi_RwLockCopyWith<EnumOpaqueTwinSseMoi_RwLock>
      get copyWith => _$EnumOpaqueTwinSseMoi_RwLockCopyWithImpl<
          EnumOpaqueTwinSseMoi_RwLock>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.rwLock(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $EnumOpaqueTwinSseMoi_RwLockCopyWith<$Res>
    implements $EnumOpaqueTwinSseMoiCopyWith<$Res> {
  factory $EnumOpaqueTwinSseMoi_RwLockCopyWith(
          EnumOpaqueTwinSseMoi_RwLock value,
          $Res Function(EnumOpaqueTwinSseMoi_RwLock) _then) =
      _$EnumOpaqueTwinSseMoi_RwLockCopyWithImpl;
  @useResult
  $Res call({RwLockHideDataTwinSseMoi field0});
}

/// @nodoc
class _$EnumOpaqueTwinSseMoi_RwLockCopyWithImpl<$Res>
    implements $EnumOpaqueTwinSseMoi_RwLockCopyWith<$Res> {
  _$EnumOpaqueTwinSseMoi_RwLockCopyWithImpl(this._self, this._then);

  final EnumOpaqueTwinSseMoi_RwLock _self;
  final $Res Function(EnumOpaqueTwinSseMoi_RwLock) _then;

  /// Create a copy of EnumOpaqueTwinSseMoi
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(EnumOpaqueTwinSseMoi_RwLock(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideDataTwinSseMoi,
    ));
  }
}

/// @nodoc

class EnumOpaqueTwinSseMoi_Nothing extends EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi_Nothing() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is EnumOpaqueTwinSseMoi_Nothing);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'EnumOpaqueTwinSseMoi.nothing()';
  }
}

// dart format on
