// dart format width=80
// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'minimal.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$ElementKind {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is ElementKind);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'ElementKind()';
  }
}

/// @nodoc
class $ElementKindCopyWith<$Res> {
  $ElementKindCopyWith(ElementKind _, $Res Function(ElementKind) __);
}

/// @nodoc

class ElementKind_Empty extends ElementKind {
  const ElementKind_Empty() : super._();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is ElementKind_Empty);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'ElementKind.empty()';
  }
}

/// @nodoc

class ElementKind_Occupied extends ElementKind {
  const ElementKind_Occupied(this.field0) : super._();

  final Entity field0;

  /// Create a copy of ElementKind
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $ElementKind_OccupiedCopyWith<ElementKind_Occupied> get copyWith =>
      _$ElementKind_OccupiedCopyWithImpl<ElementKind_Occupied>(
          this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ElementKind_Occupied &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @override
  String toString() {
    return 'ElementKind.occupied(field0: $field0)';
  }
}

/// @nodoc
abstract mixin class $ElementKind_OccupiedCopyWith<$Res>
    implements $ElementKindCopyWith<$Res> {
  factory $ElementKind_OccupiedCopyWith(ElementKind_Occupied value,
          $Res Function(ElementKind_Occupied) _then) =
      _$ElementKind_OccupiedCopyWithImpl;
  @useResult
  $Res call({Entity field0});
}

/// @nodoc
class _$ElementKind_OccupiedCopyWithImpl<$Res>
    implements $ElementKind_OccupiedCopyWith<$Res> {
  _$ElementKind_OccupiedCopyWithImpl(this._self, this._then);

  final ElementKind_Occupied _self;
  final $Res Function(ElementKind_Occupied) _then;

  /// Create a copy of ElementKind
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? field0 = null,
  }) {
    return _then(ElementKind_Occupied(
      null == field0
          ? _self.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Entity,
    ));
  }
}

// dart format on
