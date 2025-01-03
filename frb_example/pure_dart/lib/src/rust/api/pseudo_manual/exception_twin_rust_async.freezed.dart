// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'exception_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

CustomEnumErrorTwinRustAsync _$CustomEnumErrorTwinRustAsyncFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'one':
      return CustomEnumErrorTwinRustAsync_One.fromJson(json);
    case 'two':
      return CustomEnumErrorTwinRustAsync_Two.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomEnumErrorTwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomEnumErrorTwinRustAsync {
  Object get message => throw _privateConstructorUsedError;
  String get backtrace => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String backtrace) one,
    required TResult Function(int message, String backtrace) two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String backtrace)? one,
    TResult? Function(int message, String backtrace)? two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String backtrace)? one,
    TResult Function(int message, String backtrace)? two,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomEnumErrorTwinRustAsync_One value) one,
    required TResult Function(CustomEnumErrorTwinRustAsync_Two value) two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult? Function(CustomEnumErrorTwinRustAsync_Two value)? two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult Function(CustomEnumErrorTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CustomEnumErrorTwinRustAsyncCopyWith<CustomEnumErrorTwinRustAsync>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomEnumErrorTwinRustAsyncCopyWith(
          CustomEnumErrorTwinRustAsync value,
          $Res Function(CustomEnumErrorTwinRustAsync) then) =
      _$CustomEnumErrorTwinRustAsyncCopyWithImpl<$Res,
          CustomEnumErrorTwinRustAsync>;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomEnumErrorTwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomEnumErrorTwinRustAsync>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  _$CustomEnumErrorTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? backtrace = null,
  }) {
    return _then(_value.copyWith(
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CustomEnumErrorTwinRustAsync_OneImplCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory _$$CustomEnumErrorTwinRustAsync_OneImplCopyWith(
          _$CustomEnumErrorTwinRustAsync_OneImpl value,
          $Res Function(_$CustomEnumErrorTwinRustAsync_OneImpl) then) =
      __$$CustomEnumErrorTwinRustAsync_OneImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message, String backtrace});
}

