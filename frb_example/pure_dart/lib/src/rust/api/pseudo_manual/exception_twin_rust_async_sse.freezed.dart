// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_rust_async_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinRustAsyncSse {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsyncSseCopyWith<CustomEnumErrorTwinRustAsyncSse>
      get copyWith => _$CustomEnumErrorTwinRustAsyncSseCopyWithImpl<
              CustomEnumErrorTwinRustAsyncSse>(
          this as CustomEnumErrorTwinRustAsyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsyncSse(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsyncSseCopyWith(
          CustomEnumErrorTwinRustAsyncSse value,
          $Res Function(CustomEnumErrorTwinRustAsyncSse) _then) =
      _$CustomEnumErrorTwinRustAsyncSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsyncSseCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsyncSseCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsyncSse _self;
  final $Res Function(CustomEnumErrorTwinRustAsyncSse) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
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

class CustomEnumErrorTwinRustAsyncSse_One
    extends CustomEnumErrorTwinRustAsyncSse implements FrbBacktracedException {
  const CustomEnumErrorTwinRustAsyncSse_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsyncSse_OneCopyWith<
          CustomEnumErrorTwinRustAsyncSse_One>
      get copyWith => _$CustomEnumErrorTwinRustAsyncSse_OneCopyWithImpl<
          CustomEnumErrorTwinRustAsyncSse_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsyncSse_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsyncSse.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsyncSse_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsyncSse_OneCopyWith(
          CustomEnumErrorTwinRustAsyncSse_One value,
          $Res Function(CustomEnumErrorTwinRustAsyncSse_One) _then) =
      _$CustomEnumErrorTwinRustAsyncSse_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsyncSse_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsyncSse_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsyncSse_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsyncSse_One _self;
  final $Res Function(CustomEnumErrorTwinRustAsyncSse_One) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinRustAsyncSse_One(
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

class CustomEnumErrorTwinRustAsyncSse_Two
    extends CustomEnumErrorTwinRustAsyncSse implements FrbBacktracedException {
  const CustomEnumErrorTwinRustAsyncSse_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsyncSse_TwoCopyWith<
          CustomEnumErrorTwinRustAsyncSse_Two>
      get copyWith => _$CustomEnumErrorTwinRustAsyncSse_TwoCopyWithImpl<
          CustomEnumErrorTwinRustAsyncSse_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsyncSse_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsyncSse.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsyncSse_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsyncSse_TwoCopyWith(
          CustomEnumErrorTwinRustAsyncSse_Two value,
          $Res Function(CustomEnumErrorTwinRustAsyncSse_Two) _then) =
      _$CustomEnumErrorTwinRustAsyncSse_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsyncSse_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsyncSse_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsyncSse_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsyncSse_Two _self;
  final $Res Function(CustomEnumErrorTwinRustAsyncSse_Two) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinRustAsyncSse_Two(
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
mixin _$CustomErrorTwinRustAsyncSse {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsyncSseCopyWith<CustomErrorTwinRustAsyncSse>
      get copyWith => _$CustomErrorTwinRustAsyncSseCopyWithImpl<
              CustomErrorTwinRustAsyncSse>(
          this as CustomErrorTwinRustAsyncSse, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsyncSse(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomErrorTwinRustAsyncSseCopyWith(
          CustomErrorTwinRustAsyncSse value,
          $Res Function(CustomErrorTwinRustAsyncSse) _then) =
      _$CustomErrorTwinRustAsyncSseCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsyncSseCopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsyncSseCopyWith<$Res> {
  _$CustomErrorTwinRustAsyncSseCopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsyncSse _self;
  final $Res Function(CustomErrorTwinRustAsyncSse) _then;

  /// Create a copy of CustomErrorTwinRustAsyncSse
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

class CustomErrorTwinRustAsyncSse_Error0 extends CustomErrorTwinRustAsyncSse
    implements FrbBacktracedException {
  const CustomErrorTwinRustAsyncSse_Error0(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsyncSse_Error0CopyWith<
          CustomErrorTwinRustAsyncSse_Error0>
      get copyWith => _$CustomErrorTwinRustAsyncSse_Error0CopyWithImpl<
          CustomErrorTwinRustAsyncSse_Error0>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsyncSse_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsyncSse.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsyncSse_Error0CopyWith<$Res>
    implements $CustomErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomErrorTwinRustAsyncSse_Error0CopyWith(
          CustomErrorTwinRustAsyncSse_Error0 value,
          $Res Function(CustomErrorTwinRustAsyncSse_Error0) _then) =
      _$CustomErrorTwinRustAsyncSse_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsyncSse_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsyncSse_Error0CopyWith<$Res> {
  _$CustomErrorTwinRustAsyncSse_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsyncSse_Error0 _self;
  final $Res Function(CustomErrorTwinRustAsyncSse_Error0) _then;

  /// Create a copy of CustomErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinRustAsyncSse_Error0(
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

class CustomErrorTwinRustAsyncSse_Error1 extends CustomErrorTwinRustAsyncSse
    implements FrbBacktracedException {
  const CustomErrorTwinRustAsyncSse_Error1(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsyncSse_Error1CopyWith<
          CustomErrorTwinRustAsyncSse_Error1>
      get copyWith => _$CustomErrorTwinRustAsyncSse_Error1CopyWithImpl<
          CustomErrorTwinRustAsyncSse_Error1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsyncSse_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsyncSse.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsyncSse_Error1CopyWith<$Res>
    implements $CustomErrorTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomErrorTwinRustAsyncSse_Error1CopyWith(
          CustomErrorTwinRustAsyncSse_Error1 value,
          $Res Function(CustomErrorTwinRustAsyncSse_Error1) _then) =
      _$CustomErrorTwinRustAsyncSse_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsyncSse_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsyncSse_Error1CopyWith<$Res> {
  _$CustomErrorTwinRustAsyncSse_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsyncSse_Error1 _self;
  final $Res Function(CustomErrorTwinRustAsyncSse_Error1) _then;

  /// Create a copy of CustomErrorTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinRustAsyncSse_Error1(
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
mixin _$CustomNestedError1TwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinRustAsyncSseCopyWith<$Res> {
  $CustomNestedError1TwinRustAsyncSseCopyWith(
      CustomNestedError1TwinRustAsyncSse _,
      $Res Function(CustomNestedError1TwinRustAsyncSse) __);
}

/// @nodoc

class CustomNestedError1TwinRustAsyncSse_CustomNested1
    extends CustomNestedError1TwinRustAsyncSse {
  const CustomNestedError1TwinRustAsyncSse_CustomNested1(this.field0)
      : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWith<
          CustomNestedError1TwinRustAsyncSse_CustomNested1>
      get copyWith =>
          _$CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWithImpl<
                  CustomNestedError1TwinRustAsyncSse_CustomNested1>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsyncSse_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsyncSse.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWith<
    $Res> implements $CustomNestedError1TwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWith(
          CustomNestedError1TwinRustAsyncSse_CustomNested1 value,
          $Res Function(CustomNestedError1TwinRustAsyncSse_CustomNested1)
              _then) =
      _$CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinRustAsyncSse_CustomNested1CopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinRustAsyncSse_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinRustAsyncSse_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinRustAsyncSse_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinRustAsyncSse_ErrorNested
    extends CustomNestedError1TwinRustAsyncSse {
  const CustomNestedError1TwinRustAsyncSse_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinRustAsyncSse field0;

  /// Create a copy of CustomNestedError1TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWith<
          CustomNestedError1TwinRustAsyncSse_ErrorNested>
      get copyWith =>
          _$CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWithImpl<
              CustomNestedError1TwinRustAsyncSse_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsyncSse_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsyncSse.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWith<
    $Res> implements $CustomNestedError1TwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWith(
          CustomNestedError1TwinRustAsyncSse_ErrorNested value,
          $Res Function(CustomNestedError1TwinRustAsyncSse_ErrorNested) _then) =
      _$CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinRustAsyncSse field0});

  $CustomNestedError2TwinRustAsyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinRustAsyncSse_ErrorNestedCopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinRustAsyncSse_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinRustAsyncSse_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinRustAsyncSse_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinRustAsyncSse,
    ));
  }

  /// Create a copy of CustomNestedError1TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsyncSseCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinRustAsyncSseCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinRustAsyncSseCopyWith<$Res> {
  $CustomNestedError2TwinRustAsyncSseCopyWith(
      CustomNestedError2TwinRustAsyncSse _,
      $Res Function(CustomNestedError2TwinRustAsyncSse) __);
}

/// @nodoc

class CustomNestedError2TwinRustAsyncSse_CustomNested2
    extends CustomNestedError2TwinRustAsyncSse {
  const CustomNestedError2TwinRustAsyncSse_CustomNested2(this.field0)
      : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWith<
          CustomNestedError2TwinRustAsyncSse_CustomNested2>
      get copyWith =>
          _$CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWithImpl<
                  CustomNestedError2TwinRustAsyncSse_CustomNested2>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsyncSse_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsyncSse.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWith<
    $Res> implements $CustomNestedError2TwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWith(
          CustomNestedError2TwinRustAsyncSse_CustomNested2 value,
          $Res Function(CustomNestedError2TwinRustAsyncSse_CustomNested2)
              _then) =
      _$CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinRustAsyncSse_CustomNested2CopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinRustAsyncSse_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinRustAsyncSse_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinRustAsyncSse_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinRustAsyncSse_CustomNested2Number
    extends CustomNestedError2TwinRustAsyncSse {
  const CustomNestedError2TwinRustAsyncSse_CustomNested2Number(this.field0)
      : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWith<
          CustomNestedError2TwinRustAsyncSse_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWithImpl<
                  CustomNestedError2TwinRustAsyncSse_CustomNested2Number>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsyncSse_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsyncSse.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWith(
          CustomNestedError2TwinRustAsyncSse_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinRustAsyncSse_CustomNested2Number)
              _then) =
      _$CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWithImpl<$Res>
    implements
        $CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinRustAsyncSse_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinRustAsyncSse_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinRustAsyncSse_CustomNested2Number)
      _then;

  /// Create a copy of CustomNestedError2TwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinRustAsyncSse_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res> {
  $CustomNestedErrorInnerTwinRustAsyncSseCopyWith(
      CustomNestedErrorInnerTwinRustAsyncSse _,
      $Res Function(CustomNestedErrorInnerTwinRustAsyncSse) __);
}

/// @nodoc

class CustomNestedErrorInnerTwinRustAsyncSse_Three
    extends CustomNestedErrorInnerTwinRustAsyncSse {
  const CustomNestedErrorInnerTwinRustAsyncSse_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWith<
          CustomNestedErrorInnerTwinRustAsyncSse_Three>
      get copyWith =>
          _$CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWithImpl<
              CustomNestedErrorInnerTwinRustAsyncSse_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsyncSse_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsyncSse.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWith(
          CustomNestedErrorInnerTwinRustAsyncSse_Three value,
          $Res Function(CustomNestedErrorInnerTwinRustAsyncSse_Three) _then) =
      _$CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinRustAsyncSse_ThreeCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorInnerTwinRustAsyncSse_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinRustAsyncSse_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinRustAsyncSse_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinRustAsyncSse_Four
    extends CustomNestedErrorInnerTwinRustAsyncSse {
  const CustomNestedErrorInnerTwinRustAsyncSse_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWith<
          CustomNestedErrorInnerTwinRustAsyncSse_Four>
      get copyWith => _$CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWithImpl<
          CustomNestedErrorInnerTwinRustAsyncSse_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsyncSse_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsyncSse.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWith(
          CustomNestedErrorInnerTwinRustAsyncSse_Four value,
          $Res Function(CustomNestedErrorInnerTwinRustAsyncSse_Four) _then) =
      _$CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinRustAsyncSse_FourCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorInnerTwinRustAsyncSse_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinRustAsyncSse_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinRustAsyncSse_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinRustAsyncSse {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsyncSse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsyncSse(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinRustAsyncSseCopyWith<$Res> {
  $CustomNestedErrorOuterTwinRustAsyncSseCopyWith(
      CustomNestedErrorOuterTwinRustAsyncSse _,
      $Res Function(CustomNestedErrorOuterTwinRustAsyncSse) __);
}

/// @nodoc

class CustomNestedErrorOuterTwinRustAsyncSse_One
    extends CustomNestedErrorOuterTwinRustAsyncSse {
  const CustomNestedErrorOuterTwinRustAsyncSse_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWith<
          CustomNestedErrorOuterTwinRustAsyncSse_One>
      get copyWith => _$CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWithImpl<
          CustomNestedErrorOuterTwinRustAsyncSse_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsyncSse_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsyncSse.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWith(
          CustomNestedErrorOuterTwinRustAsyncSse_One value,
          $Res Function(CustomNestedErrorOuterTwinRustAsyncSse_One) _then) =
      _$CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinRustAsyncSse_OneCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorOuterTwinRustAsyncSse_One _self;
  final $Res Function(CustomNestedErrorOuterTwinRustAsyncSse_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinRustAsyncSse_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinRustAsyncSse_Two
    extends CustomNestedErrorOuterTwinRustAsyncSse {
  const CustomNestedErrorOuterTwinRustAsyncSse_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinRustAsyncSse field0;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWith<
          CustomNestedErrorOuterTwinRustAsyncSse_Two>
      get copyWith => _$CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinRustAsyncSse_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsyncSse_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsyncSse.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncSseCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWith(
          CustomNestedErrorOuterTwinRustAsyncSse_Two value,
          $Res Function(CustomNestedErrorOuterTwinRustAsyncSse_Two) _then) =
      _$CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinRustAsyncSse field0});

  $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinRustAsyncSse_TwoCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorOuterTwinRustAsyncSse_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinRustAsyncSse_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinRustAsyncSse_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinRustAsyncSse,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinRustAsyncSse
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinRustAsyncSseCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
