// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'mirror.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$ApplicationMessage {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is ApplicationMessage);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'ApplicationMessage()';
  }
}

/// @nodoc
class $ApplicationMessageCopyWith<$Res> {
  $ApplicationMessageCopyWith(
      ApplicationMessage _, $Res Function(ApplicationMessage) __);
}

/// @nodoc

class ApplicationMessage_DisplayMessage extends ApplicationMessage {
  const ApplicationMessage_DisplayMessage(this.field0) : super._();

  final String field0;

  /// Create a copy of ApplicationMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $ApplicationMessage_DisplayMessageCopyWith<ApplicationMessage_DisplayMessage>
      get copyWith => _$ApplicationMessage_DisplayMessageCopyWithImpl<
          ApplicationMessage_DisplayMessage>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ApplicationMessage_DisplayMessage &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'ApplicationMessage.displayMessage(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $ApplicationMessage_DisplayMessageCopyWith<$Res>
    implements $ApplicationMessageCopyWith<$Res> {
  factory $ApplicationMessage_DisplayMessageCopyWith(
          ApplicationMessage_DisplayMessage value,
          $Res Function(ApplicationMessage_DisplayMessage) _then) =
      _$ApplicationMessage_DisplayMessageCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$ApplicationMessage_DisplayMessageCopyWithImpl<$Res>
    implements $ApplicationMessage_DisplayMessageCopyWith<$Res> {
  _$ApplicationMessage_DisplayMessageCopyWithImpl(this._self, this._then);

  final ApplicationMessage_DisplayMessage _self;
  final $Res Function(ApplicationMessage_DisplayMessage) _then;

  /// Create a copy of ApplicationMessage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(ApplicationMessage_DisplayMessage(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class ApplicationMessage_RenderPixel extends ApplicationMessage {
  const ApplicationMessage_RenderPixel({required this.x, required this.y})
      : super._();

  final int x;
  final int y;

  /// Create a copy of ApplicationMessage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $ApplicationMessage_RenderPixelCopyWith<ApplicationMessage_RenderPixel>
      get copyWith => _$ApplicationMessage_RenderPixelCopyWithImpl<
          ApplicationMessage_RenderPixel>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ApplicationMessage_RenderPixel &&
            (identical(other.x, x) || other.x == x) &&
            (identical(other.y, y) || other.y == y));
  }

  @override
  int get hashCode => Object.hash(runtimeType, x, y);

  @override
  String toString() {
    return 'ApplicationMessage.renderPixel(x: $x, y: $y)';
  }
}

/// @nodoc
abstract mixin class $ApplicationMessage_RenderPixelCopyWith<$Res>
    implements $ApplicationMessageCopyWith<$Res> {
  factory $ApplicationMessage_RenderPixelCopyWith(
          ApplicationMessage_RenderPixel value,
          $Res Function(ApplicationMessage_RenderPixel) _then) =
      _$ApplicationMessage_RenderPixelCopyWithImpl;
  @useResult
  $Res call({int x, int y});
}

/// @nodoc
class _$ApplicationMessage_RenderPixelCopyWithImpl<$Res>
    implements $ApplicationMessage_RenderPixelCopyWith<$Res> {
  _$ApplicationMessage_RenderPixelCopyWithImpl(this._self, this._then);

  final ApplicationMessage_RenderPixel _self;
  final $Res Function(ApplicationMessage_RenderPixel) _then;

  /// Create a copy of ApplicationMessage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? x = null,
    Object? y = null,
  }) {
    return _then(ApplicationMessage_RenderPixel(
      x: null == x
          ? _self.x
          : x // ignore: cast_nullable_to_non_nullable
              as int,
      y: null == y
          ? _self.y
          : y // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class ApplicationMessage_Exit extends ApplicationMessage {
  const ApplicationMessage_Exit() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is ApplicationMessage_Exit);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }
}

/// @nodoc
mixin _$RawStringEnumMirrored {
  Object get field0;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringEnumMirrored &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @override
  String toString() {
    return 'RawStringEnumMirrored(field0: $field0)';
  }
}

/// @nodoc
class $RawStringEnumMirroredCopyWith<$Res> {
  $RawStringEnumMirroredCopyWith(
      RawStringEnumMirrored _, $Res Function(RawStringEnumMirrored) __);
}

/// @nodoc

class RawStringEnumMirrored_Raw extends RawStringEnumMirrored {
  const RawStringEnumMirrored_Raw(this.field0) : super._();

