// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinNormal {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinNormal);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinNormal()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinNormalCopyWith<$Res> {
  $BasicGeneralEnumTwinNormalCopyWith(BasicGeneralEnumTwinNormal _,
      $Res Function(BasicGeneralEnumTwinNormal) __);
}

/// Adds pattern-matching-related methods to [BasicGeneralEnumTwinNormal].
extension BasicGeneralEnumTwinNormalPatterns on BasicGeneralEnumTwinNormal {
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
    TResult Function(BasicGeneralEnumTwinNormal_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinNormal_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinNormal_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinNormal_Orange() when orange != null:
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
    required TResult Function(BasicGeneralEnumTwinNormal_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinNormal_Orange value) orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinNormal_Apple():
        return apple(_that);
      case BasicGeneralEnumTwinNormal_Orange():
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
    TResult? Function(BasicGeneralEnumTwinNormal_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinNormal_Orange value)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinNormal_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinNormal_Orange() when orange != null:
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
      case BasicGeneralEnumTwinNormal_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinNormal_Orange() when orange != null:
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
      case BasicGeneralEnumTwinNormal_Apple():
        return apple(_that.field);
      case BasicGeneralEnumTwinNormal_Orange():
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
      case BasicGeneralEnumTwinNormal_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinNormal_Orange() when orange != null:
        return orange();
      case _:
        return null;
    }
  }
}

/// @nodoc

class BasicGeneralEnumTwinNormal_Apple extends BasicGeneralEnumTwinNormal {
  const BasicGeneralEnumTwinNormal_Apple({required this.field}) : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinNormal_AppleCopyWith<BasicGeneralEnumTwinNormal_Apple>
      get copyWith => _$BasicGeneralEnumTwinNormal_AppleCopyWithImpl<
          BasicGeneralEnumTwinNormal_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinNormal_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinNormal.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinNormal_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinNormalCopyWith<$Res> {
  factory $BasicGeneralEnumTwinNormal_AppleCopyWith(
          BasicGeneralEnumTwinNormal_Apple value,
          $Res Function(BasicGeneralEnumTwinNormal_Apple) _then) =
      _$BasicGeneralEnumTwinNormal_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinNormal_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinNormal_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinNormal_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinNormal_Apple _self;
  final $Res Function(BasicGeneralEnumTwinNormal_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinNormal_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinNormal_Orange extends BasicGeneralEnumTwinNormal {
  const BasicGeneralEnumTwinNormal_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinNormal_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinNormal.orange()';
  }
}

// dart format on
