// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'context.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AudioContextLatencyCategory {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AudioContextLatencyCategory);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'AudioContextLatencyCategory()';
  }
}

/// @nodoc
class $AudioContextLatencyCategoryCopyWith<$Res> {
  $AudioContextLatencyCategoryCopyWith(AudioContextLatencyCategory _,
      $Res Function(AudioContextLatencyCategory) __);
}

/// @nodoc

class AudioContextLatencyCategory_Balanced extends AudioContextLatencyCategory {
  const AudioContextLatencyCategory_Balanced() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AudioContextLatencyCategory_Balanced);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'AudioContextLatencyCategory.balanced()';
  }
}

/// @nodoc

class AudioContextLatencyCategory_Interactive
    extends AudioContextLatencyCategory {
  const AudioContextLatencyCategory_Interactive() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AudioContextLatencyCategory_Interactive);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'AudioContextLatencyCategory.interactive()';
  }
}

/// @nodoc

class AudioContextLatencyCategory_Playback extends AudioContextLatencyCategory {
  const AudioContextLatencyCategory_Playback() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AudioContextLatencyCategory_Playback);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'AudioContextLatencyCategory.playback()';
  }
}

/// @nodoc

class AudioContextLatencyCategory_Custom extends AudioContextLatencyCategory {
  const AudioContextLatencyCategory_Custom(this.field0) : super._();

  final double field0;

  /// Create a copy of AudioContextLatencyCategory
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AudioContextLatencyCategory_CustomCopyWith<
          AudioContextLatencyCategory_Custom>
      get copyWith => _$AudioContextLatencyCategory_CustomCopyWithImpl<
          AudioContextLatencyCategory_Custom>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AudioContextLatencyCategory_Custom &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'AudioContextLatencyCategory.custom(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $AudioContextLatencyCategory_CustomCopyWith<$Res>
    implements $AudioContextLatencyCategoryCopyWith<$Res> {
  factory $AudioContextLatencyCategory_CustomCopyWith(
          AudioContextLatencyCategory_Custom value,
          $Res Function(AudioContextLatencyCategory_Custom) _then) =
      _$AudioContextLatencyCategory_CustomCopyWithImpl;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class _$AudioContextLatencyCategory_CustomCopyWithImpl<$Res>
    implements $AudioContextLatencyCategory_CustomCopyWith<$Res> {
  _$AudioContextLatencyCategory_CustomCopyWithImpl(this._self, this._then);

  final AudioContextLatencyCategory_Custom _self;
  final $Res Function(AudioContextLatencyCategory_Custom) _then;

  /// Create a copy of AudioContextLatencyCategory
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(AudioContextLatencyCategory_Custom(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

// dart format on
