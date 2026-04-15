// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is BasicGeneralEnumTwinSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSse()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinSseCopyWith<$Res> {
  $BasicGeneralEnumTwinSseCopyWith(
      BasicGeneralEnumTwinSse _, $Res Function(BasicGeneralEnumTwinSse) __);
}

/// Adds pattern-matching-related methods to [BasicGeneralEnumTwinSse].
extension BasicGeneralEnumTwinSsePatterns on BasicGeneralEnumTwinSse {
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
    TResult Function(BasicGeneralEnumTwinSse_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinSse_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinSse_Orange() when orange != null:
        return orange(_that);
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
    required TResult Function(BasicGeneralEnumTwinSse_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinSse_Orange value) orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple():
        return apple(_that);
      case BasicGeneralEnumTwinSse_Orange():
        return orange(_that);
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
    TResult? Function(BasicGeneralEnumTwinSse_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinSse_Orange value)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinSse_Orange() when orange != null:
        return orange(_that);
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
    TResult Function(String field)? apple,
    TResult Function()? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinSse_Orange() when orange != null:
        return orange();
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
    required TResult Function(String field) apple,
    required TResult Function() orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple():
        return apple(_that.field);
      case BasicGeneralEnumTwinSse_Orange():
        return orange();
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
    TResult? Function(String field)? apple,
    TResult? Function()? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinSse_Orange() when orange != null:
        return orange();
      case _:
        return null;
    }
  }
}

/// @nodoc

class BasicGeneralEnumTwinSse_Apple extends BasicGeneralEnumTwinSse {
  const BasicGeneralEnumTwinSse_Apple({required this.field}) : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinSse_AppleCopyWith<BasicGeneralEnumTwinSse_Apple>
      get copyWith => _$BasicGeneralEnumTwinSse_AppleCopyWithImpl<
          BasicGeneralEnumTwinSse_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSse_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSse.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinSse_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinSseCopyWith<$Res> {
  factory $BasicGeneralEnumTwinSse_AppleCopyWith(
          BasicGeneralEnumTwinSse_Apple value,
          $Res Function(BasicGeneralEnumTwinSse_Apple) _then) =
      _$BasicGeneralEnumTwinSse_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinSse_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinSse_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinSse_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinSse_Apple _self;
  final $Res Function(BasicGeneralEnumTwinSse_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinSse_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinSse_Orange extends BasicGeneralEnumTwinSse {
  const BasicGeneralEnumTwinSse_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSse_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSse.orange()';
  }
}

// dart format on
