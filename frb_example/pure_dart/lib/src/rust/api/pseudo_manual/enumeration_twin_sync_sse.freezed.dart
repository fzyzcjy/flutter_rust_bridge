// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'enumeration_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$ChangeStringTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String data) created,
    required TResult Function(String id, String data) updated,
    required TResult Function(String id) deleted,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String data)? created,
    TResult? Function(String id, String data)? updated,
    TResult? Function(String id)? deleted,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String data)? created,
    TResult Function(String id, String data)? updated,
    TResult Function(String id)? deleted,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ChangeStringTwinSyncSse_Created value) created,
    required TResult Function(ChangeStringTwinSyncSse_Updated value) updated,
    required TResult Function(ChangeStringTwinSyncSse_Deleted value) deleted,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult? Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult? Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ChangeStringTwinSyncSseCopyWith<$Res> {
  factory $ChangeStringTwinSyncSseCopyWith(ChangeStringTwinSyncSse value,
          $Res Function(ChangeStringTwinSyncSse) then) =
      _$ChangeStringTwinSyncSseCopyWithImpl<$Res, ChangeStringTwinSyncSse>;
}

/// @nodoc
class _$ChangeStringTwinSyncSseCopyWithImpl<$Res,
        $Val extends ChangeStringTwinSyncSse>
    implements $ChangeStringTwinSyncSseCopyWith<$Res> {
  _$ChangeStringTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ChangeStringTwinSyncSse_CreatedImplCopyWith<$Res> {
  factory _$$ChangeStringTwinSyncSse_CreatedImplCopyWith(
          _$ChangeStringTwinSyncSse_CreatedImpl value,
          $Res Function(_$ChangeStringTwinSyncSse_CreatedImpl) then) =
      __$$ChangeStringTwinSyncSse_CreatedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String data});
}

/// @nodoc
class __$$ChangeStringTwinSyncSse_CreatedImplCopyWithImpl<$Res>
    extends _$ChangeStringTwinSyncSseCopyWithImpl<$Res,
        _$ChangeStringTwinSyncSse_CreatedImpl>
    implements _$$ChangeStringTwinSyncSse_CreatedImplCopyWith<$Res> {
  __$$ChangeStringTwinSyncSse_CreatedImplCopyWithImpl(
      _$ChangeStringTwinSyncSse_CreatedImpl _value,
      $Res Function(_$ChangeStringTwinSyncSse_CreatedImpl) _then)
      : super(_value, _then);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$ChangeStringTwinSyncSse_CreatedImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ChangeStringTwinSyncSse_CreatedImpl
    extends ChangeStringTwinSyncSse_Created {
  const _$ChangeStringTwinSyncSse_CreatedImpl({required this.data}) : super._();

  @override
  final String data;

  @override
  String toString() {
    return 'ChangeStringTwinSyncSse.created(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ChangeStringTwinSyncSse_CreatedImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ChangeStringTwinSyncSse_CreatedImplCopyWith<
          _$ChangeStringTwinSyncSse_CreatedImpl>
      get copyWith => __$$ChangeStringTwinSyncSse_CreatedImplCopyWithImpl<
          _$ChangeStringTwinSyncSse_CreatedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String data) created,
    required TResult Function(String id, String data) updated,
    required TResult Function(String id) deleted,
  }) {
    return created(data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String data)? created,
    TResult? Function(String id, String data)? updated,
    TResult? Function(String id)? deleted,
  }) {
    return created?.call(data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String data)? created,
    TResult Function(String id, String data)? updated,
    TResult Function(String id)? deleted,
    required TResult orElse(),
  }) {
    if (created != null) {
      return created(data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ChangeStringTwinSyncSse_Created value) created,
    required TResult Function(ChangeStringTwinSyncSse_Updated value) updated,
    required TResult Function(ChangeStringTwinSyncSse_Deleted value) deleted,
  }) {
    return created(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult? Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult? Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
  }) {
    return created?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
    required TResult orElse(),
  }) {
    if (created != null) {
      return created(this);
    }
    return orElse();
  }
}

abstract class ChangeStringTwinSyncSse_Created extends ChangeStringTwinSyncSse {
  const factory ChangeStringTwinSyncSse_Created({required final String data}) =
      _$ChangeStringTwinSyncSse_CreatedImpl;
  const ChangeStringTwinSyncSse_Created._() : super._();

  String get data;

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ChangeStringTwinSyncSse_CreatedImplCopyWith<
          _$ChangeStringTwinSyncSse_CreatedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ChangeStringTwinSyncSse_UpdatedImplCopyWith<$Res> {
  factory _$$ChangeStringTwinSyncSse_UpdatedImplCopyWith(
          _$ChangeStringTwinSyncSse_UpdatedImpl value,
          $Res Function(_$ChangeStringTwinSyncSse_UpdatedImpl) then) =
      __$$ChangeStringTwinSyncSse_UpdatedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String id, String data});
}

/// @nodoc
class __$$ChangeStringTwinSyncSse_UpdatedImplCopyWithImpl<$Res>
    extends _$ChangeStringTwinSyncSseCopyWithImpl<$Res,
        _$ChangeStringTwinSyncSse_UpdatedImpl>
    implements _$$ChangeStringTwinSyncSse_UpdatedImplCopyWith<$Res> {
  __$$ChangeStringTwinSyncSse_UpdatedImplCopyWithImpl(
      _$ChangeStringTwinSyncSse_UpdatedImpl _value,
      $Res Function(_$ChangeStringTwinSyncSse_UpdatedImpl) _then)
      : super(_value, _then);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? data = null,
  }) {
    return _then(_$ChangeStringTwinSyncSse_UpdatedImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ChangeStringTwinSyncSse_UpdatedImpl
    extends ChangeStringTwinSyncSse_Updated {
  const _$ChangeStringTwinSyncSse_UpdatedImpl(
      {required this.id, required this.data})
      : super._();

  @override
  final String id;
  @override
  final String data;

  @override
  String toString() {
    return 'ChangeStringTwinSyncSse.updated(id: $id, data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ChangeStringTwinSyncSse_UpdatedImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, data);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ChangeStringTwinSyncSse_UpdatedImplCopyWith<
          _$ChangeStringTwinSyncSse_UpdatedImpl>
      get copyWith => __$$ChangeStringTwinSyncSse_UpdatedImplCopyWithImpl<
          _$ChangeStringTwinSyncSse_UpdatedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String data) created,
    required TResult Function(String id, String data) updated,
    required TResult Function(String id) deleted,
  }) {
    return updated(id, data);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String data)? created,
    TResult? Function(String id, String data)? updated,
    TResult? Function(String id)? deleted,
  }) {
    return updated?.call(id, data);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String data)? created,
    TResult Function(String id, String data)? updated,
    TResult Function(String id)? deleted,
    required TResult orElse(),
  }) {
    if (updated != null) {
      return updated(id, data);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ChangeStringTwinSyncSse_Created value) created,
    required TResult Function(ChangeStringTwinSyncSse_Updated value) updated,
    required TResult Function(ChangeStringTwinSyncSse_Deleted value) deleted,
  }) {
    return updated(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult? Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult? Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
  }) {
    return updated?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
    required TResult orElse(),
  }) {
    if (updated != null) {
      return updated(this);
    }
    return orElse();
  }
}

