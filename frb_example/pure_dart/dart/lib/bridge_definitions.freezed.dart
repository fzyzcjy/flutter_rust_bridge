// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_definitions.dart';

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
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
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
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
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
      _$ApplicationMessageCopyWithImpl<$Res, ApplicationMessage>;
}

/// @nodoc
class _$ApplicationMessageCopyWithImpl<$Res, $Val extends ApplicationMessage>
    implements $ApplicationMessageCopyWith<$Res> {
  _$ApplicationMessageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$ApplicationMessage_DisplayMessageCopyWith<$Res> {
  factory _$$ApplicationMessage_DisplayMessageCopyWith(
          _$ApplicationMessage_DisplayMessage value, $Res Function(_$ApplicationMessage_DisplayMessage) then) =
      __$$ApplicationMessage_DisplayMessageCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$ApplicationMessage_DisplayMessageCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_DisplayMessage>
    implements _$$ApplicationMessage_DisplayMessageCopyWith<$Res> {
  __$$ApplicationMessage_DisplayMessageCopyWithImpl(
      _$ApplicationMessage_DisplayMessage _value, $Res Function(_$ApplicationMessage_DisplayMessage) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$ApplicationMessage_DisplayMessage(
      null == field0
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
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
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
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
  @useResult
  $Res call({int x, int y});
}

/// @nodoc
class __$$ApplicationMessage_RenderPixelCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_RenderPixel>
    implements _$$ApplicationMessage_RenderPixelCopyWith<$Res> {
  __$$ApplicationMessage_RenderPixelCopyWithImpl(
      _$ApplicationMessage_RenderPixel _value, $Res Function(_$ApplicationMessage_RenderPixel) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? x = null,
    Object? y = null,
  }) {
    return _then(_$ApplicationMessage_RenderPixel(
      x: null == x
          ? _value.x
          : x // ignore: cast_nullable_to_non_nullable
              as int,
      y: null == y
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
            (identical(other.x, x) || other.x == x) &&
            (identical(other.y, y) || other.y == y));
  }

  @override
  int get hashCode => Object.hash(runtimeType, x, y);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
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
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
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
class __$$ApplicationMessage_ExitCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res, _$ApplicationMessage_Exit>
    implements _$$ApplicationMessage_ExitCopyWith<$Res> {
  __$$ApplicationMessage_ExitCopyWithImpl(
      _$ApplicationMessage_Exit _value, $Res Function(_$ApplicationMessage_Exit) _then)
      : super(_value, _then);
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
    TResult? Function(String field0)? displayMessage,
    TResult? Function(int x, int y)? renderPixel,
    TResult? Function()? exit,
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
    TResult? Function(ApplicationMessage_DisplayMessage value)? displayMessage,
    TResult? Function(ApplicationMessage_RenderPixel value)? renderPixel,
    TResult? Function(ApplicationMessage_Exit value)? exit,
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
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
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
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
  factory $DistanceCopyWith(Distance value, $Res Function(Distance) then) = _$DistanceCopyWithImpl<$Res, Distance>;
}

/// @nodoc
class _$DistanceCopyWithImpl<$Res, $Val extends Distance> implements $DistanceCopyWith<$Res> {
  _$DistanceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Distance_UnknownCopyWith<$Res> {
  factory _$$Distance_UnknownCopyWith(_$Distance_Unknown value, $Res Function(_$Distance_Unknown) then) =
      __$$Distance_UnknownCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Distance_UnknownCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res, _$Distance_Unknown>
    implements _$$Distance_UnknownCopyWith<$Res> {
  __$$Distance_UnknownCopyWithImpl(_$Distance_Unknown _value, $Res Function(_$Distance_Unknown) _then)
      : super(_value, _then);
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
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
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
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
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$Distance_MapCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res, _$Distance_Map>
    implements _$$Distance_MapCopyWith<$Res> {
  __$$Distance_MapCopyWithImpl(_$Distance_Map _value, $Res Function(_$Distance_Map) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Distance_Map(
      null == field0
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? map,
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
    TResult? Function(Distance_Unknown value)? unknown,
    TResult? Function(Distance_Map value)? map,
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
mixin _$EnumDartOpaque {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumDartOpaqueCopyWith<$Res> {
  factory $EnumDartOpaqueCopyWith(EnumDartOpaque value, $Res Function(EnumDartOpaque) then) =
      _$EnumDartOpaqueCopyWithImpl<$Res, EnumDartOpaque>;
}

/// @nodoc
class _$EnumDartOpaqueCopyWithImpl<$Res, $Val extends EnumDartOpaque> implements $EnumDartOpaqueCopyWith<$Res> {
  _$EnumDartOpaqueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumDartOpaque_PrimitiveCopyWith<$Res> {
  factory _$$EnumDartOpaque_PrimitiveCopyWith(
          _$EnumDartOpaque_Primitive value, $Res Function(_$EnumDartOpaque_Primitive) then) =
      __$$EnumDartOpaque_PrimitiveCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$EnumDartOpaque_PrimitiveCopyWithImpl<$Res>
    extends _$EnumDartOpaqueCopyWithImpl<$Res, _$EnumDartOpaque_Primitive>
    implements _$$EnumDartOpaque_PrimitiveCopyWith<$Res> {
  __$$EnumDartOpaque_PrimitiveCopyWithImpl(
      _$EnumDartOpaque_Primitive _value, $Res Function(_$EnumDartOpaque_Primitive) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumDartOpaque_Primitive(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$EnumDartOpaque_Primitive implements EnumDartOpaque_Primitive {
  const _$EnumDartOpaque_Primitive(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'EnumDartOpaque.primitive(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumDartOpaque_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumDartOpaque_PrimitiveCopyWith<_$EnumDartOpaque_Primitive> get copyWith =>
      __$$EnumDartOpaque_PrimitiveCopyWithImpl<_$EnumDartOpaque_Primitive>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }
}

abstract class EnumDartOpaque_Primitive implements EnumDartOpaque {
  const factory EnumDartOpaque_Primitive(final int field0) = _$EnumDartOpaque_Primitive;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$EnumDartOpaque_PrimitiveCopyWith<_$EnumDartOpaque_Primitive> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumDartOpaque_OpaqueCopyWith<$Res> {
  factory _$$EnumDartOpaque_OpaqueCopyWith(_$EnumDartOpaque_Opaque value, $Res Function(_$EnumDartOpaque_Opaque) then) =
      __$$EnumDartOpaque_OpaqueCopyWithImpl<$Res>;
  @useResult
  $Res call({Object field0});
}

/// @nodoc
class __$$EnumDartOpaque_OpaqueCopyWithImpl<$Res> extends _$EnumDartOpaqueCopyWithImpl<$Res, _$EnumDartOpaque_Opaque>
    implements _$$EnumDartOpaque_OpaqueCopyWith<$Res> {
  __$$EnumDartOpaque_OpaqueCopyWithImpl(_$EnumDartOpaque_Opaque _value, $Res Function(_$EnumDartOpaque_Opaque) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumDartOpaque_Opaque(
      null == field0 ? _value.field0 : field0,
    ));
  }
}

/// @nodoc

class _$EnumDartOpaque_Opaque implements EnumDartOpaque_Opaque {
  const _$EnumDartOpaque_Opaque(this.field0);

  @override
  final Object field0;

  @override
  String toString() {
    return 'EnumDartOpaque.opaque(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumDartOpaque_Opaque &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumDartOpaque_OpaqueCopyWith<_$EnumDartOpaque_Opaque> get copyWith =>
      __$$EnumDartOpaque_OpaqueCopyWithImpl<_$EnumDartOpaque_Opaque>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) primitive,
    required TResult Function(Object field0) opaque,
  }) {
    return opaque(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? primitive,
    TResult? Function(Object field0)? opaque,
  }) {
    return opaque?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? primitive,
    TResult Function(Object field0)? opaque,
    required TResult orElse(),
  }) {
    if (opaque != null) {
      return opaque(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumDartOpaque_Primitive value) primitive,
    required TResult Function(EnumDartOpaque_Opaque value) opaque,
  }) {
    return opaque(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumDartOpaque_Primitive value)? primitive,
    TResult? Function(EnumDartOpaque_Opaque value)? opaque,
  }) {
    return opaque?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumDartOpaque_Primitive value)? primitive,
    TResult Function(EnumDartOpaque_Opaque value)? opaque,
    required TResult orElse(),
  }) {
    if (opaque != null) {
      return opaque(this);
    }
    return orElse();
  }
}

abstract class EnumDartOpaque_Opaque implements EnumDartOpaque {
  const factory EnumDartOpaque_Opaque(final Object field0) = _$EnumDartOpaque_Opaque;

  @override
  Object get field0;
  @JsonKey(ignore: true)
  _$$EnumDartOpaque_OpaqueCopyWith<_$EnumDartOpaque_Opaque> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EnumOpaque {
  FrbOpaque get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumOpaqueCopyWith<$Res> {
  factory $EnumOpaqueCopyWith(EnumOpaque value, $Res Function(EnumOpaque) then) =
      _$EnumOpaqueCopyWithImpl<$Res, EnumOpaque>;
}

/// @nodoc
class _$EnumOpaqueCopyWithImpl<$Res, $Val extends EnumOpaque> implements $EnumOpaqueCopyWith<$Res> {
  _$EnumOpaqueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$EnumOpaque_StructCopyWith<$Res> {
  factory _$$EnumOpaque_StructCopyWith(_$EnumOpaque_Struct value, $Res Function(_$EnumOpaque_Struct) then) =
      __$$EnumOpaque_StructCopyWithImpl<$Res>;
  @useResult
  $Res call({HideData field0});
}

/// @nodoc
class __$$EnumOpaque_StructCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_Struct>
    implements _$$EnumOpaque_StructCopyWith<$Res> {
  __$$EnumOpaque_StructCopyWithImpl(_$EnumOpaque_Struct _value, $Res Function(_$EnumOpaque_Struct) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_Struct(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as HideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_Struct implements EnumOpaque_Struct {
  const _$EnumOpaque_Struct(this.field0);

  @override
  final HideData field0;

  @override
  String toString() {
    return 'EnumOpaque.struct(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_Struct &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_StructCopyWith<_$EnumOpaque_Struct> get copyWith =>
      __$$EnumOpaque_StructCopyWithImpl<_$EnumOpaque_Struct>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return struct(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return struct?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return struct(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return struct?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (struct != null) {
      return struct(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Struct implements EnumOpaque {
  const factory EnumOpaque_Struct(final HideData field0) = _$EnumOpaque_Struct;

  @override
  HideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_StructCopyWith<_$EnumOpaque_Struct> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_PrimitiveCopyWith<$Res> {
  factory _$$EnumOpaque_PrimitiveCopyWith(_$EnumOpaque_Primitive value, $Res Function(_$EnumOpaque_Primitive) then) =
      __$$EnumOpaque_PrimitiveCopyWithImpl<$Res>;
  @useResult
  $Res call({I32 field0});
}

/// @nodoc
class __$$EnumOpaque_PrimitiveCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_Primitive>
    implements _$$EnumOpaque_PrimitiveCopyWith<$Res> {
  __$$EnumOpaque_PrimitiveCopyWithImpl(_$EnumOpaque_Primitive _value, $Res Function(_$EnumOpaque_Primitive) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_Primitive(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as I32,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_Primitive implements EnumOpaque_Primitive {
  const _$EnumOpaque_Primitive(this.field0);

  @override
  final I32 field0;

  @override
  String toString() {
    return 'EnumOpaque.primitive(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_Primitive &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_PrimitiveCopyWith<_$EnumOpaque_Primitive> get copyWith =>
      __$$EnumOpaque_PrimitiveCopyWithImpl<_$EnumOpaque_Primitive>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return primitive(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return primitive?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return primitive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return primitive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (primitive != null) {
      return primitive(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Primitive implements EnumOpaque {
  const factory EnumOpaque_Primitive(final I32 field0) = _$EnumOpaque_Primitive;

  @override
  I32 get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_PrimitiveCopyWith<_$EnumOpaque_Primitive> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_TraitObjCopyWith<$Res> {
  factory _$$EnumOpaque_TraitObjCopyWith(_$EnumOpaque_TraitObj value, $Res Function(_$EnumOpaque_TraitObj) then) =
      __$$EnumOpaque_TraitObjCopyWithImpl<$Res>;
  @useResult
  $Res call({BoxDartDebug field0});
}

/// @nodoc
class __$$EnumOpaque_TraitObjCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_TraitObj>
    implements _$$EnumOpaque_TraitObjCopyWith<$Res> {
  __$$EnumOpaque_TraitObjCopyWithImpl(_$EnumOpaque_TraitObj _value, $Res Function(_$EnumOpaque_TraitObj) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_TraitObj(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BoxDartDebug,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_TraitObj implements EnumOpaque_TraitObj {
  const _$EnumOpaque_TraitObj(this.field0);

  @override
  final BoxDartDebug field0;

  @override
  String toString() {
    return 'EnumOpaque.traitObj(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_TraitObj &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_TraitObjCopyWith<_$EnumOpaque_TraitObj> get copyWith =>
      __$$EnumOpaque_TraitObjCopyWithImpl<_$EnumOpaque_TraitObj>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return traitObj(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return traitObj?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return traitObj(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return traitObj?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (traitObj != null) {
      return traitObj(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_TraitObj implements EnumOpaque {
  const factory EnumOpaque_TraitObj(final BoxDartDebug field0) = _$EnumOpaque_TraitObj;

  @override
  BoxDartDebug get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_TraitObjCopyWith<_$EnumOpaque_TraitObj> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_MutexCopyWith<$Res> {
  factory _$$EnumOpaque_MutexCopyWith(_$EnumOpaque_Mutex value, $Res Function(_$EnumOpaque_Mutex) then) =
      __$$EnumOpaque_MutexCopyWithImpl<$Res>;
  @useResult
  $Res call({MutexHideData field0});
}

/// @nodoc
class __$$EnumOpaque_MutexCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_Mutex>
    implements _$$EnumOpaque_MutexCopyWith<$Res> {
  __$$EnumOpaque_MutexCopyWithImpl(_$EnumOpaque_Mutex _value, $Res Function(_$EnumOpaque_Mutex) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_Mutex(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MutexHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_Mutex implements EnumOpaque_Mutex {
  const _$EnumOpaque_Mutex(this.field0);

  @override
  final MutexHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.mutex(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_Mutex &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_MutexCopyWith<_$EnumOpaque_Mutex> get copyWith =>
      __$$EnumOpaque_MutexCopyWithImpl<_$EnumOpaque_Mutex>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return mutex(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return mutex?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return mutex(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return mutex?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (mutex != null) {
      return mutex(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_Mutex implements EnumOpaque {
  const factory EnumOpaque_Mutex(final MutexHideData field0) = _$EnumOpaque_Mutex;

  @override
  MutexHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_MutexCopyWith<_$EnumOpaque_Mutex> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumOpaque_RwLockCopyWith<$Res> {
  factory _$$EnumOpaque_RwLockCopyWith(_$EnumOpaque_RwLock value, $Res Function(_$EnumOpaque_RwLock) then) =
      __$$EnumOpaque_RwLockCopyWithImpl<$Res>;
  @useResult
  $Res call({RwLockHideData field0});
}

/// @nodoc
class __$$EnumOpaque_RwLockCopyWithImpl<$Res> extends _$EnumOpaqueCopyWithImpl<$Res, _$EnumOpaque_RwLock>
    implements _$$EnumOpaque_RwLockCopyWith<$Res> {
  __$$EnumOpaque_RwLockCopyWithImpl(_$EnumOpaque_RwLock _value, $Res Function(_$EnumOpaque_RwLock) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$EnumOpaque_RwLock(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RwLockHideData,
    ));
  }
}

/// @nodoc

class _$EnumOpaque_RwLock implements EnumOpaque_RwLock {
  const _$EnumOpaque_RwLock(this.field0);

  @override
  final RwLockHideData field0;

  @override
  String toString() {
    return 'EnumOpaque.rwLock(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EnumOpaque_RwLock &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$EnumOpaque_RwLockCopyWith<_$EnumOpaque_RwLock> get copyWith =>
      __$$EnumOpaque_RwLockCopyWithImpl<_$EnumOpaque_RwLock>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(HideData field0) struct,
    required TResult Function(I32 field0) primitive,
    required TResult Function(BoxDartDebug field0) traitObj,
    required TResult Function(MutexHideData field0) mutex,
    required TResult Function(RwLockHideData field0) rwLock,
  }) {
    return rwLock(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(HideData field0)? struct,
    TResult? Function(I32 field0)? primitive,
    TResult? Function(BoxDartDebug field0)? traitObj,
    TResult? Function(MutexHideData field0)? mutex,
    TResult? Function(RwLockHideData field0)? rwLock,
  }) {
    return rwLock?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(HideData field0)? struct,
    TResult Function(I32 field0)? primitive,
    TResult Function(BoxDartDebug field0)? traitObj,
    TResult Function(MutexHideData field0)? mutex,
    TResult Function(RwLockHideData field0)? rwLock,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(EnumOpaque_Struct value) struct,
    required TResult Function(EnumOpaque_Primitive value) primitive,
    required TResult Function(EnumOpaque_TraitObj value) traitObj,
    required TResult Function(EnumOpaque_Mutex value) mutex,
    required TResult Function(EnumOpaque_RwLock value) rwLock,
  }) {
    return rwLock(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(EnumOpaque_Struct value)? struct,
    TResult? Function(EnumOpaque_Primitive value)? primitive,
    TResult? Function(EnumOpaque_TraitObj value)? traitObj,
    TResult? Function(EnumOpaque_Mutex value)? mutex,
    TResult? Function(EnumOpaque_RwLock value)? rwLock,
  }) {
    return rwLock?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(EnumOpaque_Struct value)? struct,
    TResult Function(EnumOpaque_Primitive value)? primitive,
    TResult Function(EnumOpaque_TraitObj value)? traitObj,
    TResult Function(EnumOpaque_Mutex value)? mutex,
    TResult Function(EnumOpaque_RwLock value)? rwLock,
    required TResult orElse(),
  }) {
    if (rwLock != null) {
      return rwLock(this);
    }
    return orElse();
  }
}

abstract class EnumOpaque_RwLock implements EnumOpaque {
  const factory EnumOpaque_RwLock(final RwLockHideData field0) = _$EnumOpaque_RwLock;

  @override
  RwLockHideData get field0;
  @JsonKey(ignore: true)
  _$$EnumOpaque_RwLockCopyWith<_$EnumOpaque_RwLock> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Event {
  String get address => throw _privateConstructorUsedError;
  String get payload => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $EventCopyWith<Event> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EventCopyWith<$Res> {
  factory $EventCopyWith(Event value, $Res Function(Event) then) = _$EventCopyWithImpl<$Res, Event>;
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class _$EventCopyWithImpl<$Res, $Val extends Event> implements $EventCopyWith<$Res> {
  _$EventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_value.copyWith(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_EventCopyWith<$Res> implements $EventCopyWith<$Res> {
  factory _$$_EventCopyWith(_$_Event value, $Res Function(_$_Event) then) = __$$_EventCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String address, String payload});
}

/// @nodoc
class __$$_EventCopyWithImpl<$Res> extends _$EventCopyWithImpl<$Res, _$_Event> implements _$$_EventCopyWith<$Res> {
  __$$_EventCopyWithImpl(_$_Event _value, $Res Function(_$_Event) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? address = null,
    Object? payload = null,
  }) {
    return _then(_$_Event(
      address: null == address
          ? _value.address
          : address // ignore: cast_nullable_to_non_nullable
              as String,
      payload: null == payload
          ? _value.payload
          : payload // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$_Event implements _Event {
  const _$_Event({required this.address, required this.payload});

  @override
  final String address;
  @override
  final String payload;

  @override
  String toString() {
    return 'Event(address: $address, payload: $payload)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_Event &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.payload, payload) || other.payload == payload));
  }

  @override
  int get hashCode => Object.hash(runtimeType, address, payload);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_EventCopyWith<_$_Event> get copyWith => __$$_EventCopyWithImpl<_$_Event>(this, _$identity);
}

abstract class _Event implements Event {
  const factory _Event({required final String address, required final String payload}) = _$_Event;

  @override
  String get address;
  @override
  String get payload;
  @override
  @JsonKey(ignore: true)
  _$$_EventCopyWith<_$_Event> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  factory $KitchenSinkCopyWith(KitchenSink value, $Res Function(KitchenSink) then) =
      _$KitchenSinkCopyWithImpl<$Res, KitchenSink>;
}

/// @nodoc
class _$KitchenSinkCopyWithImpl<$Res, $Val extends KitchenSink> implements $KitchenSinkCopyWith<$Res> {
  _$KitchenSinkCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$KitchenSink_EmptyCopyWith<$Res> {
  factory _$$KitchenSink_EmptyCopyWith(_$KitchenSink_Empty value, $Res Function(_$KitchenSink_Empty) then) =
      __$$KitchenSink_EmptyCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSink_EmptyCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Empty>
    implements _$$KitchenSink_EmptyCopyWith<$Res> {
  __$$KitchenSink_EmptyCopyWithImpl(_$KitchenSink_Empty _value, $Res Function(_$KitchenSink_Empty) _then)
      : super(_value, _then);
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
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return empty();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return empty?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  @useResult
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSink_PrimitivesCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Primitives>
    implements _$$KitchenSink_PrimitivesCopyWith<$Res> {
  __$$KitchenSink_PrimitivesCopyWithImpl(_$KitchenSink_Primitives _value, $Res Function(_$KitchenSink_Primitives) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? int32 = null,
    Object? float64 = null,
    Object? boolean = null,
  }) {
    return _then(_$KitchenSink_Primitives(
      int32: null == int32
          ? _value.int32
          : int32 // ignore: cast_nullable_to_non_nullable
              as int,
      float64: null == float64
          ? _value.float64
          : float64 // ignore: cast_nullable_to_non_nullable
              as double,
      boolean: null == boolean
          ? _value.boolean
          : boolean // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Primitives implements KitchenSink_Primitives {
  const _$KitchenSink_Primitives({this.int32 = -1, required this.float64, required this.boolean});

  /// Dart field comment
  @override
  @JsonKey()
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
            (identical(other.int32, int32) || other.int32 == int32) &&
            (identical(other.float64, float64) || other.float64 == float64) &&
            (identical(other.boolean, boolean) || other.boolean == boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, int32, float64, boolean);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_PrimitivesCopyWith<_$KitchenSink_Primitives> get copyWith =>
      __$$KitchenSink_PrimitivesCopyWithImpl<_$KitchenSink_Primitives>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return primitives(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return primitives?.call(int32, float64, boolean);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  const factory KitchenSink_Primitives({final int int32, required final double float64, required final bool boolean}) =
      _$KitchenSink_Primitives;

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
  @useResult
  $Res call({int field0, KitchenSink field1});

  $KitchenSinkCopyWith<$Res> get field1;
}

/// @nodoc
class __$$KitchenSink_NestedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Nested>
    implements _$$KitchenSink_NestedCopyWith<$Res> {
  __$$KitchenSink_NestedCopyWithImpl(_$KitchenSink_Nested _value, $Res Function(_$KitchenSink_Nested) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$KitchenSink_Nested(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as KitchenSink,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $KitchenSinkCopyWith<$Res> get field1 {
    return $KitchenSinkCopyWith<$Res>(_value.field1, (value) {
      return _then(_value.copyWith(field1: value));
    });
  }
}

/// @nodoc

class _$KitchenSink_Nested implements KitchenSink_Nested {
  const _$KitchenSink_Nested(this.field0, [this.field1 = const KitchenSink.empty()]);

  @override
  final int field0;
  @override
  @JsonKey()
  final KitchenSink field1;

  @override
  String toString() {
    return 'KitchenSink.nested(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KitchenSink_Nested &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_NestedCopyWith<_$KitchenSink_Nested> get copyWith =>
      __$$KitchenSink_NestedCopyWithImpl<_$KitchenSink_Nested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return nested(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return nested?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  const factory KitchenSink_Nested(final int field0, [final KitchenSink field1]) = _$KitchenSink_Nested;

  int get field0;
  KitchenSink get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_NestedCopyWith<_$KitchenSink_Nested> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_OptionalCopyWith<$Res> {
  factory _$$KitchenSink_OptionalCopyWith(_$KitchenSink_Optional value, $Res Function(_$KitchenSink_Optional) then) =
      __$$KitchenSink_OptionalCopyWithImpl<$Res>;
  @useResult
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSink_OptionalCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Optional>
    implements _$$KitchenSink_OptionalCopyWith<$Res> {
  __$$KitchenSink_OptionalCopyWithImpl(_$KitchenSink_Optional _value, $Res Function(_$KitchenSink_Optional) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_Optional(
      freezed == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int?,
      freezed == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int?,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Optional implements KitchenSink_Optional {
  const _$KitchenSink_Optional([this.field0 = -1, this.field1]);

  /// Comment on anonymous field
  @override
  @JsonKey()
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
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_OptionalCopyWith<_$KitchenSink_Optional> get copyWith =>
      __$$KitchenSink_OptionalCopyWithImpl<_$KitchenSink_Optional>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return optional(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return optional?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSink_BufferCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Buffer>
    implements _$$KitchenSink_BufferCopyWith<$Res> {
  __$$KitchenSink_BufferCopyWithImpl(_$KitchenSink_Buffer _value, $Res Function(_$KitchenSink_Buffer) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSink_Buffer(
      null == field0
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
  @pragma('vm:prefer-inline')
  _$$KitchenSink_BufferCopyWith<_$KitchenSink_Buffer> get copyWith =>
      __$$KitchenSink_BufferCopyWithImpl<_$KitchenSink_Buffer>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  @useResult
  $Res call({Weekdays field0});
}

/// @nodoc
class __$$KitchenSink_EnumsCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res, _$KitchenSink_Enums>
    implements _$$KitchenSink_EnumsCopyWith<$Res> {
  __$$KitchenSink_EnumsCopyWithImpl(_$KitchenSink_Enums _value, $Res Function(_$KitchenSink_Enums) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$KitchenSink_Enums(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Enums implements KitchenSink_Enums {
  const _$KitchenSink_Enums([this.field0 = Weekdays.Sunday]);

  @override
  @JsonKey()
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$KitchenSink_EnumsCopyWith<_$KitchenSink_Enums> get copyWith =>
      __$$KitchenSink_EnumsCopyWithImpl<_$KitchenSink_Enums>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(int field0, KitchenSink field1) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return enums(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? empty,
    TResult? Function(int int32, double float64, bool boolean)? primitives,
    TResult? Function(int field0, KitchenSink field1)? nested,
    TResult? Function(int? field0, int? field1)? optional,
    TResult? Function(Uint8List field0)? buffer,
    TResult? Function(Weekdays field0)? enums,
  }) {
    return enums?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(int field0, KitchenSink field1)? nested,
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
    TResult? Function(KitchenSink_Empty value)? empty,
    TResult? Function(KitchenSink_Primitives value)? primitives,
    TResult? Function(KitchenSink_Nested value)? nested,
    TResult? Function(KitchenSink_Optional value)? optional,
    TResult? Function(KitchenSink_Buffer value)? buffer,
    TResult? Function(KitchenSink_Enums value)? enums,
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
  const factory KitchenSink_Enums([final Weekdays field0]) = _$KitchenSink_Enums;

  Weekdays get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_EnumsCopyWith<_$KitchenSink_Enums> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Measure {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(Speed field0) speed,
    required TResult Function(Distance field0) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
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
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
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
  factory $MeasureCopyWith(Measure value, $Res Function(Measure) then) = _$MeasureCopyWithImpl<$Res, Measure>;
}

/// @nodoc
class _$MeasureCopyWithImpl<$Res, $Val extends Measure> implements $MeasureCopyWith<$Res> {
  _$MeasureCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Measure_SpeedCopyWith<$Res> {
  factory _$$Measure_SpeedCopyWith(_$Measure_Speed value, $Res Function(_$Measure_Speed) then) =
      __$$Measure_SpeedCopyWithImpl<$Res>;
  @useResult
  $Res call({Speed field0});

  $SpeedCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_SpeedCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res, _$Measure_Speed>
    implements _$$Measure_SpeedCopyWith<$Res> {
  __$$Measure_SpeedCopyWithImpl(_$Measure_Speed _value, $Res Function(_$Measure_Speed) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Measure_Speed(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Speed,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
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
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
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

  @override
  Speed get field0;
  @JsonKey(ignore: true)
  _$$Measure_SpeedCopyWith<_$Measure_Speed> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Measure_DistanceCopyWith<$Res> {
  factory _$$Measure_DistanceCopyWith(_$Measure_Distance value, $Res Function(_$Measure_Distance) then) =
      __$$Measure_DistanceCopyWithImpl<$Res>;
  @useResult
  $Res call({Distance field0});

  $DistanceCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_DistanceCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res, _$Measure_Distance>
    implements _$$Measure_DistanceCopyWith<$Res> {
  __$$Measure_DistanceCopyWithImpl(_$Measure_Distance _value, $Res Function(_$Measure_Distance) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Measure_Distance(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Distance,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function(Speed field0)? speed,
    TResult? Function(Distance field0)? distance,
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
    TResult? Function(Measure_Speed value)? speed,
    TResult? Function(Measure_Distance value)? distance,
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

  @override
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
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
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
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
  factory $SpeedCopyWith(Speed value, $Res Function(Speed) then) = _$SpeedCopyWithImpl<$Res, Speed>;
}

/// @nodoc
class _$SpeedCopyWithImpl<$Res, $Val extends Speed> implements $SpeedCopyWith<$Res> {
  _$SpeedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Speed_UnknownCopyWith<$Res> {
  factory _$$Speed_UnknownCopyWith(_$Speed_Unknown value, $Res Function(_$Speed_Unknown) then) =
      __$$Speed_UnknownCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Speed_UnknownCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res, _$Speed_Unknown>
    implements _$$Speed_UnknownCopyWith<$Res> {
  __$$Speed_UnknownCopyWithImpl(_$Speed_Unknown _value, $Res Function(_$Speed_Unknown) _then) : super(_value, _then);
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
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
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
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
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$Speed_GPSCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res, _$Speed_GPS>
    implements _$$Speed_GPSCopyWith<$Res> {
  __$$Speed_GPSCopyWithImpl(_$Speed_GPS _value, $Res Function(_$Speed_GPS) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Speed_GPS(
      null == field0
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
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
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
    TResult? Function()? unknown,
    TResult? Function(double field0)? gps,
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
    TResult? Function(Speed_Unknown value)? unknown,
    TResult? Function(Speed_GPS value)? gps,
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
  factory $UserIdCopyWith(UserId value, $Res Function(UserId) then) = _$UserIdCopyWithImpl<$Res, UserId>;
  @useResult
  $Res call({int value});
}

/// @nodoc
class _$UserIdCopyWithImpl<$Res, $Val extends UserId> implements $UserIdCopyWith<$Res> {
  _$UserIdCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_value.copyWith(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_UserIdCopyWith<$Res> implements $UserIdCopyWith<$Res> {
  factory _$$_UserIdCopyWith(_$_UserId value, $Res Function(_$_UserId) then) = __$$_UserIdCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int value});
}

/// @nodoc
class __$$_UserIdCopyWithImpl<$Res> extends _$UserIdCopyWithImpl<$Res, _$_UserId> implements _$$_UserIdCopyWith<$Res> {
  __$$_UserIdCopyWithImpl(_$_UserId _value, $Res Function(_$_UserId) _then) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? value = null,
  }) {
    return _then(_$_UserId(
      value: null == value
          ? _value.value
          : value // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$_UserId implements _UserId {
  const _$_UserId({this.value = 0});

  @override
  @JsonKey()
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
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, value);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_UserIdCopyWith<_$_UserId> get copyWith => __$$_UserIdCopyWithImpl<_$_UserId>(this, _$identity);
}

abstract class _UserId implements UserId {
  const factory _UserId({final int value}) = _$_UserId;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$_UserIdCopyWith<_$_UserId> get copyWith => throw _privateConstructorUsedError;
}
