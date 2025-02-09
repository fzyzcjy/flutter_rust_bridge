// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'minimal.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

MyEnumWithJsonSerializable _$MyEnumWithJsonSerializableFromJson(
    Map<String, dynamic> json) {
  switch (json['runtimeType']) {
    case 'apple':
      return MyEnumWithJsonSerializable_Apple.fromJson(json);
    case 'orange':
      return MyEnumWithJsonSerializable_Orange.fromJson(json);

    default:
      throw CheckedFromJsonException(
          json,
          'runtimeType',
          'MyEnumWithJsonSerializable',
          'Invalid union type "${json['runtimeType']}"!');
  }
}

/// @nodoc
mixin _$MyEnumWithJsonSerializable {
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
    required TResult Function(MyEnumWithJsonSerializable_Apple value) apple,
    required TResult Function(MyEnumWithJsonSerializable_Orange value) orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializable_Orange value)? orange,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializable_Orange value)? orange,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Serializes this MyEnumWithJsonSerializable to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyEnumWithJsonSerializableCopyWith<$Res> {
  factory $MyEnumWithJsonSerializableCopyWith(MyEnumWithJsonSerializable value,
          $Res Function(MyEnumWithJsonSerializable) then) =
      _$MyEnumWithJsonSerializableCopyWithImpl<$Res,
          MyEnumWithJsonSerializable>;
}

