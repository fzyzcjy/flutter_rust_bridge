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
  factory $ApplicationMessageCopyWith(
          ApplicationMessage value, $Res Function(ApplicationMessage) then) =
      _$ApplicationMessageCopyWithImpl<$Res>;
}

/// @nodoc
class _$ApplicationMessageCopyWithImpl<$Res>
    implements $ApplicationMessageCopyWith<$Res> {
  _$ApplicationMessageCopyWithImpl(this._value, this._then);

  final ApplicationMessage _value;
  // ignore: unused_field
  final $Res Function(ApplicationMessage) _then;
}

/// @nodoc
abstract class _$$DisplayMessageCopyWith<$Res> {
  factory _$$DisplayMessageCopyWith(
          _$DisplayMessage value, $Res Function(_$DisplayMessage) then) =
      __$$DisplayMessageCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$DisplayMessageCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$DisplayMessageCopyWith<$Res> {
  __$$DisplayMessageCopyWithImpl(
      _$DisplayMessage _value, $Res Function(_$DisplayMessage) _then)
      : super(_value, (v) => _then(v as _$DisplayMessage));

  @override
  _$DisplayMessage get _value => super._value as _$DisplayMessage;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$DisplayMessage(
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
            other is _$DisplayMessage &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$DisplayMessageCopyWith<_$DisplayMessage> get copyWith =>
      __$$DisplayMessageCopyWithImpl<_$DisplayMessage>(this, _$identity);

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
  const factory DisplayMessage(final String field0) = _$DisplayMessage;

  String get field0;
  @JsonKey(ignore: true)
  _$$DisplayMessageCopyWith<_$DisplayMessage> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RenderPixelCopyWith<$Res> {
  factory _$$RenderPixelCopyWith(
          _$RenderPixel value, $Res Function(_$RenderPixel) then) =
      __$$RenderPixelCopyWithImpl<$Res>;
  $Res call({int x, int y});
}

/// @nodoc
class __$$RenderPixelCopyWithImpl<$Res>
    extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$RenderPixelCopyWith<$Res> {
  __$$RenderPixelCopyWithImpl(
      _$RenderPixel _value, $Res Function(_$RenderPixel) _then)
      : super(_value, (v) => _then(v as _$RenderPixel));

  @override
  _$RenderPixel get _value => super._value as _$RenderPixel;

  @override
  $Res call({
    Object? x = freezed,
    Object? y = freezed,
  }) {
    return _then(_$RenderPixel(
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
            other is _$RenderPixel &&
            const DeepCollectionEquality().equals(other.x, x) &&
            const DeepCollectionEquality().equals(other.y, y));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(x),
      const DeepCollectionEquality().hash(y));

  @JsonKey(ignore: true)
  @override
  _$$RenderPixelCopyWith<_$RenderPixel> get copyWith =>
      __$$RenderPixelCopyWithImpl<_$RenderPixel>(this, _$identity);

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
  const factory RenderPixel({required final int x, required final int y}) =
      _$RenderPixel;

  int get x;
  int get y;
  @JsonKey(ignore: true)
  _$$RenderPixelCopyWith<_$RenderPixel> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ExitCopyWith<$Res> {
  factory _$$ExitCopyWith(_$Exit value, $Res Function(_$Exit) then) =
      __$$ExitCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ExitCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ExitCopyWith<$Res> {
  __$$ExitCopyWithImpl(_$Exit _value, $Res Function(_$Exit) _then)
      : super(_value, (v) => _then(v as _$Exit));

  @override
  _$Exit get _value => super._value as _$Exit;
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
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Exit);
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
mixin _$CustomError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) error0,
    required TResult Function(int field0) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error0 value) error0,
    required TResult Function(Error1 value) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomErrorCopyWith<$Res> {
  factory $CustomErrorCopyWith(
          CustomError value, $Res Function(CustomError) then) =
      _$CustomErrorCopyWithImpl<$Res>;
}

/// @nodoc
class _$CustomErrorCopyWithImpl<$Res> implements $CustomErrorCopyWith<$Res> {
  _$CustomErrorCopyWithImpl(this._value, this._then);

  final CustomError _value;
  // ignore: unused_field
  final $Res Function(CustomError) _then;
}

/// @nodoc
abstract class _$$Error0CopyWith<$Res> {
  factory _$$Error0CopyWith(_$Error0 value, $Res Function(_$Error0) then) =
      __$$Error0CopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$Error0CopyWithImpl<$Res> extends _$CustomErrorCopyWithImpl<$Res>
    implements _$$Error0CopyWith<$Res> {
  __$$Error0CopyWithImpl(_$Error0 _value, $Res Function(_$Error0) _then)
      : super(_value, (v) => _then(v as _$Error0));

  @override
  _$Error0 get _value => super._value as _$Error0;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Error0(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Error0 implements Error0 {
  const _$Error0(this.field0);

  @override
  final String field0;

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error0 &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Error0CopyWith<_$Error0> get copyWith =>
      __$$Error0CopyWithImpl<_$Error0>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) error0,
    required TResult Function(int field0) error1,
  }) {
    return error0(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
  }) {
    return error0?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error0 value) error0,
    required TResult Function(Error1 value) error1,
  }) {
    return error0(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
  }) {
    return error0?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(this);
    }
    return orElse();
  }
}

abstract class Error0 implements CustomError {
  const factory Error0(final String field0) = _$Error0;