  @override
  final RawStringMirrored field0;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringEnumMirrored_RawCopyWith<RawStringEnumMirrored_Raw> get copyWith =>
      _$RawStringEnumMirrored_RawCopyWithImpl<RawStringEnumMirrored_Raw>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringEnumMirrored_Raw &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'RawStringEnumMirrored.raw(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $RawStringEnumMirrored_RawCopyWith<$Res>
    implements $RawStringEnumMirroredCopyWith<$Res> {
  factory $RawStringEnumMirrored_RawCopyWith(RawStringEnumMirrored_Raw value,
          $Res Function(RawStringEnumMirrored_Raw) _then) =
      _$RawStringEnumMirrored_RawCopyWithImpl;
  @useResult
  $Res call({RawStringMirrored field0});
}

/// @nodoc
class _$RawStringEnumMirrored_RawCopyWithImpl<$Res>
    implements $RawStringEnumMirrored_RawCopyWith<$Res> {
  _$RawStringEnumMirrored_RawCopyWithImpl(this._self, this._then);

  final RawStringEnumMirrored_Raw _self;
  final $Res Function(RawStringEnumMirrored_Raw) _then;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(RawStringEnumMirrored_Raw(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RawStringMirrored,
    ));
  }
}

/// @nodoc

class RawStringEnumMirrored_Nested extends RawStringEnumMirrored {
  const RawStringEnumMirrored_Nested(this.field0) : super._();

  @override
  final NestedRawStringMirrored field0;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringEnumMirrored_NestedCopyWith<RawStringEnumMirrored_Nested>
      get copyWith => _$RawStringEnumMirrored_NestedCopyWithImpl<
          RawStringEnumMirrored_Nested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringEnumMirrored_Nested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'RawStringEnumMirrored.nested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $RawStringEnumMirrored_NestedCopyWith<$Res>
    implements $RawStringEnumMirroredCopyWith<$Res> {
  factory $RawStringEnumMirrored_NestedCopyWith(
          RawStringEnumMirrored_Nested value,
          $Res Function(RawStringEnumMirrored_Nested) _then) =
      _$RawStringEnumMirrored_NestedCopyWithImpl;
  @useResult
  $Res call({NestedRawStringMirrored field0});
}

/// @nodoc
class _$RawStringEnumMirrored_NestedCopyWithImpl<$Res>
    implements $RawStringEnumMirrored_NestedCopyWith<$Res> {
  _$RawStringEnumMirrored_NestedCopyWithImpl(this._self, this._then);

  final RawStringEnumMirrored_Nested _self;
  final $Res Function(RawStringEnumMirrored_Nested) _then;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(RawStringEnumMirrored_Nested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NestedRawStringMirrored,
    ));
  }
}

/// @nodoc

class RawStringEnumMirrored_ListOfNested extends RawStringEnumMirrored {
  const RawStringEnumMirrored_ListOfNested(this.field0) : super._();

  @override
  final ListOfNestedRawStringMirrored field0;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $RawStringEnumMirrored_ListOfNestedCopyWith<
          RawStringEnumMirrored_ListOfNested>
      get copyWith => _$RawStringEnumMirrored_ListOfNestedCopyWithImpl<
          RawStringEnumMirrored_ListOfNested>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is RawStringEnumMirrored_ListOfNested &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'RawStringEnumMirrored.listOfNested(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $RawStringEnumMirrored_ListOfNestedCopyWith<$Res>
    implements $RawStringEnumMirroredCopyWith<$Res> {
  factory $RawStringEnumMirrored_ListOfNestedCopyWith(
          RawStringEnumMirrored_ListOfNested value,
          $Res Function(RawStringEnumMirrored_ListOfNested) _then) =
      _$RawStringEnumMirrored_ListOfNestedCopyWithImpl;
  @useResult
  $Res call({ListOfNestedRawStringMirrored field0});
}

/// @nodoc
class _$RawStringEnumMirrored_ListOfNestedCopyWithImpl<$Res>
    implements $RawStringEnumMirrored_ListOfNestedCopyWith<$Res> {
  _$RawStringEnumMirrored_ListOfNestedCopyWithImpl(this._self, this._then);

  final RawStringEnumMirrored_ListOfNested _self;
  final $Res Function(RawStringEnumMirrored_ListOfNested) _then;

  /// Create a copy of RawStringEnumMirrored
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(RawStringEnumMirrored_ListOfNested(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ListOfNestedRawStringMirrored,
    ));
  }
}

// dart format on
