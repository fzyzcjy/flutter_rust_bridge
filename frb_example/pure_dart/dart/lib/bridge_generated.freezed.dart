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
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more informations: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
class _$ApplicationMessageTearOff {
  const _$ApplicationMessageTearOff();

  DisplayMessage displayMessage(String field0) {
    return DisplayMessage(
      field0,
    );
  }

  RenderPixel renderPixel({required int x, required int y}) {
    return RenderPixel(
      x: x,
      y: y,
    );
  }

  Exit exit() {
    return const Exit();
  }
}

/// @nodoc
const $ApplicationMessage = _$ApplicationMessageTearOff();

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
    required TResult Function(DisplayMessage value) displayMessage,
    required TResult Function(RenderPixel value) renderPixel,
    required TResult Function(Exit value) exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
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
abstract class $DisplayMessageCopyWith<$Res> {
  factory $DisplayMessageCopyWith(DisplayMessage value, $Res Function(DisplayMessage) then) =
      _$DisplayMessageCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class _$DisplayMessageCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements $DisplayMessageCopyWith<$Res> {
  _$DisplayMessageCopyWithImpl(DisplayMessage _value, $Res Function(DisplayMessage) _then)
      : super(_value, (v) => _then(v as DisplayMessage));

  @override
  DisplayMessage get _value => super._value as DisplayMessage;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(DisplayMessage(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$DisplayMessage implements DisplayMessage {
  const _$DisplayMessage(this.field0);

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
            other is DisplayMessage &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $DisplayMessageCopyWith<DisplayMessage> get copyWith =>
      _$DisplayMessageCopyWithImpl<DisplayMessage>(this, _$identity);

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
    required TResult Function(DisplayMessage value) displayMessage,
    required TResult Function(RenderPixel value) renderPixel,
    required TResult Function(Exit value) exit,
  }) {
    return displayMessage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
  }) {
    return displayMessage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(this);
    }
    return orElse();
  }
}

abstract class DisplayMessage implements ApplicationMessage {
  const factory DisplayMessage(String field0) = _$DisplayMessage;

  String get field0;
  @JsonKey(ignore: true)
  $DisplayMessageCopyWith<DisplayMessage> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RenderPixelCopyWith<$Res> {
  factory $RenderPixelCopyWith(RenderPixel value, $Res Function(RenderPixel) then) = _$RenderPixelCopyWithImpl<$Res>;
  $Res call({int x, int y});
}

/// @nodoc
class _$RenderPixelCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements $RenderPixelCopyWith<$Res> {
  _$RenderPixelCopyWithImpl(RenderPixel _value, $Res Function(RenderPixel) _then)
      : super(_value, (v) => _then(v as RenderPixel));

  @override
  RenderPixel get _value => super._value as RenderPixel;

  @override
  $Res call({
    Object? x = freezed,
    Object? y = freezed,
  }) {
    return _then(RenderPixel(
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

class _$RenderPixel implements RenderPixel {
  const _$RenderPixel({required this.x, required this.y});

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
            other is RenderPixel &&
            const DeepCollectionEquality().equals(other.x, x) &&
            const DeepCollectionEquality().equals(other.y, y));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(x), const DeepCollectionEquality().hash(y));

  @JsonKey(ignore: true)
  @override
  $RenderPixelCopyWith<RenderPixel> get copyWith => _$RenderPixelCopyWithImpl<RenderPixel>(this, _$identity);

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
    required TResult Function(DisplayMessage value) displayMessage,
    required TResult Function(RenderPixel value) renderPixel,
    required TResult Function(Exit value) exit,
  }) {
    return renderPixel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
  }) {
    return renderPixel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(this);
    }
    return orElse();
  }
}

abstract class RenderPixel implements ApplicationMessage {
  const factory RenderPixel({required int x, required int y}) = _$RenderPixel;

  int get x;
  int get y;
  @JsonKey(ignore: true)
  $RenderPixelCopyWith<RenderPixel> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ExitCopyWith<$Res> {
  factory $ExitCopyWith(Exit value, $Res Function(Exit) then) = _$ExitCopyWithImpl<$Res>;
}

/// @nodoc
class _$ExitCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res> implements $ExitCopyWith<$Res> {
  _$ExitCopyWithImpl(Exit _value, $Res Function(Exit) _then) : super(_value, (v) => _then(v as Exit));