  String get field0;
  @JsonKey(ignore: true)
  _$$Error0CopyWith<_$Error0> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Error1CopyWith<$Res> {
  factory _$$Error1CopyWith(_$Error1 value, $Res Function(_$Error1) then) =
      __$$Error1CopyWithImpl<$Res>;
  $Res call({int field0});
}

/// @nodoc
class __$$Error1CopyWithImpl<$Res> extends _$CustomErrorCopyWithImpl<$Res>
    implements _$$Error1CopyWith<$Res> {
  __$$Error1CopyWithImpl(_$Error1 _value, $Res Function(_$Error1) _then)
      : super(_value, (v) => _then(v as _$Error1));

  @override
  _$Error1 get _value => super._value as _$Error1;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Error1(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Error1 implements Error1 {
  const _$Error1(this.field0);

  @override
  final int field0;

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Error1 &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Error1CopyWith<_$Error1> get copyWith =>
      __$$Error1CopyWithImpl<_$Error1>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) error0,
    required TResult Function(int field0) error1,
  }) {
    return error1(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
  }) {
    return error1?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? error0,
    TResult Function(int field0)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Error0 value) error0,
    required TResult Function(Error1 value) error1,
  }) {
    return error1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
  }) {
    return error1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Error0 value)? error0,
    TResult Function(Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(this);
    }
    return orElse();
  }
}

abstract class Error1 implements CustomError {
  const factory Error1(final int field0) = _$Error1;

  int get field0;
  @JsonKey(ignore: true)
  _$$Error1CopyWith<_$Error1> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$CustomNestedError1 {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested1 value) customNested1,
    required TResult Function(ErrorNested value) errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError1CopyWith<$Res> {
  factory $CustomNestedError1CopyWith(
          CustomNestedError1 value, $Res Function(CustomNestedError1) then) =
      _$CustomNestedError1CopyWithImpl<$Res>;
}

/// @nodoc
class _$CustomNestedError1CopyWithImpl<$Res>
    implements $CustomNestedError1CopyWith<$Res> {
  _$CustomNestedError1CopyWithImpl(this._value, this._then);

  final CustomNestedError1 _value;
  // ignore: unused_field
  final $Res Function(CustomNestedError1) _then;
}

/// @nodoc
abstract class _$$CustomNested1CopyWith<$Res> {
  factory _$$CustomNested1CopyWith(
          _$CustomNested1 value, $Res Function(_$CustomNested1) then) =
      __$$CustomNested1CopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNested1CopyWithImpl<$Res>
    extends _$CustomNestedError1CopyWithImpl<$Res>
    implements _$$CustomNested1CopyWith<$Res> {
  __$$CustomNested1CopyWithImpl(
      _$CustomNested1 _value, $Res Function(_$CustomNested1) _then)
      : super(_value, (v) => _then(v as _$CustomNested1));

  @override
  _$CustomNested1 get _value => super._value as _$CustomNested1;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$CustomNested1(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomNested1 implements CustomNested1 {
  const _$CustomNested1(this.field0);

  @override
  final String field0;

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNested1 &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$CustomNested1CopyWith<_$CustomNested1> get copyWith =>
      __$$CustomNested1CopyWithImpl<_$CustomNested1>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) {
    return customNested1(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
  }) {
    return customNested1?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) {
    if (customNested1 != null) {
      return customNested1(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested1 value) customNested1,
    required TResult Function(ErrorNested value) errorNested,
  }) {
    return customNested1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
  }) {
    return customNested1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
    required TResult orElse(),
  }) {
    if (customNested1 != null) {
      return customNested1(this);
    }
    return orElse();
  }
}

abstract class CustomNested1 implements CustomNestedError1 {
  const factory CustomNested1(final String field0) = _$CustomNested1;

  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNested1CopyWith<_$CustomNested1> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ErrorNestedCopyWith<$Res> {
  factory _$$ErrorNestedCopyWith(
          _$ErrorNested value, $Res Function(_$ErrorNested) then) =
      __$$ErrorNestedCopyWithImpl<$Res>;
  $Res call({CustomNestedError2 field0});

  $CustomNestedError2CopyWith<$Res> get field0;
}

/// @nodoc
class __$$ErrorNestedCopyWithImpl<$Res>
    extends _$CustomNestedError1CopyWithImpl<$Res>
    implements _$$ErrorNestedCopyWith<$Res> {
  __$$ErrorNestedCopyWithImpl(
      _$ErrorNested _value, $Res Function(_$ErrorNested) _then)
      : super(_value, (v) => _then(v as _$ErrorNested));

  @override
  _$ErrorNested get _value => super._value as _$ErrorNested;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$ErrorNested(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2,
    ));
  }

