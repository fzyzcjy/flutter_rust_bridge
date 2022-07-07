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
mixin _$LinkType {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() file,
    required TResult Function(bool includeAll) dir,
    required TResult Function() http,
    required TResult Function() git,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(File value) file,
    required TResult Function(Dir value) dir,
    required TResult Function(Http value) http,
    required TResult Function(Git value) git,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LinkTypeCopyWith<$Res> {
  factory $LinkTypeCopyWith(LinkType value, $Res Function(LinkType) then) = _$LinkTypeCopyWithImpl<$Res>;
}

/// @nodoc
class _$LinkTypeCopyWithImpl<$Res> implements $LinkTypeCopyWith<$Res> {
  _$LinkTypeCopyWithImpl(this._value, this._then);

  final LinkType _value;
  // ignore: unused_field
  final $Res Function(LinkType) _then;
}

/// @nodoc
abstract class _$$FileCopyWith<$Res> {
  factory _$$FileCopyWith(_$File value, $Res Function(_$File) then) = __$$FileCopyWithImpl<$Res>;
}

/// @nodoc
class __$$FileCopyWithImpl<$Res> extends _$LinkTypeCopyWithImpl<$Res> implements _$$FileCopyWith<$Res> {
  __$$FileCopyWithImpl(_$File _value, $Res Function(_$File) _then) : super(_value, (v) => _then(v as _$File));

  @override
  _$File get _value => super._value as _$File;
}

/// @nodoc

class _$File implements File {
  const _$File();

  @override
  String toString() {
    return 'LinkType.file()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$File);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() file,
    required TResult Function(bool includeAll) dir,
    required TResult Function() http,
    required TResult Function() git,
  }) {
    return file();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
  }) {
    return file?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
    required TResult orElse(),
  }) {
    if (file != null) {
      return file();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(File value) file,
    required TResult Function(Dir value) dir,
    required TResult Function(Http value) http,
    required TResult Function(Git value) git,
  }) {
    return file(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
  }) {
    return file?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
    required TResult orElse(),
  }) {
    if (file != null) {
      return file(this);
    }
    return orElse();
  }
}

abstract class File implements LinkType {
  const factory File() = _$File;
}

/// @nodoc
abstract class _$$DirCopyWith<$Res> {
  factory _$$DirCopyWith(_$Dir value, $Res Function(_$Dir) then) = __$$DirCopyWithImpl<$Res>;
  $Res call({bool includeAll});
}

/// @nodoc
class __$$DirCopyWithImpl<$Res> extends _$LinkTypeCopyWithImpl<$Res> implements _$$DirCopyWith<$Res> {
  __$$DirCopyWithImpl(_$Dir _value, $Res Function(_$Dir) _then) : super(_value, (v) => _then(v as _$Dir));

  @override
  _$Dir get _value => super._value as _$Dir;

  @override
  $Res call({
    Object? includeAll = freezed,
  }) {
    return _then(_$Dir(
      includeAll: includeAll == freezed
          ? _value.includeAll
          : includeAll // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$Dir implements Dir {
  const _$Dir({required this.includeAll});

  @override
  final bool includeAll;

  @override
  String toString() {
    return 'LinkType.dir(includeAll: $includeAll)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Dir &&
            const DeepCollectionEquality().equals(other.includeAll, includeAll));
  }

  @override
  int get hashCode => Object.hash(runtimeType, const DeepCollectionEquality().hash(includeAll));

  @JsonKey(ignore: true)
  @override
  _$$DirCopyWith<_$Dir> get copyWith => __$$DirCopyWithImpl<_$Dir>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() file,
    required TResult Function(bool includeAll) dir,
    required TResult Function() http,
    required TResult Function() git,
  }) {
    return dir(includeAll);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
  }) {
    return dir?.call(includeAll);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
    required TResult orElse(),
  }) {
    if (dir != null) {
      return dir(includeAll);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(File value) file,
    required TResult Function(Dir value) dir,
    required TResult Function(Http value) http,
    required TResult Function(Git value) git,
  }) {
    return dir(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
  }) {
    return dir?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
    required TResult orElse(),
  }) {
    if (dir != null) {
      return dir(this);
    }
    return orElse();
  }
}

abstract class Dir implements LinkType {
  const factory Dir({required final bool includeAll}) = _$Dir;

  bool get includeAll;
  @JsonKey(ignore: true)
  _$$DirCopyWith<_$Dir> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$HttpCopyWith<$Res> {
  factory _$$HttpCopyWith(_$Http value, $Res Function(_$Http) then) = __$$HttpCopyWithImpl<$Res>;
}

/// @nodoc
class __$$HttpCopyWithImpl<$Res> extends _$LinkTypeCopyWithImpl<$Res> implements _$$HttpCopyWith<$Res> {
  __$$HttpCopyWithImpl(_$Http _value, $Res Function(_$Http) _then) : super(_value, (v) => _then(v as _$Http));

  @override
  _$Http get _value => super._value as _$Http;
}

/// @nodoc

class _$Http implements Http {
  const _$Http();

  @override
  String toString() {
    return 'LinkType.http()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Http);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() file,
    required TResult Function(bool includeAll) dir,
    required TResult Function() http,
    required TResult Function() git,
  }) {
    return http();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
  }) {
    return http?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
    required TResult orElse(),
  }) {
    if (http != null) {
      return http();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(File value) file,
    required TResult Function(Dir value) dir,
    required TResult Function(Http value) http,
    required TResult Function(Git value) git,
  }) {
    return http(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
  }) {
    return http?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
    required TResult orElse(),
  }) {
    if (http != null) {
      return http(this);
    }
    return orElse();
  }
}

abstract class Http implements LinkType {
  const factory Http() = _$Http;
}

/// @nodoc
abstract class _$$GitCopyWith<$Res> {
  factory _$$GitCopyWith(_$Git value, $Res Function(_$Git) then) = __$$GitCopyWithImpl<$Res>;
}

/// @nodoc
class __$$GitCopyWithImpl<$Res> extends _$LinkTypeCopyWithImpl<$Res> implements _$$GitCopyWith<$Res> {
  __$$GitCopyWithImpl(_$Git _value, $Res Function(_$Git) _then) : super(_value, (v) => _then(v as _$Git));

  @override
  _$Git get _value => super._value as _$Git;
}

/// @nodoc

class _$Git implements Git {
  const _$Git();

  @override
  String toString() {
    return 'LinkType.git()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$Git);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() file,
    required TResult Function(bool includeAll) dir,
    required TResult Function() http,
    required TResult Function() git,
  }) {
    return git();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
  }) {
    return git?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? file,
    TResult Function(bool includeAll)? dir,
    TResult Function()? http,
    TResult Function()? git,
    required TResult orElse(),
  }) {
    if (git != null) {
      return git();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(File value) file,
    required TResult Function(Dir value) dir,
    required TResult Function(Http value) http,
    required TResult Function(Git value) git,
  }) {
    return git(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
  }) {
    return git?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(File value)? file,
    TResult Function(Dir value)? dir,
    TResult Function(Http value)? http,
    TResult Function(Git value)? git,
    required TResult orElse(),
  }) {
    if (git != null) {
      return git(this);
    }
    return orElse();
  }
}

abstract class Git implements LinkType {
  const factory Git() = _$Git;
}
