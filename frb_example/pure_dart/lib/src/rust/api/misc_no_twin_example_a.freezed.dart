// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_no_twin_example_a.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

MyEnumWithJsonSerializableTwinNormal
    _$MyEnumWithJsonSerializableTwinNormalFromJson(Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'apple':
      return MyEnumWithJsonSerializableTwinNormal_Apple.fromJson(json);
    case 'orange':
      return MyEnumWithJsonSerializableTwinNormal_Orange.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'MyEnumWithJsonSerializableTwinNormal',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$MyEnumWithJsonSerializableTwinNormal {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) apple,
    required TResult Function(int a) orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? apple,
    TResult? Function(int a)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? apple,
    TResult Function(int a)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)
        apple,
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)
        orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Orange value)?
        orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Serializes this MyEnumWithJsonSerializableTwinNormal to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyEnumWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory $MyEnumWithJsonSerializableTwinNormalCopyWith(
          MyEnumWithJsonSerializableTwinNormal value,
          $Res Function(MyEnumWithJsonSerializableTwinNormal) then) =
      _$MyEnumWithJsonSerializableTwinNormalCopyWithImpl<$Res,
          MyEnumWithJsonSerializableTwinNormal>;
}

/// @nodoc
class _$MyEnumWithJsonSerializableTwinNormalCopyWithImpl<$Res,
        $Val extends MyEnumWithJsonSerializableTwinNormal>
    implements $MyEnumWithJsonSerializableTwinNormalCopyWith<$Res> {
  _$MyEnumWithJsonSerializableTwinNormalCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWith<$Res> {
  factory _$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWith(
          _$MyEnumWithJsonSerializableTwinNormal_AppleImpl value,
          $Res Function(_$MyEnumWithJsonSerializableTwinNormal_AppleImpl)
              then) =
      __$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWithImpl<$Res>
    extends _$MyEnumWithJsonSerializableTwinNormalCopyWithImpl<$Res,
        _$MyEnumWithJsonSerializableTwinNormal_AppleImpl>
    implements _$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWith<$Res> {
  __$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWithImpl(
      _$MyEnumWithJsonSerializableTwinNormal_AppleImpl _value,
      $Res Function(_$MyEnumWithJsonSerializableTwinNormal_AppleImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnumWithJsonSerializableTwinNormal_AppleImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyEnumWithJsonSerializableTwinNormal_AppleImpl
    extends MyEnumWithJsonSerializableTwinNormal_Apple {
  const _$MyEnumWithJsonSerializableTwinNormal_AppleImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'apple',
        super._();

  factory _$MyEnumWithJsonSerializableTwinNormal_AppleImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyEnumWithJsonSerializableTwinNormal_AppleImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MyEnumWithJsonSerializableTwinNormal.apple(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumWithJsonSerializableTwinNormal_AppleImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWith<
          _$MyEnumWithJsonSerializableTwinNormal_AppleImpl>
      get copyWith =>
          __$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWithImpl<
                  _$MyEnumWithJsonSerializableTwinNormal_AppleImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) apple,
    required TResult Function(int a) orange,
  }) {
    return apple(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? apple,
    TResult? Function(int a)? orange,
  }) {
    return apple?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? apple,
    TResult Function(int a)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)
        apple,
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)
        orange,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Orange value)?
        orange,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MyEnumWithJsonSerializableTwinNormal_AppleImplToJson(
      this,
    );
  }
}

abstract class MyEnumWithJsonSerializableTwinNormal_Apple
    extends MyEnumWithJsonSerializableTwinNormal {
  const factory MyEnumWithJsonSerializableTwinNormal_Apple(
      final String field0) = _$MyEnumWithJsonSerializableTwinNormal_AppleImpl;
  const MyEnumWithJsonSerializableTwinNormal_Apple._() : super._();

  factory MyEnumWithJsonSerializableTwinNormal_Apple.fromJson(
          Map<String, dynamic> json) =
      _$MyEnumWithJsonSerializableTwinNormal_AppleImpl.fromJson;

  String get field0;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyEnumWithJsonSerializableTwinNormal_AppleImplCopyWith<
          _$MyEnumWithJsonSerializableTwinNormal_AppleImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWith<
    $Res> {
  factory _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWith(
          _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl value,
          $Res Function(_$MyEnumWithJsonSerializableTwinNormal_OrangeImpl)
              then) =
      __$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int a});
}

/// @nodoc
class __$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWithImpl<$Res>
    extends _$MyEnumWithJsonSerializableTwinNormalCopyWithImpl<$Res,
        _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl>
    implements
        _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWith<$Res> {
  __$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWithImpl(
      _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl _value,
      $Res Function(_$MyEnumWithJsonSerializableTwinNormal_OrangeImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? a = null,
  }) {
    return _then(_$MyEnumWithJsonSerializableTwinNormal_OrangeImpl(
      a: null == a
          ? _value.a
          : a // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl
    extends MyEnumWithJsonSerializableTwinNormal_Orange {
  const _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl(
      {required this.a, final String? $type})
      : $type = $type ?? 'orange',
        super._();

  factory _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplFromJson(json);

  @override
  final int a;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MyEnumWithJsonSerializableTwinNormal.orange(a: $a)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl &&
            (identical(other.a, a) || other.a == a));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, a);

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWith<
          _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl>
      get copyWith =>
          __$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWithImpl<
                  _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) apple,
    required TResult Function(int a) orange,
  }) {
    return orange(a);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? apple,
    TResult? Function(int a)? orange,
  }) {
    return orange?.call(a);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? apple,
    TResult Function(int a)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(a);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)
        apple,
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)
        orange,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Orange value)?
        orange,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplToJson(
      this,
    );
  }
}