  @override
  Exit get _value => super._value as Exit;
}

/// @nodoc

class _$Exit implements Exit {
  const _$Exit();

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is Exit);
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
    required TResult Function(DisplayMessage value) displayMessage,
    required TResult Function(RenderPixel value) renderPixel,
    required TResult Function(Exit value) exit,
  }) {
    return exit(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
  }) {
    return exit?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DisplayMessage value)? displayMessage,
    TResult Function(RenderPixel value)? renderPixel,
    TResult Function(Exit value)? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit(this);
    }
    return orElse();
  }
}

abstract class Exit implements ApplicationMessage {
  const factory Exit() = _$Exit;
}

/// @nodoc
class _$KitchenSinkTearOff {
  const _$KitchenSinkTearOff();

  Empty empty() {
    return const Empty();
  }

  Primitives primitives({required int int32, required double float64, required bool boolean}) {
    return Primitives(
      int32: int32,
      float64: float64,
      boolean: boolean,
    );
  }

  Nested nested(KitchenSink field0) {
    return Nested(
      field0,
    );
  }

  Optional optional([int? field0, int? field1]) {
    return Optional(
      field0,
      field1,
    );
  }

  Buffer buffer(Uint8List field0) {
    return Buffer(
      field0,
    );
  }

  Enums enums(Weekdays field0) {
    return Enums(
      field0,
    );
  }
}

/// @nodoc
const $KitchenSink = _$KitchenSinkTearOff();

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
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
abstract class $EmptyCopyWith<$Res> {
  factory $EmptyCopyWith(Empty value, $Res Function(Empty) then) = _$EmptyCopyWithImpl<$Res>;
}

/// @nodoc
class _$EmptyCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $EmptyCopyWith<$Res> {
  _$EmptyCopyWithImpl(Empty _value, $Res Function(Empty) _then) : super(_value, (v) => _then(v as Empty));

  @override
  Empty get _value => super._value as Empty;
}

/// @nodoc

class _$Empty implements Empty {
  const _$Empty();

