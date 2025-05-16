// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinSse {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSseCopyWith<CustomEnumErrorTwinSse> get copyWith =>
      _$CustomEnumErrorTwinSseCopyWithImpl<CustomEnumErrorTwinSse>(
          this as CustomEnumErrorTwinSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSse &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSse(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSseCopyWith(CustomEnumErrorTwinSse value,
          $Res Function(CustomEnumErrorTwinSse) _then) =
      _$CustomEnumErrorTwinSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSseCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSseCopyWith<$Res> {
  _$CustomEnumErrorTwinSseCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSse _self;
  final $Res Function(CustomEnumErrorTwinSse) _then;

  /// Create a copy of CustomEnumErrorTwinSse
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

class CustomEnumErrorTwinSse_One extends CustomEnumErrorTwinSse
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSse_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSse_OneCopyWith<CustomEnumErrorTwinSse_One>
      get copyWith =>
          _$CustomEnumErrorTwinSse_OneCopyWithImpl<CustomEnumErrorTwinSse_One>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSse_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSse.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSse_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSse_OneCopyWith(CustomEnumErrorTwinSse_One value,
          $Res Function(CustomEnumErrorTwinSse_One) _then) =
      _$CustomEnumErrorTwinSse_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSse_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSse_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinSse_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSse_One _self;
  final $Res Function(CustomEnumErrorTwinSse_One) _then;

  /// Create a copy of CustomEnumErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSse_One(
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

class CustomEnumErrorTwinSse_Two extends CustomEnumErrorTwinSse
    implements FrbBacktracedException {
  const CustomEnumErrorTwinSse_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinSse_TwoCopyWith<CustomEnumErrorTwinSse_Two>
      get copyWith =>
          _$CustomEnumErrorTwinSse_TwoCopyWithImpl<CustomEnumErrorTwinSse_Two>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinSse_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinSse.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinSse_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinSse_TwoCopyWith(CustomEnumErrorTwinSse_Two value,
          $Res Function(CustomEnumErrorTwinSse_Two) _then) =
      _$CustomEnumErrorTwinSse_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinSse_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinSse_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinSse_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinSse_Two _self;
  final $Res Function(CustomEnumErrorTwinSse_Two) _then;

  /// Create a copy of CustomEnumErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinSse_Two(
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
mixin _$CustomErrorTwinSse {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSseCopyWith<CustomErrorTwinSse> get copyWith =>
      _$CustomErrorTwinSseCopyWithImpl<CustomErrorTwinSse>(
          this as CustomErrorTwinSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSse &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSse(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSseCopyWith<$Res> {
  factory $CustomErrorTwinSseCopyWith(
          CustomErrorTwinSse value, $Res Function(CustomErrorTwinSse) _then) =
      _$CustomErrorTwinSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSseCopyWithImpl<$Res>
    implements $CustomErrorTwinSseCopyWith<$Res> {
  _$CustomErrorTwinSseCopyWithImpl(this._self, this._then);

  final CustomErrorTwinSse _self;
  final $Res Function(CustomErrorTwinSse) _then;

  /// Create a copy of CustomErrorTwinSse
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

class CustomErrorTwinSse_Error0 extends CustomErrorTwinSse
    implements FrbBacktracedException {
  const CustomErrorTwinSse_Error0({required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSse_Error0CopyWith<CustomErrorTwinSse_Error0> get copyWith =>
      _$CustomErrorTwinSse_Error0CopyWithImpl<CustomErrorTwinSse_Error0>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSse_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSse.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSse_Error0CopyWith<$Res>
    implements $CustomErrorTwinSseCopyWith<$Res> {
  factory $CustomErrorTwinSse_Error0CopyWith(CustomErrorTwinSse_Error0 value,
          $Res Function(CustomErrorTwinSse_Error0) _then) =
      _$CustomErrorTwinSse_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSse_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinSse_Error0CopyWith<$Res> {
  _$CustomErrorTwinSse_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSse_Error0 _self;
  final $Res Function(CustomErrorTwinSse_Error0) _then;

  /// Create a copy of CustomErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSse_Error0(
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

class CustomErrorTwinSse_Error1 extends CustomErrorTwinSse
    implements FrbBacktracedException {
  const CustomErrorTwinSse_Error1({required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinSse_Error1CopyWith<CustomErrorTwinSse_Error1> get copyWith =>
      _$CustomErrorTwinSse_Error1CopyWithImpl<CustomErrorTwinSse_Error1>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinSse_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinSse.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinSse_Error1CopyWith<$Res>
    implements $CustomErrorTwinSseCopyWith<$Res> {
  factory $CustomErrorTwinSse_Error1CopyWith(CustomErrorTwinSse_Error1 value,
          $Res Function(CustomErrorTwinSse_Error1) _then) =
      _$CustomErrorTwinSse_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinSse_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinSse_Error1CopyWith<$Res> {
  _$CustomErrorTwinSse_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinSse_Error1 _self;
  final $Res Function(CustomErrorTwinSse_Error1) _then;

  /// Create a copy of CustomErrorTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinSse_Error1(
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
mixin _$CustomNestedError1TwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinSseCopyWith<$Res> {
  $CustomNestedError1TwinSseCopyWith(
      CustomNestedError1TwinSse _, $Res Function(CustomNestedError1TwinSse) __);
}

/// @nodoc

class CustomNestedError1TwinSse_CustomNested1
    extends CustomNestedError1TwinSse {
  const CustomNestedError1TwinSse_CustomNested1(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSse_CustomNested1CopyWith<
          CustomNestedError1TwinSse_CustomNested1>
      get copyWith => _$CustomNestedError1TwinSse_CustomNested1CopyWithImpl<
          CustomNestedError1TwinSse_CustomNested1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSse_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSse.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSse_CustomNested1CopyWith<$Res>
    implements $CustomNestedError1TwinSseCopyWith<$Res> {
  factory $CustomNestedError1TwinSse_CustomNested1CopyWith(
          CustomNestedError1TwinSse_CustomNested1 value,
          $Res Function(CustomNestedError1TwinSse_CustomNested1) _then) =
      _$CustomNestedError1TwinSse_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinSse_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinSse_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinSse_CustomNested1CopyWithImpl(this._self, this._then);

  final CustomNestedError1TwinSse_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinSse_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSse_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinSse_ErrorNested extends CustomNestedError1TwinSse {
  const CustomNestedError1TwinSse_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinSse field0;

  /// Create a copy of CustomNestedError1TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinSse_ErrorNestedCopyWith<
          CustomNestedError1TwinSse_ErrorNested>
      get copyWith => _$CustomNestedError1TwinSse_ErrorNestedCopyWithImpl<
          CustomNestedError1TwinSse_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinSse_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinSse.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinSse_ErrorNestedCopyWith<$Res>
    implements $CustomNestedError1TwinSseCopyWith<$Res> {
  factory $CustomNestedError1TwinSse_ErrorNestedCopyWith(
          CustomNestedError1TwinSse_ErrorNested value,
          $Res Function(CustomNestedError1TwinSse_ErrorNested) _then) =
      _$CustomNestedError1TwinSse_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinSse field0});

  $CustomNestedError2TwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinSse_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinSse_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinSse_ErrorNestedCopyWithImpl(this._self, this._then);

  final CustomNestedError1TwinSse_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinSse_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinSse_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinSse,
    ));
  }

  /// Create a copy of CustomNestedError1TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSseCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinSseCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinSseCopyWith<$Res> {
  $CustomNestedError2TwinSseCopyWith(
      CustomNestedError2TwinSse _, $Res Function(CustomNestedError2TwinSse) __);
}

/// @nodoc

class CustomNestedError2TwinSse_CustomNested2
    extends CustomNestedError2TwinSse {
  const CustomNestedError2TwinSse_CustomNested2(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSse_CustomNested2CopyWith<
          CustomNestedError2TwinSse_CustomNested2>
      get copyWith => _$CustomNestedError2TwinSse_CustomNested2CopyWithImpl<
          CustomNestedError2TwinSse_CustomNested2>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSse_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSse.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSse_CustomNested2CopyWith<$Res>
    implements $CustomNestedError2TwinSseCopyWith<$Res> {
  factory $CustomNestedError2TwinSse_CustomNested2CopyWith(
          CustomNestedError2TwinSse_CustomNested2 value,
          $Res Function(CustomNestedError2TwinSse_CustomNested2) _then) =
      _$CustomNestedError2TwinSse_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinSse_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinSse_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinSse_CustomNested2CopyWithImpl(this._self, this._then);

  final CustomNestedError2TwinSse_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinSse_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSse_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinSse_CustomNested2Number
    extends CustomNestedError2TwinSse {
  const CustomNestedError2TwinSse_CustomNested2Number(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinSse_CustomNested2NumberCopyWith<
          CustomNestedError2TwinSse_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinSse_CustomNested2NumberCopyWithImpl<
              CustomNestedError2TwinSse_CustomNested2Number>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinSse_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinSse.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinSse_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinSseCopyWith<$Res> {
  factory $CustomNestedError2TwinSse_CustomNested2NumberCopyWith(
          CustomNestedError2TwinSse_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinSse_CustomNested2Number) _then) =
      _$CustomNestedError2TwinSse_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinSse_CustomNested2NumberCopyWithImpl<$Res>
    implements $CustomNestedError2TwinSse_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinSse_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinSse_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinSse_CustomNested2Number) _then;

  /// Create a copy of CustomNestedError2TwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinSse_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinSseCopyWith<$Res> {
  $CustomNestedErrorInnerTwinSseCopyWith(CustomNestedErrorInnerTwinSse _,
      $Res Function(CustomNestedErrorInnerTwinSse) __);
}

/// @nodoc

class CustomNestedErrorInnerTwinSse_Three
    extends CustomNestedErrorInnerTwinSse {
  const CustomNestedErrorInnerTwinSse_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSse_ThreeCopyWith<
          CustomNestedErrorInnerTwinSse_Three>
      get copyWith => _$CustomNestedErrorInnerTwinSse_ThreeCopyWithImpl<
          CustomNestedErrorInnerTwinSse_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSse_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSse.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSse_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSse_ThreeCopyWith(
          CustomNestedErrorInnerTwinSse_Three value,
          $Res Function(CustomNestedErrorInnerTwinSse_Three) _then) =
      _$CustomNestedErrorInnerTwinSse_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSse_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSse_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSse_ThreeCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSse_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinSse_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSse_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinSse_Four extends CustomNestedErrorInnerTwinSse {
  const CustomNestedErrorInnerTwinSse_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSse_FourCopyWith<
          CustomNestedErrorInnerTwinSse_Four>
      get copyWith => _$CustomNestedErrorInnerTwinSse_FourCopyWithImpl<
          CustomNestedErrorInnerTwinSse_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinSse_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinSse.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinSse_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinSse_FourCopyWith(
          CustomNestedErrorInnerTwinSse_Four value,
          $Res Function(CustomNestedErrorInnerTwinSse_Four) _then) =
      _$CustomNestedErrorInnerTwinSse_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinSse_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinSse_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinSse_FourCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinSse_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinSse_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinSse_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinSseCopyWith<$Res> {
  $CustomNestedErrorOuterTwinSseCopyWith(CustomNestedErrorOuterTwinSse _,
      $Res Function(CustomNestedErrorOuterTwinSse) __);
}

/// @nodoc

class CustomNestedErrorOuterTwinSse_One extends CustomNestedErrorOuterTwinSse {
  const CustomNestedErrorOuterTwinSse_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSse_OneCopyWith<CustomNestedErrorOuterTwinSse_One>
      get copyWith => _$CustomNestedErrorOuterTwinSse_OneCopyWithImpl<
          CustomNestedErrorOuterTwinSse_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSse_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSse.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSse_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSse_OneCopyWith(
          CustomNestedErrorOuterTwinSse_One value,
          $Res Function(CustomNestedErrorOuterTwinSse_One) _then) =
      _$CustomNestedErrorOuterTwinSse_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSse_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSse_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSse_OneCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSse_One _self;
  final $Res Function(CustomNestedErrorOuterTwinSse_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSse_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinSse_Two extends CustomNestedErrorOuterTwinSse {
  const CustomNestedErrorOuterTwinSse_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinSse field0;

  /// Create a copy of CustomNestedErrorOuterTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinSse_TwoCopyWith<CustomNestedErrorOuterTwinSse_Two>
      get copyWith => _$CustomNestedErrorOuterTwinSse_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinSse_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinSse_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinSse.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinSse_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinSse_TwoCopyWith(
          CustomNestedErrorOuterTwinSse_Two value,
          $Res Function(CustomNestedErrorOuterTwinSse_Two) _then) =
      _$CustomNestedErrorOuterTwinSse_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinSse field0});

  $CustomNestedErrorInnerTwinSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinSse_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinSse_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinSse_TwoCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinSse_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinSse_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinSse_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinSse,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinSseCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinSseCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
