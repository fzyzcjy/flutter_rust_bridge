// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CustomEnumErrorTwinNormal {
  Object get message;
  String get backtrace;

  /// Create a copy of CustomEnumErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinNormalCopyWith<CustomEnumErrorTwinNormal> get copyWith =>
      _$CustomEnumErrorTwinNormalCopyWithImpl<CustomEnumErrorTwinNormal>(
          this as CustomEnumErrorTwinNormal, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinNormal &&
            const DeepCollectionEquality().equals(other.message, message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(message), backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinNormal(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinNormalCopyWith<$Res> {
  factory $CustomEnumErrorTwinNormalCopyWith(CustomEnumErrorTwinNormal value,
          $Res Function(CustomEnumErrorTwinNormal) _then) =
      _$CustomEnumErrorTwinNormalCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinNormalCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinNormalCopyWith<$Res> {
  _$CustomEnumErrorTwinNormalCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinNormal _self;
  final $Res Function(CustomEnumErrorTwinNormal) _then;

  /// Create a copy of CustomEnumErrorTwinNormal
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

/// Adds pattern-matching-related methods to [CustomEnumErrorTwinNormal].
extension CustomEnumErrorTwinNormalPatterns on CustomEnumErrorTwinNormal {
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
    TResult Function(CustomEnumErrorTwinNormal_One value)? one,
    TResult Function(CustomEnumErrorTwinNormal_Two value)? two,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One() when one != null:
        return one(_that);
      case CustomEnumErrorTwinNormal_Two() when two != null:
        return two(_that);
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
    required TResult Function(CustomEnumErrorTwinNormal_One value) one,
    required TResult Function(CustomEnumErrorTwinNormal_Two value) two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One():
        return one(_that);
      case CustomEnumErrorTwinNormal_Two():
        return two(_that);
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
    TResult? Function(CustomEnumErrorTwinNormal_One value)? one,
    TResult? Function(CustomEnumErrorTwinNormal_Two value)? two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One() when one != null:
        return one(_that);
      case CustomEnumErrorTwinNormal_Two() when two != null:
        return two(_that);
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
    TResult Function(String message, String backtrace)? one,
    TResult Function(int message, String backtrace)? two,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One() when one != null:
        return one(_that.message, _that.backtrace);
      case CustomEnumErrorTwinNormal_Two() when two != null:
        return two(_that.message, _that.backtrace);
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
    required TResult Function(String message, String backtrace) one,
    required TResult Function(int message, String backtrace) two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One():
        return one(_that.message, _that.backtrace);
      case CustomEnumErrorTwinNormal_Two():
        return two(_that.message, _that.backtrace);
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
    TResult? Function(String message, String backtrace)? one,
    TResult? Function(int message, String backtrace)? two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomEnumErrorTwinNormal_One() when one != null:
        return one(_that.message, _that.backtrace);
      case CustomEnumErrorTwinNormal_Two() when two != null:
        return two(_that.message, _that.backtrace);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomEnumErrorTwinNormal_One extends CustomEnumErrorTwinNormal
    implements FrbBacktracedException {
  const CustomEnumErrorTwinNormal_One(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final String message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinNormal_OneCopyWith<CustomEnumErrorTwinNormal_One>
      get copyWith => _$CustomEnumErrorTwinNormal_OneCopyWithImpl<
          CustomEnumErrorTwinNormal_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinNormal_One &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinNormal.one(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinNormal_OneCopyWith<$Res>
    implements $CustomEnumErrorTwinNormalCopyWith<$Res> {
  factory $CustomEnumErrorTwinNormal_OneCopyWith(
          CustomEnumErrorTwinNormal_One value,
          $Res Function(CustomEnumErrorTwinNormal_One) _then) =
      _$CustomEnumErrorTwinNormal_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinNormal_OneCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinNormal_OneCopyWith<$Res> {
  _$CustomEnumErrorTwinNormal_OneCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinNormal_One _self;
  final $Res Function(CustomEnumErrorTwinNormal_One) _then;

  /// Create a copy of CustomEnumErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinNormal_One(
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

class CustomEnumErrorTwinNormal_Two extends CustomEnumErrorTwinNormal
    implements FrbBacktracedException {
  const CustomEnumErrorTwinNormal_Two(
      {required this.message, required this.backtrace})
      : super._();

  @override
  final int message;
  @override
  final String backtrace;

  /// Create a copy of CustomEnumErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomEnumErrorTwinNormal_TwoCopyWith<CustomEnumErrorTwinNormal_Two>
      get copyWith => _$CustomEnumErrorTwinNormal_TwoCopyWithImpl<
          CustomEnumErrorTwinNormal_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomEnumErrorTwinNormal_Two &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @override
  String toString() {
    return 'CustomEnumErrorTwinNormal.two(message: $message, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomEnumErrorTwinNormal_TwoCopyWith<$Res>
    implements $CustomEnumErrorTwinNormalCopyWith<$Res> {
  factory $CustomEnumErrorTwinNormal_TwoCopyWith(
          CustomEnumErrorTwinNormal_Two value,
          $Res Function(CustomEnumErrorTwinNormal_Two) _then) =
      _$CustomEnumErrorTwinNormal_TwoCopyWithImpl;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinNormal_TwoCopyWithImpl<$Res>
    implements $CustomEnumErrorTwinNormal_TwoCopyWith<$Res> {
  _$CustomEnumErrorTwinNormal_TwoCopyWithImpl(this._self, this._then);

  final CustomEnumErrorTwinNormal_Two _self;
  final $Res Function(CustomEnumErrorTwinNormal_Two) _then;

  /// Create a copy of CustomEnumErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(CustomEnumErrorTwinNormal_Two(
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
mixin _$CustomErrorTwinNormal {
  Object get e;
  String get backtrace;

  /// Create a copy of CustomErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinNormalCopyWith<CustomErrorTwinNormal> get copyWith =>
      _$CustomErrorTwinNormalCopyWithImpl<CustomErrorTwinNormal>(
          this as CustomErrorTwinNormal, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinNormal &&
            const DeepCollectionEquality().equals(other.e, e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(e), backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinNormal(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinNormalCopyWith<$Res> {
  factory $CustomErrorTwinNormalCopyWith(CustomErrorTwinNormal value,
          $Res Function(CustomErrorTwinNormal) _then) =
      _$CustomErrorTwinNormalCopyWithImpl;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinNormalCopyWithImpl<$Res>
    implements $CustomErrorTwinNormalCopyWith<$Res> {
  _$CustomErrorTwinNormalCopyWithImpl(this._self, this._then);

  final CustomErrorTwinNormal _self;
  final $Res Function(CustomErrorTwinNormal) _then;

  /// Create a copy of CustomErrorTwinNormal
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

/// Adds pattern-matching-related methods to [CustomErrorTwinNormal].
extension CustomErrorTwinNormalPatterns on CustomErrorTwinNormal {
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
    TResult Function(CustomErrorTwinNormal_Error0 value)? error0,
    TResult Function(CustomErrorTwinNormal_Error1 value)? error1,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0() when error0 != null:
        return error0(_that);
      case CustomErrorTwinNormal_Error1() when error1 != null:
        return error1(_that);
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
    required TResult Function(CustomErrorTwinNormal_Error0 value) error0,
    required TResult Function(CustomErrorTwinNormal_Error1 value) error1,
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0():
        return error0(_that);
      case CustomErrorTwinNormal_Error1():
        return error1(_that);
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
    TResult? Function(CustomErrorTwinNormal_Error0 value)? error0,
    TResult? Function(CustomErrorTwinNormal_Error1 value)? error1,
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0() when error0 != null:
        return error0(_that);
      case CustomErrorTwinNormal_Error1() when error1 != null:
        return error1(_that);
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
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0() when error0 != null:
        return error0(_that.e, _that.backtrace);
      case CustomErrorTwinNormal_Error1() when error1 != null:
        return error1(_that.e, _that.backtrace);
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
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0():
        return error0(_that.e, _that.backtrace);
      case CustomErrorTwinNormal_Error1():
        return error1(_that.e, _that.backtrace);
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
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) {
    final _that = this;
    switch (_that) {
      case CustomErrorTwinNormal_Error0() when error0 != null:
        return error0(_that.e, _that.backtrace);
      case CustomErrorTwinNormal_Error1() when error1 != null:
        return error1(_that.e, _that.backtrace);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomErrorTwinNormal_Error0 extends CustomErrorTwinNormal
    implements FrbBacktracedException {
  const CustomErrorTwinNormal_Error0({required this.e, required this.backtrace})
      : super._();

  @override
  final String e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinNormal_Error0CopyWith<CustomErrorTwinNormal_Error0>
      get copyWith => _$CustomErrorTwinNormal_Error0CopyWithImpl<
          CustomErrorTwinNormal_Error0>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinNormal_Error0 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinNormal.error0(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinNormal_Error0CopyWith<$Res>
    implements $CustomErrorTwinNormalCopyWith<$Res> {
  factory $CustomErrorTwinNormal_Error0CopyWith(
          CustomErrorTwinNormal_Error0 value,
          $Res Function(CustomErrorTwinNormal_Error0) _then) =
      _$CustomErrorTwinNormal_Error0CopyWithImpl;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinNormal_Error0CopyWithImpl<$Res>
    implements $CustomErrorTwinNormal_Error0CopyWith<$Res> {
  _$CustomErrorTwinNormal_Error0CopyWithImpl(this._self, this._then);

  final CustomErrorTwinNormal_Error0 _self;
  final $Res Function(CustomErrorTwinNormal_Error0) _then;

  /// Create a copy of CustomErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinNormal_Error0(
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

class CustomErrorTwinNormal_Error1 extends CustomErrorTwinNormal
    implements FrbBacktracedException {
  const CustomErrorTwinNormal_Error1({required this.e, required this.backtrace})
      : super._();

  @override
  final int e;
  @override
  final String backtrace;

  /// Create a copy of CustomErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomErrorTwinNormal_Error1CopyWith<CustomErrorTwinNormal_Error1>
      get copyWith => _$CustomErrorTwinNormal_Error1CopyWithImpl<
          CustomErrorTwinNormal_Error1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomErrorTwinNormal_Error1 &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @override
  String toString() {
    return 'CustomErrorTwinNormal.error1(e: $e, backtrace: $backtrace)';
  }
}

/// @nodoc
abstract mixin class $CustomErrorTwinNormal_Error1CopyWith<$Res>
    implements $CustomErrorTwinNormalCopyWith<$Res> {
  factory $CustomErrorTwinNormal_Error1CopyWith(
          CustomErrorTwinNormal_Error1 value,
          $Res Function(CustomErrorTwinNormal_Error1) _then) =
      _$CustomErrorTwinNormal_Error1CopyWithImpl;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class _$CustomErrorTwinNormal_Error1CopyWithImpl<$Res>
    implements $CustomErrorTwinNormal_Error1CopyWith<$Res> {
  _$CustomErrorTwinNormal_Error1CopyWithImpl(this._self, this._then);

  final CustomErrorTwinNormal_Error1 _self;
  final $Res Function(CustomErrorTwinNormal_Error1) _then;

  /// Create a copy of CustomErrorTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(CustomErrorTwinNormal_Error1(
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
mixin _$CustomNestedError1TwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError1TwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError1TwinNormalCopyWith<$Res> {
  $CustomNestedError1TwinNormalCopyWith(CustomNestedError1TwinNormal _,
      $Res Function(CustomNestedError1TwinNormal) __);
}

/// Adds pattern-matching-related methods to [CustomNestedError1TwinNormal].
extension CustomNestedError1TwinNormalPatterns on CustomNestedError1TwinNormal {
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
    TResult Function(CustomNestedError1TwinNormal_CustomNested1 value)?
        customNested1,
    TResult Function(CustomNestedError1TwinNormal_ErrorNested value)?
        errorNested,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1()
          when customNested1 != null:
        return customNested1(_that);
      case CustomNestedError1TwinNormal_ErrorNested() when errorNested != null:
        return errorNested(_that);
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
    required TResult Function(CustomNestedError1TwinNormal_CustomNested1 value)
        customNested1,
    required TResult Function(CustomNestedError1TwinNormal_ErrorNested value)
        errorNested,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1():
        return customNested1(_that);
      case CustomNestedError1TwinNormal_ErrorNested():
        return errorNested(_that);
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
    TResult? Function(CustomNestedError1TwinNormal_CustomNested1 value)?
        customNested1,
    TResult? Function(CustomNestedError1TwinNormal_ErrorNested value)?
        errorNested,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1()
          when customNested1 != null:
        return customNested1(_that);
      case CustomNestedError1TwinNormal_ErrorNested() when errorNested != null:
        return errorNested(_that);
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
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2TwinNormal field0)? errorNested,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1()
          when customNested1 != null:
        return customNested1(_that.field0);
      case CustomNestedError1TwinNormal_ErrorNested() when errorNested != null:
        return errorNested(_that.field0);
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
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2TwinNormal field0) errorNested,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1():
        return customNested1(_that.field0);
      case CustomNestedError1TwinNormal_ErrorNested():
        return errorNested(_that.field0);
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
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2TwinNormal field0)? errorNested,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError1TwinNormal_CustomNested1()
          when customNested1 != null:
        return customNested1(_that.field0);
      case CustomNestedError1TwinNormal_ErrorNested() when errorNested != null:
        return errorNested(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomNestedError1TwinNormal_CustomNested1
    extends CustomNestedError1TwinNormal {
  const CustomNestedError1TwinNormal_CustomNested1(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError1TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinNormal_CustomNested1CopyWith<
          CustomNestedError1TwinNormal_CustomNested1>
      get copyWith => _$CustomNestedError1TwinNormal_CustomNested1CopyWithImpl<
          CustomNestedError1TwinNormal_CustomNested1>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinNormal_CustomNested1 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinNormal.customNested1(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinNormal_CustomNested1CopyWith<$Res>
    implements $CustomNestedError1TwinNormalCopyWith<$Res> {
  factory $CustomNestedError1TwinNormal_CustomNested1CopyWith(
          CustomNestedError1TwinNormal_CustomNested1 value,
          $Res Function(CustomNestedError1TwinNormal_CustomNested1) _then) =
      _$CustomNestedError1TwinNormal_CustomNested1CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError1TwinNormal_CustomNested1CopyWithImpl<$Res>
    implements $CustomNestedError1TwinNormal_CustomNested1CopyWith<$Res> {
  _$CustomNestedError1TwinNormal_CustomNested1CopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinNormal_CustomNested1 _self;
  final $Res Function(CustomNestedError1TwinNormal_CustomNested1) _then;

  /// Create a copy of CustomNestedError1TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinNormal_CustomNested1(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError1TwinNormal_ErrorNested
    extends CustomNestedError1TwinNormal {
  const CustomNestedError1TwinNormal_ErrorNested(this.field0) : super._();

  @override
  final CustomNestedError2TwinNormal field0;

  /// Create a copy of CustomNestedError1TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError1TwinNormal_ErrorNestedCopyWith<
          CustomNestedError1TwinNormal_ErrorNested>
      get copyWith => _$CustomNestedError1TwinNormal_ErrorNestedCopyWithImpl<
          CustomNestedError1TwinNormal_ErrorNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError1TwinNormal_ErrorNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError1TwinNormal.errorNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError1TwinNormal_ErrorNestedCopyWith<$Res>
    implements $CustomNestedError1TwinNormalCopyWith<$Res> {
  factory $CustomNestedError1TwinNormal_ErrorNestedCopyWith(
          CustomNestedError1TwinNormal_ErrorNested value,
          $Res Function(CustomNestedError1TwinNormal_ErrorNested) _then) =
      _$CustomNestedError1TwinNormal_ErrorNestedCopyWithImpl;
  @useResult
  $Res call({CustomNestedError2TwinNormal field0});

  $CustomNestedError2TwinNormalCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedError1TwinNormal_ErrorNestedCopyWithImpl<$Res>
    implements $CustomNestedError1TwinNormal_ErrorNestedCopyWith<$Res> {
  _$CustomNestedError1TwinNormal_ErrorNestedCopyWithImpl(
      this._self, this._then);

  final CustomNestedError1TwinNormal_ErrorNested _self;
  final $Res Function(CustomNestedError1TwinNormal_ErrorNested) _then;

  /// Create a copy of CustomNestedError1TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError1TwinNormal_ErrorNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinNormal,
    ));
  }

  /// Create a copy of CustomNestedError1TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinNormalCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinNormalCopyWith<$Res>(_self.field0, (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedError2TwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedError2TwinNormalCopyWith<$Res> {
  $CustomNestedError2TwinNormalCopyWith(CustomNestedError2TwinNormal _,
      $Res Function(CustomNestedError2TwinNormal) __);
}

/// Adds pattern-matching-related methods to [CustomNestedError2TwinNormal].
extension CustomNestedError2TwinNormalPatterns on CustomNestedError2TwinNormal {
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
    TResult Function(CustomNestedError2TwinNormal_CustomNested2 value)?
        customNested2,
    TResult Function(CustomNestedError2TwinNormal_CustomNested2Number value)?
        customNested2Number,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2()
          when customNested2 != null:
        return customNested2(_that);
      case CustomNestedError2TwinNormal_CustomNested2Number()
          when customNested2Number != null:
        return customNested2Number(_that);
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
    required TResult Function(CustomNestedError2TwinNormal_CustomNested2 value)
        customNested2,
    required TResult Function(
            CustomNestedError2TwinNormal_CustomNested2Number value)
        customNested2Number,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2():
        return customNested2(_that);
      case CustomNestedError2TwinNormal_CustomNested2Number():
        return customNested2Number(_that);
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
    TResult? Function(CustomNestedError2TwinNormal_CustomNested2 value)?
        customNested2,
    TResult? Function(CustomNestedError2TwinNormal_CustomNested2Number value)?
        customNested2Number,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2()
          when customNested2 != null:
        return customNested2(_that);
      case CustomNestedError2TwinNormal_CustomNested2Number()
          when customNested2Number != null:
        return customNested2Number(_that);
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
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2()
          when customNested2 != null:
        return customNested2(_that.field0);
      case CustomNestedError2TwinNormal_CustomNested2Number()
          when customNested2Number != null:
        return customNested2Number(_that.field0);
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
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2():
        return customNested2(_that.field0);
      case CustomNestedError2TwinNormal_CustomNested2Number():
        return customNested2Number(_that.field0);
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
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedError2TwinNormal_CustomNested2()
          when customNested2 != null:
        return customNested2(_that.field0);
      case CustomNestedError2TwinNormal_CustomNested2Number()
          when customNested2Number != null:
        return customNested2Number(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomNestedError2TwinNormal_CustomNested2
    extends CustomNestedError2TwinNormal {
  const CustomNestedError2TwinNormal_CustomNested2(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedError2TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinNormal_CustomNested2CopyWith<
          CustomNestedError2TwinNormal_CustomNested2>
      get copyWith => _$CustomNestedError2TwinNormal_CustomNested2CopyWithImpl<
          CustomNestedError2TwinNormal_CustomNested2>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinNormal_CustomNested2 &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinNormal.customNested2(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinNormal_CustomNested2CopyWith<$Res>
    implements $CustomNestedError2TwinNormalCopyWith<$Res> {
  factory $CustomNestedError2TwinNormal_CustomNested2CopyWith(
          CustomNestedError2TwinNormal_CustomNested2 value,
          $Res Function(CustomNestedError2TwinNormal_CustomNested2) _then) =
      _$CustomNestedError2TwinNormal_CustomNested2CopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedError2TwinNormal_CustomNested2CopyWithImpl<$Res>
    implements $CustomNestedError2TwinNormal_CustomNested2CopyWith<$Res> {
  _$CustomNestedError2TwinNormal_CustomNested2CopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinNormal_CustomNested2 _self;
  final $Res Function(CustomNestedError2TwinNormal_CustomNested2) _then;

  /// Create a copy of CustomNestedError2TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinNormal_CustomNested2(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedError2TwinNormal_CustomNested2Number
    extends CustomNestedError2TwinNormal {
  const CustomNestedError2TwinNormal_CustomNested2Number(this.field0)
      : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedError2TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinNormal_CustomNested2NumberCopyWith<
          CustomNestedError2TwinNormal_CustomNested2Number>
      get copyWith =>
          _$CustomNestedError2TwinNormal_CustomNested2NumberCopyWithImpl<
                  CustomNestedError2TwinNormal_CustomNested2Number>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedError2TwinNormal_CustomNested2Number &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedError2TwinNormal.customNested2Number(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedError2TwinNormal_CustomNested2NumberCopyWith<
    $Res> implements $CustomNestedError2TwinNormalCopyWith<$Res> {
  factory $CustomNestedError2TwinNormal_CustomNested2NumberCopyWith(
          CustomNestedError2TwinNormal_CustomNested2Number value,
          $Res Function(CustomNestedError2TwinNormal_CustomNested2Number)
              _then) =
      _$CustomNestedError2TwinNormal_CustomNested2NumberCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedError2TwinNormal_CustomNested2NumberCopyWithImpl<$Res>
    implements $CustomNestedError2TwinNormal_CustomNested2NumberCopyWith<$Res> {
  _$CustomNestedError2TwinNormal_CustomNested2NumberCopyWithImpl(
      this._self, this._then);

  final CustomNestedError2TwinNormal_CustomNested2Number _self;
  final $Res Function(CustomNestedError2TwinNormal_CustomNested2Number) _then;

  /// Create a copy of CustomNestedError2TwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedError2TwinNormal_CustomNested2Number(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorInnerTwinNormalCopyWith<$Res> {
  $CustomNestedErrorInnerTwinNormalCopyWith(CustomNestedErrorInnerTwinNormal _,
      $Res Function(CustomNestedErrorInnerTwinNormal) __);
}

/// Adds pattern-matching-related methods to [CustomNestedErrorInnerTwinNormal].
extension CustomNestedErrorInnerTwinNormalPatterns
    on CustomNestedErrorInnerTwinNormal {
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
    TResult Function(CustomNestedErrorInnerTwinNormal_Three value)? three,
    TResult Function(CustomNestedErrorInnerTwinNormal_Four value)? four,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three() when three != null:
        return three(_that);
      case CustomNestedErrorInnerTwinNormal_Four() when four != null:
        return four(_that);
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
    required TResult Function(CustomNestedErrorInnerTwinNormal_Three value)
        three,
    required TResult Function(CustomNestedErrorInnerTwinNormal_Four value) four,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three():
        return three(_that);
      case CustomNestedErrorInnerTwinNormal_Four():
        return four(_that);
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
    TResult? Function(CustomNestedErrorInnerTwinNormal_Three value)? three,
    TResult? Function(CustomNestedErrorInnerTwinNormal_Four value)? four,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three() when three != null:
        return three(_that);
      case CustomNestedErrorInnerTwinNormal_Four() when four != null:
        return four(_that);
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
    TResult Function(String field0)? three,
    TResult Function(int field0)? four,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three() when three != null:
        return three(_that.field0);
      case CustomNestedErrorInnerTwinNormal_Four() when four != null:
        return four(_that.field0);
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
    required TResult Function(String field0) three,
    required TResult Function(int field0) four,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three():
        return three(_that.field0);
      case CustomNestedErrorInnerTwinNormal_Four():
        return four(_that.field0);
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
    TResult? Function(String field0)? three,
    TResult? Function(int field0)? four,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorInnerTwinNormal_Three() when three != null:
        return three(_that.field0);
      case CustomNestedErrorInnerTwinNormal_Four() when four != null:
        return four(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinNormal_Three
    extends CustomNestedErrorInnerTwinNormal {
  const CustomNestedErrorInnerTwinNormal_Three(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorInnerTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinNormal_ThreeCopyWith<
          CustomNestedErrorInnerTwinNormal_Three>
      get copyWith => _$CustomNestedErrorInnerTwinNormal_ThreeCopyWithImpl<
          CustomNestedErrorInnerTwinNormal_Three>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinNormal_Three &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinNormal.three(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinNormal_ThreeCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinNormalCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinNormal_ThreeCopyWith(
          CustomNestedErrorInnerTwinNormal_Three value,
          $Res Function(CustomNestedErrorInnerTwinNormal_Three) _then) =
      _$CustomNestedErrorInnerTwinNormal_ThreeCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinNormal_ThreeCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinNormal_ThreeCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinNormal_ThreeCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinNormal_Three _self;
  final $Res Function(CustomNestedErrorInnerTwinNormal_Three) _then;

  /// Create a copy of CustomNestedErrorInnerTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinNormal_Three(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorInnerTwinNormal_Four
    extends CustomNestedErrorInnerTwinNormal {
  const CustomNestedErrorInnerTwinNormal_Four(this.field0) : super._();

  @override
  final int field0;

  /// Create a copy of CustomNestedErrorInnerTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinNormal_FourCopyWith<
          CustomNestedErrorInnerTwinNormal_Four>
      get copyWith => _$CustomNestedErrorInnerTwinNormal_FourCopyWithImpl<
          CustomNestedErrorInnerTwinNormal_Four>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorInnerTwinNormal_Four &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinNormal.four(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorInnerTwinNormal_FourCopyWith<$Res>
    implements $CustomNestedErrorInnerTwinNormalCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinNormal_FourCopyWith(
          CustomNestedErrorInnerTwinNormal_Four value,
          $Res Function(CustomNestedErrorInnerTwinNormal_Four) _then) =
      _$CustomNestedErrorInnerTwinNormal_FourCopyWithImpl;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$CustomNestedErrorInnerTwinNormal_FourCopyWithImpl<$Res>
    implements $CustomNestedErrorInnerTwinNormal_FourCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinNormal_FourCopyWithImpl(this._self, this._then);

  final CustomNestedErrorInnerTwinNormal_Four _self;
  final $Res Function(CustomNestedErrorInnerTwinNormal_Four) _then;

  /// Create a copy of CustomNestedErrorInnerTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorInnerTwinNormal_Four(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinNormal {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinNormal &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinNormal(field0: $field0)';
  }
}

/// @nodoc
class $CustomNestedErrorOuterTwinNormalCopyWith<$Res> {
  $CustomNestedErrorOuterTwinNormalCopyWith(CustomNestedErrorOuterTwinNormal _,
      $Res Function(CustomNestedErrorOuterTwinNormal) __);
}

/// Adds pattern-matching-related methods to [CustomNestedErrorOuterTwinNormal].
extension CustomNestedErrorOuterTwinNormalPatterns
    on CustomNestedErrorOuterTwinNormal {
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
    TResult Function(CustomNestedErrorOuterTwinNormal_One value)? one,
    TResult Function(CustomNestedErrorOuterTwinNormal_Two value)? two,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One() when one != null:
        return one(_that);
      case CustomNestedErrorOuterTwinNormal_Two() when two != null:
        return two(_that);
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
    required TResult Function(CustomNestedErrorOuterTwinNormal_One value) one,
    required TResult Function(CustomNestedErrorOuterTwinNormal_Two value) two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One():
        return one(_that);
      case CustomNestedErrorOuterTwinNormal_Two():
        return two(_that);
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
    TResult? Function(CustomNestedErrorOuterTwinNormal_One value)? one,
    TResult? Function(CustomNestedErrorOuterTwinNormal_Two value)? two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One() when one != null:
        return one(_that);
      case CustomNestedErrorOuterTwinNormal_Two() when two != null:
        return two(_that);
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
    TResult Function(String field0)? one,
    TResult Function(CustomNestedErrorInnerTwinNormal field0)? two,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One() when one != null:
        return one(_that.field0);
      case CustomNestedErrorOuterTwinNormal_Two() when two != null:
        return two(_that.field0);
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
    required TResult Function(String field0) one,
    required TResult Function(CustomNestedErrorInnerTwinNormal field0) two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One():
        return one(_that.field0);
      case CustomNestedErrorOuterTwinNormal_Two():
        return two(_that.field0);
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
    TResult? Function(String field0)? one,
    TResult? Function(CustomNestedErrorInnerTwinNormal field0)? two,
  }) {
    final _that = this;
    switch (_that) {
      case CustomNestedErrorOuterTwinNormal_One() when one != null:
        return one(_that.field0);
      case CustomNestedErrorOuterTwinNormal_Two() when two != null:
        return two(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinNormal_One
    extends CustomNestedErrorOuterTwinNormal {
  const CustomNestedErrorOuterTwinNormal_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of CustomNestedErrorOuterTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinNormal_OneCopyWith<
          CustomNestedErrorOuterTwinNormal_One>
      get copyWith => _$CustomNestedErrorOuterTwinNormal_OneCopyWithImpl<
          CustomNestedErrorOuterTwinNormal_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinNormal_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinNormal.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinNormal_OneCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinNormalCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinNormal_OneCopyWith(
          CustomNestedErrorOuterTwinNormal_One value,
          $Res Function(CustomNestedErrorOuterTwinNormal_One) _then) =
      _$CustomNestedErrorOuterTwinNormal_OneCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$CustomNestedErrorOuterTwinNormal_OneCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinNormal_OneCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinNormal_OneCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinNormal_One _self;
  final $Res Function(CustomNestedErrorOuterTwinNormal_One) _then;

  /// Create a copy of CustomNestedErrorOuterTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinNormal_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class CustomNestedErrorOuterTwinNormal_Two
    extends CustomNestedErrorOuterTwinNormal {
  const CustomNestedErrorOuterTwinNormal_Two(this.field0) : super._();

  @override
  final CustomNestedErrorInnerTwinNormal field0;

  /// Create a copy of CustomNestedErrorOuterTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CustomNestedErrorOuterTwinNormal_TwoCopyWith<
          CustomNestedErrorOuterTwinNormal_Two>
      get copyWith => _$CustomNestedErrorOuterTwinNormal_TwoCopyWithImpl<
          CustomNestedErrorOuterTwinNormal_Two>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CustomNestedErrorOuterTwinNormal_Two &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinNormal.two(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $CustomNestedErrorOuterTwinNormal_TwoCopyWith<$Res>
    implements $CustomNestedErrorOuterTwinNormalCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinNormal_TwoCopyWith(
          CustomNestedErrorOuterTwinNormal_Two value,
          $Res Function(CustomNestedErrorOuterTwinNormal_Two) _then) =
      _$CustomNestedErrorOuterTwinNormal_TwoCopyWithImpl;
  @useResult
  $Res call({CustomNestedErrorInnerTwinNormal field0});

  $CustomNestedErrorInnerTwinNormalCopyWith<$Res> get field0;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinNormal_TwoCopyWithImpl<$Res>
    implements $CustomNestedErrorOuterTwinNormal_TwoCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinNormal_TwoCopyWithImpl(this._self, this._then);

  final CustomNestedErrorOuterTwinNormal_Two _self;
  final $Res Function(CustomNestedErrorOuterTwinNormal_Two) _then;

  /// Create a copy of CustomNestedErrorOuterTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(CustomNestedErrorOuterTwinNormal_Two(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinNormal,
    ));
  }

  /// Create a copy of CustomNestedErrorOuterTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinNormalCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinNormalCopyWith<$Res>(_self.field0,
        (value) {
      return _then(_self.copyWith(field0: value));
    });
  }
}

// dart format on