  @override
  String toString() {
    return 'KitchenSink.empty()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class Empty implements KitchenSink {
  const factory Empty() = _$Empty;
}

/// @nodoc
abstract class $PrimitivesCopyWith<$Res> {
  factory $PrimitivesCopyWith(Primitives value, $Res Function(Primitives) then) = _$PrimitivesCopyWithImpl<$Res>;
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class _$PrimitivesCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $PrimitivesCopyWith<$Res> {
  _$PrimitivesCopyWithImpl(Primitives _value, $Res Function(Primitives) _then)
      : super(_value, (v) => _then(v as Primitives));

  @override
  Primitives get _value => super._value as Primitives;

  @override
  $Res call({
    Object? int32 = freezed,
    Object? float64 = freezed,
    Object? boolean = freezed,
  }) {
    return _then(Primitives(
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

class _$Primitives implements Primitives {
  const _$Primitives({required this.int32, required this.float64, required this.boolean});

  @override

  /// Dart field comment
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
            other is Primitives &&
            const DeepCollectionEquality().equals(other.int32, int32) &&
            const DeepCollectionEquality().equals(other.float64, float64) &&
            const DeepCollectionEquality().equals(other.boolean, boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(int32),
      const DeepCollectionEquality().hash(float64), const DeepCollectionEquality().hash(boolean));

  @JsonKey(ignore: true)
  @override
  $PrimitivesCopyWith<Primitives> get copyWith => _$PrimitivesCopyWithImpl<Primitives>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }
}

abstract class Primitives implements KitchenSink {
  const factory Primitives({required int int32, required double float64, required bool boolean}) = _$Primitives;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  $PrimitivesCopyWith<Primitives> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $NestedCopyWith<$Res> {
  factory $NestedCopyWith(Nested value, $Res Function(Nested) then) = _$NestedCopyWithImpl<$Res>;
  $Res call({KitchenSink field0});

  $KitchenSinkCopyWith<$Res> get field0;
}

/// @nodoc
class _$NestedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $NestedCopyWith<$Res> {
  _$NestedCopyWithImpl(Nested _value, $Res Function(Nested) _then) : super(_value, (v) => _then(v as Nested));

  @override
  Nested get _value => super._value as Nested;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Nested(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KitchenSink,
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

class _$Nested implements Nested {
  const _$Nested(this.field0);

  @override
  final KitchenSink field0;

  @override
  String toString() {
    return 'KitchenSink.nested(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Nested &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $NestedCopyWith<Nested> get copyWith => _$NestedCopyWithImpl<Nested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
    required TResult Function(int? field0, int? field1) optional,
    required TResult Function(Uint8List field0) buffer,
    required TResult Function(Weekdays field0) enums,
  }) {
    return nested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
  }) {
    return nested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? empty,
    TResult Function(int int32, double float64, bool boolean)? primitives,
    TResult Function(KitchenSink field0)? nested,
    TResult Function(int? field0, int? field1)? optional,
    TResult Function(Uint8List field0)? buffer,
    TResult Function(Weekdays field0)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class Nested implements KitchenSink {
  const factory Nested(KitchenSink field0) = _$Nested;

  KitchenSink get field0;
  @JsonKey(ignore: true)
  $NestedCopyWith<Nested> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OptionalCopyWith<$Res> {
  factory $OptionalCopyWith(Optional value, $Res Function(Optional) then) = _$OptionalCopyWithImpl<$Res>;
  $Res call({int? field0, int? field1});
}

/// @nodoc
class _$OptionalCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $OptionalCopyWith<$Res> {
  _$OptionalCopyWithImpl(Optional _value, $Res Function(Optional) _then) : super(_value, (v) => _then(v as Optional));

  @override
  Optional get _value => super._value as Optional;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(Optional(
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

class _$Optional implements Optional {
  const _$Optional([this.field0, this.field1]);

  @override

  /// Comment on anonymous field
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
            other is Optional &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  $OptionalCopyWith<Optional> get copyWith => _$OptionalCopyWithImpl<Optional>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class Optional implements KitchenSink {
  const factory Optional([int? field0, int? field1]) = _$Optional;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  $OptionalCopyWith<Optional> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $BufferCopyWith<$Res> {
  factory $BufferCopyWith(Buffer value, $Res Function(Buffer) then) = _$BufferCopyWithImpl<$Res>;
  $Res call({Uint8List field0});
}

/// @nodoc
class _$BufferCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $BufferCopyWith<$Res> {
  _$BufferCopyWithImpl(Buffer _value, $Res Function(Buffer) _then) : super(_value, (v) => _then(v as Buffer));

  @override
  Buffer get _value => super._value as Buffer;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Buffer(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$Buffer implements Buffer {
  const _$Buffer(this.field0);

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
            other is Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $BufferCopyWith<Buffer> get copyWith => _$BufferCopyWithImpl<Buffer>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class Buffer implements KitchenSink {
  const factory Buffer(Uint8List field0) = _$Buffer;

  Uint8List get field0;
  @JsonKey(ignore: true)
  $BufferCopyWith<Buffer> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EnumsCopyWith<$Res> {
  factory $EnumsCopyWith(Enums value, $Res Function(Enums) then) = _$EnumsCopyWithImpl<$Res>;
  $Res call({Weekdays field0});
}

/// @nodoc
class _$EnumsCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res> implements $EnumsCopyWith<$Res> {
  _$EnumsCopyWithImpl(Enums _value, $Res Function(Enums) _then) : super(_value, (v) => _then(v as Enums));

  @override
  Enums get _value => super._value as Enums;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(Enums(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$Enums implements Enums {
  const _$Enums(this.field0);

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
            other is Enums &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  $EnumsCopyWith<Enums> get copyWith => _$EnumsCopyWithImpl<Enums>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean) primitives,
    required TResult Function(KitchenSink field0) nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    TResult Function(KitchenSink field0)? nested,
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
    required TResult Function(Empty value) empty,
    required TResult Function(Primitives value) primitives,
    required TResult Function(Nested value) nested,
    required TResult Function(Optional value) optional,
    required TResult Function(Buffer value) buffer,
    required TResult Function(Enums value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Empty value)? empty,
    TResult Function(Primitives value)? primitives,
    TResult Function(Nested value)? nested,
    TResult Function(Optional value)? optional,
    TResult Function(Buffer value)? buffer,
    TResult Function(Enums value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class Enums implements KitchenSink {
  const factory Enums(Weekdays field0) = _$Enums;

  Weekdays get field0;
  @JsonKey(ignore: true)
  $EnumsCopyWith<Enums> get copyWith => throw _privateConstructorUsedError;
}
