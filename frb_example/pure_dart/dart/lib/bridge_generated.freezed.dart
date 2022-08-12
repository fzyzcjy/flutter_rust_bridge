// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target

part of 'bridge_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$ApplicationMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ApplicationMessageCopyWith<$Res> {
  factory $ApplicationMessageCopyWith(ApplicationMessage value, $Res Function(ApplicationMessage) then) =
      _$ApplicationMessageCopyWithImpl<$Res>;
}

/// @nodoc
class _$ApplicationMessageCopyWithImpl<$Res> implements $ApplicationMessageCopyWith<$Res> {
  _$ApplicationMessageCopyWithImpl(this._value, this._then);

  final ApplicationMessage _value;
  // ignore: unused_field
  final $Res Function(ApplicationMessage) _then;
}

/// @nodoc
abstract class _$$ApplicationMessage_DisplayMessageCopyWith<$Res> {
  factory _$$ApplicationMessage_DisplayMessageCopyWith(
          _$ApplicationMessage_DisplayMessage value, $Res Function(_$ApplicationMessage_DisplayMessage) then) =
      __$$ApplicationMessage_DisplayMessageCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$ApplicationMessage_DisplayMessageCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_DisplayMessageCopyWith<$Res> {
  __$$ApplicationMessage_DisplayMessageCopyWithImpl(
      _$ApplicationMessage_DisplayMessage _value, $Res Function(_$ApplicationMessage_DisplayMessage) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_DisplayMessage));