  @override
  $CustomNestedError2CopyWith<$Res> get field0 {
    return $CustomNestedError2CopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$ErrorNested implements ErrorNested {
  const _$ErrorNested(this.field0);

  @override
  final CustomNestedError2 field0;

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ErrorNested &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$ErrorNestedCopyWith<_$ErrorNested> get copyWith =>
      __$$ErrorNestedCopyWithImpl<_$ErrorNested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2 field0) errorNested,
  }) {
    return errorNested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
  }) {
    return errorNested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2 field0)? errorNested,
    required TResult orElse(),
  }) {
    if (errorNested != null) {
      return errorNested(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested1 value) customNested1,
    required TResult Function(ErrorNested value) errorNested,
  }) {
    return errorNested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
  }) {
    return errorNested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested1 value)? customNested1,
    TResult Function(ErrorNested value)? errorNested,
    required TResult orElse(),
  }) {
    if (errorNested != null) {
      return errorNested(this);
    }
    return orElse();
  }
}

abstract class ErrorNested implements CustomNestedError1 {
  const factory ErrorNested(final CustomNestedError2 field0) = _$ErrorNested;

  CustomNestedError2 get field0;
  @JsonKey(ignore: true)
  _$$ErrorNestedCopyWith<_$ErrorNested> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$CustomNestedError2 {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested2 value) customNested2,
    required TResult Function(CustomNested2Number value) customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError2CopyWith<$Res> {
  factory $CustomNestedError2CopyWith(
          CustomNestedError2 value, $Res Function(CustomNestedError2) then) =
      _$CustomNestedError2CopyWithImpl<$Res>;
}

/// @nodoc
class _$CustomNestedError2CopyWithImpl<$Res>
    implements $CustomNestedError2CopyWith<$Res> {
  _$CustomNestedError2CopyWithImpl(this._value, this._then);

  final CustomNestedError2 _value;
  // ignore: unused_field
  final $Res Function(CustomNestedError2) _then;
}

/// @nodoc
abstract class _$$CustomNested2CopyWith<$Res> {
  factory _$$CustomNested2CopyWith(
          _$CustomNested2 value, $Res Function(_$CustomNested2) then) =
      __$$CustomNested2CopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNested2CopyWithImpl<$Res>
    extends _$CustomNestedError2CopyWithImpl<$Res>
    implements _$$CustomNested2CopyWith<$Res> {
  __$$CustomNested2CopyWithImpl(
      _$CustomNested2 _value, $Res Function(_$CustomNested2) _then)
      : super(_value, (v) => _then(v as _$CustomNested2));

  @override
  _$CustomNested2 get _value => super._value as _$CustomNested2;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$CustomNested2(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$CustomNested2 implements CustomNested2 {
  const _$CustomNested2(this.field0);

  @override
  final String field0;

  @override
  String toString() {
    return 'CustomNestedError2.customNested2(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNested2 &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$CustomNested2CopyWith<_$CustomNested2> get copyWith =>
      __$$CustomNested2CopyWithImpl<_$CustomNested2>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) {
    return customNested2(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
  }) {
    return customNested2?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2 != null) {
      return customNested2(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested2 value) customNested2,
    required TResult Function(CustomNested2Number value) customNested2Number,
  }) {
    return customNested2(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
  }) {
    return customNested2?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2 != null) {
      return customNested2(this);
    }
    return orElse();
  }
}

abstract class CustomNested2 implements CustomNestedError2 {
  const factory CustomNested2(final String field0) = _$CustomNested2;

  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNested2CopyWith<_$CustomNested2> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNested2NumberCopyWith<$Res> {
  factory _$$CustomNested2NumberCopyWith(_$CustomNested2Number value,
          $Res Function(_$CustomNested2Number) then) =
      __$$CustomNested2NumberCopyWithImpl<$Res>;
  $Res call({int field0});
}

/// @nodoc
class __$$CustomNested2NumberCopyWithImpl<$Res>
    extends _$CustomNestedError2CopyWithImpl<$Res>
    implements _$$CustomNested2NumberCopyWith<$Res> {
  __$$CustomNested2NumberCopyWithImpl(
      _$CustomNested2Number _value, $Res Function(_$CustomNested2Number) _then)
      : super(_value, (v) => _then(v as _$CustomNested2Number));

  @override
  _$CustomNested2Number get _value => super._value as _$CustomNested2Number;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$CustomNested2Number(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$CustomNested2Number implements CustomNested2Number {
  const _$CustomNested2Number(this.field0);

  @override
  final int field0;

  @override
  String toString() {
    return 'CustomNestedError2.customNested2Number(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNested2Number &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$CustomNested2NumberCopyWith<_$CustomNested2Number> get copyWith =>
      __$$CustomNested2NumberCopyWithImpl<_$CustomNested2Number>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) {
    return customNested2Number(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
  }) {
    return customNested2Number?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested2,
    TResult Function(int field0)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2Number != null) {
      return customNested2Number(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNested2 value) customNested2,
    required TResult Function(CustomNested2Number value) customNested2Number,
  }) {
    return customNested2Number(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
  }) {
    return customNested2Number?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNested2 value)? customNested2,
    TResult Function(CustomNested2Number value)? customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2Number != null) {
      return customNested2Number(this);
    }
    return orElse();
  }
}

abstract class CustomNested2Number implements CustomNestedError2 {
  const factory CustomNested2Number(final int field0) = _$CustomNested2Number;