/// @nodoc
class __$$CustomEnumErrorTwinRustAsync_OneImplCopyWithImpl<$Res>
    extends _$CustomEnumErrorTwinRustAsyncCopyWithImpl<$Res,
        _$CustomEnumErrorTwinRustAsync_OneImpl>
    implements _$$CustomEnumErrorTwinRustAsync_OneImplCopyWith<$Res> {
  __$$CustomEnumErrorTwinRustAsync_OneImplCopyWithImpl(
      _$CustomEnumErrorTwinRustAsync_OneImpl _value,
      $Res Function(_$CustomEnumErrorTwinRustAsync_OneImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomEnumErrorTwinRustAsync_OneImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomEnumErrorTwinRustAsync_OneImpl
    extends CustomEnumErrorTwinRustAsync_One {
  const _$CustomEnumErrorTwinRustAsync_OneImpl(
      {required this.message, required this.backtrace, final String? $type})
      : $type = $type ?? 'one',
        super._();

  factory _$CustomEnumErrorTwinRustAsync_OneImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomEnumErrorTwinRustAsync_OneImplFromJson(json);

  @override
  final String message;
  @override
  final String backtrace;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsync.one(message: $message, backtrace: $backtrace)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomEnumErrorTwinRustAsync_OneImpl &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomEnumErrorTwinRustAsync_OneImplCopyWith<
          _$CustomEnumErrorTwinRustAsync_OneImpl>
      get copyWith => __$$CustomEnumErrorTwinRustAsync_OneImplCopyWithImpl<
          _$CustomEnumErrorTwinRustAsync_OneImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String backtrace) one,
    required TResult Function(int message, String backtrace) two,
  }) {
    return one(message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String backtrace)? one,
    TResult? Function(int message, String backtrace)? two,
  }) {
    return one?.call(message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String backtrace)? one,
    TResult Function(int message, String backtrace)? two,
    required TResult orElse(),
  }) {
    if (one != null) {
      return one(message, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomEnumErrorTwinRustAsync_One value) one,
    required TResult Function(CustomEnumErrorTwinRustAsync_Two value) two,
  }) {
    return one(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult? Function(CustomEnumErrorTwinRustAsync_Two value)? two,
  }) {
    return one?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult Function(CustomEnumErrorTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) {
    if (one != null) {
      return one(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomEnumErrorTwinRustAsync_OneImplToJson(
      this,
    );
  }
}

abstract class CustomEnumErrorTwinRustAsync_One
    extends CustomEnumErrorTwinRustAsync implements FrbBacktracedException {
  const factory CustomEnumErrorTwinRustAsync_One(
          {required final String message, required final String backtrace}) =
      _$CustomEnumErrorTwinRustAsync_OneImpl;
  const CustomEnumErrorTwinRustAsync_One._() : super._();

  factory CustomEnumErrorTwinRustAsync_One.fromJson(Map<String, dynamic> json) =
      _$CustomEnumErrorTwinRustAsync_OneImpl.fromJson;

  @override
  String get message;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomEnumErrorTwinRustAsync_OneImplCopyWith<
          _$CustomEnumErrorTwinRustAsync_OneImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomEnumErrorTwinRustAsync_TwoImplCopyWith<$Res>
    implements $CustomEnumErrorTwinRustAsyncCopyWith<$Res> {
  factory _$$CustomEnumErrorTwinRustAsync_TwoImplCopyWith(
          _$CustomEnumErrorTwinRustAsync_TwoImpl value,
          $Res Function(_$CustomEnumErrorTwinRustAsync_TwoImpl) then) =
      __$$CustomEnumErrorTwinRustAsync_TwoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int message, String backtrace});
}

/// @nodoc
class __$$CustomEnumErrorTwinRustAsync_TwoImplCopyWithImpl<$Res>
    extends _$CustomEnumErrorTwinRustAsyncCopyWithImpl<$Res,
        _$CustomEnumErrorTwinRustAsync_TwoImpl>
    implements _$$CustomEnumErrorTwinRustAsync_TwoImplCopyWith<$Res> {
  __$$CustomEnumErrorTwinRustAsync_TwoImplCopyWithImpl(
      _$CustomEnumErrorTwinRustAsync_TwoImpl _value,
      $Res Function(_$CustomEnumErrorTwinRustAsync_TwoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomEnumErrorTwinRustAsync_TwoImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as int,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomEnumErrorTwinRustAsync_TwoImpl
    extends CustomEnumErrorTwinRustAsync_Two {
  const _$CustomEnumErrorTwinRustAsync_TwoImpl(
      {required this.message, required this.backtrace, final String? $type})
      : $type = $type ?? 'two',
        super._();

  factory _$CustomEnumErrorTwinRustAsync_TwoImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomEnumErrorTwinRustAsync_TwoImplFromJson(json);

  @override
  final int message;
  @override
  final String backtrace;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomEnumErrorTwinRustAsync.two(message: $message, backtrace: $backtrace)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomEnumErrorTwinRustAsync_TwoImpl &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, message, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomEnumErrorTwinRustAsync_TwoImplCopyWith<
          _$CustomEnumErrorTwinRustAsync_TwoImpl>
      get copyWith => __$$CustomEnumErrorTwinRustAsync_TwoImplCopyWithImpl<
          _$CustomEnumErrorTwinRustAsync_TwoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message, String backtrace) one,
    required TResult Function(int message, String backtrace) two,
  }) {
    return two(message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message, String backtrace)? one,
    TResult? Function(int message, String backtrace)? two,
  }) {
    return two?.call(message, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message, String backtrace)? one,
    TResult Function(int message, String backtrace)? two,
    required TResult orElse(),
  }) {
    if (two != null) {
      return two(message, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomEnumErrorTwinRustAsync_One value) one,
    required TResult Function(CustomEnumErrorTwinRustAsync_Two value) two,
  }) {
    return two(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult? Function(CustomEnumErrorTwinRustAsync_Two value)? two,
  }) {
    return two?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomEnumErrorTwinRustAsync_One value)? one,
    TResult Function(CustomEnumErrorTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) {
    if (two != null) {
      return two(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomEnumErrorTwinRustAsync_TwoImplToJson(
      this,
    );
  }
}

abstract class CustomEnumErrorTwinRustAsync_Two
    extends CustomEnumErrorTwinRustAsync implements FrbBacktracedException {
  const factory CustomEnumErrorTwinRustAsync_Two(
          {required final int message, required final String backtrace}) =
      _$CustomEnumErrorTwinRustAsync_TwoImpl;
  const CustomEnumErrorTwinRustAsync_Two._() : super._();

  factory CustomEnumErrorTwinRustAsync_Two.fromJson(Map<String, dynamic> json) =
      _$CustomEnumErrorTwinRustAsync_TwoImpl.fromJson;

  @override
  int get message;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomEnumErrorTwinRustAsync_TwoImplCopyWith<
          _$CustomEnumErrorTwinRustAsync_TwoImpl>
      get copyWith => throw _privateConstructorUsedError;
}

CustomErrorTwinRustAsync _$CustomErrorTwinRustAsyncFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'error0':
      return CustomErrorTwinRustAsync_Error0.fromJson(json);
    case 'error1':
      return CustomErrorTwinRustAsync_Error1.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomErrorTwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomErrorTwinRustAsync {
  Object get e => throw _privateConstructorUsedError;
  String get backtrace => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomErrorTwinRustAsync_Error0 value) error0,
    required TResult Function(CustomErrorTwinRustAsync_Error1 value) error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult? Function(CustomErrorTwinRustAsync_Error1 value)? error1,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult Function(CustomErrorTwinRustAsync_Error1 value)? error1,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CustomErrorTwinRustAsyncCopyWith<CustomErrorTwinRustAsync> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory $CustomErrorTwinRustAsyncCopyWith(CustomErrorTwinRustAsync value,
          $Res Function(CustomErrorTwinRustAsync) then) =
      _$CustomErrorTwinRustAsyncCopyWithImpl<$Res, CustomErrorTwinRustAsync>;
  @useResult
  $Res call({String backtrace});
}

/// @nodoc
class _$CustomErrorTwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomErrorTwinRustAsync>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  _$CustomErrorTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? backtrace = null,
  }) {
    return _then(_value.copyWith(
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CustomErrorTwinRustAsync_Error0ImplCopyWith<$Res>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory _$$CustomErrorTwinRustAsync_Error0ImplCopyWith(
          _$CustomErrorTwinRustAsync_Error0Impl value,
          $Res Function(_$CustomErrorTwinRustAsync_Error0Impl) then) =
      __$$CustomErrorTwinRustAsync_Error0ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String e, String backtrace});
}