abstract class MyEnumWithJsonSerializableTwinNormal_Orange
    extends MyEnumWithJsonSerializableTwinNormal {
  const factory MyEnumWithJsonSerializableTwinNormal_Orange(
          {required final int a}) =
      _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl;
  const MyEnumWithJsonSerializableTwinNormal_Orange._() : super._();

  factory MyEnumWithJsonSerializableTwinNormal_Orange.fromJson(
          Map<String, dynamic> json) =
      _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl.fromJson;

  int get a;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyEnumWithJsonSerializableTwinNormal_OrangeImplCopyWith<
          _$MyEnumWithJsonSerializableTwinNormal_OrangeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

MyStructWithJsonSerializableTwinNormal
    _$MyStructWithJsonSerializableTwinNormalFromJson(
        Map<String, dynamic> json) {
  return _MyStructWithJsonSerializableTwinNormal.fromJson(json);
}

/// @nodoc
mixin _$MyStructWithJsonSerializableTwinNormal {
  String get fieldOne => throw _privateConstructorUsedError;

  /// Serializes this MyStructWithJsonSerializableTwinNormal to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $MyStructWithJsonSerializableTwinNormalCopyWith<
          MyStructWithJsonSerializableTwinNormal>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory $MyStructWithJsonSerializableTwinNormalCopyWith(
          MyStructWithJsonSerializableTwinNormal value,
          $Res Function(MyStructWithJsonSerializableTwinNormal) then) =
      _$MyStructWithJsonSerializableTwinNormalCopyWithImpl<$Res,
          MyStructWithJsonSerializableTwinNormal>;
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class _$MyStructWithJsonSerializableTwinNormalCopyWithImpl<$Res,
        $Val extends MyStructWithJsonSerializableTwinNormal>
    implements $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  _$MyStructWithJsonSerializableTwinNormalCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fieldOne = null,
  }) {
    return _then(_value.copyWith(
      fieldOne: null == fieldOne
          ? _value.fieldOne
          : fieldOne // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MyStructWithJsonSerializableTwinNormalImplCopyWith<$Res>
    implements $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory _$$MyStructWithJsonSerializableTwinNormalImplCopyWith(
          _$MyStructWithJsonSerializableTwinNormalImpl value,
          $Res Function(_$MyStructWithJsonSerializableTwinNormalImpl) then) =
      __$$MyStructWithJsonSerializableTwinNormalImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class __$$MyStructWithJsonSerializableTwinNormalImplCopyWithImpl<$Res>
    extends _$MyStructWithJsonSerializableTwinNormalCopyWithImpl<$Res,
        _$MyStructWithJsonSerializableTwinNormalImpl>
    implements _$$MyStructWithJsonSerializableTwinNormalImplCopyWith<$Res> {
  __$$MyStructWithJsonSerializableTwinNormalImplCopyWithImpl(
      _$MyStructWithJsonSerializableTwinNormalImpl _value,
      $Res Function(_$MyStructWithJsonSerializableTwinNormalImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fieldOne = null,
  }) {
    return _then(_$MyStructWithJsonSerializableTwinNormalImpl(
      fieldOne: null == fieldOne
          ? _value.fieldOne
          : fieldOne // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyStructWithJsonSerializableTwinNormalImpl
    extends _MyStructWithJsonSerializableTwinNormal {
  const _$MyStructWithJsonSerializableTwinNormalImpl({required this.fieldOne})
      : super._();

  factory _$MyStructWithJsonSerializableTwinNormalImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyStructWithJsonSerializableTwinNormalImplFromJson(json);

  @override
  final String fieldOne;

  @override
  String toString() {
    return 'MyStructWithJsonSerializableTwinNormal(fieldOne: $fieldOne)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyStructWithJsonSerializableTwinNormalImpl &&
            (identical(other.fieldOne, fieldOne) ||
                other.fieldOne == fieldOne));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, fieldOne);

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyStructWithJsonSerializableTwinNormalImplCopyWith<
          _$MyStructWithJsonSerializableTwinNormalImpl>
      get copyWith =>
          __$$MyStructWithJsonSerializableTwinNormalImplCopyWithImpl<
              _$MyStructWithJsonSerializableTwinNormalImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$MyStructWithJsonSerializableTwinNormalImplToJson(
      this,
    );
  }
}

abstract class _MyStructWithJsonSerializableTwinNormal
    extends MyStructWithJsonSerializableTwinNormal {
  const factory _MyStructWithJsonSerializableTwinNormal(
          {required final String fieldOne}) =
      _$MyStructWithJsonSerializableTwinNormalImpl;
  const _MyStructWithJsonSerializableTwinNormal._() : super._();

  factory _MyStructWithJsonSerializableTwinNormal.fromJson(
          Map<String, dynamic> json) =
      _$MyStructWithJsonSerializableTwinNormalImpl.fromJson;

  @override
  String get fieldOne;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyStructWithJsonSerializableTwinNormalImplCopyWith<
          _$MyStructWithJsonSerializableTwinNormalImpl>
      get copyWith => throw _privateConstructorUsedError;
}
