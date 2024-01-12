// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'generator_utils.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$DuplicatorMode {
  List<DuplicatorComponentMode> get components =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $DuplicatorModeCopyWith<DuplicatorMode> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DuplicatorModeCopyWith<$Res> {
  factory $DuplicatorModeCopyWith(
          DuplicatorMode value, $Res Function(DuplicatorMode) then) =
      _$DuplicatorModeCopyWithImpl<$Res, DuplicatorMode>;
  @useResult
  $Res call({List<DuplicatorComponentMode> components});
}

/// @nodoc
class _$DuplicatorModeCopyWithImpl<$Res, $Val extends DuplicatorMode>
    implements $DuplicatorModeCopyWith<$Res> {
  _$DuplicatorModeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? components = null,
  }) {
    return _then(_value.copyWith(
      components: null == components
          ? _value.components
          : components // ignore: cast_nullable_to_non_nullable
              as List<DuplicatorComponentMode>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$DuplicatorModeImplCopyWith<$Res>
    implements $DuplicatorModeCopyWith<$Res> {
  factory _$$DuplicatorModeImplCopyWith(_$DuplicatorModeImpl value,
          $Res Function(_$DuplicatorModeImpl) then) =
      __$$DuplicatorModeImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<DuplicatorComponentMode> components});
}

/// @nodoc
class __$$DuplicatorModeImplCopyWithImpl<$Res>
    extends _$DuplicatorModeCopyWithImpl<$Res, _$DuplicatorModeImpl>
    implements _$$DuplicatorModeImplCopyWith<$Res> {
  __$$DuplicatorModeImplCopyWithImpl(
      _$DuplicatorModeImpl _value, $Res Function(_$DuplicatorModeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? components = null,
  }) {
    return _then(_$DuplicatorModeImpl(
      null == components
          ? _value._components
          : components // ignore: cast_nullable_to_non_nullable
              as List<DuplicatorComponentMode>,
    ));
  }
}

/// @nodoc

class _$DuplicatorModeImpl extends _DuplicatorMode {
  const _$DuplicatorModeImpl(final List<DuplicatorComponentMode> components)
      : _components = components,
        super._();

  final List<DuplicatorComponentMode> _components;
  @override
  List<DuplicatorComponentMode> get components {
    if (_components is EqualUnmodifiableListView) return _components;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_components);
  }

  @override
  String toString() {
    return 'DuplicatorMode(components: $components)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DuplicatorModeImpl &&
            const DeepCollectionEquality()
                .equals(other._components, _components));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(_components));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DuplicatorModeImplCopyWith<_$DuplicatorModeImpl> get copyWith =>
      __$$DuplicatorModeImplCopyWithImpl<_$DuplicatorModeImpl>(
          this, _$identity);
}

abstract class _DuplicatorMode extends DuplicatorMode {
  const factory _DuplicatorMode(
      final List<DuplicatorComponentMode> components) = _$DuplicatorModeImpl;
  const _DuplicatorMode._() : super._();

  @override
  List<DuplicatorComponentMode> get components;
  @override
  @JsonKey(ignore: true)
  _$$DuplicatorModeImplCopyWith<_$DuplicatorModeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