/// @nodoc
class __$$CustomErrorTwinRustAsync_Error0ImplCopyWithImpl<$Res>
    extends _$CustomErrorTwinRustAsyncCopyWithImpl<$Res,
        _$CustomErrorTwinRustAsync_Error0Impl>
    implements _$$CustomErrorTwinRustAsync_Error0ImplCopyWith<$Res> {
  __$$CustomErrorTwinRustAsync_Error0ImplCopyWithImpl(
      _$CustomErrorTwinRustAsync_Error0Impl _value,
      $Res Function(_$CustomErrorTwinRustAsync_Error0Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomErrorTwinRustAsync_Error0Impl(
      e: null == e
          ? _value.e
          : e // ignore: cast_nullable_to_non_nullable
              as String,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomErrorTwinRustAsync_Error0Impl
    extends CustomErrorTwinRustAsync_Error0 {
  const _$CustomErrorTwinRustAsync_Error0Impl(
      {required this.e, required this.backtrace, final String? $type})
      : $type = $type ?? 'error0',
        super._();

  factory _$CustomErrorTwinRustAsync_Error0Impl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomErrorTwinRustAsync_Error0ImplFromJson(json);

  @override
  final String e;
  @override
  final String backtrace;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomErrorTwinRustAsync.error0(e: $e, backtrace: $backtrace)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomErrorTwinRustAsync_Error0Impl &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomErrorTwinRustAsync_Error0ImplCopyWith<
          _$CustomErrorTwinRustAsync_Error0Impl>
      get copyWith => __$$CustomErrorTwinRustAsync_Error0ImplCopyWithImpl<
          _$CustomErrorTwinRustAsync_Error0Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) {
    return error0(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) {
    return error0?.call(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(e, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomErrorTwinRustAsync_Error0 value) error0,
    required TResult Function(CustomErrorTwinRustAsync_Error1 value) error1,
  }) {
    return error0(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult? Function(CustomErrorTwinRustAsync_Error1 value)? error1,
  }) {
    return error0?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult Function(CustomErrorTwinRustAsync_Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error0 != null) {
      return error0(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomErrorTwinRustAsync_Error0ImplToJson(
      this,
    );
  }
}

abstract class CustomErrorTwinRustAsync_Error0 extends CustomErrorTwinRustAsync
    implements FrbBacktracedException {
  const factory CustomErrorTwinRustAsync_Error0(
      {required final String e,
      required final String backtrace}) = _$CustomErrorTwinRustAsync_Error0Impl;
  const CustomErrorTwinRustAsync_Error0._() : super._();

  factory CustomErrorTwinRustAsync_Error0.fromJson(Map<String, dynamic> json) =
      _$CustomErrorTwinRustAsync_Error0Impl.fromJson;

  @override
  String get e;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomErrorTwinRustAsync_Error0ImplCopyWith<
          _$CustomErrorTwinRustAsync_Error0Impl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomErrorTwinRustAsync_Error1ImplCopyWith<$Res>
    implements $CustomErrorTwinRustAsyncCopyWith<$Res> {
  factory _$$CustomErrorTwinRustAsync_Error1ImplCopyWith(
          _$CustomErrorTwinRustAsync_Error1Impl value,
          $Res Function(_$CustomErrorTwinRustAsync_Error1Impl) then) =
      __$$CustomErrorTwinRustAsync_Error1ImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int e, String backtrace});
}

/// @nodoc
class __$$CustomErrorTwinRustAsync_Error1ImplCopyWithImpl<$Res>
    extends _$CustomErrorTwinRustAsyncCopyWithImpl<$Res,
        _$CustomErrorTwinRustAsync_Error1Impl>
    implements _$$CustomErrorTwinRustAsync_Error1ImplCopyWith<$Res> {
  __$$CustomErrorTwinRustAsync_Error1ImplCopyWithImpl(
      _$CustomErrorTwinRustAsync_Error1Impl _value,
      $Res Function(_$CustomErrorTwinRustAsync_Error1Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? e = null,
    Object? backtrace = null,
  }) {
    return _then(_$CustomErrorTwinRustAsync_Error1Impl(
      e: null == e
          ? _value.e
          : e // ignore: cast_nullable_to_non_nullable
              as int,
      backtrace: null == backtrace
          ? _value.backtrace
          : backtrace // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomErrorTwinRustAsync_Error1Impl
    extends CustomErrorTwinRustAsync_Error1 {
  const _$CustomErrorTwinRustAsync_Error1Impl(
      {required this.e, required this.backtrace, final String? $type})
      : $type = $type ?? 'error1',
        super._();

  factory _$CustomErrorTwinRustAsync_Error1Impl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomErrorTwinRustAsync_Error1ImplFromJson(json);

  @override
  final int e;
  @override
  final String backtrace;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomErrorTwinRustAsync.error1(e: $e, backtrace: $backtrace)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomErrorTwinRustAsync_Error1Impl &&
            (identical(other.e, e) || other.e == e) &&
            (identical(other.backtrace, backtrace) ||
                other.backtrace == backtrace));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, e, backtrace);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomErrorTwinRustAsync_Error1ImplCopyWith<
          _$CustomErrorTwinRustAsync_Error1Impl>
      get copyWith => __$$CustomErrorTwinRustAsync_Error1ImplCopyWithImpl<
          _$CustomErrorTwinRustAsync_Error1Impl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String e, String backtrace) error0,
    required TResult Function(int e, String backtrace) error1,
  }) {
    return error1(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String e, String backtrace)? error0,
    TResult? Function(int e, String backtrace)? error1,
  }) {
    return error1?.call(e, backtrace);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String e, String backtrace)? error0,
    TResult Function(int e, String backtrace)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(e, backtrace);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomErrorTwinRustAsync_Error0 value) error0,
    required TResult Function(CustomErrorTwinRustAsync_Error1 value) error1,
  }) {
    return error1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult? Function(CustomErrorTwinRustAsync_Error1 value)? error1,
  }) {
    return error1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomErrorTwinRustAsync_Error0 value)? error0,
    TResult Function(CustomErrorTwinRustAsync_Error1 value)? error1,
    required TResult orElse(),
  }) {
    if (error1 != null) {
      return error1(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomErrorTwinRustAsync_Error1ImplToJson(
      this,
    );
  }
}

abstract class CustomErrorTwinRustAsync_Error1 extends CustomErrorTwinRustAsync
    implements FrbBacktracedException {
  const factory CustomErrorTwinRustAsync_Error1(
      {required final int e,
      required final String backtrace}) = _$CustomErrorTwinRustAsync_Error1Impl;
  const CustomErrorTwinRustAsync_Error1._() : super._();

  factory CustomErrorTwinRustAsync_Error1.fromJson(Map<String, dynamic> json) =
      _$CustomErrorTwinRustAsync_Error1Impl.fromJson;

  @override
  int get e;
  @override
  String get backtrace;
  @override
  @JsonKey(ignore: true)
  _$$CustomErrorTwinRustAsync_Error1ImplCopyWith<
          _$CustomErrorTwinRustAsync_Error1Impl>
      get copyWith => throw _privateConstructorUsedError;
}

CustomNestedError1TwinRustAsync _$CustomNestedError1TwinRustAsyncFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'customNested1':
      return CustomNestedError1TwinRustAsync_CustomNested1.fromJson(json);
    case 'errorNested':
      return CustomNestedError1TwinRustAsync_ErrorNested.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomNestedError1TwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomNestedError1TwinRustAsync {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2TwinRustAsync field0)
        errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2TwinRustAsync field0)? errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2TwinRustAsync field0)? errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(
            CustomNestedError1TwinRustAsync_CustomNested1 value)
        customNested1,
    required TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)
        errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult? Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError1TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError1TwinRustAsyncCopyWith(
          CustomNestedError1TwinRustAsync value,
          $Res Function(CustomNestedError1TwinRustAsync) then) =
      _$CustomNestedError1TwinRustAsyncCopyWithImpl<$Res,
          CustomNestedError1TwinRustAsync>;
}

