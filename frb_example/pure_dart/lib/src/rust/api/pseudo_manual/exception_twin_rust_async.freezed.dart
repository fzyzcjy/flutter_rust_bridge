// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinRustAsync {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsyncCopyWith<CustomEnumErrorTwinRustAsync>
      get copyWith => _$CustomEnumErrorTwinRustAsyncCopyWithImpl<
              CustomEnumErrorTwinRustAsync>(
          this as CustomEnumErrorTwinRustAsync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsync &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsync(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsyncCopyWith(
          CustomEnumErrorTwinRustAsync value,
          $Res Function(CustomEnumErrorTwinRustAsync) _then) =
      _$CustomEnumErrorTwinRustAsyncCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsyncCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsyncCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsync _self;
  final $Res Function(CustomEnumErrorTwinRustAsync) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsync
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

class CustomEnumErrorTwinRustAsync_One extends CustomEnumErrorTwinRustAsync
    implements FrbBacktracedException {
  const CustomEnumErrorTwinRustAsync_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsync_OneCopyWith<CustomEnumErrorTwinRustAsync_One>
      get copyWith => _$CustomEnumErrorTwinRustAsync_OneCopyWithImpl<
          CustomEnumErrorTwinRustAsync_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsync_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsync.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsync_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsync_OneCopyWith(
          CustomEnumErrorTwinRustAsync_One value,
          $Res Function(CustomEnumErrorTwinRustAsync_One) _then) =
      _$CustomEnumErrorTwinRustAsync_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsync_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsync_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsync_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsync_One _self;
  final $Res Function(CustomEnumErrorTwinRustAsync_One) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinRustAsync_One(
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

class CustomEnumErrorTwinRustAsync_Two extends CustomEnumErrorTwinRustAsync
    implements FrbBacktracedException {
  const CustomEnumErrorTwinRustAsync_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinRustAsync_TwoCopyWith<CustomEnumErrorTwinRustAsync_Two>
      get copyWith => _$CustomEnumErrorTwinRustAsync_TwoCopyWithImpl<
          CustomEnumErrorTwinRustAsync_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinRustAsync_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsync.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinRustAsync_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsync_TwoCopyWith(
          CustomEnumErrorTwinRustAsync_Two value,
          $Res Function(CustomEnumErrorTwinRustAsync_Two) _then) =
      _$CustomEnumErrorTwinRustAsync_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsync_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinRustAsync_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsync_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinRustAsync_Two _self;
  final $Res Function(CustomEnumErrorTwinRustAsync_Two) _then;

  /// Create a copy of CustomEnumErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinRustAsync_Two(
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
mixin _$CustomErrorTwinRustAsync {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsyncCopyWith<CustomErrorTwinRustAsync> get copyWith =>
      _$CustomErrorTwinRustAsyncCopyWithImpl<CustomErrorTwinRustAsync>(
          this as CustomErrorTwinRustAsync, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsync &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsync(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomErrorTwinRustAsyncCopyWith(CustomErrorTwinRustAsync value,
          $Res Function(CustomErrorTwinRustAsync) _then) =
      _$CustomErrorTwinRustAsyncCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsyncCopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  _$CustomErrorTwinRustAsyncCopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsync _self;
  final $Res Function(CustomErrorTwinRustAsync) _then;

  /// Create a copy of CustomErrorTwinRustAsync
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

class CustomErrorTwinRustAsync_Error0 extends CustomErrorTwinRustAsync
    implements FrbBacktracedException {
  const CustomErrorTwinRustAsync_Error0(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsync_Error0CopyWith<CustomErrorTwinRustAsync_Error0>
      get copyWith => _$CustomErrorTwinRustAsync_Error0CopyWithImpl<
          CustomErrorTwinRustAsync_Error0>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsync_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsync.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsync_Error0CopyWith<$Res>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomErrorTwinRustAsync_Error0CopyWith(
          CustomErrorTwinRustAsync_Error0 value,
          $Res Function(CustomErrorTwinRustAsync_Error0) _then) =
      _$CustomErrorTwinRustAsync_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsync_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsync_Error0CopyWith<$Res> {
  _$CustomErrorTwinRustAsync_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsync_Error0 _self;
  final $Res Function(CustomErrorTwinRustAsync_Error0) _then;

  /// Create a copy of CustomErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinRustAsync_Error0(
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

class CustomErrorTwinRustAsync_Error1 extends CustomErrorTwinRustAsync
    implements FrbBacktracedException {
  const CustomErrorTwinRustAsync_Error1(
      {required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinRustAsync_Error1CopyWith<CustomErrorTwinRustAsync_Error1>
      get copyWith => _$CustomErrorTwinRustAsync_Error1CopyWithImpl<
          CustomErrorTwinRustAsync_Error1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinRustAsync_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinRustAsync.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinRustAsync_Error1CopyWith<$Res>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomErrorTwinRustAsync_Error1CopyWith(
          CustomErrorTwinRustAsync_Error1 value,
          $Res Function(CustomErrorTwinRustAsync_Error1) _then) =
      _$CustomErrorTwinRustAsync_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsync_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinRustAsync_Error1CopyWith<$Res> {
  _$CustomErrorTwinRustAsync_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinRustAsync_Error1 _self;
  final $Res Function(CustomErrorTwinRustAsync_Error1) _then;

  /// Create a copy of CustomErrorTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinRustAsync_Error1(
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
mixin _$CustomNestedError1TwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinRustAsyncCopyWith<$Res> {
  $CustomNestedError1TwinRustAsyncCopyWith(CustomNestedError1TwinRustAsync _,
      $Res Function(CustomNestedError1TwinRustAsync) __);
}

/// @nodoc

class CustomNestedError1TwinRustAsync_CustomNested1
    extends CustomNestedError1TwinRustAsync {
  const CustomNestedError1TwinRustAsync_CustomNested1(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinRustAsync_CustomNested1CopyWith<
          CustomNestedError1TwinRustAsync_CustomNested1>
      get copyWith =>
          _$CustomNestedError1TwinRustAsync_CustomNested1CopyWithImpl<
              CustomNestedError1TwinRustAsync_CustomNested1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsync_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsync.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinRustAsync_CustomNested1CopyWith<
    $Res> implements $CustomNestedError1TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError1TwinRustAsync_CustomNested1CopyWith(
          CustomNestedError1TwinRustAsync_CustomNested1 value,
          $Res Function(CustomNestedError1TwinRustAsync_CustomNested1) _then) =
      _$CustomNestedError1TwinRustAsync_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinRustAsync_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinRustAsync_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinRustAsync_CustomNested1CopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinRustAsync_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinRustAsync_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinRustAsync_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinRustAsync_ErrorNested
    extends CustomNestedError1TwinRustAsync {
  const CustomNestedError1TwinRustAsync_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinRustAsync field0;

  /// Create a copy of CustomNestedError1TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinRustAsync_ErrorNestedCopyWith<
          CustomNestedError1TwinRustAsync_ErrorNested>
      get copyWith => _$CustomNestedError1TwinRustAsync_ErrorNestedCopyWithImpl<
          CustomNestedError1TwinRustAsync_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinRustAsync_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsync.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinRustAsync_ErrorNestedCopyWith<$Res>
    implements $CustomNestedError1TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError1TwinRustAsync_ErrorNestedCopyWith(
          CustomNestedError1TwinRustAsync_ErrorNested value,
          $Res Function(CustomNestedError1TwinRustAsync_ErrorNested) _then) =
      _$CustomNestedError1TwinRustAsync_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinRustAsync field0});

  $CustomNestedError2TwinRustAsyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinRustAsync_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinRustAsync_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinRustAsync_ErrorNestedCopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinRustAsync_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinRustAsync_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinRustAsync_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinRustAsync,
    ));
  }

  /// Create a copy of CustomNestedError1TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsyncCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinRustAsyncCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinRustAsyncCopyWith<$Res> {
  $CustomNestedError2TwinRustAsyncCopyWith(CustomNestedError2TwinRustAsync _,
      $Res Function(CustomNestedError2TwinRustAsync) __);
}

/// @nodoc

class CustomNestedError2TwinRustAsync_CustomNested2
    extends CustomNestedError2TwinRustAsync {
  const CustomNestedError2TwinRustAsync_CustomNested2(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsync_CustomNested2CopyWith<
          CustomNestedError2TwinRustAsync_CustomNested2>
      get copyWith =>
          _$CustomNestedError2TwinRustAsync_CustomNested2CopyWithImpl<
              CustomNestedError2TwinRustAsync_CustomNested2>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsync_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsync.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinRustAsync_CustomNested2CopyWith<
    $Res> implements $CustomNestedError2TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError2TwinRustAsync_CustomNested2CopyWith(
          CustomNestedError2TwinRustAsync_CustomNested2 value,
          $Res Function(CustomNestedError2TwinRustAsync_CustomNested2) _then) =
      _$CustomNestedError2TwinRustAsync_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinRustAsync_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinRustAsync_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinRustAsync_CustomNested2CopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinRustAsync_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinRustAsync_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinRustAsync_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinRustAsync_CustomNested2Number
    extends CustomNestedError2TwinRustAsync {
  const CustomNestedError2TwinRustAsync_CustomNested2Number(this.field0)
      : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWith<
          CustomNestedError2TwinRustAsync_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWithImpl<
                  CustomNestedError2TwinRustAsync_CustomNested2Number>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinRustAsync_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsync.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWith(
          CustomNestedError2TwinRustAsync_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinRustAsync_CustomNested2Number)
              _then) =
      _$CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWithImpl<$Res>
    implements
        $CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinRustAsync_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinRustAsync_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinRustAsync_CustomNested2Number)
      _then;

  /// Create a copy of CustomNestedError2TwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinRustAsync_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> {
  $CustomNestedErrorInnerTwinRustAsyncCopyWith(
      CustomNestedErrorInnerTwinRustAsync _,
      $Res Function(CustomNestedErrorInnerTwinRustAsync) __);
}

/// @nodoc

class CustomNestedErrorInnerTwinRustAsync_Three
    extends CustomNestedErrorInnerTwinRustAsync {
  const CustomNestedErrorInnerTwinRustAsync_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsync_ThreeCopyWith<
          CustomNestedErrorInnerTwinRustAsync_Three>
      get copyWith => _$CustomNestedErrorInnerTwinRustAsync_ThreeCopyWithImpl<
          CustomNestedErrorInnerTwinRustAsync_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsync_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsync.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinRustAsync_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinRustAsync_ThreeCopyWith(
          CustomNestedErrorInnerTwinRustAsync_Three value,
          $Res Function(CustomNestedErrorInnerTwinRustAsync_Three) _then) =
      _$CustomNestedErrorInnerTwinRustAsync_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinRustAsync_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinRustAsync_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinRustAsync_ThreeCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorInnerTwinRustAsync_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinRustAsync_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinRustAsync_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinRustAsync_Four
    extends CustomNestedErrorInnerTwinRustAsync {
  const CustomNestedErrorInnerTwinRustAsync_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsync_FourCopyWith<
          CustomNestedErrorInnerTwinRustAsync_Four>
      get copyWith => _$CustomNestedErrorInnerTwinRustAsync_FourCopyWithImpl<
          CustomNestedErrorInnerTwinRustAsync_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinRustAsync_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsync.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinRustAsync_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinRustAsync_FourCopyWith(
          CustomNestedErrorInnerTwinRustAsync_Four value,
          $Res Function(CustomNestedErrorInnerTwinRustAsync_Four) _then) =
      _$CustomNestedErrorInnerTwinRustAsync_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinRustAsync_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinRustAsync_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinRustAsync_FourCopyWithImpl(
      this._self, this._then);

  final CustomNestedErrorInnerTwinRustAsync_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinRustAsync_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinRustAsync_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinRustAsync {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsync &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsync(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinRustAsyncCopyWith<$Res> {
  $CustomNestedErrorOuterTwinRustAsyncCopyWith(
      CustomNestedErrorOuterTwinRustAsync _,
      $Res Function(CustomNestedErrorOuterTwinRustAsync) __);
}

/// @nodoc

class CustomNestedErrorOuterTwinRustAsync_One
    extends CustomNestedErrorOuterTwinRustAsync {
  const CustomNestedErrorOuterTwinRustAsync_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinRustAsync_OneCopyWith<
          CustomNestedErrorOuterTwinRustAsync_One>
      get copyWith => _$CustomNestedErrorOuterTwinRustAsync_OneCopyWithImpl<
          CustomNestedErrorOuterTwinRustAsync_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsync_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsync.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinRustAsync_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinRustAsync_OneCopyWith(
          CustomNestedErrorOuterTwinRustAsync_One value,
          $Res Function(CustomNestedErrorOuterTwinRustAsync_One) _then) =
      _$CustomNestedErrorOuterTwinRustAsync_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinRustAsync_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinRustAsync_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinRustAsync_OneCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinRustAsync_One _self;
  final $Res Function(CustomNestedErrorOuterTwinRustAsync_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinRustAsync_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinRustAsync_Two
    extends CustomNestedErrorOuterTwinRustAsync {
  const CustomNestedErrorOuterTwinRustAsync_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinRustAsync field0;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinRustAsync_TwoCopyWith<
          CustomNestedErrorOuterTwinRustAsync_Two>
      get copyWith => _$CustomNestedErrorOuterTwinRustAsync_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinRustAsync_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinRustAsync_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsync.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinRustAsync_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinRustAsync_TwoCopyWith(
          CustomNestedErrorOuterTwinRustAsync_Two value,
          $Res Function(CustomNestedErrorOuterTwinRustAsync_Two) _then) =
      _$CustomNestedErrorOuterTwinRustAsync_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinRustAsync field0});

  $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinRustAsync_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinRustAsync_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinRustAsync_TwoCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinRustAsync_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinRustAsync_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinRustAsync_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinRustAsync,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
