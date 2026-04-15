// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'misc_no_twin_example_a.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
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
  /// Serializes this MyEnumWithJsonSerializableTwinNormal to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyEnumWithJsonSerializableTwinNormal);
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'MyEnumWithJsonSerializableTwinNormal()';
  }
}

/// @nodoc
class $MyEnumWithJsonSerializableTwinNormalCopyWith<$Res> {
  $MyEnumWithJsonSerializableTwinNormalCopyWith(
      MyEnumWithJsonSerializableTwinNormal _,
      $Res Function(MyEnumWithJsonSerializableTwinNormal) __);
}

/// Adds pattern-matching-related methods to [MyEnumWithJsonSerializableTwinNormal].
extension MyEnumWithJsonSerializableTwinNormalPatterns
    on MyEnumWithJsonSerializableTwinNormal {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple() when apple != null:
        return apple(_that);
      case MyEnumWithJsonSerializableTwinNormal_Orange() when orange != null:
        return orange(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Apple value)
        apple,
    required TResult Function(MyEnumWithJsonSerializableTwinNormal_Orange value)
        orange,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple():
        return apple(_that);
      case MyEnumWithJsonSerializableTwinNormal_Orange():
        return orange(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Apple value)? apple,
    TResult? Function(MyEnumWithJsonSerializableTwinNormal_Orange value)?
        orange,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple() when apple != null:
        return apple(_that);
      case MyEnumWithJsonSerializableTwinNormal_Orange() when orange != null:
        return orange(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? apple,
    TResult Function(int a)? orange,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple() when apple != null:
        return apple(_that.field0);
      case MyEnumWithJsonSerializableTwinNormal_Orange() when orange != null:
        return orange(_that.a);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) apple,
    required TResult Function(int a) orange,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple():
        return apple(_that.field0);
      case MyEnumWithJsonSerializableTwinNormal_Orange():
        return orange(_that.a);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? apple,
    TResult? Function(int a)? orange,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithJsonSerializableTwinNormal_Apple() when apple != null:
        return apple(_that.field0);
      case MyEnumWithJsonSerializableTwinNormal_Orange() when orange != null:
        return orange(_that.a);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class MyEnumWithJsonSerializableTwinNormal_Apple
    extends MyEnumWithJsonSerializableTwinNormal {
  const MyEnumWithJsonSerializableTwinNormal_Apple(this.field0,
      {final String? $type})
      : $type = $type ?? 'apple',
        super._();
  factory MyEnumWithJsonSerializableTwinNormal_Apple.fromJson(
          Map<String, dynamic> json) =>
      _$MyEnumWithJsonSerializableTwinNormal_AppleFromJson(json);

  final String field0;

  @JsonKey(name: 'runtimeType')
  final String $type;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyEnumWithJsonSerializableTwinNormal_AppleCopyWith<
          MyEnumWithJsonSerializableTwinNormal_Apple>
      get copyWith => _$MyEnumWithJsonSerializableTwinNormal_AppleCopyWithImpl<
          MyEnumWithJsonSerializableTwinNormal_Apple>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$MyEnumWithJsonSerializableTwinNormal_AppleToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyEnumWithJsonSerializableTwinNormal_Apple &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MyEnumWithJsonSerializableTwinNormal.apple(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MyEnumWithJsonSerializableTwinNormal_AppleCopyWith<$Res>
    implements $MyEnumWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory $MyEnumWithJsonSerializableTwinNormal_AppleCopyWith(
          MyEnumWithJsonSerializableTwinNormal_Apple value,
          $Res Function(MyEnumWithJsonSerializableTwinNormal_Apple) _then) =
      _$MyEnumWithJsonSerializableTwinNormal_AppleCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$MyEnumWithJsonSerializableTwinNormal_AppleCopyWithImpl<$Res>
    implements $MyEnumWithJsonSerializableTwinNormal_AppleCopyWith<$Res> {
  _$MyEnumWithJsonSerializableTwinNormal_AppleCopyWithImpl(
      this._self, this._then);

  final MyEnumWithJsonSerializableTwinNormal_Apple _self;
  final $Res Function(MyEnumWithJsonSerializableTwinNormal_Apple) _then;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MyEnumWithJsonSerializableTwinNormal_Apple(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class MyEnumWithJsonSerializableTwinNormal_Orange
    extends MyEnumWithJsonSerializableTwinNormal {
  const MyEnumWithJsonSerializableTwinNormal_Orange(
      {required this.a, final String? $type})
      : $type = $type ?? 'orange',
        super._();
  factory MyEnumWithJsonSerializableTwinNormal_Orange.fromJson(
          Map<String, dynamic> json) =>
      _$MyEnumWithJsonSerializableTwinNormal_OrangeFromJson(json);

  final int a;

  @JsonKey(name: 'runtimeType')
  final String $type;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyEnumWithJsonSerializableTwinNormal_OrangeCopyWith<
          MyEnumWithJsonSerializableTwinNormal_Orange>
      get copyWith => _$MyEnumWithJsonSerializableTwinNormal_OrangeCopyWithImpl<
          MyEnumWithJsonSerializableTwinNormal_Orange>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$MyEnumWithJsonSerializableTwinNormal_OrangeToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyEnumWithJsonSerializableTwinNormal_Orange &&
            (identical(other.a, a) || other.a == a));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, a);

  @override
  String toString() {
    return 'MyEnumWithJsonSerializableTwinNormal.orange(a: $a)';
  }
}

/// @nodoc
abstract mixin class $MyEnumWithJsonSerializableTwinNormal_OrangeCopyWith<$Res>
    implements $MyEnumWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory $MyEnumWithJsonSerializableTwinNormal_OrangeCopyWith(
          MyEnumWithJsonSerializableTwinNormal_Orange value,
          $Res Function(MyEnumWithJsonSerializableTwinNormal_Orange) _then) =
      _$MyEnumWithJsonSerializableTwinNormal_OrangeCopyWithImpl;
  @useResult
  $Res call({int a});
}

/// @nodoc
class _$MyEnumWithJsonSerializableTwinNormal_OrangeCopyWithImpl<$Res>
    implements $MyEnumWithJsonSerializableTwinNormal_OrangeCopyWith<$Res> {
  _$MyEnumWithJsonSerializableTwinNormal_OrangeCopyWithImpl(
      this._self, this._then);

  final MyEnumWithJsonSerializableTwinNormal_Orange _self;
  final $Res Function(MyEnumWithJsonSerializableTwinNormal_Orange) _then;

  /// Create a copy of MyEnumWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? a = null,
  }) {
    return _then(MyEnumWithJsonSerializableTwinNormal_Orange(
      a: null == a
          ? _self.a
          : a // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
mixin _$MyEnumWithoutFnWithUnignoreTwinNormal {
  String get field0;

  /// Create a copy of MyEnumWithoutFnWithUnignoreTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyEnumWithoutFnWithUnignoreTwinNormalCopyWith<
          MyEnumWithoutFnWithUnignoreTwinNormal>
      get copyWith => _$MyEnumWithoutFnWithUnignoreTwinNormalCopyWithImpl<
              MyEnumWithoutFnWithUnignoreTwinNormal>(
          this as MyEnumWithoutFnWithUnignoreTwinNormal, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyEnumWithoutFnWithUnignoreTwinNormal &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MyEnumWithoutFnWithUnignoreTwinNormal(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MyEnumWithoutFnWithUnignoreTwinNormalCopyWith<$Res> {
  factory $MyEnumWithoutFnWithUnignoreTwinNormalCopyWith(
          MyEnumWithoutFnWithUnignoreTwinNormal value,
          $Res Function(MyEnumWithoutFnWithUnignoreTwinNormal) _then) =
      _$MyEnumWithoutFnWithUnignoreTwinNormalCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$MyEnumWithoutFnWithUnignoreTwinNormalCopyWithImpl<$Res>
    implements $MyEnumWithoutFnWithUnignoreTwinNormalCopyWith<$Res> {
  _$MyEnumWithoutFnWithUnignoreTwinNormalCopyWithImpl(this._self, this._then);

  final MyEnumWithoutFnWithUnignoreTwinNormal _self;
  final $Res Function(MyEnumWithoutFnWithUnignoreTwinNormal) _then;

  /// Create a copy of MyEnumWithoutFnWithUnignoreTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_self.copyWith(
      field0: null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// Adds pattern-matching-related methods to [MyEnumWithoutFnWithUnignoreTwinNormal].
extension MyEnumWithoutFnWithUnignoreTwinNormalPatterns
    on MyEnumWithoutFnWithUnignoreTwinNormal {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MyEnumWithoutFnWithUnignoreTwinNormal_One value)? one,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One() when one != null:
        return one(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MyEnumWithoutFnWithUnignoreTwinNormal_One value)
        one,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One():
        return one(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MyEnumWithoutFnWithUnignoreTwinNormal_One value)? one,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One() when one != null:
        return one(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? one,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One() when one != null:
        return one(_that.field0);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) one,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One():
        return one(_that.field0);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? one,
  }) {
    final _that = this;
    switch (_that) {
      case MyEnumWithoutFnWithUnignoreTwinNormal_One() when one != null:
        return one(_that.field0);
      case _:
        return null;
    }
  }
}

/// @nodoc

class MyEnumWithoutFnWithUnignoreTwinNormal_One
    extends MyEnumWithoutFnWithUnignoreTwinNormal {
  const MyEnumWithoutFnWithUnignoreTwinNormal_One(this.field0) : super._();

  @override
  final String field0;

  /// Create a copy of MyEnumWithoutFnWithUnignoreTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWith<
          MyEnumWithoutFnWithUnignoreTwinNormal_One>
      get copyWith => _$MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWithImpl<
          MyEnumWithoutFnWithUnignoreTwinNormal_One>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyEnumWithoutFnWithUnignoreTwinNormal_One &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'MyEnumWithoutFnWithUnignoreTwinNormal.one(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWith<$Res>
    implements $MyEnumWithoutFnWithUnignoreTwinNormalCopyWith<$Res> {
  factory $MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWith(
          MyEnumWithoutFnWithUnignoreTwinNormal_One value,
          $Res Function(MyEnumWithoutFnWithUnignoreTwinNormal_One) _then) =
      _$MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWithImpl;
  @override
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWithImpl<$Res>
    implements $MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWith<$Res> {
  _$MyEnumWithoutFnWithUnignoreTwinNormal_OneCopyWithImpl(
      this._self, this._then);

  final MyEnumWithoutFnWithUnignoreTwinNormal_One _self;
  final $Res Function(MyEnumWithoutFnWithUnignoreTwinNormal_One) _then;

  /// Create a copy of MyEnumWithoutFnWithUnignoreTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(MyEnumWithoutFnWithUnignoreTwinNormal_One(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$MyStructWithJsonSerializableTwinNormal {
  String get fieldOne;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyStructWithJsonSerializableTwinNormalCopyWith<
          MyStructWithJsonSerializableTwinNormal>
      get copyWith => _$MyStructWithJsonSerializableTwinNormalCopyWithImpl<
              MyStructWithJsonSerializableTwinNormal>(
          this as MyStructWithJsonSerializableTwinNormal, _$identity);

  /// Serializes this MyStructWithJsonSerializableTwinNormal to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MyStructWithJsonSerializableTwinNormal &&
            (identical(other.fieldOne, fieldOne) ||
                other.fieldOne == fieldOne));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, fieldOne);

  @override
  String toString() {
    return 'MyStructWithJsonSerializableTwinNormal(fieldOne: $fieldOne)';
  }
}

/// @nodoc
abstract mixin class $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory $MyStructWithJsonSerializableTwinNormalCopyWith(
          MyStructWithJsonSerializableTwinNormal value,
          $Res Function(MyStructWithJsonSerializableTwinNormal) _then) =
      _$MyStructWithJsonSerializableTwinNormalCopyWithImpl;
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class _$MyStructWithJsonSerializableTwinNormalCopyWithImpl<$Res>
    implements $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  _$MyStructWithJsonSerializableTwinNormalCopyWithImpl(this._self, this._then);

  final MyStructWithJsonSerializableTwinNormal _self;
  final $Res Function(MyStructWithJsonSerializableTwinNormal) _then;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? fieldOne = null,
  }) {
    return _then(_self.copyWith(
      fieldOne: null == fieldOne
          ? _self.fieldOne
          : fieldOne // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// Adds pattern-matching-related methods to [MyStructWithJsonSerializableTwinNormal].
extension MyStructWithJsonSerializableTwinNormalPatterns
    on MyStructWithJsonSerializableTwinNormal {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_MyStructWithJsonSerializableTwinNormal value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal() when $default != null:
        return $default(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(_MyStructWithJsonSerializableTwinNormal value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal():
        return $default(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_MyStructWithJsonSerializableTwinNormal value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal() when $default != null:
        return $default(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(String fieldOne)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal() when $default != null:
        return $default(_that.fieldOne);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(String fieldOne) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal():
        return $default(_that.fieldOne);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(String fieldOne)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithJsonSerializableTwinNormal() when $default != null:
        return $default(_that.fieldOne);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class _MyStructWithJsonSerializableTwinNormal
    extends MyStructWithJsonSerializableTwinNormal {
  const _MyStructWithJsonSerializableTwinNormal({required this.fieldOne})
      : super._();
  factory _MyStructWithJsonSerializableTwinNormal.fromJson(
          Map<String, dynamic> json) =>
      _$MyStructWithJsonSerializableTwinNormalFromJson(json);

  @override
  final String fieldOne;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$MyStructWithJsonSerializableTwinNormalCopyWith<
          _MyStructWithJsonSerializableTwinNormal>
      get copyWith => __$MyStructWithJsonSerializableTwinNormalCopyWithImpl<
          _MyStructWithJsonSerializableTwinNormal>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$MyStructWithJsonSerializableTwinNormalToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _MyStructWithJsonSerializableTwinNormal &&
            (identical(other.fieldOne, fieldOne) ||
                other.fieldOne == fieldOne));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, fieldOne);

  @override
  String toString() {
    return 'MyStructWithJsonSerializableTwinNormal(fieldOne: $fieldOne)';
  }
}

/// @nodoc
abstract mixin class _$MyStructWithJsonSerializableTwinNormalCopyWith<$Res>
    implements $MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  factory _$MyStructWithJsonSerializableTwinNormalCopyWith(
          _MyStructWithJsonSerializableTwinNormal value,
          $Res Function(_MyStructWithJsonSerializableTwinNormal) _then) =
      __$MyStructWithJsonSerializableTwinNormalCopyWithImpl;
  @override
  @useResult
  $Res call({String fieldOne});
}

/// @nodoc
class __$MyStructWithJsonSerializableTwinNormalCopyWithImpl<$Res>
    implements _$MyStructWithJsonSerializableTwinNormalCopyWith<$Res> {
  __$MyStructWithJsonSerializableTwinNormalCopyWithImpl(this._self, this._then);

  final _MyStructWithJsonSerializableTwinNormal _self;
  final $Res Function(_MyStructWithJsonSerializableTwinNormal) _then;

  /// Create a copy of MyStructWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? fieldOne = null,
  }) {
    return _then(_MyStructWithJsonSerializableTwinNormal(
      fieldOne: null == fieldOne
          ? _self.fieldOne
          : fieldOne // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal {
  String get a;

  /// Create a copy of MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
          MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal>
      get copyWith =>
          _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl<
                  MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal>(
              this
                  as MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal,
              _$identity);

  /// Serializes this MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other
                is MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal &&
            (identical(other.a, a) || other.a == a));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, a);

  @override
  String toString() {
    return 'MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal(a: $a)';
  }
}

/// @nodoc
abstract mixin class $MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
    $Res> {
  factory $MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith(
          MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal value,
          $Res Function(
                  MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal)
              _then) =
      _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl;
  @useResult
  $Res call({String a});
}

/// @nodoc
class _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl<
        $Res>
    implements
        $MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
            $Res> {
  _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl(
      this._self, this._then);

  final MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal _self;
  final $Res Function(
      MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal) _then;

  /// Create a copy of MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? a = null,
  }) {
    return _then(_self.copyWith(
      a: null == a
          ? _self.a
          : a // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// Adds pattern-matching-related methods to [MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal].
extension MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalPatterns
    on MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(
            _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal value)?
        $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal()
          when $default != null:
        return $default(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(
            _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal value)
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal():
        return $default(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(
            _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal value)?
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal()
          when $default != null:
        return $default(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(String a)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal()
          when $default != null:
        return $default(_that.a);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(String a) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal():
        return $default(_that.a);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(String a)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal()
          when $default != null:
        return $default(_that.a);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
    implements MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal {
  const _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal(
      {required this.a});
  factory _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal.fromJson(
          Map<String, dynamic> json) =>
      _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalFromJson(
          json);

  @override
  final String a;

  /// Create a copy of MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
          _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal>
      get copyWith =>
          __$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl<
                  _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal>(
              this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other
                is _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal &&
            (identical(other.a, a) || other.a == a));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, a);

  @override
  String toString() {
    return 'MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal(a: $a)';
  }
}

/// @nodoc
abstract mixin class _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
        $Res>
    implements
        $MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
            $Res> {
  factory _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith(
          _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal value,
          $Res Function(
                  _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal)
              _then) =
      __$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl;
  @override
  @useResult
  $Res call({String a});
}

/// @nodoc
class __$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl<
        $Res>
    implements
        _$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWith<
            $Res> {
  __$MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormalCopyWithImpl(
      this._self, this._then);

  final _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal _self;
  final $Res Function(
      _MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal) _then;

  /// Create a copy of MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? a = null,
  }) {
    return _then(_MyStructWithoutFnWithUnignoreWithJsonSerializableTwinNormal(
      a: null == a
          ? _self.a
          : a // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