abstract class ChangeStringTwinSyncSse_Updated extends ChangeStringTwinSyncSse {
  const factory ChangeStringTwinSyncSse_Updated(
      {required final String id,
      required final String data}) = _$ChangeStringTwinSyncSse_UpdatedImpl;
  const ChangeStringTwinSyncSse_Updated._() : super._();

  String get id;
  String get data;

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ChangeStringTwinSyncSse_UpdatedImplCopyWith<
          _$ChangeStringTwinSyncSse_UpdatedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ChangeStringTwinSyncSse_DeletedImplCopyWith<$Res> {
  factory _$$ChangeStringTwinSyncSse_DeletedImplCopyWith(
          _$ChangeStringTwinSyncSse_DeletedImpl value,
          $Res Function(_$ChangeStringTwinSyncSse_DeletedImpl) then) =
      __$$ChangeStringTwinSyncSse_DeletedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String id});
}

/// @nodoc
class __$$ChangeStringTwinSyncSse_DeletedImplCopyWithImpl<$Res>
    extends _$ChangeStringTwinSyncSseCopyWithImpl<$Res,
        _$ChangeStringTwinSyncSse_DeletedImpl>
    implements _$$ChangeStringTwinSyncSse_DeletedImplCopyWith<$Res> {
  __$$ChangeStringTwinSyncSse_DeletedImplCopyWithImpl(
      _$ChangeStringTwinSyncSse_DeletedImpl _value,
      $Res Function(_$ChangeStringTwinSyncSse_DeletedImpl) _then)
      : super(_value, _then);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
  }) {
    return _then(_$ChangeStringTwinSyncSse_DeletedImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ChangeStringTwinSyncSse_DeletedImpl
    extends ChangeStringTwinSyncSse_Deleted {
  const _$ChangeStringTwinSyncSse_DeletedImpl({required this.id}) : super._();

  @override
  final String id;

  @override
  String toString() {
    return 'ChangeStringTwinSyncSse.deleted(id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ChangeStringTwinSyncSse_DeletedImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ChangeStringTwinSyncSse_DeletedImplCopyWith<
          _$ChangeStringTwinSyncSse_DeletedImpl>
      get copyWith => __$$ChangeStringTwinSyncSse_DeletedImplCopyWithImpl<
          _$ChangeStringTwinSyncSse_DeletedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String data) created,
    required TResult Function(String id, String data) updated,
    required TResult Function(String id) deleted,
  }) {
    return deleted(id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String data)? created,
    TResult? Function(String id, String data)? updated,
    TResult? Function(String id)? deleted,
  }) {
    return deleted?.call(id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String data)? created,
    TResult Function(String id, String data)? updated,
    TResult Function(String id)? deleted,
    required TResult orElse(),
  }) {
    if (deleted != null) {
      return deleted(id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ChangeStringTwinSyncSse_Created value) created,
    required TResult Function(ChangeStringTwinSyncSse_Updated value) updated,
    required TResult Function(ChangeStringTwinSyncSse_Deleted value) deleted,
  }) {
    return deleted(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult? Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult? Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
  }) {
    return deleted?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ChangeStringTwinSyncSse_Created value)? created,
    TResult Function(ChangeStringTwinSyncSse_Updated value)? updated,
    TResult Function(ChangeStringTwinSyncSse_Deleted value)? deleted,
    required TResult orElse(),
  }) {
    if (deleted != null) {
      return deleted(this);
    }
    return orElse();
  }
}

abstract class ChangeStringTwinSyncSse_Deleted extends ChangeStringTwinSyncSse {
  const factory ChangeStringTwinSyncSse_Deleted({required final String id}) =
      _$ChangeStringTwinSyncSse_DeletedImpl;
  const ChangeStringTwinSyncSse_Deleted._() : super._();

  String get id;

  /// Create a copy of ChangeStringTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ChangeStringTwinSyncSse_DeletedImplCopyWith<
          _$ChangeStringTwinSyncSse_DeletedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$DistanceTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSyncSse_Map value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSyncSse_Map value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSyncSse_Map value)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DistanceTwinSyncSseCopyWith<$Res> {
  factory $DistanceTwinSyncSseCopyWith(
          DistanceTwinSyncSse value, $Res Function(DistanceTwinSyncSse) then) =
      _$DistanceTwinSyncSseCopyWithImpl<$Res, DistanceTwinSyncSse>;
}