/// @nodoc
class _$MyEnumWithJsonSerializableCopyWithImpl<$Res,
        $Val extends MyEnumWithJsonSerializable>
    implements $MyEnumWithJsonSerializableCopyWith<$Res> {
  _$MyEnumWithJsonSerializableCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$MyEnumWithJsonSerializable_AppleImplCopyWith<$Res> {
  factory _$$MyEnumWithJsonSerializable_AppleImplCopyWith(
          _$MyEnumWithJsonSerializable_AppleImpl value,
          $Res Function(_$MyEnumWithJsonSerializable_AppleImpl) then) =
      __$$MyEnumWithJsonSerializable_AppleImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$MyEnumWithJsonSerializable_AppleImplCopyWithImpl<$Res>
    extends _$MyEnumWithJsonSerializableCopyWithImpl<$Res,
        _$MyEnumWithJsonSerializable_AppleImpl>
    implements _$$MyEnumWithJsonSerializable_AppleImplCopyWith<$Res> {
  __$$MyEnumWithJsonSerializable_AppleImplCopyWithImpl(
      _$MyEnumWithJsonSerializable_AppleImpl _value,
      $Res Function(_$MyEnumWithJsonSerializable_AppleImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$MyEnumWithJsonSerializable_AppleImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyEnumWithJsonSerializable_AppleImpl
    extends MyEnumWithJsonSerializable_Apple {
  const _$MyEnumWithJsonSerializable_AppleImpl(this.field0,
      {final String? $type})
      : $type = $type ?? 'apple',
        super._();

  factory _$MyEnumWithJsonSerializable_AppleImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyEnumWithJsonSerializable_AppleImplFromJson(json);

  @override
  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MyEnumWithJsonSerializable.apple(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumWithJsonSerializable_AppleImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumWithJsonSerializable_AppleImplCopyWith<
          _$MyEnumWithJsonSerializable_AppleImpl>
      get copyWith => __$$MyEnumWithJsonSerializable_AppleImplCopyWithImpl<
          _$MyEnumWithJsonSerializable_AppleImpl>(this, _$identity);

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
    required TResult Function(MyEnumWithJsonSerializable_Apple value) apple,
    required TResult Function(MyEnumWithJsonSerializable_Orange value) orange,
  }) {
    return apple(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializable_Orange value)? orange,
  }) {
    return apple?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializable_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (apple != null) {
      return apple(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MyEnumWithJsonSerializable_AppleImplToJson(
      this,
    );
  }
}

abstract class MyEnumWithJsonSerializable_Apple
    extends MyEnumWithJsonSerializable {
  const factory MyEnumWithJsonSerializable_Apple(final String field0) =
      _$MyEnumWithJsonSerializable_AppleImpl;
  const MyEnumWithJsonSerializable_Apple._() : super._();

  factory MyEnumWithJsonSerializable_Apple.fromJson(Map<String, dynamic> json) =
      _$MyEnumWithJsonSerializable_AppleImpl.fromJson;

  String get field0;

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyEnumWithJsonSerializable_AppleImplCopyWith<
          _$MyEnumWithJsonSerializable_AppleImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MyEnumWithJsonSerializable_OrangeImplCopyWith<$Res> {
  factory _$$MyEnumWithJsonSerializable_OrangeImplCopyWith(
          _$MyEnumWithJsonSerializable_OrangeImpl value,
          $Res Function(_$MyEnumWithJsonSerializable_OrangeImpl) then) =
      __$$MyEnumWithJsonSerializable_OrangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int a});
}

/// @nodoc
class __$$MyEnumWithJsonSerializable_OrangeImplCopyWithImpl<$Res>
    extends _$MyEnumWithJsonSerializableCopyWithImpl<$Res,
        _$MyEnumWithJsonSerializable_OrangeImpl>
    implements _$$MyEnumWithJsonSerializable_OrangeImplCopyWith<$Res> {
  __$$MyEnumWithJsonSerializable_OrangeImplCopyWithImpl(
      _$MyEnumWithJsonSerializable_OrangeImpl _value,
      $Res Function(_$MyEnumWithJsonSerializable_OrangeImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? a = null,
  }) {
    return _then(_$MyEnumWithJsonSerializable_OrangeImpl(
      a: null == a
          ? _value.a
          : a // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyEnumWithJsonSerializable_OrangeImpl
    extends MyEnumWithJsonSerializable_Orange {
  const _$MyEnumWithJsonSerializable_OrangeImpl(
      {required this.a, final String? $type})
      : $type = $type ?? 'orange',
        super._();

  factory _$MyEnumWithJsonSerializable_OrangeImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyEnumWithJsonSerializable_OrangeImplFromJson(json);

  @override
  final int a;

  @JsonKey(name: 'runtimeType')
  final String $type;

  @override
  String toString() {
    return 'MyEnumWithJsonSerializable.orange(a: $a)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyEnumWithJsonSerializable_OrangeImpl &&
            (identical(other.a, a) || other.a == a));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, a);

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyEnumWithJsonSerializable_OrangeImplCopyWith<
          _$MyEnumWithJsonSerializable_OrangeImpl>
      get copyWith => __$$MyEnumWithJsonSerializable_OrangeImplCopyWithImpl<
          _$MyEnumWithJsonSerializable_OrangeImpl>(this, _$identity);

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
    required TResult Function(MyEnumWithJsonSerializable_Apple value) apple,
    required TResult Function(MyEnumWithJsonSerializable_Orange value) orange,
  }) {
    return orange(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializable_Orange value)? orange,
  }) {
    return orange?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializable_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializable_Orange value)? orange,
    required TResult orElse(),
  }) {
    if (orange != null) {
      return orange(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$MyEnumWithJsonSerializable_OrangeImplToJson(
      this,
    );
  }
}

abstract class MyEnumWithJsonSerializable_Orange
    extends MyEnumWithJsonSerializable {
  const factory MyEnumWithJsonSerializable_Orange({required final int a}) =
      _$MyEnumWithJsonSerializable_OrangeImpl;
  const MyEnumWithJsonSerializable_Orange._() : super._();

  factory MyEnumWithJsonSerializable_Orange.fromJson(
          Map<String, dynamic> json) =
      _$MyEnumWithJsonSerializable_OrangeImpl.fromJson;

  int get a;

  /// Create a copy of MyEnumWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyEnumWithJsonSerializable_OrangeImplCopyWith<
          _$MyEnumWithJsonSerializable_OrangeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

MyStructWithJsonSerializable _$MyStructWithJsonSerializableFromJson(
    Map<String, dynamic> json) {
  return _MyStructWithJsonSerializable.fromJson(json);
}

/// @nodoc
mixin _$MyStructWithJsonSerializable {
  String get fieldOne => throw _privateConstructorUsedError;

  /// Serializes this MyStructWithJsonSerializable to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of MyStructWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $MyStructWithJsonSerializableCopyWith<MyStructWithJsonSerializable>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MyStructWithJsonSerializableCopyWith<$Res> {
  factory $MyStructWithJsonSerializableCopyWith(
          MyStructWithJsonSerializable value,
          $Res Function(MyStructWithJsonSerializable) then) =
      _$MyStructWithJsonSerializableCopyWithImpl<$Res,
          MyStructWithJsonSerializable>;
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class _$MyStructWithJsonSerializableCopyWithImpl<$Res,
        $Val extends MyStructWithJsonSerializable>
    implements $MyStructWithJsonSerializableCopyWith<$Res> {
  _$MyStructWithJsonSerializableCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MyStructWithJsonSerializable
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
abstract class _$$MyStructWithJsonSerializableImplCopyWith<$Res>
    implements $MyStructWithJsonSerializableCopyWith<$Res> {
  factory _$$MyStructWithJsonSerializableImplCopyWith(
          _$MyStructWithJsonSerializableImpl value,
          $Res Function(_$MyStructWithJsonSerializableImpl) then) =
      __$$MyStructWithJsonSerializableImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class __$$MyStructWithJsonSerializableImplCopyWithImpl<$Res>
    extends _$MyStructWithJsonSerializableCopyWithImpl<$Res,
        _$MyStructWithJsonSerializableImpl>
    implements _$$MyStructWithJsonSerializableImplCopyWith<$Res> {
  __$$MyStructWithJsonSerializableImplCopyWithImpl(
      _$MyStructWithJsonSerializableImpl _value,
      $Res Function(_$MyStructWithJsonSerializableImpl) _then)
      : super(_value, _then);

  /// Create a copy of MyStructWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fieldOne = null,
  }) {
    return _then(_$MyStructWithJsonSerializableImpl(
      fieldOne: null == fieldOne
          ? _value.fieldOne
          : fieldOne // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$MyStructWithJsonSerializableImpl extends _MyStructWithJsonSerializable {
  const _$MyStructWithJsonSerializableImpl({required this.fieldOne})
      : super._();

  factory _$MyStructWithJsonSerializableImpl.fromJson(
          Map<String, dynamic> json) =>
      _$$MyStructWithJsonSerializableImplFromJson(json);

  @override
  final String fieldOne;

  @override
  String toString() {
    return 'MyStructWithJsonSerializable(fieldOne: $fieldOne)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MyStructWithJsonSerializableImpl &&
            (identical(other.fieldOne, fieldOne) ||
                other.fieldOne == fieldOne));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, fieldOne);

  /// Create a copy of MyStructWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MyStructWithJsonSerializableImplCopyWith<
          _$MyStructWithJsonSerializableImpl>
      get copyWith => __$$MyStructWithJsonSerializableImplCopyWithImpl<
          _$MyStructWithJsonSerializableImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$MyStructWithJsonSerializableImplToJson(
      this,
    );
  }
}

abstract class _MyStructWithJsonSerializable
    extends MyStructWithJsonSerializable {
  const factory _MyStructWithJsonSerializable(
      {required final String fieldOne}) = _$MyStructWithJsonSerializableImpl;
  const _MyStructWithJsonSerializable._() : super._();

  factory _MyStructWithJsonSerializable.fromJson(Map<String, dynamic> json) =
      _$MyStructWithJsonSerializableImpl.fromJson;

  @override
  String get fieldOne;

  /// Create a copy of MyStructWithJsonSerializable
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MyStructWithJsonSerializableImplCopyWith<
          _$MyStructWithJsonSerializableImpl>
      get copyWith => throw _privateConstructorUsedError;
}
