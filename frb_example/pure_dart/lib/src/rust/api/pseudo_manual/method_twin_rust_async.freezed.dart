// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'method_twin_rust_async.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$SimpleEnumTwinRustAsync {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is SimpleEnumTwinRustAsync);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinRustAsync()';
  }
}

/// @nodoc
class $SimpleEnumTwinRustAsyncCopyWith<$Res> {
  $SimpleEnumTwinRustAsyncCopyWith(
      SimpleEnumTwinRustAsync _, $Res Function(SimpleEnumTwinRustAsync) __);
}

/// @nodoc

class SimpleEnumTwinRustAsync_First extends SimpleEnumTwinRustAsync {
  const SimpleEnumTwinRustAsync_First() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinRustAsync_First);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'SimpleEnumTwinRustAsync.first()';
  }
}

/// @nodoc

class SimpleEnumTwinRustAsync_Second extends SimpleEnumTwinRustAsync {
  const SimpleEnumTwinRustAsync_Second(this.field0) : super._();

  final String field0;

  /// Create a copy of SimpleEnumTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $SimpleEnumTwinRustAsync_SecondCopyWith<SimpleEnumTwinRustAsync_Second>
      get copyWith => _$SimpleEnumTwinRustAsync_SecondCopyWithImpl<
          SimpleEnumTwinRustAsync_Second>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is SimpleEnumTwinRustAsync_Second &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'SimpleEnumTwinRustAsync.second(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $SimpleEnumTwinRustAsync_SecondCopyWith<$Res>
    implements $SimpleEnumTwinRustAsyncCopyWith<$Res> {
  factory $SimpleEnumTwinRustAsync_SecondCopyWith(
          SimpleEnumTwinRustAsync_Second value,
          $Res Function(SimpleEnumTwinRustAsync_Second) _then) =
      _$SimpleEnumTwinRustAsync_SecondCopyWithImpl;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class _$SimpleEnumTwinRustAsync_SecondCopyWithImpl<$Res>
    implements $SimpleEnumTwinRustAsync_SecondCopyWith<$Res> {
  _$SimpleEnumTwinRustAsync_SecondCopyWithImpl(this._self, this._then);

  final SimpleEnumTwinRustAsync_Second _self;
  final $Res Function(SimpleEnumTwinRustAsync_Second) _then;

  /// Create a copy of SimpleEnumTwinRustAsync
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(SimpleEnumTwinRustAsync_Second(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
