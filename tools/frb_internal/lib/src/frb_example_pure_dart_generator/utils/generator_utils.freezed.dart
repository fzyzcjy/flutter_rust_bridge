// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'generator_utils.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$DuplicatorMode {

 List<DuplicatorComponentMode> get components;
/// Create a copy of DuplicatorMode
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DuplicatorModeCopyWith<DuplicatorMode> get copyWith => _$DuplicatorModeCopyWithImpl<DuplicatorMode>(this as DuplicatorMode, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DuplicatorMode&&const DeepCollectionEquality().equals(other.components, components));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(components));

@override
String toString() {
  return 'DuplicatorMode(components: $components)';
}


}

/// @nodoc
abstract mixin class $DuplicatorModeCopyWith<$Res>  {
  factory $DuplicatorModeCopyWith(DuplicatorMode value, $Res Function(DuplicatorMode) _then) = _$DuplicatorModeCopyWithImpl;
@useResult
$Res call({
 List<DuplicatorComponentMode> components
});




}
/// @nodoc
class _$DuplicatorModeCopyWithImpl<$Res>
    implements $DuplicatorModeCopyWith<$Res> {
  _$DuplicatorModeCopyWithImpl(this._self, this._then);

  final DuplicatorMode _self;
  final $Res Function(DuplicatorMode) _then;

/// Create a copy of DuplicatorMode
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? components = null,}) {
  return _then(_self.copyWith(
components: null == components ? _self.components : components // ignore: cast_nullable_to_non_nullable
as List<DuplicatorComponentMode>,
  ));
}

}


/// Adds pattern-matching-related methods to [DuplicatorMode].
extension DuplicatorModePatterns on DuplicatorMode {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DuplicatorMode value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DuplicatorMode() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DuplicatorMode value)  $default,){
final _that = this;
switch (_that) {
case _DuplicatorMode():
return $default(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DuplicatorMode value)?  $default,){
final _that = this;
switch (_that) {
case _DuplicatorMode() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<DuplicatorComponentMode> components)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DuplicatorMode() when $default != null:
return $default(_that.components);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<DuplicatorComponentMode> components)  $default,) {final _that = this;
switch (_that) {
case _DuplicatorMode():
return $default(_that.components);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<DuplicatorComponentMode> components)?  $default,) {final _that = this;
switch (_that) {
case _DuplicatorMode() when $default != null:
return $default(_that.components);case _:
  return null;

}
}

}

/// @nodoc


class _DuplicatorMode extends DuplicatorMode {
  const _DuplicatorMode(final  List<DuplicatorComponentMode> components): _components = components,super._();
  

 final  List<DuplicatorComponentMode> _components;
@override List<DuplicatorComponentMode> get components {
  if (_components is EqualUnmodifiableListView) return _components;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_components);
}


/// Create a copy of DuplicatorMode
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DuplicatorModeCopyWith<_DuplicatorMode> get copyWith => __$DuplicatorModeCopyWithImpl<_DuplicatorMode>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DuplicatorMode&&const DeepCollectionEquality().equals(other._components, _components));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_components));

@override
String toString() {
  return 'DuplicatorMode(components: $components)';
}


}

/// @nodoc
abstract mixin class _$DuplicatorModeCopyWith<$Res> implements $DuplicatorModeCopyWith<$Res> {
  factory _$DuplicatorModeCopyWith(_DuplicatorMode value, $Res Function(_DuplicatorMode) _then) = __$DuplicatorModeCopyWithImpl;
@override @useResult
$Res call({
 List<DuplicatorComponentMode> components
});




}
/// @nodoc
class __$DuplicatorModeCopyWithImpl<$Res>
    implements _$DuplicatorModeCopyWith<$Res> {
  __$DuplicatorModeCopyWithImpl(this._self, this._then);

  final _DuplicatorMode _self;
  final $Res Function(_DuplicatorMode) _then;

/// Create a copy of DuplicatorMode
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? components = null,}) {
  return _then(_DuplicatorMode(
null == components ? _self._components : components // ignore: cast_nullable_to_non_nullable
as List<DuplicatorComponentMode>,
  ));
}


}

// dart format on
