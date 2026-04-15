// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basic_twin_sync_sse.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$BasicGeneralEnumTwinSyncSse {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSyncSse);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSyncSse()';
  }
}

/// @nodoc
class $BasicGeneralEnumTwinSyncSseCopyWith<$Res> {
  $BasicGeneralEnumTwinSyncSseCopyWith(BasicGeneralEnumTwinSyncSse _,
      $Res Function(BasicGeneralEnumTwinSyncSse) __);
}

/// Adds pattern-matching-related methods to [BasicGeneralEnumTwinSyncSse].
extension BasicGeneralEnumTwinSyncSsePatterns on BasicGeneralEnumTwinSyncSse {
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
    TResult Function(BasicGeneralEnumTwinSyncSse_Apple value)? apple,
    TResult Function(BasicGeneralEnumTwinSyncSse_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSyncSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinSyncSse_Orange() when orange != null:
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
    required TResult Function(BasicGeneralEnumTwinSyncSse_Apple value) apple,
    required TResult Function(BasicGeneralEnumTwinSyncSse_Orange value) orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSyncSse_Apple():
        return apple(_that);
      case BasicGeneralEnumTwinSyncSse_Orange():
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
    TResult? Function(BasicGeneralEnumTwinSyncSse_Apple value)? apple,
    TResult? Function(BasicGeneralEnumTwinSyncSse_Orange value)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case BasicGeneralEnumTwinSyncSse_Apple() when apple != null:
        return apple(_that);
      case BasicGeneralEnumTwinSyncSse_Orange() when orange != null:
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
      case BasicGeneralEnumTwinSyncSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinSyncSse_Orange() when orange != null:
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
      case BasicGeneralEnumTwinSyncSse_Apple():
        return apple(_that.field);
      case BasicGeneralEnumTwinSyncSse_Orange():
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
      case BasicGeneralEnumTwinSyncSse_Apple() when apple != null:
        return apple(_that.field);
      case BasicGeneralEnumTwinSyncSse_Orange() when orange != null:
        return orange();
      case _:
        return null;
    }
  }
}

/// @nodoc

class BasicGeneralEnumTwinSyncSse_Apple extends BasicGeneralEnumTwinSyncSse {
  const BasicGeneralEnumTwinSyncSse_Apple({required this.field}) : super._();

  final String field;

  /// Create a copy of BasicGeneralEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $BasicGeneralEnumTwinSyncSse_AppleCopyWith<BasicGeneralEnumTwinSyncSse_Apple>
      get copyWith => _$BasicGeneralEnumTwinSyncSse_AppleCopyWithImpl<
          BasicGeneralEnumTwinSyncSse_Apple>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSyncSse_Apple &&
            (identical(other.field, field) || other.field == field));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field);

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSyncSse.apple(field: $field)';
  }
}

/// @nodoc
abstract mixin class $BasicGeneralEnumTwinSyncSse_AppleCopyWith<$Res>
    implements $BasicGeneralEnumTwinSyncSseCopyWith<$Res> {
  factory $BasicGeneralEnumTwinSyncSse_AppleCopyWith(
          BasicGeneralEnumTwinSyncSse_Apple value,
          $Res Function(BasicGeneralEnumTwinSyncSse_Apple) _then) =
      _$BasicGeneralEnumTwinSyncSse_AppleCopyWithImpl;
  @useResult
  $Res call({String field});
}

/// @nodoc
class _$BasicGeneralEnumTwinSyncSse_AppleCopyWithImpl<$Res>
    implements $BasicGeneralEnumTwinSyncSse_AppleCopyWith<$Res> {
  _$BasicGeneralEnumTwinSyncSse_AppleCopyWithImpl(this._self, this._then);

  final BasicGeneralEnumTwinSyncSse_Apple _self;
  final $Res Function(BasicGeneralEnumTwinSyncSse_Apple) _then;

  /// Create a copy of BasicGeneralEnumTwinSyncSse
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field = null,
  }) {
    return _then(BasicGeneralEnumTwinSyncSse_Apple(
      field: null == field
          ? _self.field
          : field // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class BasicGeneralEnumTwinSyncSse_Orange extends BasicGeneralEnumTwinSyncSse {
  const BasicGeneralEnumTwinSyncSse_Orange() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is BasicGeneralEnumTwinSyncSse_Orange);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'BasicGeneralEnumTwinSyncSse.orange()';
  }
}

// dart format on
