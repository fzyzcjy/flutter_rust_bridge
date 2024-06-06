// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'mirror.dart';

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
    required TResult Function(ApplicationMessage_DisplayMessage value)
        displayMessage,
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
  factory $ApplicationMessageCopyWith(
          ApplicationMessage value, $Res Function(ApplicationMessage) then) =
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
abstract class _$$ApplicationMessage_DisplayMessageImplCopyWith<$Res> {
  factory _$$ApplicationMessage_DisplayMessageImplCopyWith(
          _$ApplicationMessage_DisplayMessageImpl value,
          $Res Function(_$ApplicationMessage_DisplayMessageImpl) then) =
      __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res,
        _$ApplicationMessage_DisplayMessageImpl>
    implements _$$ApplicationMessage_DisplayMessageImplCopyWith<$Res> {
  __$$ApplicationMessage_DisplayMessageImplCopyWithImpl(
      _$ApplicationMessage_DisplayMessageImpl _value,
      $Res Function(_$ApplicationMessage_DisplayMessageImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$ApplicationMessage_DisplayMessageImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_DisplayMessageImpl
    extends ApplicationMessage_DisplayMessage {
  const _$ApplicationMessage_DisplayMessageImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'ApplicationMessage.displayMessage(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_DisplayMessageImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ApplicationMessage_DisplayMessageImplCopyWith<
          _$ApplicationMessage_DisplayMessageImpl>
      get copyWith => __$$ApplicationMessage_DisplayMessageImplCopyWithImpl<
          _$ApplicationMessage_DisplayMessageImpl>(this, _$identity);

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
    required TResult Function(ApplicationMessage_DisplayMessage value)
        displayMessage,
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

abstract class ApplicationMessage_DisplayMessage extends ApplicationMessage {
  const factory ApplicationMessage_DisplayMessage(final String field0) =
      _$ApplicationMessage_DisplayMessageImpl;
  const ApplicationMessage_DisplayMessage._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_DisplayMessageImplCopyWith<
          _$ApplicationMessage_DisplayMessageImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_RenderPixelImplCopyWith<$Res> {
  factory _$$ApplicationMessage_RenderPixelImplCopyWith(
          _$ApplicationMessage_RenderPixelImpl value,
          $Res Function(_$ApplicationMessage_RenderPixelImpl) then) =
      __$$ApplicationMessage_RenderPixelImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int x, int y});
}

/// @nodoc
class __$$ApplicationMessage_RenderPixelImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res,
        _$ApplicationMessage_RenderPixelImpl>
    implements _$$ApplicationMessage_RenderPixelImplCopyWith<$Res> {
  __$$ApplicationMessage_RenderPixelImplCopyWithImpl(
      _$ApplicationMessage_RenderPixelImpl _value,
      $Res Function(_$ApplicationMessage_RenderPixelImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? x = null,
    Object? y = null,
  }) {
    return _then(_$ApplicationMessage_RenderPixelImpl(
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

class _$ApplicationMessage_RenderPixelImpl
    extends ApplicationMessage_RenderPixel {
  const _$ApplicationMessage_RenderPixelImpl({required this.x, required this.y})
      : super._();

  @override
  final int x;
  @override
  final int y;

  @override
  String toString() {
    return 'ApplicationMessage.renderPixel(x: $x, y: $y)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_RenderPixelImpl &&
            (identical(other.x, x) || other.x == x) &&
            (identical(other.y, y) || other.y == y));
  }

  @override
  int get hashCode => Object.hash(runtimeType, x, y);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ApplicationMessage_RenderPixelImplCopyWith<
          _$ApplicationMessage_RenderPixelImpl>
      get copyWith => __$$ApplicationMessage_RenderPixelImplCopyWithImpl<
          _$ApplicationMessage_RenderPixelImpl>(this, _$identity);

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
    required TResult Function(ApplicationMessage_DisplayMessage value)
        displayMessage,
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

abstract class ApplicationMessage_RenderPixel extends ApplicationMessage {
  const factory ApplicationMessage_RenderPixel(
      {required final int x,
      required final int y}) = _$ApplicationMessage_RenderPixelImpl;
  const ApplicationMessage_RenderPixel._() : super._();

  int get x;
  int get y;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_RenderPixelImplCopyWith<
          _$ApplicationMessage_RenderPixelImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_ExitImplCopyWith<$Res> {
  factory _$$ApplicationMessage_ExitImplCopyWith(
          _$ApplicationMessage_ExitImpl value,
          $Res Function(_$ApplicationMessage_ExitImpl) then) =
      __$$ApplicationMessage_ExitImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ApplicationMessage_ExitImplCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res,
        _$ApplicationMessage_ExitImpl>
    implements _$$ApplicationMessage_ExitImplCopyWith<$Res> {
  __$$ApplicationMessage_ExitImplCopyWithImpl(
      _$ApplicationMessage_ExitImpl _value,
      $Res Function(_$ApplicationMessage_ExitImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$ApplicationMessage_ExitImpl extends ApplicationMessage_Exit {
  const _$ApplicationMessage_ExitImpl() : super._();

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ApplicationMessage_ExitImpl);
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
    required TResult Function(ApplicationMessage_DisplayMessage value)
        displayMessage,
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

abstract class ApplicationMessage_Exit extends ApplicationMessage {
  const factory ApplicationMessage_Exit() = _$ApplicationMessage_ExitImpl;
  const ApplicationMessage_Exit._() : super._();
}

/// @nodoc
mixin _$RawStringEnumMirrored {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0)
        listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value)
        listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RawStringEnumMirroredCopyWith<$Res> {
  factory $RawStringEnumMirroredCopyWith(RawStringEnumMirrored value,
          $Res Function(RawStringEnumMirrored) then) =
      _$RawStringEnumMirroredCopyWithImpl<$Res, RawStringEnumMirrored>;
}

/// @nodoc
class _$RawStringEnumMirroredCopyWithImpl<$Res,
        $Val extends RawStringEnumMirrored>
    implements $RawStringEnumMirroredCopyWith<$Res> {
  _$RawStringEnumMirroredCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_RawImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_RawImplCopyWith(
          _$RawStringEnumMirrored_RawImpl value,
          $Res Function(_$RawStringEnumMirrored_RawImpl) then) =
      __$$RawStringEnumMirrored_RawImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_RawImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res,
        _$RawStringEnumMirrored_RawImpl>
    implements _$$RawStringEnumMirrored_RawImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_RawImplCopyWithImpl(
      _$RawStringEnumMirrored_RawImpl _value,
      $Res Function(_$RawStringEnumMirrored_RawImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_RawImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as RawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_RawImpl extends RawStringEnumMirrored_Raw {
  const _$RawStringEnumMirrored_RawImpl(this.field0) : super._();

  @override
  final RawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.raw(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_RawImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_RawImplCopyWith<_$RawStringEnumMirrored_RawImpl>
      get copyWith => __$$RawStringEnumMirrored_RawImplCopyWithImpl<
          _$RawStringEnumMirrored_RawImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0)
        listOfNested,
  }) {
    return raw(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return raw?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value)
        listOfNested,
  }) {
    return raw(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return raw?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (raw != null) {
      return raw(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_Raw extends RawStringEnumMirrored {
  const factory RawStringEnumMirrored_Raw(final RawStringMirrored field0) =
      _$RawStringEnumMirrored_RawImpl;
  const RawStringEnumMirrored_Raw._() : super._();

  @override
  RawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_RawImplCopyWith<_$RawStringEnumMirrored_RawImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_NestedImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_NestedImplCopyWith(
          _$RawStringEnumMirrored_NestedImpl value,
          $Res Function(_$RawStringEnumMirrored_NestedImpl) then) =
      __$$RawStringEnumMirrored_NestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({NestedRawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_NestedImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res,
        _$RawStringEnumMirrored_NestedImpl>
    implements _$$RawStringEnumMirrored_NestedImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_NestedImplCopyWithImpl(
      _$RawStringEnumMirrored_NestedImpl _value,
      $Res Function(_$RawStringEnumMirrored_NestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_NestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as NestedRawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_NestedImpl extends RawStringEnumMirrored_Nested {
  const _$RawStringEnumMirrored_NestedImpl(this.field0) : super._();

  @override
  final NestedRawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.nested(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_NestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_NestedImplCopyWith<
          _$RawStringEnumMirrored_NestedImpl>
      get copyWith => __$$RawStringEnumMirrored_NestedImplCopyWithImpl<
          _$RawStringEnumMirrored_NestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0)
        listOfNested,
  }) {
    return nested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return nested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
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
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value)
        listOfNested,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_Nested extends RawStringEnumMirrored {
  const factory RawStringEnumMirrored_Nested(
          final NestedRawStringMirrored field0) =
      _$RawStringEnumMirrored_NestedImpl;
  const RawStringEnumMirrored_Nested._() : super._();

  @override
  NestedRawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_NestedImplCopyWith<
          _$RawStringEnumMirrored_NestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<$Res> {
  factory _$$RawStringEnumMirrored_ListOfNestedImplCopyWith(
          _$RawStringEnumMirrored_ListOfNestedImpl value,
          $Res Function(_$RawStringEnumMirrored_ListOfNestedImpl) then) =
      __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ListOfNestedRawStringMirrored field0});
}

/// @nodoc
class __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<$Res>
    extends _$RawStringEnumMirroredCopyWithImpl<$Res,
        _$RawStringEnumMirrored_ListOfNestedImpl>
    implements _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<$Res> {
  __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl(
      _$RawStringEnumMirrored_ListOfNestedImpl _value,
      $Res Function(_$RawStringEnumMirrored_ListOfNestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$RawStringEnumMirrored_ListOfNestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ListOfNestedRawStringMirrored,
    ));
  }
}

/// @nodoc

class _$RawStringEnumMirrored_ListOfNestedImpl
    extends RawStringEnumMirrored_ListOfNested {
  const _$RawStringEnumMirrored_ListOfNestedImpl(this.field0) : super._();

  @override
  final ListOfNestedRawStringMirrored field0;

  @override
  String toString() {
    return 'RawStringEnumMirrored.listOfNested(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RawStringEnumMirrored_ListOfNestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<
          _$RawStringEnumMirrored_ListOfNestedImpl>
      get copyWith => __$$RawStringEnumMirrored_ListOfNestedImplCopyWithImpl<
          _$RawStringEnumMirrored_ListOfNestedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(RawStringMirrored field0) raw,
    required TResult Function(NestedRawStringMirrored field0) nested,
    required TResult Function(ListOfNestedRawStringMirrored field0)
        listOfNested,
  }) {
    return listOfNested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(RawStringMirrored field0)? raw,
    TResult? Function(NestedRawStringMirrored field0)? nested,
    TResult? Function(ListOfNestedRawStringMirrored field0)? listOfNested,
  }) {
    return listOfNested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(RawStringMirrored field0)? raw,
    TResult Function(NestedRawStringMirrored field0)? nested,
    TResult Function(ListOfNestedRawStringMirrored field0)? listOfNested,
    required TResult orElse(),
  }) {
    if (listOfNested != null) {
      return listOfNested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RawStringEnumMirrored_Raw value) raw,
    required TResult Function(RawStringEnumMirrored_Nested value) nested,
    required TResult Function(RawStringEnumMirrored_ListOfNested value)
        listOfNested,
  }) {
    return listOfNested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RawStringEnumMirrored_Raw value)? raw,
    TResult? Function(RawStringEnumMirrored_Nested value)? nested,
    TResult? Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
  }) {
    return listOfNested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RawStringEnumMirrored_Raw value)? raw,
    TResult Function(RawStringEnumMirrored_Nested value)? nested,
    TResult Function(RawStringEnumMirrored_ListOfNested value)? listOfNested,
    required TResult orElse(),
  }) {
    if (listOfNested != null) {
      return listOfNested(this);
    }
    return orElse();
  }
}

abstract class RawStringEnumMirrored_ListOfNested
    extends RawStringEnumMirrored {
  const factory RawStringEnumMirrored_ListOfNested(
          final ListOfNestedRawStringMirrored field0) =
      _$RawStringEnumMirrored_ListOfNestedImpl;
  const RawStringEnumMirrored_ListOfNested._() : super._();

  @override
  ListOfNestedRawStringMirrored get field0;
  @JsonKey(ignore: true)
  _$$RawStringEnumMirrored_ListOfNestedImplCopyWith<
          _$RawStringEnumMirrored_ListOfNestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}
