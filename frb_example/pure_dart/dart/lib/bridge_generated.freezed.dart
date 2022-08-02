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
    required TResult Function(ApplicationMessage_DisplayMessage_Variant value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel_Variant value) renderPixel,
    required TResult Function(ApplicationMessage_Exit_Variant value) exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
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
abstract class _$$ApplicationMessage_DisplayMessage_VariantCopyWith<$Res> {
  factory _$$ApplicationMessage_DisplayMessage_VariantCopyWith(_$ApplicationMessage_DisplayMessage_Variant value,
          $Res Function(_$ApplicationMessage_DisplayMessage_Variant) then) =
      __$$ApplicationMessage_DisplayMessage_VariantCopyWithImpl<$Res>;
  $Res call({String field0});
}

/// @nodoc
class __$$ApplicationMessage_DisplayMessage_VariantCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_DisplayMessage_VariantCopyWith<$Res> {
  __$$ApplicationMessage_DisplayMessage_VariantCopyWithImpl(_$ApplicationMessage_DisplayMessage_Variant _value,
      $Res Function(_$ApplicationMessage_DisplayMessage_Variant) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_DisplayMessage_Variant));

  @override
  _$ApplicationMessage_DisplayMessage_Variant get _value => super._value as _$ApplicationMessage_DisplayMessage_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$ApplicationMessage_DisplayMessage_Variant(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ApplicationMessage_DisplayMessage_Variant implements ApplicationMessage_DisplayMessage_Variant {
  const _$ApplicationMessage_DisplayMessage_Variant(this.field0);

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
            other is _$ApplicationMessage_DisplayMessage_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$ApplicationMessage_DisplayMessage_VariantCopyWith<_$ApplicationMessage_DisplayMessage_Variant> get copyWith =>
      __$$ApplicationMessage_DisplayMessage_VariantCopyWithImpl<_$ApplicationMessage_DisplayMessage_Variant>(
          this, _$identity);

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
    required TResult Function(ApplicationMessage_DisplayMessage_Variant value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel_Variant value) renderPixel,
    required TResult Function(ApplicationMessage_Exit_Variant value) exit,
  }) {
    return displayMessage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
  }) {
    return displayMessage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
    required TResult orElse(),
  }) {
    if (displayMessage != null) {
      return displayMessage(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_DisplayMessage_Variant implements ApplicationMessage {
  const factory ApplicationMessage_DisplayMessage_Variant(final String field0) =
      _$ApplicationMessage_DisplayMessage_Variant;

  String get field0;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_DisplayMessage_VariantCopyWith<_$ApplicationMessage_DisplayMessage_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_RenderPixel_VariantCopyWith<$Res> {
  factory _$$ApplicationMessage_RenderPixel_VariantCopyWith(_$ApplicationMessage_RenderPixel_Variant value,
          $Res Function(_$ApplicationMessage_RenderPixel_Variant) then) =
      __$$ApplicationMessage_RenderPixel_VariantCopyWithImpl<$Res>;
  $Res call({int x, int y});
}

/// @nodoc
class __$$ApplicationMessage_RenderPixel_VariantCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_RenderPixel_VariantCopyWith<$Res> {
  __$$ApplicationMessage_RenderPixel_VariantCopyWithImpl(
      _$ApplicationMessage_RenderPixel_Variant _value, $Res Function(_$ApplicationMessage_RenderPixel_Variant) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_RenderPixel_Variant));

  @override
  _$ApplicationMessage_RenderPixel_Variant get _value => super._value as _$ApplicationMessage_RenderPixel_Variant;

  @override
  $Res call({
    Object? x = freezed,
    Object? y = freezed,
  }) {
    return _then(_$ApplicationMessage_RenderPixel_Variant(
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

class _$ApplicationMessage_RenderPixel_Variant implements ApplicationMessage_RenderPixel_Variant {
  const _$ApplicationMessage_RenderPixel_Variant({required this.x, required this.y});

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
            other is _$ApplicationMessage_RenderPixel_Variant &&
            const DeepCollectionEquality().equals(other.x, x) &&
            const DeepCollectionEquality().equals(other.y, y));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(x), const DeepCollectionEquality().hash(y));

  @JsonKey(ignore: true)
  @override
  _$$ApplicationMessage_RenderPixel_VariantCopyWith<_$ApplicationMessage_RenderPixel_Variant> get copyWith =>
      __$$ApplicationMessage_RenderPixel_VariantCopyWithImpl<_$ApplicationMessage_RenderPixel_Variant>(
          this, _$identity);

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
    required TResult Function(ApplicationMessage_DisplayMessage_Variant value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel_Variant value) renderPixel,
    required TResult Function(ApplicationMessage_Exit_Variant value) exit,
  }) {
    return renderPixel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
  }) {
    return renderPixel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
    required TResult orElse(),
  }) {
    if (renderPixel != null) {
      return renderPixel(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_RenderPixel_Variant implements ApplicationMessage {
  const factory ApplicationMessage_RenderPixel_Variant({required final int x, required final int y}) =
      _$ApplicationMessage_RenderPixel_Variant;

  int get x;
  int get y;
  @JsonKey(ignore: true)
  _$$ApplicationMessage_RenderPixel_VariantCopyWith<_$ApplicationMessage_RenderPixel_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ApplicationMessage_Exit_VariantCopyWith<$Res> {
  factory _$$ApplicationMessage_Exit_VariantCopyWith(
          _$ApplicationMessage_Exit_Variant value, $Res Function(_$ApplicationMessage_Exit_Variant) then) =
      __$$ApplicationMessage_Exit_VariantCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ApplicationMessage_Exit_VariantCopyWithImpl<$Res> extends _$ApplicationMessageCopyWithImpl<$Res>
    implements _$$ApplicationMessage_Exit_VariantCopyWith<$Res> {
  __$$ApplicationMessage_Exit_VariantCopyWithImpl(
      _$ApplicationMessage_Exit_Variant _value, $Res Function(_$ApplicationMessage_Exit_Variant) _then)
      : super(_value, (v) => _then(v as _$ApplicationMessage_Exit_Variant));

  @override
  _$ApplicationMessage_Exit_Variant get _value => super._value as _$ApplicationMessage_Exit_Variant;
}

/// @nodoc

class _$ApplicationMessage_Exit_Variant implements ApplicationMessage_Exit_Variant {
  const _$ApplicationMessage_Exit_Variant();

  @override
  String toString() {
    return 'ApplicationMessage.exit()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$ApplicationMessage_Exit_Variant);
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
    required TResult Function(ApplicationMessage_DisplayMessage_Variant value) displayMessage,
    required TResult Function(ApplicationMessage_RenderPixel_Variant value) renderPixel,
    required TResult Function(ApplicationMessage_Exit_Variant value) exit,
  }) {
    return exit(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
  }) {
    return exit?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ApplicationMessage_DisplayMessage_Variant value)? displayMessage,
    TResult Function(ApplicationMessage_RenderPixel_Variant value)? renderPixel,
    TResult Function(ApplicationMessage_Exit_Variant value)? exit,
    required TResult orElse(),
  }) {
    if (exit != null) {
      return exit(this);
    }
    return orElse();
  }
}

abstract class ApplicationMessage_Exit_Variant implements ApplicationMessage {
  const factory ApplicationMessage_Exit_Variant() = _$ApplicationMessage_Exit_Variant;
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
    required TResult Function(Distance_Unknown_Variant value) unknown,
    required TResult Function(Distance_Map_Variant value) map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
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
abstract class _$$Distance_Unknown_VariantCopyWith<$Res> {
  factory _$$Distance_Unknown_VariantCopyWith(
          _$Distance_Unknown_Variant value, $Res Function(_$Distance_Unknown_Variant) then) =
      __$$Distance_Unknown_VariantCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Distance_Unknown_VariantCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res>
    implements _$$Distance_Unknown_VariantCopyWith<$Res> {
  __$$Distance_Unknown_VariantCopyWithImpl(
      _$Distance_Unknown_Variant _value, $Res Function(_$Distance_Unknown_Variant) _then)
      : super(_value, (v) => _then(v as _$Distance_Unknown_Variant));

  @override
  _$Distance_Unknown_Variant get _value => super._value as _$Distance_Unknown_Variant;
}

/// @nodoc

class _$Distance_Unknown_Variant implements Distance_Unknown_Variant {
  const _$Distance_Unknown_Variant();

  @override
  String toString() {
    return 'Distance.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Distance_Unknown_Variant);
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
    required TResult Function(Distance_Unknown_Variant value) unknown,
    required TResult Function(Distance_Map_Variant value) map,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Distance_Unknown_Variant implements Distance {
  const factory Distance_Unknown_Variant() = _$Distance_Unknown_Variant;
}

/// @nodoc
abstract class _$$Distance_Map_VariantCopyWith<$Res> {
  factory _$$Distance_Map_VariantCopyWith(_$Distance_Map_Variant value, $Res Function(_$Distance_Map_Variant) then) =
      __$$Distance_Map_VariantCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$Distance_Map_VariantCopyWithImpl<$Res> extends _$DistanceCopyWithImpl<$Res>
    implements _$$Distance_Map_VariantCopyWith<$Res> {
  __$$Distance_Map_VariantCopyWithImpl(_$Distance_Map_Variant _value, $Res Function(_$Distance_Map_Variant) _then)
      : super(_value, (v) => _then(v as _$Distance_Map_Variant));

  @override
  _$Distance_Map_Variant get _value => super._value as _$Distance_Map_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Distance_Map_Variant(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Distance_Map_Variant implements Distance_Map_Variant {
  const _$Distance_Map_Variant(this.field0);

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
            other is _$Distance_Map_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Distance_Map_VariantCopyWith<_$Distance_Map_Variant> get copyWith =>
      __$$Distance_Map_VariantCopyWithImpl<_$Distance_Map_Variant>(this, _$identity);

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
    required TResult Function(Distance_Unknown_Variant value) unknown,
    required TResult Function(Distance_Map_Variant value) map,
  }) {
    return map(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
  }) {
    return map?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Distance_Unknown_Variant value)? unknown,
    TResult Function(Distance_Map_Variant value)? map,
    required TResult orElse(),
  }) {
    if (map != null) {
      return map(this);
    }
    return orElse();
  }
}

abstract class Distance_Map_Variant implements Distance {
  const factory Distance_Map_Variant(final double field0) = _$Distance_Map_Variant;

  double get field0;
  @JsonKey(ignore: true)
  _$$Distance_Map_VariantCopyWith<_$Distance_Map_Variant> get copyWith => throw _privateConstructorUsedError;
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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
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
abstract class _$$KitchenSink_Empty_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Empty_VariantCopyWith(
          _$KitchenSink_Empty_Variant value, $Res Function(_$KitchenSink_Empty_Variant) then) =
      __$$KitchenSink_Empty_VariantCopyWithImpl<$Res>;
}

/// @nodoc
class __$$KitchenSink_Empty_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Empty_VariantCopyWith<$Res> {
  __$$KitchenSink_Empty_VariantCopyWithImpl(
      _$KitchenSink_Empty_Variant _value, $Res Function(_$KitchenSink_Empty_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Empty_Variant));

  @override
  _$KitchenSink_Empty_Variant get _value => super._value as _$KitchenSink_Empty_Variant;
}

/// @nodoc

class _$KitchenSink_Empty_Variant implements KitchenSink_Empty_Variant {
  const _$KitchenSink_Empty_Variant();

  @override
  String toString() {
    return 'KitchenSink.empty()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$KitchenSink_Empty_Variant);
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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return empty(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return empty?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (empty != null) {
      return empty(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Empty_Variant implements KitchenSink {
  const factory KitchenSink_Empty_Variant() = _$KitchenSink_Empty_Variant;
}

/// @nodoc
abstract class _$$KitchenSink_Primitives_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Primitives_VariantCopyWith(
          _$KitchenSink_Primitives_Variant value, $Res Function(_$KitchenSink_Primitives_Variant) then) =
      __$$KitchenSink_Primitives_VariantCopyWithImpl<$Res>;
  $Res call({int int32, double float64, bool boolean});
}

/// @nodoc
class __$$KitchenSink_Primitives_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Primitives_VariantCopyWith<$Res> {
  __$$KitchenSink_Primitives_VariantCopyWithImpl(
      _$KitchenSink_Primitives_Variant _value, $Res Function(_$KitchenSink_Primitives_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Primitives_Variant));

  @override
  _$KitchenSink_Primitives_Variant get _value => super._value as _$KitchenSink_Primitives_Variant;

  @override
  $Res call({
    Object? int32 = freezed,
    Object? float64 = freezed,
    Object? boolean = freezed,
  }) {
    return _then(_$KitchenSink_Primitives_Variant(
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

class _$KitchenSink_Primitives_Variant implements KitchenSink_Primitives_Variant {
  const _$KitchenSink_Primitives_Variant({required this.int32, required this.float64, required this.boolean});

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
            other is _$KitchenSink_Primitives_Variant &&
            const DeepCollectionEquality().equals(other.int32, int32) &&
            const DeepCollectionEquality().equals(other.float64, float64) &&
            const DeepCollectionEquality().equals(other.boolean, boolean));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(int32),
      const DeepCollectionEquality().hash(float64), const DeepCollectionEquality().hash(boolean));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_Primitives_VariantCopyWith<_$KitchenSink_Primitives_Variant> get copyWith =>
      __$$KitchenSink_Primitives_VariantCopyWithImpl<_$KitchenSink_Primitives_Variant>(this, _$identity);

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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return primitives(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return primitives?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (primitives != null) {
      return primitives(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Primitives_Variant implements KitchenSink {
  const factory KitchenSink_Primitives_Variant(
      {required final int int32,
      required final double float64,
      required final bool boolean}) = _$KitchenSink_Primitives_Variant;

  /// Dart field comment
  int get int32;
  double get float64;
  bool get boolean;
  @JsonKey(ignore: true)
  _$$KitchenSink_Primitives_VariantCopyWith<_$KitchenSink_Primitives_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_Nested_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Nested_VariantCopyWith(
          _$KitchenSink_Nested_Variant value, $Res Function(_$KitchenSink_Nested_Variant) then) =
      __$$KitchenSink_Nested_VariantCopyWithImpl<$Res>;
  $Res call({KitchenSink field0, int field1});

  $KitchenSinkCopyWith<$Res> get field0;
}

/// @nodoc
class __$$KitchenSink_Nested_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Nested_VariantCopyWith<$Res> {
  __$$KitchenSink_Nested_VariantCopyWithImpl(
      _$KitchenSink_Nested_Variant _value, $Res Function(_$KitchenSink_Nested_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Nested_Variant));

  @override
  _$KitchenSink_Nested_Variant get _value => super._value as _$KitchenSink_Nested_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_Nested_Variant(
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

class _$KitchenSink_Nested_Variant implements KitchenSink_Nested_Variant {
  const _$KitchenSink_Nested_Variant(this.field0, this.field1);

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
            other is _$KitchenSink_Nested_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_Nested_VariantCopyWith<_$KitchenSink_Nested_Variant> get copyWith =>
      __$$KitchenSink_Nested_VariantCopyWithImpl<_$KitchenSink_Nested_Variant>(this, _$identity);

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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return nested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return nested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (nested != null) {
      return nested(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Nested_Variant implements KitchenSink {
  const factory KitchenSink_Nested_Variant(final KitchenSink field0, final int field1) = _$KitchenSink_Nested_Variant;

  KitchenSink get field0;
  int get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_Nested_VariantCopyWith<_$KitchenSink_Nested_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_Optional_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Optional_VariantCopyWith(
          _$KitchenSink_Optional_Variant value, $Res Function(_$KitchenSink_Optional_Variant) then) =
      __$$KitchenSink_Optional_VariantCopyWithImpl<$Res>;
  $Res call({int? field0, int? field1});
}

/// @nodoc
class __$$KitchenSink_Optional_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Optional_VariantCopyWith<$Res> {
  __$$KitchenSink_Optional_VariantCopyWithImpl(
      _$KitchenSink_Optional_Variant _value, $Res Function(_$KitchenSink_Optional_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Optional_Variant));

  @override
  _$KitchenSink_Optional_Variant get _value => super._value as _$KitchenSink_Optional_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
  }) {
    return _then(_$KitchenSink_Optional_Variant(
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

class _$KitchenSink_Optional_Variant implements KitchenSink_Optional_Variant {
  const _$KitchenSink_Optional_Variant([this.field0, this.field1]);

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
            other is _$KitchenSink_Optional_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(field0), const DeepCollectionEquality().hash(field1));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_Optional_VariantCopyWith<_$KitchenSink_Optional_Variant> get copyWith =>
      __$$KitchenSink_Optional_VariantCopyWithImpl<_$KitchenSink_Optional_Variant>(this, _$identity);

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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return optional(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return optional?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (optional != null) {
      return optional(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Optional_Variant implements KitchenSink {
  const factory KitchenSink_Optional_Variant([final int? field0, final int? field1]) = _$KitchenSink_Optional_Variant;

  /// Comment on anonymous field
  int? get field0;
  int? get field1;
  @JsonKey(ignore: true)
  _$$KitchenSink_Optional_VariantCopyWith<_$KitchenSink_Optional_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_Buffer_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Buffer_VariantCopyWith(
          _$KitchenSink_Buffer_Variant value, $Res Function(_$KitchenSink_Buffer_Variant) then) =
      __$$KitchenSink_Buffer_VariantCopyWithImpl<$Res>;
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$KitchenSink_Buffer_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Buffer_VariantCopyWith<$Res> {
  __$$KitchenSink_Buffer_VariantCopyWithImpl(
      _$KitchenSink_Buffer_Variant _value, $Res Function(_$KitchenSink_Buffer_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Buffer_Variant));

  @override
  _$KitchenSink_Buffer_Variant get _value => super._value as _$KitchenSink_Buffer_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KitchenSink_Buffer_Variant(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Buffer_Variant implements KitchenSink_Buffer_Variant {
  const _$KitchenSink_Buffer_Variant(this.field0);

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
            other is _$KitchenSink_Buffer_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_Buffer_VariantCopyWith<_$KitchenSink_Buffer_Variant> get copyWith =>
      __$$KitchenSink_Buffer_VariantCopyWithImpl<_$KitchenSink_Buffer_Variant>(this, _$identity);

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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Buffer_Variant implements KitchenSink {
  const factory KitchenSink_Buffer_Variant(final Uint8List field0) = _$KitchenSink_Buffer_Variant;

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_Buffer_VariantCopyWith<_$KitchenSink_Buffer_Variant> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KitchenSink_Enums_VariantCopyWith<$Res> {
  factory _$$KitchenSink_Enums_VariantCopyWith(
          _$KitchenSink_Enums_Variant value, $Res Function(_$KitchenSink_Enums_Variant) then) =
      __$$KitchenSink_Enums_VariantCopyWithImpl<$Res>;
  $Res call({Weekdays field0});
}

/// @nodoc
class __$$KitchenSink_Enums_VariantCopyWithImpl<$Res> extends _$KitchenSinkCopyWithImpl<$Res>
    implements _$$KitchenSink_Enums_VariantCopyWith<$Res> {
  __$$KitchenSink_Enums_VariantCopyWithImpl(
      _$KitchenSink_Enums_Variant _value, $Res Function(_$KitchenSink_Enums_Variant) _then)
      : super(_value, (v) => _then(v as _$KitchenSink_Enums_Variant));

  @override
  _$KitchenSink_Enums_Variant get _value => super._value as _$KitchenSink_Enums_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KitchenSink_Enums_Variant(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Weekdays,
    ));
  }
}

/// @nodoc

class _$KitchenSink_Enums_Variant implements KitchenSink_Enums_Variant {
  const _$KitchenSink_Enums_Variant(this.field0);

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
            other is _$KitchenSink_Enums_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KitchenSink_Enums_VariantCopyWith<_$KitchenSink_Enums_Variant> get copyWith =>
      __$$KitchenSink_Enums_VariantCopyWithImpl<_$KitchenSink_Enums_Variant>(this, _$identity);

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
    required TResult Function(KitchenSink_Empty_Variant value) empty,
    required TResult Function(KitchenSink_Primitives_Variant value) primitives,
    required TResult Function(KitchenSink_Nested_Variant value) nested,
    required TResult Function(KitchenSink_Optional_Variant value) optional,
    required TResult Function(KitchenSink_Buffer_Variant value) buffer,
    required TResult Function(KitchenSink_Enums_Variant value) enums,
  }) {
    return enums(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
  }) {
    return enums?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KitchenSink_Empty_Variant value)? empty,
    TResult Function(KitchenSink_Primitives_Variant value)? primitives,
    TResult Function(KitchenSink_Nested_Variant value)? nested,
    TResult Function(KitchenSink_Optional_Variant value)? optional,
    TResult Function(KitchenSink_Buffer_Variant value)? buffer,
    TResult Function(KitchenSink_Enums_Variant value)? enums,
    required TResult orElse(),
  }) {
    if (enums != null) {
      return enums(this);
    }
    return orElse();
  }
}

abstract class KitchenSink_Enums_Variant implements KitchenSink {
  const factory KitchenSink_Enums_Variant(final Weekdays field0) = _$KitchenSink_Enums_Variant;

  Weekdays get field0;
  @JsonKey(ignore: true)
  _$$KitchenSink_Enums_VariantCopyWith<_$KitchenSink_Enums_Variant> get copyWith => throw _privateConstructorUsedError;
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
    required TResult Function(Measure_Speed_Variant value) speed,
    required TResult Function(Measure_Distance_Variant value) distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
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
abstract class _$$Measure_Speed_VariantCopyWith<$Res> {
  factory _$$Measure_Speed_VariantCopyWith(_$Measure_Speed_Variant value, $Res Function(_$Measure_Speed_Variant) then) =
      __$$Measure_Speed_VariantCopyWithImpl<$Res>;
  $Res call({Speed field0});

  $SpeedCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_Speed_VariantCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res>
    implements _$$Measure_Speed_VariantCopyWith<$Res> {
  __$$Measure_Speed_VariantCopyWithImpl(_$Measure_Speed_Variant _value, $Res Function(_$Measure_Speed_Variant) _then)
      : super(_value, (v) => _then(v as _$Measure_Speed_Variant));

  @override
  _$Measure_Speed_Variant get _value => super._value as _$Measure_Speed_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Measure_Speed_Variant(
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

class _$Measure_Speed_Variant implements Measure_Speed_Variant {
  const _$Measure_Speed_Variant(this.field0);

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
            other is _$Measure_Speed_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Measure_Speed_VariantCopyWith<_$Measure_Speed_Variant> get copyWith =>
      __$$Measure_Speed_VariantCopyWithImpl<_$Measure_Speed_Variant>(this, _$identity);

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
    required TResult Function(Measure_Speed_Variant value) speed,
    required TResult Function(Measure_Distance_Variant value) distance,
  }) {
    return speed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
  }) {
    return speed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
    required TResult orElse(),
  }) {
    if (speed != null) {
      return speed(this);
    }
    return orElse();
  }
}

abstract class Measure_Speed_Variant implements Measure {
  const factory Measure_Speed_Variant(final Speed field0) = _$Measure_Speed_Variant;

  Speed get field0;
  @JsonKey(ignore: true)
  _$$Measure_Speed_VariantCopyWith<_$Measure_Speed_Variant> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Measure_Distance_VariantCopyWith<$Res> {
  factory _$$Measure_Distance_VariantCopyWith(
          _$Measure_Distance_Variant value, $Res Function(_$Measure_Distance_Variant) then) =
      __$$Measure_Distance_VariantCopyWithImpl<$Res>;
  $Res call({Distance field0});

  $DistanceCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Measure_Distance_VariantCopyWithImpl<$Res> extends _$MeasureCopyWithImpl<$Res>
    implements _$$Measure_Distance_VariantCopyWith<$Res> {
  __$$Measure_Distance_VariantCopyWithImpl(
      _$Measure_Distance_Variant _value, $Res Function(_$Measure_Distance_Variant) _then)
      : super(_value, (v) => _then(v as _$Measure_Distance_Variant));

  @override
  _$Measure_Distance_Variant get _value => super._value as _$Measure_Distance_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Measure_Distance_Variant(
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

class _$Measure_Distance_Variant implements Measure_Distance_Variant {
  const _$Measure_Distance_Variant(this.field0);

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
            other is _$Measure_Distance_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Measure_Distance_VariantCopyWith<_$Measure_Distance_Variant> get copyWith =>
      __$$Measure_Distance_VariantCopyWithImpl<_$Measure_Distance_Variant>(this, _$identity);

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
    required TResult Function(Measure_Speed_Variant value) speed,
    required TResult Function(Measure_Distance_Variant value) distance,
  }) {
    return distance(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
  }) {
    return distance?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Measure_Speed_Variant value)? speed,
    TResult Function(Measure_Distance_Variant value)? distance,
    required TResult orElse(),
  }) {
    if (distance != null) {
      return distance(this);
    }
    return orElse();
  }
}

abstract class Measure_Distance_Variant implements Measure {
  const factory Measure_Distance_Variant(final Distance field0) = _$Measure_Distance_Variant;

  Distance get field0;
  @JsonKey(ignore: true)
  _$$Measure_Distance_VariantCopyWith<_$Measure_Distance_Variant> get copyWith => throw _privateConstructorUsedError;
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
    required TResult Function(Speed_Unknown_Variant value) unknown,
    required TResult Function(Speed_GPS_Variant value) gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
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
abstract class _$$Speed_Unknown_VariantCopyWith<$Res> {
  factory _$$Speed_Unknown_VariantCopyWith(_$Speed_Unknown_Variant value, $Res Function(_$Speed_Unknown_Variant) then) =
      __$$Speed_Unknown_VariantCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Speed_Unknown_VariantCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res>
    implements _$$Speed_Unknown_VariantCopyWith<$Res> {
  __$$Speed_Unknown_VariantCopyWithImpl(_$Speed_Unknown_Variant _value, $Res Function(_$Speed_Unknown_Variant) _then)
      : super(_value, (v) => _then(v as _$Speed_Unknown_Variant));

  @override
  _$Speed_Unknown_Variant get _value => super._value as _$Speed_Unknown_Variant;
}

/// @nodoc

class _$Speed_Unknown_Variant implements Speed_Unknown_Variant {
  const _$Speed_Unknown_Variant();

  @override
  String toString() {
    return 'Speed.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Speed_Unknown_Variant);
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
    required TResult Function(Speed_Unknown_Variant value) unknown,
    required TResult Function(Speed_GPS_Variant value) gps,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Speed_Unknown_Variant implements Speed {
  const factory Speed_Unknown_Variant() = _$Speed_Unknown_Variant;
}

/// @nodoc
abstract class _$$Speed_GPS_VariantCopyWith<$Res> {
  factory _$$Speed_GPS_VariantCopyWith(_$Speed_GPS_Variant value, $Res Function(_$Speed_GPS_Variant) then) =
      __$$Speed_GPS_VariantCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$Speed_GPS_VariantCopyWithImpl<$Res> extends _$SpeedCopyWithImpl<$Res>
    implements _$$Speed_GPS_VariantCopyWith<$Res> {
  __$$Speed_GPS_VariantCopyWithImpl(_$Speed_GPS_Variant _value, $Res Function(_$Speed_GPS_Variant) _then)
      : super(_value, (v) => _then(v as _$Speed_GPS_Variant));

  @override
  _$Speed_GPS_Variant get _value => super._value as _$Speed_GPS_Variant;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Speed_GPS_Variant(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Speed_GPS_Variant implements Speed_GPS_Variant {
  const _$Speed_GPS_Variant(this.field0);

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
            other is _$Speed_GPS_Variant &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$Speed_GPS_VariantCopyWith<_$Speed_GPS_Variant> get copyWith =>
      __$$Speed_GPS_VariantCopyWithImpl<_$Speed_GPS_Variant>(this, _$identity);

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
    required TResult Function(Speed_Unknown_Variant value) unknown,
    required TResult Function(Speed_GPS_Variant value) gps,
  }) {
    return gps(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
  }) {
    return gps?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Speed_Unknown_Variant value)? unknown,
    TResult Function(Speed_GPS_Variant value)? gps,
    required TResult orElse(),
  }) {
    if (gps != null) {
      return gps(this);
    }
    return orElse();
  }
}

abstract class Speed_GPS_Variant implements Speed {
  const factory Speed_GPS_Variant(final double field0) = _$Speed_GPS_Variant;

  double get field0;
  @JsonKey(ignore: true)
  _$$Speed_GPS_VariantCopyWith<_$Speed_GPS_Variant> get copyWith => throw _privateConstructorUsedError;
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
