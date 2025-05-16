// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_sync.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinSync {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSyncCopyWith<CustomEnumErrorTwinSync> get copyWith =>
      _$CustomEnumErrorTwinSyncCopyWithImpl<CustomEnumErrorTwinSync>(
          this as CustomEnumErrorTwinSync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSync &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSync(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinSyncCopyWith(CustomEnumErrorTwinSync value,
          $Res Function(CustomEnumErrorTwinSync) _then) =
      _$CustomEnumErrorTwinSyncCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSyncCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSyncCopyWith<$Res> {
  _$CustomEnumErrorTwinSyncCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSync _self;
  final $Res Function(CustomEnumErrorTwinSync) _then;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? backtrace = null,
  }) {
    return _then(_self.copyWith(
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomEnumErrorTwinSync_One extends CustomEnumErrorTwinSync
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSync_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSync_OneCopyWith<CustomEnumErrorTwinSync_One>
      get copyWith => _$CustomEnumErrorTwinSync_OneCopyWithImpl<
          CustomEnumErrorTwinSync_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSync_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSync.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSync_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinSyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinSync_OneCopyWith(
          CustomEnumErrorTwinSync_One value,
          $Res Function(CustomEnumErrorTwinSync_One) _then) =
      _$CustomEnumErrorTwinSync_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSync_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSync_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinSync_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSync_One _self;
  final $Res Function(CustomEnumErrorTwinSync_One) _then;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSync_One(
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomEnumErrorTwinSync_Two extends CustomEnumErrorTwinSync
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSync_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSync_TwoCopyWith<CustomEnumErrorTwinSync_Two>
      get copyWith => _$CustomEnumErrorTwinSync_TwoCopyWithImpl<
          CustomEnumErrorTwinSync_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSync_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSync.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSync_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinSyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinSync_TwoCopyWith(
          CustomEnumErrorTwinSync_Two value,
          $Res Function(CustomEnumErrorTwinSync_Two) _then) =
      _$CustomEnumErrorTwinSync_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSync_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSync_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinSync_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSync_Two _self;
  final $Res Function(CustomEnumErrorTwinSync_Two) _then;

  /// Create a copy of CustomEnumErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSync_Two(
      message: null == message
          ? _self.message
          : message // ignore: cast_nullable_to_non_nullable
              as int,
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$CustomErrorTwinSync {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSyncCopyWith<CustomErrorTwinSync> get copyWith =>
      _$CustomErrorTwinSyncCopyWithImpl<CustomErrorTwinSync>(
          this as CustomErrorTwinSync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSync &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSync(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSyncCopyWith<$Res> {
  factory $CustomErrorTwinSyncCopyWith(
          CustomErrorTwinSync value, $Res Function(CustomErrorTwinSync) _then) =
      _$CustomErrorTwinSyncCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSyncCopyWithImpl<$Res>
    implements $CustomErrorTwinSyncCopyWith<$Res> {
  _$CustomErrorTwinSyncCopyWithImpl(this._self, this._then);

  final CustomErrorTwinSync _self;
  final $Res Function(CustomErrorTwinSync) _then;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? backtrace = null,
  }) {
    return _then(_self.copyWith(
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomErrorTwinSync_Error0 extends CustomErrorTwinSync
    implements FrbBacktracedException {
  const CustomErrorTwinSync_Error0({required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSync_Error0CopyWith<CustomErrorTwinSync_Error0>
      get copyWith =>
          _$CustomErrorTwinSync_Error0CopyWithImpl<CustomErrorTwinSync_Error0>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSync_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSync.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSync_Error0CopyWith<$Res>
    implements $CustomErrorTwinSyncCopyWith<$Res> {
  factory $CustomErrorTwinSync_Error0CopyWith(CustomErrorTwinSync_Error0 value,
          $Res Function(CustomErrorTwinSync_Error0) _then) =
      _$CustomErrorTwinSync_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSync_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinSync_Error0CopyWith<$Res> {
  _$CustomErrorTwinSync_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSync_Error0 _self;
  final $Res Function(CustomErrorTwinSync_Error0) _then;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSync_Error0(
      e: null == e
          ? _self.e
          : e // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomErrorTwinSync_Error1 extends CustomErrorTwinSync
    implements FrbBacktracedException {
  const CustomErrorTwinSync_Error1({required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSync_Error1CopyWith<CustomErrorTwinSync_Error1>
      get copyWith =>
          _$CustomErrorTwinSync_Error1CopyWithImpl<CustomErrorTwinSync_Error1>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSync_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSync.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSync_Error1CopyWith<$Res>
    implements $CustomErrorTwinSyncCopyWith<$Res> {
  factory $CustomErrorTwinSync_Error1CopyWith(CustomErrorTwinSync_Error1 value,
          $Res Function(CustomErrorTwinSync_Error1) _then) =
      _$CustomErrorTwinSync_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSync_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinSync_Error1CopyWith<$Res> {
  _$CustomErrorTwinSync_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSync_Error1 _self;
  final $Res Function(CustomErrorTwinSync_Error1) _then;

  /// Create a copy of CustomErrorTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSync_Error1(
      e: null == e
          ? _self.e
          : e // ignore: cast_nullable_to_non_nullable
              as int,
      backtrace: null == backtrace
          ? _self.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedError1TwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinSync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinSyncCopyWith<$Res> {
  $CustomNestedError1TwinSyncCopyWith(CustomNestedError1TwinSync _,
      $Res Function(CustomNestedError1TwinSync) __);
}

/// @nodoc

class CustomNestedError1TwinSync_CustomNested1
    extends CustomNestedError1TwinSync {
  const CustomNestedError1TwinSync_CustomNested1(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSync_CustomNested1CopyWith<
          CustomNestedError1TwinSync_CustomNested1>
      get copyWith => _$CustomNestedError1TwinSync_CustomNested1CopyWithImpl<
          CustomNestedError1TwinSync_CustomNested1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSync_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSync.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSync_CustomNested1CopyWith<$Res>
    implements $CustomNestedError1TwinSyncCopyWith<$Res> {
  factory $CustomNestedError1TwinSync_CustomNested1CopyWith(
          CustomNestedError1TwinSync_CustomNested1 value,
          $Res Function(CustomNestedError1TwinSync_CustomNested1) _then) =
      _$CustomNestedError1TwinSync_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinSync_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinSync_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinSync_CustomNested1CopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinSync_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinSync_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSync_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinSync_ErrorNested
    extends CustomNestedError1TwinSync {
  const CustomNestedError1TwinSync_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinSync field0;

  /// Create a copy of CustomNestedError1TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSync_ErrorNestedCopyWith<
          CustomNestedError1TwinSync_ErrorNested>
      get copyWith => _$CustomNestedError1TwinSync_ErrorNestedCopyWithImpl<
          CustomNestedError1TwinSync_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSync_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSync.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSync_ErrorNestedCopyWith<$Res>
    implements $CustomNestedError1TwinSyncCopyWith<$Res> {
  factory $CustomNestedError1TwinSync_ErrorNestedCopyWith(
          CustomNestedError1TwinSync_ErrorNested value,
          $Res Function(CustomNestedError1TwinSync_ErrorNested) _then) =
      _$CustomNestedError1TwinSync_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinSync field0});

  $CustomNestedError2TwinSyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinSync_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinSync_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinSync_ErrorNestedCopyWithImpl(this._self, this._then);

  final CustomNestedError1TwinSync_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinSync_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSync_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinSync,
    ));
  }

  /// Create a copy of CustomNestedError1TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSyncCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinSyncCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinSync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinSyncCopyWith<$Res> {
  $CustomNestedError2TwinSyncCopyWith(CustomNestedError2TwinSync _,
      $Res Function(CustomNestedError2TwinSync) __);
}

/// @nodoc

class CustomNestedError2TwinSync_CustomNested2
    extends CustomNestedError2TwinSync {
  const CustomNestedError2TwinSync_CustomNested2(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSync_CustomNested2CopyWith<
          CustomNestedError2TwinSync_CustomNested2>
      get copyWith => _$CustomNestedError2TwinSync_CustomNested2CopyWithImpl<
          CustomNestedError2TwinSync_CustomNested2>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSync_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSync.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSync_CustomNested2CopyWith<$Res>
    implements $CustomNestedError2TwinSyncCopyWith<$Res> {
  factory $CustomNestedError2TwinSync_CustomNested2CopyWith(
          CustomNestedError2TwinSync_CustomNested2 value,
          $Res Function(CustomNestedError2TwinSync_CustomNested2) _then) =
      _$CustomNestedError2TwinSync_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinSync_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinSync_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinSync_CustomNested2CopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinSync_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinSync_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSync_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinSync_CustomNested2Number
    extends CustomNestedError2TwinSync {
  const CustomNestedError2TwinSync_CustomNested2Number(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSync_CustomNested2NumberCopyWith<
          CustomNestedError2TwinSync_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinSync_CustomNested2NumberCopyWithImpl<
              CustomNestedError2TwinSync_CustomNested2Number>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSync_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSync.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSync_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinSyncCopyWith<$Res> {
  factory $CustomNestedError2TwinSync_CustomNested2NumberCopyWith(
          CustomNestedError2TwinSync_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinSync_CustomNested2Number) _then) =
      _$CustomNestedError2TwinSync_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinSync_CustomNested2NumberCopyWithImpl<$Res>
    implements $CustomNestedError2TwinSync_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinSync_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinSync_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinSync_CustomNested2Number) _then;

  /// Create a copy of CustomNestedError2TwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSync_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinSyncCopyWith<$Res> {
  $CustomNestedErrorInnerTwinSyncCopyWith(CustomNestedErrorInnerTwinSync _,
      $Res Function(CustomNestedErrorInnerTwinSync) __);
}

/// @nodoc

class CustomNestedErrorInnerTwinSync_Three
    extends CustomNestedErrorInnerTwinSync {
  const CustomNestedErrorInnerTwinSync_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSync_ThreeCopyWith<
          CustomNestedErrorInnerTwinSync_Three>
      get copyWith => _$CustomNestedErrorInnerTwinSync_ThreeCopyWithImpl<
          CustomNestedErrorInnerTwinSync_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSync_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSync.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSync_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSyncCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSync_ThreeCopyWith(
          CustomNestedErrorInnerTwinSync_Three value,
          $Res Function(CustomNestedErrorInnerTwinSync_Three) _then) =
      _$CustomNestedErrorInnerTwinSync_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSync_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSync_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSync_ThreeCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSync_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinSync_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSync_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinSync_Four
    extends CustomNestedErrorInnerTwinSync {
  const CustomNestedErrorInnerTwinSync_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSync_FourCopyWith<
          CustomNestedErrorInnerTwinSync_Four>
      get copyWith => _$CustomNestedErrorInnerTwinSync_FourCopyWithImpl<
          CustomNestedErrorInnerTwinSync_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSync_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSync.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSync_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSyncCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSync_FourCopyWith(
          CustomNestedErrorInnerTwinSync_Four value,
          $Res Function(CustomNestedErrorInnerTwinSync_Four) _then) =
      _$CustomNestedErrorInnerTwinSync_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSync_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSync_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSync_FourCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSync_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinSync_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSync_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinSync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinSyncCopyWith<$Res> {
  $CustomNestedErrorOuterTwinSyncCopyWith(CustomNestedErrorOuterTwinSync _,
      $Res Function(CustomNestedErrorOuterTwinSync) __);
}

/// @nodoc

class CustomNestedErrorOuterTwinSync_One
    extends CustomNestedErrorOuterTwinSync {
  const CustomNestedErrorOuterTwinSync_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSync_OneCopyWith<
          CustomNestedErrorOuterTwinSync_One>
      get copyWith => _$CustomNestedErrorOuterTwinSync_OneCopyWithImpl<
          CustomNestedErrorOuterTwinSync_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSync_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSync.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSync_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSyncCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSync_OneCopyWith(
          CustomNestedErrorOuterTwinSync_One value,
          $Res Function(CustomNestedErrorOuterTwinSync_One) _then) =
      _$CustomNestedErrorOuterTwinSync_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSync_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSync_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSync_OneCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSync_One _self;
  final $Res Function(CustomNestedErrorOuterTwinSync_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSync_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinSync_Two
    extends CustomNestedErrorOuterTwinSync {
  const CustomNestedErrorOuterTwinSync_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinSync field0;

  /// Create a copy of CustomNestedErrorOuterTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSync_TwoCopyWith<
          CustomNestedErrorOuterTwinSync_Two>
      get copyWith => _$CustomNestedErrorOuterTwinSync_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinSync_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSync_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSync.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSync_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSyncCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSync_TwoCopyWith(
          CustomNestedErrorOuterTwinSync_Two value,
          $Res Function(CustomNestedErrorOuterTwinSync_Two) _then) =
      _$CustomNestedErrorOuterTwinSync_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinSync field0});

  $CustomNestedErrorInnerTwinSyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSync_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSync_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSync_TwoCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSync_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinSync_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSync_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinSync,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinSync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSyncCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinSyncCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
