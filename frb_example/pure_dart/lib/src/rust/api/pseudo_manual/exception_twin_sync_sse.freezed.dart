// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinSyncSse {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSyncSseCopyWith<CustomEnumErrorTwinSyncSse>
      get copyWith =>
          _$CustomEnumErrorTwinSyncSseCopyWithImpl<CustomEnumErrorTwinSyncSse>(
              this as CustomEnumErrorTwinSyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSyncSse &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSyncSse(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSyncSseCopyWith(CustomEnumErrorTwinSyncSse value,
          $Res Function(CustomEnumErrorTwinSyncSse) _then) =
      _$CustomEnumErrorTwinSyncSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSyncSseCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSyncSseCopyWith<$Res> {
  _$CustomEnumErrorTwinSyncSseCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSyncSse _self;
  final $Res Function(CustomEnumErrorTwinSyncSse) _then;

  /// Create a copy of CustomEnumErrorTwinSyncSse
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

class CustomEnumErrorTwinSyncSse_One extends CustomEnumErrorTwinSyncSse
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSyncSse_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSyncSse_OneCopyWith<CustomEnumErrorTwinSyncSse_One>
      get copyWith => _$CustomEnumErrorTwinSyncSse_OneCopyWithImpl<
          CustomEnumErrorTwinSyncSse_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSyncSse_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSyncSse.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSyncSse_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSyncSse_OneCopyWith(
          CustomEnumErrorTwinSyncSse_One value,
          $Res Function(CustomEnumErrorTwinSyncSse_One) _then) =
      _$CustomEnumErrorTwinSyncSse_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSyncSse_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSyncSse_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinSyncSse_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSyncSse_One _self;
  final $Res Function(CustomEnumErrorTwinSyncSse_One) _then;

  /// Create a copy of CustomEnumErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSyncSse_One(
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

class CustomEnumErrorTwinSyncSse_Two extends CustomEnumErrorTwinSyncSse
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSyncSse_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSyncSse_TwoCopyWith<CustomEnumErrorTwinSyncSse_Two>
      get copyWith => _$CustomEnumErrorTwinSyncSse_TwoCopyWithImpl<
          CustomEnumErrorTwinSyncSse_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSyncSse_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSyncSse.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSyncSse_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSyncSse_TwoCopyWith(
          CustomEnumErrorTwinSyncSse_Two value,
          $Res Function(CustomEnumErrorTwinSyncSse_Two) _then) =
      _$CustomEnumErrorTwinSyncSse_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSyncSse_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSyncSse_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinSyncSse_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSyncSse_Two _self;
  final $Res Function(CustomEnumErrorTwinSyncSse_Two) _then;

  /// Create a copy of CustomEnumErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSyncSse_Two(
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
mixin _$CustomErrorTwinSyncSse {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSyncSseCopyWith<CustomErrorTwinSyncSse> get copyWith =>
      _$CustomErrorTwinSyncSseCopyWithImpl<CustomErrorTwinSyncSse>(
          this as CustomErrorTwinSyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSyncSse &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSyncSse(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomErrorTwinSyncSseCopyWith(CustomErrorTwinSyncSse value,
          $Res Function(CustomErrorTwinSyncSse) _then) =
      _$CustomErrorTwinSyncSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSyncSseCopyWithImpl<$Res>
    implements $CustomErrorTwinSyncSseCopyWith<$Res> {
  _$CustomErrorTwinSyncSseCopyWithImpl(this._self, this._then);

  final CustomErrorTwinSyncSse _self;
  final $Res Function(CustomErrorTwinSyncSse) _then;

  /// Create a copy of CustomErrorTwinSyncSse
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

class CustomErrorTwinSyncSse_Error0 extends CustomErrorTwinSyncSse
    implements FrbBacktracedException {
  const CustomErrorTwinSyncSse_Error0(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSyncSse_Error0CopyWith<CustomErrorTwinSyncSse_Error0>
      get copyWith => _$CustomErrorTwinSyncSse_Error0CopyWithImpl<
          CustomErrorTwinSyncSse_Error0>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSyncSse_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSyncSse.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSyncSse_Error0CopyWith<$Res>
    implements $CustomErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomErrorTwinSyncSse_Error0CopyWith(
          CustomErrorTwinSyncSse_Error0 value,
          $Res Function(CustomErrorTwinSyncSse_Error0) _then) =
      _$CustomErrorTwinSyncSse_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSyncSse_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinSyncSse_Error0CopyWith<$Res> {
  _$CustomErrorTwinSyncSse_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSyncSse_Error0 _self;
  final $Res Function(CustomErrorTwinSyncSse_Error0) _then;

  /// Create a copy of CustomErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSyncSse_Error0(
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

class CustomErrorTwinSyncSse_Error1 extends CustomErrorTwinSyncSse
    implements FrbBacktracedException {
  const CustomErrorTwinSyncSse_Error1(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSyncSse_Error1CopyWith<CustomErrorTwinSyncSse_Error1>
      get copyWith => _$CustomErrorTwinSyncSse_Error1CopyWithImpl<
          CustomErrorTwinSyncSse_Error1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSyncSse_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSyncSse.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSyncSse_Error1CopyWith<$Res>
    implements $CustomErrorTwinSyncSseCopyWith<$Res> {
  factory $CustomErrorTwinSyncSse_Error1CopyWith(
          CustomErrorTwinSyncSse_Error1 value,
          $Res Function(CustomErrorTwinSyncSse_Error1) _then) =
      _$CustomErrorTwinSyncSse_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSyncSse_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinSyncSse_Error1CopyWith<$Res> {
  _$CustomErrorTwinSyncSse_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSyncSse_Error1 _self;
  final $Res Function(CustomErrorTwinSyncSse_Error1) _then;

  /// Create a copy of CustomErrorTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSyncSse_Error1(
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
mixin _$CustomNestedError1TwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinSyncSseCopyWith<$Res> {
  $CustomNestedError1TwinSyncSseCopyWith(CustomNestedError1TwinSyncSse _,
      $Res Function(CustomNestedError1TwinSyncSse) __);
}

/// @nodoc

class CustomNestedError1TwinSyncSse_CustomNested1
    extends CustomNestedError1TwinSyncSse {
  const CustomNestedError1TwinSyncSse_CustomNested1(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSyncSse_CustomNested1CopyWith<
          CustomNestedError1TwinSyncSse_CustomNested1>
      get copyWith => _$CustomNestedError1TwinSyncSse_CustomNested1CopyWithImpl<
          CustomNestedError1TwinSyncSse_CustomNested1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSyncSse_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSyncSse.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSyncSse_CustomNested1CopyWith<$Res>
    implements $CustomNestedError1TwinSyncSseCopyWith<$Res> {
  factory $CustomNestedError1TwinSyncSse_CustomNested1CopyWith(
          CustomNestedError1TwinSyncSse_CustomNested1 value,
          $Res Function(CustomNestedError1TwinSyncSse_CustomNested1) _then) =
      _$CustomNestedError1TwinSyncSse_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinSyncSse_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinSyncSse_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinSyncSse_CustomNested1CopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinSyncSse_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinSyncSse_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSyncSse_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinSyncSse_ErrorNested
    extends CustomNestedError1TwinSyncSse {
  const CustomNestedError1TwinSyncSse_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinSyncSse field0;

  /// Create a copy of CustomNestedError1TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSyncSse_ErrorNestedCopyWith<
          CustomNestedError1TwinSyncSse_ErrorNested>
      get copyWith => _$CustomNestedError1TwinSyncSse_ErrorNestedCopyWithImpl<
          CustomNestedError1TwinSyncSse_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSyncSse_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSyncSse.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSyncSse_ErrorNestedCopyWith<$Res>
    implements $CustomNestedError1TwinSyncSseCopyWith<$Res> {
  factory $CustomNestedError1TwinSyncSse_ErrorNestedCopyWith(
          CustomNestedError1TwinSyncSse_ErrorNested value,
          $Res Function(CustomNestedError1TwinSyncSse_ErrorNested) _then) =
      _$CustomNestedError1TwinSyncSse_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinSyncSse field0});

  $CustomNestedError2TwinSyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinSyncSse_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinSyncSse_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinSyncSse_ErrorNestedCopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinSyncSse_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinSyncSse_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSyncSse_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinSyncSse,
    ));
  }

  /// Create a copy of CustomNestedError1TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSyncSseCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinSyncSseCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinSyncSseCopyWith<$Res> {
  $CustomNestedError2TwinSyncSseCopyWith(CustomNestedError2TwinSyncSse _,
      $Res Function(CustomNestedError2TwinSyncSse) __);
}

/// @nodoc

class CustomNestedError2TwinSyncSse_CustomNested2
    extends CustomNestedError2TwinSyncSse {
  const CustomNestedError2TwinSyncSse_CustomNested2(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSyncSse_CustomNested2CopyWith<
          CustomNestedError2TwinSyncSse_CustomNested2>
      get copyWith => _$CustomNestedError2TwinSyncSse_CustomNested2CopyWithImpl<
          CustomNestedError2TwinSyncSse_CustomNested2>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSyncSse_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSyncSse.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSyncSse_CustomNested2CopyWith<$Res>
    implements $CustomNestedError2TwinSyncSseCopyWith<$Res> {
  factory $CustomNestedError2TwinSyncSse_CustomNested2CopyWith(
          CustomNestedError2TwinSyncSse_CustomNested2 value,
          $Res Function(CustomNestedError2TwinSyncSse_CustomNested2) _then) =
      _$CustomNestedError2TwinSyncSse_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinSyncSse_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinSyncSse_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinSyncSse_CustomNested2CopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinSyncSse_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinSyncSse_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSyncSse_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinSyncSse_CustomNested2Number
    extends CustomNestedError2TwinSyncSse {
  const CustomNestedError2TwinSyncSse_CustomNested2Number(this.field0)
      : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWith<
          CustomNestedError2TwinSyncSse_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWithImpl<
                  CustomNestedError2TwinSyncSse_CustomNested2Number>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSyncSse_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSyncSse.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinSyncSseCopyWith<$Res> {
  factory $CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWith(
          CustomNestedError2TwinSyncSse_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinSyncSse_CustomNested2Number)
              _then) =
      _$CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWithImpl<$Res>
    implements
        $CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinSyncSse_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinSyncSse_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinSyncSse_CustomNested2Number) _then;

  /// Create a copy of CustomNestedError2TwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSyncSse_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res> {
  $CustomNestedErrorInnerTwinSyncSseCopyWith(
      CustomNestedErrorInnerTwinSyncSse _,
      $Res Function(CustomNestedErrorInnerTwinSyncSse) __);
}

/// @nodoc

class CustomNestedErrorInnerTwinSyncSse_Three
    extends CustomNestedErrorInnerTwinSyncSse {
  const CustomNestedErrorInnerTwinSyncSse_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSyncSse_ThreeCopyWith<
          CustomNestedErrorInnerTwinSyncSse_Three>
      get copyWith => _$CustomNestedErrorInnerTwinSyncSse_ThreeCopyWithImpl<
          CustomNestedErrorInnerTwinSyncSse_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSyncSse_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSyncSse.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSyncSse_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSyncSse_ThreeCopyWith(
          CustomNestedErrorInnerTwinSyncSse_Three value,
          $Res Function(CustomNestedErrorInnerTwinSyncSse_Three) _then) =
      _$CustomNestedErrorInnerTwinSyncSse_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSyncSse_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSyncSse_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSyncSse_ThreeCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSyncSse_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinSyncSse_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSyncSse_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinSyncSse_Four
    extends CustomNestedErrorInnerTwinSyncSse {
  const CustomNestedErrorInnerTwinSyncSse_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSyncSse_FourCopyWith<
          CustomNestedErrorInnerTwinSyncSse_Four>
      get copyWith => _$CustomNestedErrorInnerTwinSyncSse_FourCopyWithImpl<
          CustomNestedErrorInnerTwinSyncSse_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSyncSse_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSyncSse.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSyncSse_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSyncSse_FourCopyWith(
          CustomNestedErrorInnerTwinSyncSse_Four value,
          $Res Function(CustomNestedErrorInnerTwinSyncSse_Four) _then) =
      _$CustomNestedErrorInnerTwinSyncSse_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSyncSse_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSyncSse_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSyncSse_FourCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSyncSse_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinSyncSse_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSyncSse_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinSyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinSyncSseCopyWith<$Res> {
  $CustomNestedErrorOuterTwinSyncSseCopyWith(
      CustomNestedErrorOuterTwinSyncSse _,
      $Res Function(CustomNestedErrorOuterTwinSyncSse) __);
}

/// @nodoc

class CustomNestedErrorOuterTwinSyncSse_One
    extends CustomNestedErrorOuterTwinSyncSse {
  const CustomNestedErrorOuterTwinSyncSse_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSyncSse_OneCopyWith<
          CustomNestedErrorOuterTwinSyncSse_One>
      get copyWith => _$CustomNestedErrorOuterTwinSyncSse_OneCopyWithImpl<
          CustomNestedErrorOuterTwinSyncSse_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSyncSse_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSyncSse.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSyncSse_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSyncSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSyncSse_OneCopyWith(
          CustomNestedErrorOuterTwinSyncSse_One value,
          $Res Function(CustomNestedErrorOuterTwinSyncSse_One) _then) =
      _$CustomNestedErrorOuterTwinSyncSse_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSyncSse_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSyncSse_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSyncSse_OneCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSyncSse_One _self;
  final $Res Function(CustomNestedErrorOuterTwinSyncSse_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSyncSse_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinSyncSse_Two
    extends CustomNestedErrorOuterTwinSyncSse {
  const CustomNestedErrorOuterTwinSyncSse_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinSyncSse field0;

  /// Create a copy of CustomNestedErrorOuterTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSyncSse_TwoCopyWith<
          CustomNestedErrorOuterTwinSyncSse_Two>
      get copyWith => _$CustomNestedErrorOuterTwinSyncSse_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinSyncSse_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSyncSse_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSyncSse.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSyncSse_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSyncSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSyncSse_TwoCopyWith(
          CustomNestedErrorOuterTwinSyncSse_Two value,
          $Res Function(CustomNestedErrorOuterTwinSyncSse_Two) _then) =
      _$CustomNestedErrorOuterTwinSyncSse_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinSyncSse field0});

  $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSyncSse_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSyncSse_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSyncSse_TwoCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSyncSse_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinSyncSse_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSyncSse_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinSyncSse,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinSyncSseCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