  int get field0;
  @JsonKey(ignore: true)
  _$$CustomNested2NumberCopyWith<_$CustomNested2Number> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KitchenSink {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  factory $KitchenSinkCopyWith(
          KitchenSink value, $Res Function(KitchenSink) then) =
      _$KitchenSinkCopyWithImpl<$Res>;
}

/// @nodoc
class _$KitchenSinkCopyWithImpl<$Res> implements $KitchenSinkCopyWith<$Res> {
  _$KitchenSinkCopyWithImpl(this._value, this._then);

  final KitchenSink _value;
  // ignore: unused_field
  final $Res Function(KitchenSink) _then;
}

/// @nodoc
abstract class _$$EmptyCopyWith<$Res> {
  factory _$$EmptyCopyWith(_$Empty value, $Res Function(_$Empty) then) =
      __$$EmptyCopyWithImpl<$Res>;
}

/// @nodoc
class __$$EmptyCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$EmptyCopyWith<$Res> {
  __$$EmptyCopyWithImpl(_$Empty _value, $Res Function(_$Empty) _then)
      : super(_value, (v) => _then(v as _$Empty));

  @override
  _$Empty get _value => super._value as _$Empty;
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
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
abstract class _$$PrimitivesCopyWith<$Res> {
  factory _$$PrimitivesCopyWith(
          _$Primitives value, $Res Function(_$Primitives) then) =
      __$$PrimitivesCopyWithImpl<$Res>;
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$PrimitivesCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$PrimitivesCopyWith<$Res> {
  __$$PrimitivesCopyWithImpl(
      _$Primitives _value, $Res Function(_$Primitives) _then)
      : super(_value, (v) => _then(v as _$Primitives));

  @override
  _$Primitives get _value => super._value as _$Primitives;

  @override
  $Res call({
    Object? int32 = freezed,
    Object? float64 = freezed,
    Object? boolean = freezed,
  }) {
    return _then(_$Primitives(
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
  const _$Primitives(
      {required this.int32, required this.float64, required this.boolean});

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
            other is _$Primitives &&
            const DeepCollectionEquality().equals(other.int32, int32) &&
            const DeepCollectionEquality().equals(other.float64, float64) &&
            const DeepCollectionEquality().equals(other.boolean, boolean));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(int32),
      const DeepCollectionEquality().hash(float64),
      const DeepCollectionEquality().hash(boolean));

  @JsonKey(ignore: true)
  @override
  _$$PrimitivesCopyWith<_$Primitives> get copyWith =>
      __$$PrimitivesCopyWithImpl<_$Primitives>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  const factory Primitives(
      {required final int int32,
      required final double float64,
      required final bool boolean}) = _$Primitives;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$PrimitivesCopyWith<_$Primitives> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$NestedCopyWith<$Res> {
  factory _$$NestedCopyWith(_$Nested value, $Res Function(_$Nested) then) =
      __$$NestedCopyWithImpl<$Res>;
  $Res call({KitchenSink field0, int field1});

  $KitchenSinkCopyWith<$Res> get field0;
}

/// @nodoc
class __$$NestedCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$NestedCopyWith<$Res> {
  __$$NestedCopyWithImpl(_$Nested _value, $Res Function(_$Nested) _then)
      : super(_value, (v) => _then(v as _$Nested));

  @override
  _$Nested get _value => super._value as _$Nested;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$Nested(
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

class _$Nested implements Nested {
  const _$Nested(this.field0, this.field1);

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
            other is _$Nested &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(field0),
      const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$NestedCopyWith<_$Nested> get copyWith =>
      __$$NestedCopyWithImpl<_$Nested>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  const factory Nested(final KitchenSink field0, final int field1) = _$Nested;

  KitchenSink get field0;
  int get field1;
  @JsonKey(ignore: true)
  _$$NestedCopyWith<_$Nested> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$OptionalCopyWith<$Res> {
  factory _$$OptionalCopyWith(
          _$Optional value, $Res Function(_$Optional) then) =
      __$$OptionalCopyWithImpl<$Res>;
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$OptionalCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$OptionalCopyWith<$Res> {
  __$$OptionalCopyWithImpl(_$Optional _value, $Res Function(_$Optional) _then)
      : super(_value, (v) => _then(v as _$Optional));

  @override
  _$Optional get _value => super._value as _$Optional;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$Optional(
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
            other is _$Optional &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(field0),
      const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$OptionalCopyWith<_$Optional> get copyWith =>
      __$$OptionalCopyWithImpl<_$Optional>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  const factory Optional([final int? field0, final int? field1]) = _$Optional;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$OptionalCopyWith<_$Optional> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BufferCopyWith<$Res> {
  factory _$$BufferCopyWith(_$Buffer value, $Res Function(_$Buffer) then) =
      __$$BufferCopyWithImpl<$Res>;
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$BufferCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$BufferCopyWith<$Res> {
  __$$BufferCopyWithImpl(_$Buffer _value, $Res Function(_$Buffer) _then)
      : super(_value, (v) => _then(v as _$Buffer));

  @override
  _$Buffer get _value => super._value as _$Buffer;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Buffer(
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
            other is _$Buffer &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$BufferCopyWith<_$Buffer> get copyWith =>
      __$$BufferCopyWithImpl<_$Buffer>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  const factory Buffer(final Uint8List field0) = _$Buffer;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$BufferCopyWith<_$Buffer> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$EnumsCopyWith<$Res> {
  factory _$$EnumsCopyWith(_$Enums value, $Res Function(_$Enums) then) =
      __$$EnumsCopyWithImpl<$Res>;
  $Res call({Weekdays field0});
}

/// @nodoc
class __$$EnumsCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$EnumsCopyWith<$Res> {
  __$$EnumsCopyWithImpl(_$Enums _value, $Res Function(_$Enums) _then)
      : super(_value, (v) => _then(v as _$Enums));

  @override
  _$Enums get _value => super._value as _$Enums;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Enums(
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
            other is _$Enums &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$EnumsCopyWith<_$Enums> get copyWith =>
      __$$EnumsCopyWithImpl<_$Enums>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() empty,
    required TResult Function(int int32, double float64, bool boolean)
        primitives,
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
  const factory Enums(final Weekdays field0) = _$Enums;

  Weekdays get field0;
  @JsonKey(ignore: true)
  _$$EnumsCopyWith<_$Enums> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$UserId {
  int get value => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $UserIdCopyWith<UserId> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $UserIdCopyWith<$Res> {
  factory $UserIdCopyWith(UserId value, $Res Function(UserId) then) =
      _$UserIdCopyWithImpl<$Res>;
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
  factory _$$_UserIdCopyWith(_$_UserId value, $Res Function(_$_UserId) then) =
      __$$_UserIdCopyWithImpl<$Res>;
  @override
  $Res call({int value});
}

/// @nodoc
class __$$_UserIdCopyWithImpl<$Res> extends _$UserIdCopyWithImpl<$Res>
    implements _$$_UserIdCopyWith<$Res> {
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
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(value));

  @JsonKey(ignore: true)
  @override
  _$$_UserIdCopyWith<_$_UserId> get copyWith =>
      __$$_UserIdCopyWithImpl<_$_UserId>(this, _$identity);
}

abstract class _UserId implements UserId {
  const factory _UserId({required final int value}) = _$_UserId;

  @override
  int get value;
  @override
  @JsonKey(ignore: true)
  _$$_UserIdCopyWith<_$_UserId> get copyWith =>
      throw _privateConstructorUsedError;
}