/// @nodoc
class _$DistanceTwinSyncSseCopyWithImpl<$Res, $Val extends DistanceTwinSyncSse>
    implements $DistanceTwinSyncSseCopyWith<$Res> {
  _$DistanceTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DistanceTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$DistanceTwinSyncSse_UnknownImplCopyWith<$Res> {
  factory _$$DistanceTwinSyncSse_UnknownImplCopyWith(
          _$DistanceTwinSyncSse_UnknownImpl value,
          $Res Function(_$DistanceTwinSyncSse_UnknownImpl) then) =
      __$$DistanceTwinSyncSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DistanceTwinSyncSse_UnknownImplCopyWithImpl<$Res>
    extends _$DistanceTwinSyncSseCopyWithImpl<$Res,
        _$DistanceTwinSyncSse_UnknownImpl>
    implements _$$DistanceTwinSyncSse_UnknownImplCopyWith<$Res> {
  __$$DistanceTwinSyncSse_UnknownImplCopyWithImpl(
      _$DistanceTwinSyncSse_UnknownImpl _value,
      $Res Function(_$DistanceTwinSyncSse_UnknownImpl) _then)
      : super(_value, _then);

  /// Create a copy of DistanceTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$DistanceTwinSyncSse_UnknownImpl extends DistanceTwinSyncSse_Unknown {
  const _$DistanceTwinSyncSse_UnknownImpl() : super._();

  @override
  String toString() {
    return 'DistanceTwinSyncSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinSyncSse_UnknownImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSyncSse_Map value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSyncSse_Map value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSyncSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class DistanceTwinSyncSse_Unknown extends DistanceTwinSyncSse {
  const factory DistanceTwinSyncSse_Unknown() =
      _$DistanceTwinSyncSse_UnknownImpl;
  const DistanceTwinSyncSse_Unknown._() : super._();
}

/// @nodoc
abstract class _$$DistanceTwinSyncSse_MapImplCopyWith<$Res> {
  factory _$$DistanceTwinSyncSse_MapImplCopyWith(
          _$DistanceTwinSyncSse_MapImpl value,
          $Res Function(_$DistanceTwinSyncSse_MapImpl) then) =
      __$$DistanceTwinSyncSse_MapImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$DistanceTwinSyncSse_MapImplCopyWithImpl<$Res>
    extends _$DistanceTwinSyncSseCopyWithImpl<$Res,
        _$DistanceTwinSyncSse_MapImpl>
    implements _$$DistanceTwinSyncSse_MapImplCopyWith<$Res> {
  __$$DistanceTwinSyncSse_MapImplCopyWithImpl(
      _$DistanceTwinSyncSse_MapImpl _value,
      $Res Function(_$DistanceTwinSyncSse_MapImpl) _then)
      : super(_value, _then);

  /// Create a copy of DistanceTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DistanceTwinSyncSse_MapImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$DistanceTwinSyncSse_MapImpl extends DistanceTwinSyncSse_Map {
  const _$DistanceTwinSyncSse_MapImpl(this.field0) : super._();

  @override
  final double field0;

  @override
  String toString() {
    return 'DistanceTwinSyncSse.map(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DistanceTwinSyncSse_MapImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of DistanceTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DistanceTwinSyncSse_MapImplCopyWith<_$DistanceTwinSyncSse_MapImpl>
      get copyWith => __$$DistanceTwinSyncSse_MapImplCopyWithImpl<
          _$DistanceTwinSyncSse_MapImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return map(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
  }) {
    return map?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DistanceTwinSyncSse_Unknown value) unknown,
    required TResult Function(DistanceTwinSyncSse_Map value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult? Function(DistanceTwinSyncSse_Map value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DistanceTwinSyncSse_Unknown value)? unknown,
    TResult Function(DistanceTwinSyncSse_Map value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }
}

abstract class DistanceTwinSyncSse_Map extends DistanceTwinSyncSse {
  const factory DistanceTwinSyncSse_Map(final double field0) =
      _$DistanceTwinSyncSse_MapImpl;
  const DistanceTwinSyncSse_Map._() : super._();

  double get field0;

  /// Create a copy of DistanceTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DistanceTwinSyncSse_MapImplCopyWith<_$DistanceTwinSyncSse_MapImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumWithItemMixedTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSyncSse_C value) c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSyncSse_C value)? c,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSyncSse_C value)? c,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemMixedTwinSyncSseCopyWith<$Res> {
  factory $EnumWithItemMixedTwinSyncSseCopyWith(
          EnumWithItemMixedTwinSyncSse value,
          $Res Function(EnumWithItemMixedTwinSyncSse) then) =
      _$EnumWithItemMixedTwinSyncSseCopyWithImpl<$Res,
          EnumWithItemMixedTwinSyncSse>;
}

/// @nodoc
class _$EnumWithItemMixedTwinSyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemMixedTwinSyncSse>
    implements $EnumWithItemMixedTwinSyncSseCopyWith<$Res> {
  _$EnumWithItemMixedTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSyncSse_AImplCopyWith(
          _$EnumWithItemMixedTwinSyncSse_AImpl value,
          $Res Function(_$EnumWithItemMixedTwinSyncSse_AImpl) then) =
      __$$EnumWithItemMixedTwinSyncSse_AImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$EnumWithItemMixedTwinSyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSyncSse_AImpl>
    implements _$$EnumWithItemMixedTwinSyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSyncSse_AImplCopyWithImpl(
      _$EnumWithItemMixedTwinSyncSse_AImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSyncSse_AImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$EnumWithItemMixedTwinSyncSse_AImpl
    extends EnumWithItemMixedTwinSyncSse_A {
  const _$EnumWithItemMixedTwinSyncSse_AImpl() : super._();

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSyncSse.a()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSyncSse_AImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return a();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return a?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSyncSse_C value) c,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSyncSse_C value)? c,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemMixedTwinSyncSse_A
    extends EnumWithItemMixedTwinSyncSse {
  const factory EnumWithItemMixedTwinSyncSse_A() =
      _$EnumWithItemMixedTwinSyncSse_AImpl;
  const EnumWithItemMixedTwinSyncSse_A._() : super._();
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSyncSse_BImplCopyWith(
          _$EnumWithItemMixedTwinSyncSse_BImpl value,
          $Res Function(_$EnumWithItemMixedTwinSyncSse_BImpl) then) =
      __$$EnumWithItemMixedTwinSyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemMixedTwinSyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSyncSse_BImpl>
    implements _$$EnumWithItemMixedTwinSyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSyncSse_BImplCopyWithImpl(
      _$EnumWithItemMixedTwinSyncSse_BImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSyncSse_BImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemMixedTwinSyncSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$EnumWithItemMixedTwinSyncSse_BImpl
    extends EnumWithItemMixedTwinSyncSse_B {
  const _$EnumWithItemMixedTwinSyncSse_BImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSyncSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSyncSse_BImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinSyncSse_BImplCopyWith<
          _$EnumWithItemMixedTwinSyncSse_BImpl>
      get copyWith => __$$EnumWithItemMixedTwinSyncSse_BImplCopyWithImpl<
          _$EnumWithItemMixedTwinSyncSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSyncSse_C value) c,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSyncSse_C value)? c,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemMixedTwinSyncSse_B
    extends EnumWithItemMixedTwinSyncSse {
  const factory EnumWithItemMixedTwinSyncSse_B(final Uint8List field0) =
      _$EnumWithItemMixedTwinSyncSse_BImpl;
  const EnumWithItemMixedTwinSyncSse_B._() : super._();

  Uint8List get field0;

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemMixedTwinSyncSse_BImplCopyWith<
          _$EnumWithItemMixedTwinSyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemMixedTwinSyncSse_CImplCopyWith<$Res> {
  factory _$$EnumWithItemMixedTwinSyncSse_CImplCopyWith(
          _$EnumWithItemMixedTwinSyncSse_CImpl value,
          $Res Function(_$EnumWithItemMixedTwinSyncSse_CImpl) then) =
      __$$EnumWithItemMixedTwinSyncSse_CImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String cField});
}

/// @nodoc
class __$$EnumWithItemMixedTwinSyncSse_CImplCopyWithImpl<$Res>
    extends _$EnumWithItemMixedTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemMixedTwinSyncSse_CImpl>
    implements _$$EnumWithItemMixedTwinSyncSse_CImplCopyWith<$Res> {
  __$$EnumWithItemMixedTwinSyncSse_CImplCopyWithImpl(
      _$EnumWithItemMixedTwinSyncSse_CImpl _value,
      $Res Function(_$EnumWithItemMixedTwinSyncSse_CImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? cField = null,
  }) {
    return _then(_$EnumWithItemMixedTwinSyncSse_CImpl(
      cField: null == cField
          ? _value.cField
          : cField // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$EnumWithItemMixedTwinSyncSse_CImpl
    extends EnumWithItemMixedTwinSyncSse_C {
  const _$EnumWithItemMixedTwinSyncSse_CImpl({required this.cField})
      : super._();

  @override
  final String cField;

  @override
  String toString() {
    return 'EnumWithItemMixedTwinSyncSse.c(cField: $cField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemMixedTwinSyncSse_CImpl &&
            (identical(other.cField, cField) || other.cField == cField));
  }

  @override
  int get hashCode => Object.hash(runtimeType, cField);

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemMixedTwinSyncSse_CImplCopyWith<
          _$EnumWithItemMixedTwinSyncSse_CImpl>
      get copyWith => __$$EnumWithItemMixedTwinSyncSse_CImplCopyWithImpl<
          _$EnumWithItemMixedTwinSyncSse_CImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() a,
    required TResult Function(Uint8List field0) b,
    required TResult Function(String cField) c,
  }) {
    return c(cField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? a,
    TResult? Function(Uint8List field0)? b,
    TResult? Function(String cField)? c,
  }) {
    return c?.call(cField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? a,
    TResult Function(Uint8List field0)? b,
    TResult Function(String cField)? c,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(cField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemMixedTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemMixedTwinSyncSse_B value) b,
    required TResult Function(EnumWithItemMixedTwinSyncSse_C value) c,
  }) {
    return c(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult? Function(EnumWithItemMixedTwinSyncSse_C value)? c,
  }) {
    return c?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemMixedTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemMixedTwinSyncSse_B value)? b,
    TResult Function(EnumWithItemMixedTwinSyncSse_C value)? c,
    required TResult orElse(),
  }) {
    if (c != null) {
      return c(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemMixedTwinSyncSse_C
    extends EnumWithItemMixedTwinSyncSse {
  const factory EnumWithItemMixedTwinSyncSse_C({required final String cField}) =
      _$EnumWithItemMixedTwinSyncSse_CImpl;
  const EnumWithItemMixedTwinSyncSse_C._() : super._();

  String get cField;

  /// Create a copy of EnumWithItemMixedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemMixedTwinSyncSse_CImplCopyWith<
          _$EnumWithItemMixedTwinSyncSse_CImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumWithItemStructTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSyncSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSyncSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemStructTwinSyncSseCopyWith<$Res> {
  factory $EnumWithItemStructTwinSyncSseCopyWith(
          EnumWithItemStructTwinSyncSse value,
          $Res Function(EnumWithItemStructTwinSyncSse) then) =
      _$EnumWithItemStructTwinSyncSseCopyWithImpl<$Res,
          EnumWithItemStructTwinSyncSse>;
}

/// @nodoc
class _$EnumWithItemStructTwinSyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemStructTwinSyncSse>
    implements $EnumWithItemStructTwinSyncSseCopyWith<$Res> {
  _$EnumWithItemStructTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinSyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinSyncSse_AImplCopyWith(
          _$EnumWithItemStructTwinSyncSse_AImpl value,
          $Res Function(_$EnumWithItemStructTwinSyncSse_AImpl) then) =
      __$$EnumWithItemStructTwinSyncSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List aField});
}

/// @nodoc
class __$$EnumWithItemStructTwinSyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinSyncSse_AImpl>
    implements _$$EnumWithItemStructTwinSyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinSyncSse_AImplCopyWithImpl(
      _$EnumWithItemStructTwinSyncSse_AImpl _value,
      $Res Function(_$EnumWithItemStructTwinSyncSse_AImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? aField = null,
  }) {
    return _then(_$EnumWithItemStructTwinSyncSse_AImpl(
      aField: null == aField
          ? _value.aField
          : aField // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$EnumWithItemStructTwinSyncSse_AImpl
    extends EnumWithItemStructTwinSyncSse_A {
  const _$EnumWithItemStructTwinSyncSse_AImpl({required this.aField})
      : super._();

  @override
  final Uint8List aField;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSyncSse.a(aField: $aField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinSyncSse_AImpl &&
            const DeepCollectionEquality().equals(other.aField, aField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(aField));

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinSyncSse_AImplCopyWith<
          _$EnumWithItemStructTwinSyncSse_AImpl>
      get copyWith => __$$EnumWithItemStructTwinSyncSse_AImplCopyWithImpl<
          _$EnumWithItemStructTwinSyncSse_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) {
    return a(aField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) {
    return a?.call(aField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(aField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSyncSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSyncSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemStructTwinSyncSse_A
    extends EnumWithItemStructTwinSyncSse {
  const factory EnumWithItemStructTwinSyncSse_A(
          {required final Uint8List aField}) =
      _$EnumWithItemStructTwinSyncSse_AImpl;
  const EnumWithItemStructTwinSyncSse_A._() : super._();

  Uint8List get aField;

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemStructTwinSyncSse_AImplCopyWith<
          _$EnumWithItemStructTwinSyncSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemStructTwinSyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemStructTwinSyncSse_BImplCopyWith(
          _$EnumWithItemStructTwinSyncSse_BImpl value,
          $Res Function(_$EnumWithItemStructTwinSyncSse_BImpl) then) =
      __$$EnumWithItemStructTwinSyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Int32List bField});
}

/// @nodoc
class __$$EnumWithItemStructTwinSyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemStructTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemStructTwinSyncSse_BImpl>
    implements _$$EnumWithItemStructTwinSyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemStructTwinSyncSse_BImplCopyWithImpl(
      _$EnumWithItemStructTwinSyncSse_BImpl _value,
      $Res Function(_$EnumWithItemStructTwinSyncSse_BImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? bField = null,
  }) {
    return _then(_$EnumWithItemStructTwinSyncSse_BImpl(
      bField: null == bField
          ? _value.bField
          : bField // ignore: cast_nullable_to_non_nullable
              as Int32List,
    ));
  }
}

/// @nodoc

class _$EnumWithItemStructTwinSyncSse_BImpl
    extends EnumWithItemStructTwinSyncSse_B {
  const _$EnumWithItemStructTwinSyncSse_BImpl({required this.bField})
      : super._();

  @override
  final Int32List bField;

  @override
  String toString() {
    return 'EnumWithItemStructTwinSyncSse.b(bField: $bField)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemStructTwinSyncSse_BImpl &&
            const DeepCollectionEquality().equals(other.bField, bField));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(bField));

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemStructTwinSyncSse_BImplCopyWith<
          _$EnumWithItemStructTwinSyncSse_BImpl>
      get copyWith => __$$EnumWithItemStructTwinSyncSse_BImplCopyWithImpl<
          _$EnumWithItemStructTwinSyncSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List aField) a,
    required TResult Function(Int32List bField) b,
  }) {
    return b(bField);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List aField)? a,
    TResult? Function(Int32List bField)? b,
  }) {
    return b?.call(bField);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List aField)? a,
    TResult Function(Int32List bField)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(bField);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemStructTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemStructTwinSyncSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemStructTwinSyncSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemStructTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemStructTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemStructTwinSyncSse_B
    extends EnumWithItemStructTwinSyncSse {
  const factory EnumWithItemStructTwinSyncSse_B(
          {required final Int32List bField}) =
      _$EnumWithItemStructTwinSyncSse_BImpl;
  const EnumWithItemStructTwinSyncSse_B._() : super._();

  Int32List get bField;

  /// Create a copy of EnumWithItemStructTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemStructTwinSyncSse_BImplCopyWith<
          _$EnumWithItemStructTwinSyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumWithItemTupleTwinSyncSse {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(int field0) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(int field0)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSyncSse_B value) b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSyncSse_B value)? b,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumWithItemTupleTwinSyncSseCopyWith<$Res> {
  factory $EnumWithItemTupleTwinSyncSseCopyWith(
          EnumWithItemTupleTwinSyncSse value,
          $Res Function(EnumWithItemTupleTwinSyncSse) then) =
      _$EnumWithItemTupleTwinSyncSseCopyWithImpl<$Res,
          EnumWithItemTupleTwinSyncSse>;
}

/// @nodoc
class _$EnumWithItemTupleTwinSyncSseCopyWithImpl<$Res,
        $Val extends EnumWithItemTupleTwinSyncSse>
    implements $EnumWithItemTupleTwinSyncSseCopyWith<$Res> {
  _$EnumWithItemTupleTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinSyncSse_AImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinSyncSse_AImplCopyWith(
          _$EnumWithItemTupleTwinSyncSse_AImpl value,
          $Res Function(_$EnumWithItemTupleTwinSyncSse_AImpl) then) =
      __$$EnumWithItemTupleTwinSyncSse_AImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinSyncSse_AImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinSyncSse_AImpl>
    implements _$$EnumWithItemTupleTwinSyncSse_AImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinSyncSse_AImplCopyWithImpl(
      _$EnumWithItemTupleTwinSyncSse_AImpl _value,
      $Res Function(_$EnumWithItemTupleTwinSyncSse_AImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinSyncSse_AImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$EnumWithItemTupleTwinSyncSse_AImpl
    extends EnumWithItemTupleTwinSyncSse_A {
  const _$EnumWithItemTupleTwinSyncSse_AImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSyncSse.a(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinSyncSse_AImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinSyncSse_AImplCopyWith<
          _$EnumWithItemTupleTwinSyncSse_AImpl>
      get copyWith => __$$EnumWithItemTupleTwinSyncSse_AImplCopyWithImpl<
          _$EnumWithItemTupleTwinSyncSse_AImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(int field0) b,
  }) {
    return a(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(int field0)? b,
  }) {
    return a?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSyncSse_B value) b,
  }) {
    return a(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSyncSse_B value)? b,
  }) {
    return a?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (a != null) {
      return a(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemTupleTwinSyncSse_A
    extends EnumWithItemTupleTwinSyncSse {
  const factory EnumWithItemTupleTwinSyncSse_A(final Uint8List field0) =
      _$EnumWithItemTupleTwinSyncSse_AImpl;
  const EnumWithItemTupleTwinSyncSse_A._() : super._();

  @override
  Uint8List get field0;

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemTupleTwinSyncSse_AImplCopyWith<
          _$EnumWithItemTupleTwinSyncSse_AImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumWithItemTupleTwinSyncSse_BImplCopyWith<$Res> {
  factory _$$EnumWithItemTupleTwinSyncSse_BImplCopyWith(
          _$EnumWithItemTupleTwinSyncSse_BImpl value,
          $Res Function(_$EnumWithItemTupleTwinSyncSse_BImpl) then) =
      __$$EnumWithItemTupleTwinSyncSse_BImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$EnumWithItemTupleTwinSyncSse_BImplCopyWithImpl<$Res>
    extends _$EnumWithItemTupleTwinSyncSseCopyWithImpl<$Res,
        _$EnumWithItemTupleTwinSyncSse_BImpl>
    implements _$$EnumWithItemTupleTwinSyncSse_BImplCopyWith<$Res> {
  __$$EnumWithItemTupleTwinSyncSse_BImplCopyWithImpl(
      _$EnumWithItemTupleTwinSyncSse_BImpl _value,
      $Res Function(_$EnumWithItemTupleTwinSyncSse_BImpl) _then)
      : super(_value, _then);

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumWithItemTupleTwinSyncSse_BImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$EnumWithItemTupleTwinSyncSse_BImpl
    extends EnumWithItemTupleTwinSyncSse_B {
  const _$EnumWithItemTupleTwinSyncSse_BImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'EnumWithItemTupleTwinSyncSse.b(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumWithItemTupleTwinSyncSse_BImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumWithItemTupleTwinSyncSse_BImplCopyWith<
          _$EnumWithItemTupleTwinSyncSse_BImpl>
      get copyWith => __$$EnumWithItemTupleTwinSyncSse_BImplCopyWithImpl<
          _$EnumWithItemTupleTwinSyncSse_BImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Uint8List field0) a,
    required TResult Function(int field0) b,
  }) {
    return b(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Uint8List field0)? a,
    TResult? Function(int field0)? b,
  }) {
    return b?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Uint8List field0)? a,
    TResult Function(int field0)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumWithItemTupleTwinSyncSse_A value) a,
    required TResult Function(EnumWithItemTupleTwinSyncSse_B value) b,
  }) {
    return b(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult? Function(EnumWithItemTupleTwinSyncSse_B value)? b,
  }) {
    return b?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumWithItemTupleTwinSyncSse_A value)? a,
    TResult Function(EnumWithItemTupleTwinSyncSse_B value)? b,
    required TResult orElse(),
  }) {
    if (b != null) {
      return b(this);
    }
    return orElse();
  }
}

abstract class EnumWithItemTupleTwinSyncSse_B
    extends EnumWithItemTupleTwinSyncSse {
  const factory EnumWithItemTupleTwinSyncSse_B(final int field0) =
      _$EnumWithItemTupleTwinSyncSse_BImpl;
  const EnumWithItemTupleTwinSyncSse_B._() : super._();

  @override
  int get field0;

  /// Create a copy of EnumWithItemTupleTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EnumWithItemTupleTwinSyncSse_BImplCopyWith<
          _$EnumWithItemTupleTwinSyncSse_BImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KitchenSinkTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkTwinSyncSseCopyWith<$Res> {
  factory $KitchenSinkTwinSyncSseCopyWith(KitchenSinkTwinSyncSse value,
          $Res Function(KitchenSinkTwinSyncSse) then) =
      _$KitchenSinkTwinSyncSseCopyWithImpl<$Res, KitchenSinkTwinSyncSse>;
}

/// @nodoc
class _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        $Val extends KitchenSinkTwinSyncSse>
    implements $KitchenSinkTwinSyncSseCopyWith<$Res> {
  _$KitchenSinkTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_EmptyImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_EmptyImplCopyWith(
          _$KitchenSinkTwinSyncSse_EmptyImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_EmptyImpl) then) =
      __$$KitchenSinkTwinSyncSse_EmptyImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_EmptyImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_EmptyImpl>
    implements _$$KitchenSinkTwinSyncSse_EmptyImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_EmptyImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_EmptyImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_EmptyImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_EmptyImpl extends KitchenSinkTwinSyncSse_Empty {
  const _$KitchenSinkTwinSyncSse_EmptyImpl() : super._();

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.empty()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_EmptyImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Empty extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Empty() =
      _$KitchenSinkTwinSyncSse_EmptyImpl;
  const KitchenSinkTwinSyncSse_Empty._() : super._();
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWith(
          _$KitchenSinkTwinSyncSse_PrimitivesImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_PrimitivesImpl) then) =
      __$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_PrimitivesImpl>
    implements _$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_PrimitivesImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_PrimitivesImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(_$KitchenSinkTwinSyncSse_PrimitivesImpl(
      int32: null == int32
          ? _value.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: null == float64
          ? _value.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: null == boolean
          ? _value.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_PrimitivesImpl
    extends KitchenSinkTwinSyncSse_Primitives {
  const _$KitchenSinkTwinSyncSse_PrimitivesImpl(
      {this.int32 = -1, required this.float64, required this.boolean})
      : super._();

  /// Dart field comment
  @override
  @JsonKey()
  final int int32;
  @override
  final double float64;
  @override
  final bool boolean;

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_PrimitivesImpl &&
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinSyncSse_PrimitivesImpl>
      get copyWith => __$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWithImpl<
          _$KitchenSinkTwinSyncSse_PrimitivesImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(int32, float64, boolean);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Primitives
    extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Primitives(
      {final int int32,
      required final double float64,
      required final bool boolean}) = _$KitchenSinkTwinSyncSse_PrimitivesImpl;
  const KitchenSinkTwinSyncSse_Primitives._() : super._();

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KitchenSinkTwinSyncSse_PrimitivesImplCopyWith<
          _$KitchenSinkTwinSyncSse_PrimitivesImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_NestedImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_NestedImplCopyWith(
          _$KitchenSinkTwinSyncSse_NestedImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_NestedImpl) then) =
      __$$KitchenSinkTwinSyncSse_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, KitchenSinkTwinSyncSse field1});

  $KitchenSinkTwinSyncSseCopyWith<$Res> get field1;
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_NestedImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_NestedImpl>
    implements _$$KitchenSinkTwinSyncSse_NestedImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_NestedImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_NestedImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_NestedImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$KitchenSinkTwinSyncSse_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSinkTwinSyncSse,
    ));
  }

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkTwinSyncSseCopyWith<$Res> get field1 {
    return $KitchenSinkTwinSyncSseCopyWith<$Res>(_value.field1, (value) {
      return _then(_value.copyWith(field1: value));
    });
  }
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_NestedImpl
    extends KitchenSinkTwinSyncSse_Nested {
  const _$KitchenSinkTwinSyncSse_NestedImpl(this.field0,
      [this.field1 = const KitchenSinkTwinSyncSse.empty()])
      : super._();

  @override
  final int field0;
  @override
  @JsonKey()
  final KitchenSinkTwinSyncSse field1;

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSyncSse_NestedImplCopyWith<
          _$KitchenSinkTwinSyncSse_NestedImpl>
      get copyWith => __$$KitchenSinkTwinSyncSse_NestedImplCopyWithImpl<
          _$KitchenSinkTwinSyncSse_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Nested extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Nested(final int field0,
          [final KitchenSinkTwinSyncSse field1]) =
      _$KitchenSinkTwinSyncSse_NestedImpl;
  const KitchenSinkTwinSyncSse_Nested._() : super._();

  int get field0;
  KitchenSinkTwinSyncSse get field1;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KitchenSinkTwinSyncSse_NestedImplCopyWith<
          _$KitchenSinkTwinSyncSse_NestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_OptionalImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_OptionalImplCopyWith(
          _$KitchenSinkTwinSyncSse_OptionalImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_OptionalImpl) then) =
      __$$KitchenSinkTwinSyncSse_OptionalImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_OptionalImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_OptionalImpl>
    implements _$$KitchenSinkTwinSyncSse_OptionalImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_OptionalImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_OptionalImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_OptionalImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSinkTwinSyncSse_OptionalImpl(
      freezed == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      freezed == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_OptionalImpl
    extends KitchenSinkTwinSyncSse_Optional {
  const _$KitchenSinkTwinSyncSse_OptionalImpl([this.field0 = -1, this.field1])
      : super._();

  /// Comment on anonymous field
  @override
  @JsonKey()
  final int? field0;
  @override
  final int? field1;

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.optional(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_OptionalImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSyncSse_OptionalImplCopyWith<
          _$KitchenSinkTwinSyncSse_OptionalImpl>
      get copyWith => __$$KitchenSinkTwinSyncSse_OptionalImplCopyWithImpl<
          _$KitchenSinkTwinSyncSse_OptionalImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Optional extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Optional(
      [final int? field0,
      final int? field1]) = _$KitchenSinkTwinSyncSse_OptionalImpl;
  const KitchenSinkTwinSyncSse_Optional._() : super._();

  /// Comment on anonymous field
  int? get field0;
  int? get field1;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KitchenSinkTwinSyncSse_OptionalImplCopyWith<
          _$KitchenSinkTwinSyncSse_OptionalImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_BufferImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_BufferImplCopyWith(
          _$KitchenSinkTwinSyncSse_BufferImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_BufferImpl) then) =
      __$$KitchenSinkTwinSyncSse_BufferImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_BufferImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_BufferImpl>
    implements _$$KitchenSinkTwinSyncSse_BufferImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_BufferImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_BufferImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_BufferImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinSyncSse_BufferImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_BufferImpl
    extends KitchenSinkTwinSyncSse_Buffer {
  const _$KitchenSinkTwinSyncSse_BufferImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.buffer(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_BufferImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSyncSse_BufferImplCopyWith<
          _$KitchenSinkTwinSyncSse_BufferImpl>
      get copyWith => __$$KitchenSinkTwinSyncSse_BufferImplCopyWithImpl<
          _$KitchenSinkTwinSyncSse_BufferImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Buffer extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Buffer(final Uint8List field0) =
      _$KitchenSinkTwinSyncSse_BufferImpl;
  const KitchenSinkTwinSyncSse_Buffer._() : super._();

  Uint8List get field0;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KitchenSinkTwinSyncSse_BufferImplCopyWith<
          _$KitchenSinkTwinSyncSse_BufferImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSinkTwinSyncSse_EnumsImplCopyWith<$Res> {
  factory _$$KitchenSinkTwinSyncSse_EnumsImplCopyWith(
          _$KitchenSinkTwinSyncSse_EnumsImpl value,
          $Res Function(_$KitchenSinkTwinSyncSse_EnumsImpl) then) =
      __$$KitchenSinkTwinSyncSse_EnumsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({WeekdaysTwinSyncSse field0});
}

/// @nodoc
class __$$KitchenSinkTwinSyncSse_EnumsImplCopyWithImpl<$Res>
    extends _$KitchenSinkTwinSyncSseCopyWithImpl<$Res,
        _$KitchenSinkTwinSyncSse_EnumsImpl>
    implements _$$KitchenSinkTwinSyncSse_EnumsImplCopyWith<$Res> {
  __$$KitchenSinkTwinSyncSse_EnumsImplCopyWithImpl(
      _$KitchenSinkTwinSyncSse_EnumsImpl _value,
      $Res Function(_$KitchenSinkTwinSyncSse_EnumsImpl) _then)
      : super(_value, _then);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSinkTwinSyncSse_EnumsImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WeekdaysTwinSyncSse,
    ));
  }
}

/// @nodoc

class _$KitchenSinkTwinSyncSse_EnumsImpl extends KitchenSinkTwinSyncSse_Enums {
  const _$KitchenSinkTwinSyncSse_EnumsImpl(
      [this.field0 = WeekdaysTwinSyncSse.sunday])
      : super._();

  @override
  @JsonKey()
  final WeekdaysTwinSyncSse field0;

  @override
  String toString() {
    return 'KitchenSinkTwinSyncSse.enums(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSinkTwinSyncSse_EnumsImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSinkTwinSyncSse_EnumsImplCopyWith<
          _$KitchenSinkTwinSyncSse_EnumsImpl>
      get copyWith => __$$KitchenSinkTwinSyncSse_EnumsImplCopyWithImpl<
          _$KitchenSinkTwinSyncSse_EnumsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
    required TResult Function(int field0, KitchenSinkTwinSyncSse field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(WeekdaysTwinSyncSse field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(WeekdaysTwinSyncSse field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSinkTwinSyncSse field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(WeekdaysTwinSyncSse field0)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSinkTwinSyncSse_Empty value) empty,
    required TResult Function(KitchenSinkTwinSyncSse_Primitives value)
        primitives,
    required TResult Function(KitchenSinkTwinSyncSse_Nested value) nested,
    required TResult Function(KitchenSinkTwinSyncSse_Optional value) optional,
    required TResult Function(KitchenSinkTwinSyncSse_Buffer value) buffer,
    required TResult Function(KitchenSinkTwinSyncSse_Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult? Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult? Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult? Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult? Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult? Function(KitchenSinkTwinSyncSse_Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSinkTwinSyncSse_Empty value)? empty,
    TResult Function(KitchenSinkTwinSyncSse_Primitives value)? primitives,
    TResult Function(KitchenSinkTwinSyncSse_Nested value)? nested,
    TResult Function(KitchenSinkTwinSyncSse_Optional value)? optional,
    TResult Function(KitchenSinkTwinSyncSse_Buffer value)? buffer,
    TResult Function(KitchenSinkTwinSyncSse_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class KitchenSinkTwinSyncSse_Enums extends KitchenSinkTwinSyncSse {
  const factory KitchenSinkTwinSyncSse_Enums(
      [final WeekdaysTwinSyncSse field0]) = _$KitchenSinkTwinSyncSse_EnumsImpl;
  const KitchenSinkTwinSyncSse_Enums._() : super._();

  WeekdaysTwinSyncSse get field0;

  /// Create a copy of KitchenSinkTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$KitchenSinkTwinSyncSse_EnumsImplCopyWith<
          _$KitchenSinkTwinSyncSse_EnumsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$MeasureTwinSyncSse {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse field0) speed,
    required TResult Function(DistanceTwinSyncSse field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse field0)? speed,
    TResult? Function(DistanceTwinSyncSse field0)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse field0)? speed,
    TResult Function(DistanceTwinSyncSse field0)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSyncSse_Speed value) speed,
    required TResult Function(MeasureTwinSyncSse_Distance value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinSyncSse_Distance value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult Function(MeasureTwinSyncSse_Distance value)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MeasureTwinSyncSseCopyWith<$Res> {
  factory $MeasureTwinSyncSseCopyWith(
          MeasureTwinSyncSse value, $Res Function(MeasureTwinSyncSse) then) =
      _$MeasureTwinSyncSseCopyWithImpl<$Res, MeasureTwinSyncSse>;
}

/// @nodoc
class _$MeasureTwinSyncSseCopyWithImpl<$Res, $Val extends MeasureTwinSyncSse>
    implements $MeasureTwinSyncSseCopyWith<$Res> {
  _$MeasureTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$MeasureTwinSyncSse_SpeedImplCopyWith<$Res> {
  factory _$$MeasureTwinSyncSse_SpeedImplCopyWith(
          _$MeasureTwinSyncSse_SpeedImpl value,
          $Res Function(_$MeasureTwinSyncSse_SpeedImpl) then) =
      __$$MeasureTwinSyncSse_SpeedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({SpeedTwinSyncSse field0});

  $SpeedTwinSyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinSyncSse_SpeedImplCopyWithImpl<$Res>
    extends _$MeasureTwinSyncSseCopyWithImpl<$Res,
        _$MeasureTwinSyncSse_SpeedImpl>
    implements _$$MeasureTwinSyncSse_SpeedImplCopyWith<$Res> {
  __$$MeasureTwinSyncSse_SpeedImplCopyWithImpl(
      _$MeasureTwinSyncSse_SpeedImpl _value,
      $Res Function(_$MeasureTwinSyncSse_SpeedImpl) _then)
      : super(_value, _then);

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinSyncSse_SpeedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpeedTwinSyncSse,
    ));
  }

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $SpeedTwinSyncSseCopyWith<$Res> get field0 {
    return $SpeedTwinSyncSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$MeasureTwinSyncSse_SpeedImpl extends MeasureTwinSyncSse_Speed {
  const _$MeasureTwinSyncSse_SpeedImpl(this.field0) : super._();

  @override
  final SpeedTwinSyncSse field0;

  @override
  String toString() {
    return 'MeasureTwinSyncSse.speed(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinSyncSse_SpeedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinSyncSse_SpeedImplCopyWith<_$MeasureTwinSyncSse_SpeedImpl>
      get copyWith => __$$MeasureTwinSyncSse_SpeedImplCopyWithImpl<
          _$MeasureTwinSyncSse_SpeedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse field0) speed,
    required TResult Function(DistanceTwinSyncSse field0) distance,
  }) {
    return speed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse field0)? speed,
    TResult? Function(DistanceTwinSyncSse field0)? distance,
  }) {
    return speed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse field0)? speed,
    TResult Function(DistanceTwinSyncSse field0)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSyncSse_Speed value) speed,
    required TResult Function(MeasureTwinSyncSse_Distance value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinSyncSse_Distance value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult Function(MeasureTwinSyncSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }
}

abstract class MeasureTwinSyncSse_Speed extends MeasureTwinSyncSse {
  const factory MeasureTwinSyncSse_Speed(final SpeedTwinSyncSse field0) =
      _$MeasureTwinSyncSse_SpeedImpl;
  const MeasureTwinSyncSse_Speed._() : super._();

  @override
  SpeedTwinSyncSse get field0;

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MeasureTwinSyncSse_SpeedImplCopyWith<_$MeasureTwinSyncSse_SpeedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MeasureTwinSyncSse_DistanceImplCopyWith<$Res> {
  factory _$$MeasureTwinSyncSse_DistanceImplCopyWith(
          _$MeasureTwinSyncSse_DistanceImpl value,
          $Res Function(_$MeasureTwinSyncSse_DistanceImpl) then) =
      __$$MeasureTwinSyncSse_DistanceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DistanceTwinSyncSse field0});

  $DistanceTwinSyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MeasureTwinSyncSse_DistanceImplCopyWithImpl<$Res>
    extends _$MeasureTwinSyncSseCopyWithImpl<$Res,
        _$MeasureTwinSyncSse_DistanceImpl>
    implements _$$MeasureTwinSyncSse_DistanceImplCopyWith<$Res> {
  __$$MeasureTwinSyncSse_DistanceImplCopyWithImpl(
      _$MeasureTwinSyncSse_DistanceImpl _value,
      $Res Function(_$MeasureTwinSyncSse_DistanceImpl) _then)
      : super(_value, _then);

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MeasureTwinSyncSse_DistanceImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DistanceTwinSyncSse,
    ));
  }

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $DistanceTwinSyncSseCopyWith<$Res> get field0 {
    return $DistanceTwinSyncSseCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$MeasureTwinSyncSse_DistanceImpl extends MeasureTwinSyncSse_Distance {
  const _$MeasureTwinSyncSse_DistanceImpl(this.field0) : super._();

  @override
  final DistanceTwinSyncSse field0;

  @override
  String toString() {
    return 'MeasureTwinSyncSse.distance(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MeasureTwinSyncSse_DistanceImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MeasureTwinSyncSse_DistanceImplCopyWith<_$MeasureTwinSyncSse_DistanceImpl>
      get copyWith => __$$MeasureTwinSyncSse_DistanceImplCopyWithImpl<
          _$MeasureTwinSyncSse_DistanceImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse field0) speed,
    required TResult Function(DistanceTwinSyncSse field0) distance,
  }) {
    return distance(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse field0)? speed,
    TResult? Function(DistanceTwinSyncSse field0)? distance,
  }) {
    return distance?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse field0)? speed,
    TResult Function(DistanceTwinSyncSse field0)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MeasureTwinSyncSse_Speed value) speed,
    required TResult Function(MeasureTwinSyncSse_Distance value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult? Function(MeasureTwinSyncSse_Distance value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MeasureTwinSyncSse_Speed value)? speed,
    TResult Function(MeasureTwinSyncSse_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }
}

abstract class MeasureTwinSyncSse_Distance extends MeasureTwinSyncSse {
  const factory MeasureTwinSyncSse_Distance(final DistanceTwinSyncSse field0) =
      _$MeasureTwinSyncSse_DistanceImpl;
  const MeasureTwinSyncSse_Distance._() : super._();

  @override
  DistanceTwinSyncSse get field0;

  /// Create a copy of MeasureTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MeasureTwinSyncSse_DistanceImplCopyWith<_$MeasureTwinSyncSse_DistanceImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SpeedTwinSyncSse {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSyncSse_GPS value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSyncSse_GPS value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSyncSse_GPS value)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpeedTwinSyncSseCopyWith<$Res> {
  factory $SpeedTwinSyncSseCopyWith(
          SpeedTwinSyncSse value, $Res Function(SpeedTwinSyncSse) then) =
      _$SpeedTwinSyncSseCopyWithImpl<$Res, SpeedTwinSyncSse>;
}

/// @nodoc
class _$SpeedTwinSyncSseCopyWithImpl<$Res, $Val extends SpeedTwinSyncSse>
    implements $SpeedTwinSyncSseCopyWith<$Res> {
  _$SpeedTwinSyncSseCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of SpeedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$SpeedTwinSyncSse_UnknownImplCopyWith<$Res> {
  factory _$$SpeedTwinSyncSse_UnknownImplCopyWith(
          _$SpeedTwinSyncSse_UnknownImpl value,
          $Res Function(_$SpeedTwinSyncSse_UnknownImpl) then) =
      __$$SpeedTwinSyncSse_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SpeedTwinSyncSse_UnknownImplCopyWithImpl<$Res>
    extends _$SpeedTwinSyncSseCopyWithImpl<$Res, _$SpeedTwinSyncSse_UnknownImpl>
    implements _$$SpeedTwinSyncSse_UnknownImplCopyWith<$Res> {
  __$$SpeedTwinSyncSse_UnknownImplCopyWithImpl(
      _$SpeedTwinSyncSse_UnknownImpl _value,
      $Res Function(_$SpeedTwinSyncSse_UnknownImpl) _then)
      : super(_value, _then);

  /// Create a copy of SpeedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$SpeedTwinSyncSse_UnknownImpl extends SpeedTwinSyncSse_Unknown {
  const _$SpeedTwinSyncSse_UnknownImpl() : super._();

  @override
  String toString() {
    return 'SpeedTwinSyncSse.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinSyncSse_UnknownImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSyncSse_GPS value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSyncSse_GPS value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSyncSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class SpeedTwinSyncSse_Unknown extends SpeedTwinSyncSse {
  const factory SpeedTwinSyncSse_Unknown() = _$SpeedTwinSyncSse_UnknownImpl;
  const SpeedTwinSyncSse_Unknown._() : super._();
}

/// @nodoc
abstract class _$$SpeedTwinSyncSse_GPSImplCopyWith<$Res> {
  factory _$$SpeedTwinSyncSse_GPSImplCopyWith(_$SpeedTwinSyncSse_GPSImpl value,
          $Res Function(_$SpeedTwinSyncSse_GPSImpl) then) =
      __$$SpeedTwinSyncSse_GPSImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$SpeedTwinSyncSse_GPSImplCopyWithImpl<$Res>
    extends _$SpeedTwinSyncSseCopyWithImpl<$Res, _$SpeedTwinSyncSse_GPSImpl>
    implements _$$SpeedTwinSyncSse_GPSImplCopyWith<$Res> {
  __$$SpeedTwinSyncSse_GPSImplCopyWithImpl(_$SpeedTwinSyncSse_GPSImpl _value,
      $Res Function(_$SpeedTwinSyncSse_GPSImpl) _then)
      : super(_value, _then);

  /// Create a copy of SpeedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$SpeedTwinSyncSse_GPSImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$SpeedTwinSyncSse_GPSImpl extends SpeedTwinSyncSse_GPS {
  const _$SpeedTwinSyncSse_GPSImpl(this.field0) : super._();

  @override
  final double field0;

  @override
  String toString() {
    return 'SpeedTwinSyncSse.gps(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpeedTwinSyncSse_GPSImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of SpeedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SpeedTwinSyncSse_GPSImplCopyWith<_$SpeedTwinSyncSse_GPSImpl>
      get copyWith =>
          __$$SpeedTwinSyncSse_GPSImplCopyWithImpl<_$SpeedTwinSyncSse_GPSImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return gps(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
  }) {
    return gps?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpeedTwinSyncSse_Unknown value) unknown,
    required TResult Function(SpeedTwinSyncSse_GPS value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult? Function(SpeedTwinSyncSse_GPS value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpeedTwinSyncSse_Unknown value)? unknown,
    TResult Function(SpeedTwinSyncSse_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }
}

abstract class SpeedTwinSyncSse_GPS extends SpeedTwinSyncSse {
  const factory SpeedTwinSyncSse_GPS(final double field0) =
      _$SpeedTwinSyncSse_GPSImpl;
  const SpeedTwinSyncSse_GPS._() : super._();

  double get field0;

  /// Create a copy of SpeedTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SpeedTwinSyncSse_GPSImplCopyWith<_$SpeedTwinSyncSse_GPSImpl>
      get copyWith => throw _privateConstructorUsedError;
}