/// @nodoc
class _$CustomNestedError1TwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomNestedError1TwinRustAsync>
    implements $CustomNestedError1TwinRustAsyncCopyWith<$Res> {
  _$CustomNestedError1TwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWith<
    $Res> {
  factory _$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWith(
          _$CustomNestedError1TwinRustAsync_CustomNested1Impl value,
          $Res Function(_$CustomNestedError1TwinRustAsync_CustomNested1Impl)
              then) =
      __$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWithImpl<$Res>
    extends _$CustomNestedError1TwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedError1TwinRustAsync_CustomNested1Impl>
    implements
        _$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWith<$Res> {
  __$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWithImpl(
      _$CustomNestedError1TwinRustAsync_CustomNested1Impl _value,
      $Res Function(_$CustomNestedError1TwinRustAsync_CustomNested1Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError1TwinRustAsync_CustomNested1Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedError1TwinRustAsync_CustomNested1Impl
    extends CustomNestedError1TwinRustAsync_CustomNested1 {
  const _$CustomNestedError1TwinRustAsync_CustomNested1Impl(this.field0,
      {final String? $type})
      : $type = $type ?? 'customNested1',
        super._();

  factory _$CustomNestedError1TwinRustAsync_CustomNested1Impl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedError1TwinRustAsync_CustomNested1ImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsync.customNested1(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError1TwinRustAsync_CustomNested1Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWith<
          _$CustomNestedError1TwinRustAsync_CustomNested1Impl>
      get copyWith =>
          __$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWithImpl<
                  _$CustomNestedError1TwinRustAsync_CustomNested1Impl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2TwinRustAsync field0)
        errorNested,
  }) {
    return customNested1(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2TwinRustAsync field0)? errorNested,
  }) {
    return customNested1?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2TwinRustAsync field0)? errorNested,
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
    required TResult Function(
            CustomNestedError1TwinRustAsync_CustomNested1 value)
        customNested1,
    required TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)
        errorNested,
  }) {
    return customNested1(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult? Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
  }) {
    return customNested1?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
    required TResult orElse(),
  }) {
    if (customNested1 != null) {
      return customNested1(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedError1TwinRustAsync_CustomNested1ImplToJson(
      this,
    );
  }
}

abstract class CustomNestedError1TwinRustAsync_CustomNested1
    extends CustomNestedError1TwinRustAsync {
  const factory CustomNestedError1TwinRustAsync_CustomNested1(
          final String field0) =
      _$CustomNestedError1TwinRustAsync_CustomNested1Impl;
  const CustomNestedError1TwinRustAsync_CustomNested1._() : super._();

  factory CustomNestedError1TwinRustAsync_CustomNested1.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedError1TwinRustAsync_CustomNested1Impl.fromJson;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError1TwinRustAsync_CustomNested1ImplCopyWith<
          _$CustomNestedError1TwinRustAsync_CustomNested1Impl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWith<
    $Res> {
  factory _$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWith(
          _$CustomNestedError1TwinRustAsync_ErrorNestedImpl value,
          $Res Function(_$CustomNestedError1TwinRustAsync_ErrorNestedImpl)
              then) =
      __$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({CustomNestedError2TwinRustAsync field0});

  $CustomNestedError2TwinRustAsyncCopyWith<$Res> get field0;
}

/// @nodoc
class __$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWithImpl<$Res>
    extends _$CustomNestedError1TwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedError1TwinRustAsync_ErrorNestedImpl>
    implements
        _$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWith<$Res> {
  __$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWithImpl(
      _$CustomNestedError1TwinRustAsync_ErrorNestedImpl _value,
      $Res Function(_$CustomNestedError1TwinRustAsync_ErrorNestedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError1TwinRustAsync_ErrorNestedImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedError2TwinRustAsync,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $CustomNestedError2TwinRustAsyncCopyWith<$Res> get field0 {
    return $CustomNestedError2TwinRustAsyncCopyWith<$Res>(_value.field0,
        (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedError1TwinRustAsync_ErrorNestedImpl
    extends CustomNestedError1TwinRustAsync_ErrorNested {
  const _$CustomNestedError1TwinRustAsync_ErrorNestedImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'errorNested',
        super._();

  factory _$CustomNestedError1TwinRustAsync_ErrorNestedImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedError1TwinRustAsync_ErrorNestedImplFromJson(json);

  @override
  final CustomNestedError2TwinRustAsync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedError1TwinRustAsync.errorNested(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError1TwinRustAsync_ErrorNestedImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWith<
          _$CustomNestedError1TwinRustAsync_ErrorNestedImpl>
      get copyWith =>
          __$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWithImpl<
                  _$CustomNestedError1TwinRustAsync_ErrorNestedImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested1,
    required TResult Function(CustomNestedError2TwinRustAsync field0)
        errorNested,
  }) {
    return errorNested(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested1,
    TResult? Function(CustomNestedError2TwinRustAsync field0)? errorNested,
  }) {
    return errorNested?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? customNested1,
    TResult Function(CustomNestedError2TwinRustAsync field0)? errorNested,
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
    required TResult Function(
            CustomNestedError1TwinRustAsync_CustomNested1 value)
        customNested1,
    required TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)
        errorNested,
  }) {
    return errorNested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult? Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
  }) {
    return errorNested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError1TwinRustAsync_CustomNested1 value)?
        customNested1,
    TResult Function(CustomNestedError1TwinRustAsync_ErrorNested value)?
        errorNested,
    required TResult orElse(),
  }) {
    if (errorNested != null) {
      return errorNested(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedError1TwinRustAsync_ErrorNestedImplToJson(
      this,
    );
  }
}

abstract class CustomNestedError1TwinRustAsync_ErrorNested
    extends CustomNestedError1TwinRustAsync {
  const factory CustomNestedError1TwinRustAsync_ErrorNested(
          final CustomNestedError2TwinRustAsync field0) =
      _$CustomNestedError1TwinRustAsync_ErrorNestedImpl;
  const CustomNestedError1TwinRustAsync_ErrorNested._() : super._();

  factory CustomNestedError1TwinRustAsync_ErrorNested.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedError1TwinRustAsync_ErrorNestedImpl.fromJson;

  @override
  CustomNestedError2TwinRustAsync get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError1TwinRustAsync_ErrorNestedImplCopyWith<
          _$CustomNestedError1TwinRustAsync_ErrorNestedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

CustomNestedError2TwinRustAsync _$CustomNestedError2TwinRustAsyncFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'customNested2':
      return CustomNestedError2TwinRustAsync_CustomNested2.fromJson(json);
    case 'customNested2Number':
      return CustomNestedError2TwinRustAsync_CustomNested2Number.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomNestedError2TwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomNestedError2TwinRustAsync {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) customNested2,
    required TResult Function(int field0) customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
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
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2 value)
        customNested2,
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)
        customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult? Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedError2TwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedError2TwinRustAsyncCopyWith(
          CustomNestedError2TwinRustAsync value,
          $Res Function(CustomNestedError2TwinRustAsync) then) =
      _$CustomNestedError2TwinRustAsyncCopyWithImpl<$Res,
          CustomNestedError2TwinRustAsync>;
}

/// @nodoc
class _$CustomNestedError2TwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomNestedError2TwinRustAsync>
    implements $CustomNestedError2TwinRustAsyncCopyWith<$Res> {
  _$CustomNestedError2TwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWith<
    $Res> {
  factory _$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWith(
          _$CustomNestedError2TwinRustAsync_CustomNested2Impl value,
          $Res Function(_$CustomNestedError2TwinRustAsync_CustomNested2Impl)
              then) =
      __$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWithImpl<$Res>
    extends _$CustomNestedError2TwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedError2TwinRustAsync_CustomNested2Impl>
    implements
        _$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWith<$Res> {
  __$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWithImpl(
      _$CustomNestedError2TwinRustAsync_CustomNested2Impl _value,
      $Res Function(_$CustomNestedError2TwinRustAsync_CustomNested2Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError2TwinRustAsync_CustomNested2Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedError2TwinRustAsync_CustomNested2Impl
    extends CustomNestedError2TwinRustAsync_CustomNested2 {
  const _$CustomNestedError2TwinRustAsync_CustomNested2Impl(this.field0,
      {final String? $type})
      : $type = $type ?? 'customNested2',
        super._();

  factory _$CustomNestedError2TwinRustAsync_CustomNested2Impl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedError2TwinRustAsync_CustomNested2ImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsync.customNested2(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedError2TwinRustAsync_CustomNested2Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWith<
          _$CustomNestedError2TwinRustAsync_CustomNested2Impl>
      get copyWith =>
          __$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWithImpl<
                  _$CustomNestedError2TwinRustAsync_CustomNested2Impl>(
              this, _$identity);

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
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
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
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2 value)
        customNested2,
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)
        customNested2Number,
  }) {
    return customNested2(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult? Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
  }) {
    return customNested2?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2 != null) {
      return customNested2(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedError2TwinRustAsync_CustomNested2ImplToJson(
      this,
    );
  }
}

abstract class CustomNestedError2TwinRustAsync_CustomNested2
    extends CustomNestedError2TwinRustAsync {
  const factory CustomNestedError2TwinRustAsync_CustomNested2(
          final String field0) =
      _$CustomNestedError2TwinRustAsync_CustomNested2Impl;
  const CustomNestedError2TwinRustAsync_CustomNested2._() : super._();

  factory CustomNestedError2TwinRustAsync_CustomNested2.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedError2TwinRustAsync_CustomNested2Impl.fromJson;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError2TwinRustAsync_CustomNested2ImplCopyWith<
          _$CustomNestedError2TwinRustAsync_CustomNested2Impl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWith<
    $Res> {
  factory _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWith(
          _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl value,
          $Res Function(
                  _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl)
              then) =
      __$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWithImpl<
          $Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWithImpl<
        $Res>
    extends _$CustomNestedError2TwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl>
    implements
        _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWith<
            $Res> {
  __$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWithImpl(
      _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl _value,
      $Res Function(_$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl)
          _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl
    extends CustomNestedError2TwinRustAsync_CustomNested2Number {
  const _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'customNested2Number',
        super._();

  factory _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplFromJson(json);

  @override
  final int field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedError2TwinRustAsync.customNested2Number(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other
                is _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWith<
          _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl>
      get copyWith =>
          __$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWithImpl<
                  _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl>(
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
    TResult? Function(String field0)? customNested2,
    TResult? Function(int field0)? customNested2Number,
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
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2 value)
        customNested2,
    required TResult Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)
        customNested2Number,
  }) {
    return customNested2Number(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult? Function(
            CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
  }) {
    return customNested2Number?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2 value)?
        customNested2,
    TResult Function(CustomNestedError2TwinRustAsync_CustomNested2Number value)?
        customNested2Number,
    required TResult orElse(),
  }) {
    if (customNested2Number != null) {
      return customNested2Number(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplToJson(
      this,
    );
  }
}

abstract class CustomNestedError2TwinRustAsync_CustomNested2Number
    extends CustomNestedError2TwinRustAsync {
  const factory CustomNestedError2TwinRustAsync_CustomNested2Number(
          final int field0) =
      _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl;
  const CustomNestedError2TwinRustAsync_CustomNested2Number._() : super._();

  factory CustomNestedError2TwinRustAsync_CustomNested2Number.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl.fromJson;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedError2TwinRustAsync_CustomNested2NumberImplCopyWith<
          _$CustomNestedError2TwinRustAsync_CustomNested2NumberImpl>
      get copyWith => throw _privateConstructorUsedError;
}

CustomNestedErrorInnerTwinRustAsync
    _$CustomNestedErrorInnerTwinRustAsyncFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'three':
      return CustomNestedErrorInnerTwinRustAsync_Three.fromJson(json);
    case 'four':
      return CustomNestedErrorInnerTwinRustAsync_Four.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomNestedErrorInnerTwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomNestedErrorInnerTwinRustAsync {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) three,
    required TResult Function(int field0) four,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? three,
    TResult? Function(int field0)? four,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? three,
    TResult Function(int field0)? four,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)
        three,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)
        four,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorInnerTwinRustAsyncCopyWith(
          CustomNestedErrorInnerTwinRustAsync value,
          $Res Function(CustomNestedErrorInnerTwinRustAsync) then) =
      _$CustomNestedErrorInnerTwinRustAsyncCopyWithImpl<$Res,
          CustomNestedErrorInnerTwinRustAsync>;
}

/// @nodoc
class _$CustomNestedErrorInnerTwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomNestedErrorInnerTwinRustAsync>
    implements $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> {
  _$CustomNestedErrorInnerTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWith<$Res> {
  factory _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWith(
          _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl value,
          $Res Function(_$CustomNestedErrorInnerTwinRustAsync_ThreeImpl) then) =
      __$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWithImpl<$Res>
    extends _$CustomNestedErrorInnerTwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl>
    implements _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWith<$Res> {
  __$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWithImpl(
      _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl _value,
      $Res Function(_$CustomNestedErrorInnerTwinRustAsync_ThreeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedErrorInnerTwinRustAsync_ThreeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl
    extends CustomNestedErrorInnerTwinRustAsync_Three {
  const _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'three',
        super._();

  factory _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsync.three(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWith<
          _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl>
      get copyWith =>
          __$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWithImpl<
                  _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) three,
    required TResult Function(int field0) four,
  }) {
    return three(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? three,
    TResult? Function(int field0)? four,
  }) {
    return three?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? three,
    TResult Function(int field0)? four,
    required TResult orElse(),
  }) {
    if (three != null) {
      return three(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)
        three,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)
        four,
  }) {
    return three(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
  }) {
    return three?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
    required TResult orElse(),
  }) {
    if (three != null) {
      return three(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplToJson(
      this,
    );
  }
}

abstract class CustomNestedErrorInnerTwinRustAsync_Three
    extends CustomNestedErrorInnerTwinRustAsync {
  const factory CustomNestedErrorInnerTwinRustAsync_Three(final String field0) =
      _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl;
  const CustomNestedErrorInnerTwinRustAsync_Three._() : super._();

  factory CustomNestedErrorInnerTwinRustAsync_Three.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl.fromJson;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedErrorInnerTwinRustAsync_ThreeImplCopyWith<
          _$CustomNestedErrorInnerTwinRustAsync_ThreeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWith<$Res> {
  factory _$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWith(
          _$CustomNestedErrorInnerTwinRustAsync_FourImpl value,
          $Res Function(_$CustomNestedErrorInnerTwinRustAsync_FourImpl) then) =
      __$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWithImpl<$Res>
    extends _$CustomNestedErrorInnerTwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedErrorInnerTwinRustAsync_FourImpl>
    implements _$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWith<$Res> {
  __$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWithImpl(
      _$CustomNestedErrorInnerTwinRustAsync_FourImpl _value,
      $Res Function(_$CustomNestedErrorInnerTwinRustAsync_FourImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedErrorInnerTwinRustAsync_FourImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedErrorInnerTwinRustAsync_FourImpl
    extends CustomNestedErrorInnerTwinRustAsync_Four {
  const _$CustomNestedErrorInnerTwinRustAsync_FourImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'four',
        super._();

  factory _$CustomNestedErrorInnerTwinRustAsync_FourImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedErrorInnerTwinRustAsync_FourImplFromJson(json);

  @override
  final int field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedErrorInnerTwinRustAsync.four(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedErrorInnerTwinRustAsync_FourImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWith<
          _$CustomNestedErrorInnerTwinRustAsync_FourImpl>
      get copyWith =>
          __$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWithImpl<
              _$CustomNestedErrorInnerTwinRustAsync_FourImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) three,
    required TResult Function(int field0) four,
  }) {
    return four(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? three,
    TResult? Function(int field0)? four,
  }) {
    return four?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? three,
    TResult Function(int field0)? four,
    required TResult orElse(),
  }) {
    if (four != null) {
      return four(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)
        three,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)
        four,
  }) {
    return four(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
  }) {
    return four?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Three value)? three,
    TResult Function(CustomNestedErrorInnerTwinRustAsync_Four value)? four,
    required TResult orElse(),
  }) {
    if (four != null) {
      return four(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedErrorInnerTwinRustAsync_FourImplToJson(
      this,
    );
  }
}

abstract class CustomNestedErrorInnerTwinRustAsync_Four
    extends CustomNestedErrorInnerTwinRustAsync {
  const factory CustomNestedErrorInnerTwinRustAsync_Four(final int field0) =
      _$CustomNestedErrorInnerTwinRustAsync_FourImpl;
  const CustomNestedErrorInnerTwinRustAsync_Four._() : super._();

  factory CustomNestedErrorInnerTwinRustAsync_Four.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedErrorInnerTwinRustAsync_FourImpl.fromJson;

  @override
  int get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedErrorInnerTwinRustAsync_FourImplCopyWith<
          _$CustomNestedErrorInnerTwinRustAsync_FourImpl>
      get copyWith => throw _privateConstructorUsedError;
}

CustomNestedErrorOuterTwinRustAsync
    _$CustomNestedErrorOuterTwinRustAsyncFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'one':
      return CustomNestedErrorOuterTwinRustAsync_One.fromJson(json);
    case 'two':
      return CustomNestedErrorOuterTwinRustAsync_Two.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'CustomNestedErrorOuterTwinRustAsync',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$CustomNestedErrorOuterTwinRustAsync {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) one,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync field0) two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? one,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? one,
    TResult Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)
        one,
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)
        two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CustomNestedErrorOuterTwinRustAsyncCopyWith<$Res> {
  factory $CustomNestedErrorOuterTwinRustAsyncCopyWith(
          CustomNestedErrorOuterTwinRustAsync value,
          $Res Function(CustomNestedErrorOuterTwinRustAsync) then) =
      _$CustomNestedErrorOuterTwinRustAsyncCopyWithImpl<$Res,
          CustomNestedErrorOuterTwinRustAsync>;
}

/// @nodoc
class _$CustomNestedErrorOuterTwinRustAsyncCopyWithImpl<$Res,
        $Val extends CustomNestedErrorOuterTwinRustAsync>
    implements $CustomNestedErrorOuterTwinRustAsyncCopyWith<$Res> {
  _$CustomNestedErrorOuterTwinRustAsyncCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWith<$Res> {
  factory _$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWith(
          _$CustomNestedErrorOuterTwinRustAsync_OneImpl value,
          $Res Function(_$CustomNestedErrorOuterTwinRustAsync_OneImpl) then) =
      __$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWithImpl<$Res>
    extends _$CustomNestedErrorOuterTwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedErrorOuterTwinRustAsync_OneImpl>
    implements _$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWith<$Res> {
  __$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWithImpl(
      _$CustomNestedErrorOuterTwinRustAsync_OneImpl _value,
      $Res Function(_$CustomNestedErrorOuterTwinRustAsync_OneImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedErrorOuterTwinRustAsync_OneImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedErrorOuterTwinRustAsync_OneImpl
    extends CustomNestedErrorOuterTwinRustAsync_One {
  const _$CustomNestedErrorOuterTwinRustAsync_OneImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'one',
        super._();

  factory _$CustomNestedErrorOuterTwinRustAsync_OneImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedErrorOuterTwinRustAsync_OneImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsync.one(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedErrorOuterTwinRustAsync_OneImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWith<
          _$CustomNestedErrorOuterTwinRustAsync_OneImpl>
      get copyWith =>
          __$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWithImpl<
              _$CustomNestedErrorOuterTwinRustAsync_OneImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) one,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync field0) two,
  }) {
    return one(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? one,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
  }) {
    return one?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? one,
    TResult Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
    required TResult orElse(),
  }) {
    if (one != null) {
      return one(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)
        one,
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)
        two,
  }) {
    return one(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
  }) {
    return one?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) {
    if (one != null) {
      return one(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedErrorOuterTwinRustAsync_OneImplToJson(
      this,
    );
  }
}

abstract class CustomNestedErrorOuterTwinRustAsync_One
    extends CustomNestedErrorOuterTwinRustAsync {
  const factory CustomNestedErrorOuterTwinRustAsync_One(final String field0) =
      _$CustomNestedErrorOuterTwinRustAsync_OneImpl;
  const CustomNestedErrorOuterTwinRustAsync_One._() : super._();

  factory CustomNestedErrorOuterTwinRustAsync_One.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedErrorOuterTwinRustAsync_OneImpl.fromJson;

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedErrorOuterTwinRustAsync_OneImplCopyWith<
          _$CustomNestedErrorOuterTwinRustAsync_OneImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWith<$Res> {
  factory _$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWith(
          _$CustomNestedErrorOuterTwinRustAsync_TwoImpl value,
          $Res Function(_$CustomNestedErrorOuterTwinRustAsync_TwoImpl) then) =
      __$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({CustomNestedErrorInnerTwinRustAsync field0});

  $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> get field0;
}

/// @nodoc
class __$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWithImpl<$Res>
    extends _$CustomNestedErrorOuterTwinRustAsyncCopyWithImpl<$Res,
        _$CustomNestedErrorOuterTwinRustAsync_TwoImpl>
    implements _$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWith<$Res> {
  __$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWithImpl(
      _$CustomNestedErrorOuterTwinRustAsync_TwoImpl _value,
      $Res Function(_$CustomNestedErrorOuterTwinRustAsync_TwoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$CustomNestedErrorOuterTwinRustAsync_TwoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CustomNestedErrorInnerTwinRustAsync,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res> get field0 {
    return $CustomNestedErrorInnerTwinRustAsyncCopyWith<$Res>(_value.field0,
        (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc
@JsonSerializable()
class _$CustomNestedErrorOuterTwinRustAsync_TwoImpl
    extends CustomNestedErrorOuterTwinRustAsync_Two {
  const _$CustomNestedErrorOuterTwinRustAsync_TwoImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'two',
        super._();

  factory _$CustomNestedErrorOuterTwinRustAsync_TwoImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$CustomNestedErrorOuterTwinRustAsync_TwoImplFromJson(json);

  @override
  final CustomNestedErrorInnerTwinRustAsync field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'CustomNestedErrorOuterTwinRustAsync.two(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CustomNestedErrorOuterTwinRustAsync_TwoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWith<
          _$CustomNestedErrorOuterTwinRustAsync_TwoImpl>
      get copyWith =>
          __$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWithImpl<
              _$CustomNestedErrorOuterTwinRustAsync_TwoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) one,
    required TResult Function(CustomNestedErrorInnerTwinRustAsync field0) two,
  }) {
    return two(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? one,
    TResult? Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
  }) {
    return two?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? one,
    TResult Function(CustomNestedErrorInnerTwinRustAsync field0)? two,
    required TResult orElse(),
  }) {
    if (two != null) {
      return two(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)
        one,
    required TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)
        two,
  }) {
    return two(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult? Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
  }) {
    return two?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CustomNestedErrorOuterTwinRustAsync_One value)? one,
    TResult Function(CustomNestedErrorOuterTwinRustAsync_Two value)? two,
    required TResult orElse(),
  }) {
    if (two != null) {
      return two(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$CustomNestedErrorOuterTwinRustAsync_TwoImplToJson(
      this,
    );
  }
}

abstract class CustomNestedErrorOuterTwinRustAsync_Two
    extends CustomNestedErrorOuterTwinRustAsync {
  const factory CustomNestedErrorOuterTwinRustAsync_Two(
          final CustomNestedErrorInnerTwinRustAsync field0) =
      _$CustomNestedErrorOuterTwinRustAsync_TwoImpl;
  const CustomNestedErrorOuterTwinRustAsync_Two._() : super._();

  factory CustomNestedErrorOuterTwinRustAsync_Two.fromJson(
          Map<String, dynamic> json) =
      _$CustomNestedErrorOuterTwinRustAsync_TwoImpl.fromJson;

  @override
  CustomNestedErrorInnerTwinRustAsync get field0;
  @JsonKey(ignore: true)
  _$$CustomNestedErrorOuterTwinRustAsync_TwoImplCopyWith<
          _$CustomNestedErrorOuterTwinRustAsync_TwoImpl>
      get copyWith => throw _privateConstructorUsedError;
}