  @override
  _$ApplicationMessage_DisplayMessage get _value => super._value as _$ApplicationMessage_DisplayMessage;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$ApplicationMessage_DisplayMessage(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_DisplayMessage implements ApplicationMessage_DisplayMessage {
  const _$ApplicationMessage_DisplayMessage(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'ApplicationMessage.displayMessage(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_DisplayMessage &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$ApplicationMessage_DisplayMessageCopyWith<_$ApplicationMessage_DisplayMessage> get copyWith =>
      __$$ApplicationMessage_DisplayMessageCopyWithImpl<_$ApplicationMessage_DisplayMessage>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return displayMessage(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
  }) {
    return displayMessage?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return displayMessage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
  }) {
    return displayMessage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_DisplayMessage implements ApplicationMessage {
  const factory ApplicationMessage_DisplayMessage(final String field0) = _$ApplicationMessage_DisplayMessage;

  String get field0;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_DisplayMessageCopyWith<_$ApplicationMessage_DisplayMessage> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_RenderPixelCopyWith<$Res> {
  factory _$$ApplicationMessage_RenderPixelCopyWith(
          _$ApplicationMessage_RenderPixel value, $Res Function(_$ApplicationMessage_RenderPixel) then) =
      __$$ApplicationMessage_RenderPixelCopyWithImpl<$Res>;
  $Res call({int x, int y});
}

/// @nodoc
class __$$ApplicationMessage_RenderPixelCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_RenderPixelCopyWith<$Res> {
  __$$ApplicationMessage_RenderPixelCopyWithImpl(
      _$ApplicationMessage_RenderPixel _value, $Res Function(_$ApplicationMessage_RenderPixel) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_RenderPixel));

  @override
  _$ApplicationMessage_RenderPixel get _value => super._value as _$ApplicationMessage_RenderPixel;

  @override
  $Res call({
    Object? x = freezed,
    Object? y = freezed,
  }) {
    return _then(_$ApplicationMessage_RenderPixel(
      x: x == freezed
          ? _value.x
          : x // ignore: cast_nullable_to_non_nullable
              as int,
      y: y == freezed
          ? _value.y
          : y // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_RenderPixel implements ApplicationMessage_RenderPixel {
  const _$ApplicationMessage_RenderPixel({required this.x, required this.y});

  @override
  final int x;
  @override
  final int y;

  @override
  String toString() {
    return 'ApplicationMessage.renderPixel(x: $x, y: $y)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_RenderPixel &&
            const DeepCollectionEquality().equals(other.x, x) &&
            const DeepCollectionEquality().equals(other.y, y));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(x), const DeepCollectionEquality().hash(y));

  @JsonKey(ignore: true)
  @override
  _$$ApplicationMessage_RenderPixelCopyWith<_$ApplicationMessage_RenderPixel> get copyWith =>
      __$$ApplicationMessage_RenderPixelCopyWithImpl<_$ApplicationMessage_RenderPixel>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return renderPixel(x, y);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
  }) {
    return renderPixel?.call(x, y);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(x, y);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return renderPixel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
  }) {
    return renderPixel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_RenderPixel implements ApplicationMessage {
  const factory ApplicationMessage_RenderPixel({required final int x, required final int y}) =
      _$ApplicationMessage_RenderPixel;

  int get x;
  int get y;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_RenderPixelCopyWith<_$ApplicationMessage_RenderPixel> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_ExitCopyWith<$Res> {
  factory _$$ApplicationMessage_ExitCopyWith(
          _$ApplicationMessage_Exit value, $Res Function(_$ApplicationMessage_Exit) then) =
      __$$ApplicationMessage_ExitCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ApplicationMessage_ExitCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_ExitCopyWith<$Res> {
  __$$ApplicationMessage_ExitCopyWithImpl(
      _$ApplicationMessage_Exit _value, $Res Function(_$ApplicationMessage_Exit) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_Exit));

  @override
  _$ApplicationMessage_Exit get _value => super._value as _$ApplicationMessage_Exit;
}

/// @nodoc

class _$ApplicationMessage_Exit implements ApplicationMessage_Exit {
  const _$ApplicationMessage_Exit();

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$ApplicationMessage_Exit);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) displayMessage,
    required TResult Function(int x, int y) renderPixel,
    required TResult Function() exit,
  }) {
    return exit();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
  }) {
    return exit?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? displayMessage,
    TResult Function(int x, int y)? renderPixel,
    TResult Function()? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ApplicationMessage_DisplayMessage value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel value) renderPixel,
    required TResult Function(ApplicationMessage_Exit value) exit,
  }) {
    return exit(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
  }) {
    return exit?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult Function(ApplicationMessage_Exit value)? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_Exit implements ApplicationMessage {
  const factory ApplicationMessage_Exit() = _$ApplicationMessage_Exit;
}

/// @nodoc
mixin _$Distance {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DistanceCopyWith<$Res> {
  factory $DistanceCopyWith(Distance value, $Res Function(Distance) then) = _$DistanceCopyWithImpl<$Res>;
}

/// @nodoc
class _$DistanceCopyWithImpl<$Res> implements $DistanceCopyWith<$Res> {
  _$DistanceCopyWithImpl(this._value, this._then);

  final Distance _value;
  // ignore: unused_field
  final $Res Function(Distance) _then;
}

/// @nodoc
abstract class _$$Distance_UnknownCopyWith<$Res> {
  factory _$$Distance_UnknownCopyWith(_$Distance_Unknown value, $Res Function(_$Distance_Unknown) then) =
      __$$Distance_UnknownCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Distance_UnknownCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res>
    implements _$$Distance_UnknownCopyWith<$Res> {
  __$$Distance_UnknownCopyWithImpl(_$Distance_Unknown _value, $Res Function(_$Distance_Unknown) _then)
      : super(_value, (v) => _then(v as _$Distance_Unknown));

  @override
  _$Distance_Unknown get _value => super._value as _$Distance_Unknown;
}

/// @nodoc

class _$Distance_Unknown implements Distance_Unknown {
  const _$Distance_Unknown();

  @override
  String toString() {
    return 'Distance.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Distance_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Distance_Unknown implements Distance {
  const factory Distance_Unknown() = _$Distance_Unknown;
}

/// @nodoc
abstract class _$$Distance_MapCopyWith<$Res> {
  factory _$$Distance_MapCopyWith(_$Distance_Map value, $Res Function(_$Distance_Map) then) =
      __$$Distance_MapCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$Distance_MapCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res> implements _$$Distance_MapCopyWith<$Res> {
  __$$Distance_MapCopyWithImpl(_$Distance_Map _value, $Res Function(_$Distance_Map) _then)
      : super(_value, (v) => _then(v as _$Distance_Map));

  @override
  _$Distance_Map get _value => super._value as _$Distance_Map;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Distance_Map(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Distance_Map implements Distance_Map {
  const _$Distance_Map(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'Distance.map(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Distance_Map &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Distance_MapCopyWith<_$Distance_Map> get copyWith =>
      __$$Distance_MapCopyWithImpl<_$Distance_Map>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) map,
  }) {
    return map(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
  }) {
    return map?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Distance_Unknown value) unknown,
    required TResult Function(Distance_Map value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown value)? unknown,
    TResult Function(Distance_Map value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }
}

abstract class Distance_Map implements Distance {
  const factory Distance_Map(final double field0) = _$Distance_Map;

  double get field0;
  @JsonKey(ignore: true)
  _$$Distance_MapCopyWith<_$Distance_Map> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KitchenSinkCopyWith<$Res> {
  factory $KitchenSinkCopyWith(KitchenSink value, $Res Function(KitchenSink) then) = _$KitchenSinkCopyWithImpl<$Res>;
}

/// @nodoc
class _$KitchenSinkCopyWithImpl<$Res> implements $KitchenSinkCopyWith<$Res> {
  _$KitchenSinkCopyWithImpl(this._value, this._then);

  final KitchenSink _value;
  // ignore: unused_field
  final $Res Function(KitchenSink) _then;
}

/// @nodoc
abstract class _$$KitchenSink_EmptyCopyWith<$Res> {
  factory _$$KitchenSink_EmptyCopyWith(_$KitchenSink_Empty value, $Res Function(_$KitchenSink_Empty) then) =
      __$$KitchenSink_EmptyCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSink_EmptyCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_EmptyCopyWith<$Res> {
  __$$KitchenSink_EmptyCopyWithImpl(_$KitchenSink_Empty _value, $Res Function(_$KitchenSink_Empty) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Empty));

  @override
  _$KitchenSink_Empty get _value => super._value as _$KitchenSink_Empty;
}

/// @nodoc

class _$KitchenSink_Empty implements KitchenSink_Empty {
  const _$KitchenSink_Empty();

  @override
  String toString() {
    return 'KitchenSink.empty()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$KitchenSink_Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Empty implements KitchenSink {
  const factory KitchenSink_Empty() = _$KitchenSink_Empty;
}

/// @nodoc
abstract class _$$KitchenSink_PrimitivesCopyWith<$Res> {
  factory _$$KitchenSink_PrimitivesCopyWith(
          _$KitchenSink_Primitives value, $Res Function(_$KitchenSink_Primitives) then) =
      __$$KitchenSink_PrimitivesCopyWithImpl<$Res>;
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSink_PrimitivesCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_PrimitivesCopyWith<$Res> {
  __$$KitchenSink_PrimitivesCopyWithImpl(_$KitchenSink_Primitives _value, $Res Function(_$KitchenSink_Primitives) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Primitives));

  @override
  _$KitchenSink_Primitives get _value => super._value as _$KitchenSink_Primitives;

  @override
  $Res call({
    Object? int32 = freezed,
    Object? float64 = freezed,
    Object? boolean = freezed,
  }) {
    return _then(_$KitchenSink_Primitives(
      int32: int32 == freezed
          ? _value.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: float64 == freezed
          ? _value.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: boolean == freezed
          ? _value.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Primitives implements KitchenSink_Primitives {
  const _$KitchenSink_Primitives({required this.int32, required this.float64, required this.boolean});

  /// Dart field comment
  @override
  final int int32;
  @override
  final double float64;
  @override
  final bool boolean;

  @override
  String toString() {
    return 'KitchenSink.primitives(int32: $int32, float64: $float64, boolean: $boolean)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Primitives &&
            const DeepCollectionEquality().equals(other.int32, int32) &&
            const DeepCollectionEquality().equals(other.float64, float64) &&
            const DeepCollectionEquality().equals(other.boolean, boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(int32),
      const DeepCollectionEquality().hash(float64), const DeepCollectionEquality().hash(boolean));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_PrimitivesCopyWith<_$KitchenSink_Primitives> get copyWith =>
      __$$KitchenSink_PrimitivesCopyWithImpl<_$KitchenSink_Primitives>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(int32, float64, boolean);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Primitives implements KitchenSink {
  const factory KitchenSink_Primitives(
      {required final int int32,
      required final double float64,
      required final bool boolean}) = _$KitchenSink_Primitives;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$KitchenSink_PrimitivesCopyWith<_$KitchenSink_Primitives> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_NestedCopyWith<$Res> {
  factory _$$KitchenSink_NestedCopyWith(_$KitchenSink_Nested value, $Res Function(_$KitchenSink_Nested) then) =
      __$$KitchenSink_NestedCopyWithImpl<$Res>;
  $Res call({KitchenSink field0, int field1});

  $KitchenSinkCopyWith<$Res> get field0;
}

/// @nodoc
class __$$KitchenSink_NestedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_NestedCopyWith<$Res> {
  __$$KitchenSink_NestedCopyWithImpl(_$KitchenSink_Nested _value, $Res Function(_$KitchenSink_Nested) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Nested));

  @override
  _$KitchenSink_Nested get _value => super._value as _$KitchenSink_Nested;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_Nested(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KitchenSink,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }

  @override
  $KitchenSinkCopyWith<$Res> get field0 {
    return $KitchenSinkCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$KitchenSink_Nested implements KitchenSink_Nested {
  const _$KitchenSink_Nested(this.field0, this.field1);

  @override
  final KitchenSink field0;
  @override
  final int field1;

  @override
  String toString() {
    return 'KitchenSink.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Nested &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_NestedCopyWith<_$KitchenSink_Nested> get copyWith =>
      __$$KitchenSink_NestedCopyWithImpl<_$KitchenSink_Nested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Nested implements KitchenSink {
  const factory KitchenSink_Nested(final KitchenSink field0, final int field1) = _$KitchenSink_Nested;

  KitchenSink get field0;
  int get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_NestedCopyWith<_$KitchenSink_Nested> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_OptionalCopyWith<$Res> {
  factory _$$KitchenSink_OptionalCopyWith(_$KitchenSink_Optional value, $Res Function(_$KitchenSink_Optional) then) =
      __$$KitchenSink_OptionalCopyWithImpl<$Res>;
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSink_OptionalCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_OptionalCopyWith<$Res> {
  __$$KitchenSink_OptionalCopyWithImpl(_$KitchenSink_Optional _value, $Res Function(_$KitchenSink_Optional) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Optional));

  @override
  _$KitchenSink_Optional get _value => super._value as _$KitchenSink_Optional;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_Optional(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Optional implements KitchenSink_Optional {
  const _$KitchenSink_Optional([this.field0, this.field1]);

  /// Comment on anonymous field
  @override
  final int? field0;
  @override
  final int? field1;

  @override
  String toString() {
    return 'KitchenSink.optional(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Optional &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_OptionalCopyWith<_$KitchenSink_Optional> get copyWith =>
      __$$KitchenSink_OptionalCopyWithImpl<_$KitchenSink_Optional>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Optional implements KitchenSink {
  const factory KitchenSink_Optional([final int? field0, final int? field1]) = _$KitchenSink_Optional;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_OptionalCopyWith<_$KitchenSink_Optional> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_BufferCopyWith<$Res> {
  factory _$$KitchenSink_BufferCopyWith(_$KitchenSink_Buffer value, $Res Function(_$KitchenSink_Buffer) then) =
      __$$KitchenSink_BufferCopyWithImpl<$Res>;
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSink_BufferCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_BufferCopyWith<$Res> {
  __$$KitchenSink_BufferCopyWithImpl(_$KitchenSink_Buffer _value, $Res Function(_$KitchenSink_Buffer) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Buffer));

  @override
  _$KitchenSink_Buffer get _value => super._value as _$KitchenSink_Buffer;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KitchenSink_Buffer(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Buffer implements KitchenSink_Buffer {
  const _$KitchenSink_Buffer(this.field0);

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'KitchenSink.buffer(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_BufferCopyWith<_$KitchenSink_Buffer> get copyWith =>
      __$$KitchenSink_BufferCopyWithImpl<_$KitchenSink_Buffer>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Buffer implements KitchenSink {
  const factory KitchenSink_Buffer(final Uint8List field0) = _$KitchenSink_Buffer;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_BufferCopyWith<_$KitchenSink_Buffer> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_EnumsCopyWith<$Res> {
  factory _$$KitchenSink_EnumsCopyWith(_$KitchenSink_Enums value, $Res Function(_$KitchenSink_Enums) then) =
      __$$KitchenSink_EnumsCopyWithImpl<$Res>;
  $Res call({Weekdays field0});
}

/// @nodoc
class __$$KitchenSink_EnumsCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_EnumsCopyWith<$Res> {
  __$$KitchenSink_EnumsCopyWithImpl(_$KitchenSink_Enums _value, $Res Function(_$KitchenSink_Enums) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Enums));

  @override
  _$KitchenSink_Enums get _value => super._value as _$KitchenSink_Enums;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KitchenSink_Enums(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Enums implements KitchenSink_Enums {
  const _$KitchenSink_Enums(this.field0);

  @override
  final Weekdays field0;

  @override
  String toString() {
    return 'KitchenSink.enums(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Enums &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_EnumsCopyWith<_$KitchenSink_Enums> get copyWith =>
      __$$KitchenSink_EnumsCopyWithImpl<_$KitchenSink_Enums>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0, int field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0, int field1)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KitchenSink_Empty value) empty,
    required TResult Function(KitchenSink_Primitives value) primitives,
    required TResult Function(KitchenSink_Nested value) nested,
    required TResult Function(KitchenSink_Optional value) optional,
    required TResult Function(KitchenSink_Buffer value) buffer,
    required TResult Function(KitchenSink_Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty value)? empty,
    TResult Function(KitchenSink_Primitives value)? primitives,
    TResult Function(KitchenSink_Nested value)? nested,
    TResult Function(KitchenSink_Optional value)? optional,
    TResult Function(KitchenSink_Buffer value)? buffer,
    TResult Function(KitchenSink_Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Enums implements KitchenSink {
  const factory KitchenSink_Enums(final Weekdays field0) = _$KitchenSink_Enums;

  Weekdays get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_EnumsCopyWith<_$KitchenSink_Enums> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Measure {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MeasureCopyWith<$Res> {
  factory $MeasureCopyWith(Measure value, $Res Function(Measure) then) = _$MeasureCopyWithImpl<$Res>;
}

/// @nodoc
class _$MeasureCopyWithImpl<$Res> implements $MeasureCopyWith<$Res> {
  _$MeasureCopyWithImpl(this._value, this._then);

  final Measure _value;
  // ignore: unused_field
  final $Res Function(Measure) _then;
}

/// @nodoc
abstract class _$$Measure_SpeedCopyWith<$Res> {
  factory _$$Measure_SpeedCopyWith(_$Measure_Speed value, $Res Function(_$Measure_Speed) then) =
      __$$Measure_SpeedCopyWithImpl<$Res>;
  $Res call({Speed field0});

  $SpeedCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_SpeedCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res>
    implements _$$Measure_SpeedCopyWith<$Res> {
  __$$Measure_SpeedCopyWithImpl(_$Measure_Speed _value, $Res Function(_$Measure_Speed) _then)
      : super(_value, (v) => _then(v as _$Measure_Speed));

  @override
  _$Measure_Speed get _value => super._value as _$Measure_Speed;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Measure_Speed(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Speed,
    ));
  }

  @override
  $SpeedCopyWith<$Res> get field0 {
    return $SpeedCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Measure_Speed implements Measure_Speed {
  const _$Measure_Speed(this.field0);

  @override
  final Speed field0;

  @override
  String toString() {
    return 'Measure.speed(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Measure_Speed &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Measure_SpeedCopyWith<_$Measure_Speed> get copyWith =>
      __$$Measure_SpeedCopyWithImpl<_$Measure_Speed>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) {
    return speed(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
  }) {
    return speed?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }
}

abstract class Measure_Speed implements Measure {
  const factory Measure_Speed(final Speed field0) = _$Measure_Speed;

  Speed get field0;
  @JsonKey(ignore: true)
  _$$Measure_SpeedCopyWith<_$Measure_Speed> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Measure_DistanceCopyWith<$Res> {
  factory _$$Measure_DistanceCopyWith(_$Measure_Distance value, $Res Function(_$Measure_Distance) then) =
      __$$Measure_DistanceCopyWithImpl<$Res>;
  $Res call({Distance field0});

  $DistanceCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_DistanceCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res>
    implements _$$Measure_DistanceCopyWith<$Res> {
  __$$Measure_DistanceCopyWithImpl(_$Measure_Distance _value, $Res Function(_$Measure_Distance) _then)
      : super(_value, (v) => _then(v as _$Measure_Distance));

  @override
  _$Measure_Distance get _value => super._value as _$Measure_Distance;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Measure_Distance(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Distance,
    ));
  }

  @override
  $DistanceCopyWith<$Res> get field0 {
    return $DistanceCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Measure_Distance implements Measure_Distance {
  const _$Measure_Distance(this.field0);

  @override
  final Distance field0;

  @override
  String toString() {
    return 'Measure.distance(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Measure_Distance &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Measure_DistanceCopyWith<_$Measure_Distance> get copyWith =>
      __$$Measure_DistanceCopyWithImpl<_$Measure_Distance>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) {
    return distance(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
  }) {
    return distance?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(Speed field0)? speed,
    TResult Function(Distance field0)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Measure_Speed value) speed,
    required TResult Function(Measure_Distance value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed value)? speed,
    TResult Function(Measure_Distance value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }
}

abstract class Measure_Distance implements Measure {
  const factory Measure_Distance(final Distance field0) = _$Measure_Distance;

  Distance get field0;
  @JsonKey(ignore: true)
  _$$Measure_DistanceCopyWith<_$Measure_Distance> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Speed {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpeedCopyWith<$Res> {
  factory $SpeedCopyWith(Speed value, $Res Function(Speed) then) = _$SpeedCopyWithImpl<$Res>;
}

/// @nodoc
class _$SpeedCopyWithImpl<$Res> implements $SpeedCopyWith<$Res> {
  _$SpeedCopyWithImpl(this._value, this._then);

  final Speed _value;
  // ignore: unused_field
  final $Res Function(Speed) _then;
}

/// @nodoc
abstract class _$$Speed_UnknownCopyWith<$Res> {
  factory _$$Speed_UnknownCopyWith(_$Speed_Unknown value, $Res Function(_$Speed_Unknown) then) =
      __$$Speed_UnknownCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Speed_UnknownCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res> implements _$$Speed_UnknownCopyWith<$Res> {
  __$$Speed_UnknownCopyWithImpl(_$Speed_Unknown _value, $Res Function(_$Speed_Unknown) _then)
      : super(_value, (v) => _then(v as _$Speed_Unknown));

  @override
  _$Speed_Unknown get _value => super._value as _$Speed_Unknown;
}

/// @nodoc

class _$Speed_Unknown implements Speed_Unknown {
  const _$Speed_Unknown();

  @override
  String toString() {
    return 'Speed.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Speed_Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Speed_Unknown implements Speed {
  const factory Speed_Unknown() = _$Speed_Unknown;
}

/// @nodoc
abstract class _$$Speed_GPSCopyWith<$Res> {
  factory _$$Speed_GPSCopyWith(_$Speed_GPS value, $Res Function(_$Speed_GPS) then) = __$$Speed_GPSCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$Speed_GPSCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res> implements _$$Speed_GPSCopyWith<$Res> {
  __$$Speed_GPSCopyWithImpl(_$Speed_GPS _value, $Res Function(_$Speed_GPS) _then)
      : super(_value, (v) => _then(v as _$Speed_GPS));

  @override
  _$Speed_GPS get _value => super._value as _$Speed_GPS;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Speed_GPS(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Speed_GPS implements Speed_GPS {
  const _$Speed_GPS(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'Speed.gps(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Speed_GPS &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Speed_GPSCopyWith<_$Speed_GPS> get copyWith => __$$Speed_GPSCopyWithImpl<_$Speed_GPS>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() unknown,
    required TResult Function(double field0) gps,
  }) {
    return gps(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
  }) {
    return gps?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? unknown,
    TResult Function(double field0)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Speed_Unknown value) unknown,
    required TResult Function(Speed_GPS value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown value)? unknown,
    TResult Function(Speed_GPS value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }
}

abstract class Speed_GPS implements Speed {
  const factory Speed_GPS(final double field0) = _$Speed_GPS;

  double get field0;
  @JsonKey(ignore: true)
  _$$Speed_GPSCopyWith<_$Speed_GPS> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$UserId {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdCopyWith<UserId> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdCopyWith<$Res> {
  factory $UserIdCopyWith(UserId value, $Res Function(UserId) then) = _$UserIdCopyWithImpl<$Res>;
  $Res call({int value});
}

/// @nodoc
class _$UserIdCopyWithImpl<$Res> implements $UserIdCopyWith<$Res> {
  _$UserIdCopyWithImpl(this._value, this._then);

  final UserId _value;
  // ignore: unused_field
  final $Res Function(UserId) _then;

  @override
  $Res call({
    Object? value = freezed,
  }) {
    return _then(_value.copyWith(
      value: value == freezed
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
abstract class _$$_UserIdCopyWith<$Res> implements $UserIdCopyWith<$Res> {
  factory _$$_UserIdCopyWith(_$_UserId value, $Res Function(_$_UserId) then) = __$$_UserIdCopyWithImpl<$Res>;
  @override
  $Res call({int value});
}

/// @nodoc
class __$$_UserIdCopyWithImpl<$Res> extends _$UserIdCopyWithImpl<$Res> implements _$$_UserIdCopyWith<$Res> {
  __$$_UserIdCopyWithImpl(_$_UserId _value, $Res Function(_$_UserId) _then)
      : super(_value, (v) => _then(v as _$_UserId));

  @override
  _$_UserId get _value => super._value as _$_UserId;

  @override
  $Res call({
    Object? value = freezed,
  }) {
    return _then(_$_UserId(
      value: value == freezed
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$_UserId implements _UserId {
  const _$_UserId({required this.value});

  @override
  final int value;

  @override
  String toString() {
    return 'UserId(value: $value)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_UserId &&
            const DeepCollectionEquality().equals(other.value, value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(value));

  @JsonKey(ignore: true)
  @override
  _$$_UserIdCopyWith<_$_UserId> get copyWith => __$$_UserIdCopyWithImpl<_$_UserId>(this, _$identity);
}

abstract class _UserId implements UserId {
  const factory _UserId({required final int value}) = _$_UserId;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$_UserIdCopyWith<_$_UserId> get copyWith => throw _privateConstructorUsedError;
}
