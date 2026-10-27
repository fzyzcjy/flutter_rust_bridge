// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_devices.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$MediaStreamConstraints {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is MediaStreamConstraints);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'MediaStreamConstraints()';
  }
}

/// @nodoc
class $MediaStreamConstraintsCopyWith<$Res> {
  $MediaStreamConstraintsCopyWith(
      MediaStreamConstraints _, $Res Function(MediaStreamConstraints) __);
}

/// Adds pattern-matching-related methods to [MediaStreamConstraints].
extension MediaStreamConstraintsPatterns on MediaStreamConstraints {
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
    TResult Function(MediaStreamConstraints_Audio value)? audio,
    TResult Function(MediaStreamConstraints_AudioWithConstraints value)?
        audioWithConstraints,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio() when audio != null:
        return audio(_that);
      case MediaStreamConstraints_AudioWithConstraints()
          when audioWithConstraints != null:
        return audioWithConstraints(_that);
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
    required TResult Function(MediaStreamConstraints_Audio value) audio,
    required TResult Function(MediaStreamConstraints_AudioWithConstraints value)
        audioWithConstraints,
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio():
        return audio(_that);
      case MediaStreamConstraints_AudioWithConstraints():
        return audioWithConstraints(_that);
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
    TResult? Function(MediaStreamConstraints_Audio value)? audio,
    TResult? Function(MediaStreamConstraints_AudioWithConstraints value)?
        audioWithConstraints,
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio() when audio != null:
        return audio(_that);
      case MediaStreamConstraints_AudioWithConstraints()
          when audioWithConstraints != null:
        return audioWithConstraints(_that);
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
    TResult Function()? audio,
    TResult Function(MediaTrackConstraints field0)? audioWithConstraints,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio() when audio != null:
        return audio();
      case MediaStreamConstraints_AudioWithConstraints()
          when audioWithConstraints != null:
        return audioWithConstraints(_that.field0);
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
    required TResult Function() audio,
    required TResult Function(MediaTrackConstraints field0)
        audioWithConstraints,
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio():
        return audio();
      case MediaStreamConstraints_AudioWithConstraints():
        return audioWithConstraints(_that.field0);
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
    TResult? Function()? audio,
    TResult? Function(MediaTrackConstraints field0)? audioWithConstraints,
  }) {
    final _that = this;
    switch (_that) {
      case MediaStreamConstraints_Audio() when audio != null:
        return audio();
      case MediaStreamConstraints_AudioWithConstraints()
          when audioWithConstraints != null:
        return audioWithConstraints(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class MediaStreamConstraints_Audio extends MediaStreamConstraints {
  const MediaStreamConstraints_Audio() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MediaStreamConstraints_Audio);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'MediaStreamConstraints.audio()';
  }
}

/// @nodoc

class MediaStreamConstraints_AudioWithConstraints
    extends MediaStreamConstraints {
  const MediaStreamConstraints_AudioWithConstraints(this.field0) : super._();

  final MediaTrackConstraints field0;

  /// Create a copy of MediaStreamConstraints
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MediaStreamConstraints_AudioWithConstraintsCopyWith<
          MediaStreamConstraints_AudioWithConstraints>
      get copyWith => _$MediaStreamConstraints_AudioWithConstraintsCopyWithImpl<
          MediaStreamConstraints_AudioWithConstraints>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MediaStreamConstraints_AudioWithConstraints &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MediaStreamConstraints.audioWithConstraints(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MediaStreamConstraints_AudioWithConstraintsCopyWith<$Res>
    implements $MediaStreamConstraintsCopyWith<$Res> {
  factory $MediaStreamConstraints_AudioWithConstraintsCopyWith(
          MediaStreamConstraints_AudioWithConstraints value,
          $Res Function(MediaStreamConstraints_AudioWithConstraints) _then) =
      _$MediaStreamConstraints_AudioWithConstraintsCopyWithImpl;
  @useResult
  $Res call({MediaTrackConstraints field0});
}

/// @nodoc
class _$MediaStreamConstraints_AudioWithConstraintsCopyWithImpl<$Res>
    implements $MediaStreamConstraints_AudioWithConstraintsCopyWith<$Res> {
  _$MediaStreamConstraints_AudioWithConstraintsCopyWithImpl(
      this._self, this._then);

  final MediaStreamConstraints_AudioWithConstraints _self;
  final $Res Function(MediaStreamConstraints_AudioWithConstraints) _then;

  /// Create a copy of MediaStreamConstraints
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MediaStreamConstraints_AudioWithConstraints(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MediaTrackConstraints,
    ));
  }
}

// dart format on
