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
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$SpecificMetadata {
  int get width => throw _privateConstructorUsedError;
  int get height => throw _privateConstructorUsedError;
  RatioU32 get pixelAspect => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int width, int height, RatioU32 pixelAspect)
        image,
    required TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)
        video,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult? Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpecificMetadata_Image value) image,
    required TResult Function(SpecificMetadata_Video value) video,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpecificMetadata_Image value)? image,
    TResult? Function(SpecificMetadata_Video value)? video,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpecificMetadata_Image value)? image,
    TResult Function(SpecificMetadata_Video value)? video,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $SpecificMetadataCopyWith<SpecificMetadata> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SpecificMetadataCopyWith<$Res> {
  factory $SpecificMetadataCopyWith(
          SpecificMetadata value, $Res Function(SpecificMetadata) then) =
      _$SpecificMetadataCopyWithImpl<$Res, SpecificMetadata>;
  @useResult
  $Res call({int width, int height, RatioU32 pixelAspect});
}

/// @nodoc
class _$SpecificMetadataCopyWithImpl<$Res, $Val extends SpecificMetadata>
    implements $SpecificMetadataCopyWith<$Res> {
  _$SpecificMetadataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
    Object? pixelAspect = null,
  }) {
    return _then(_value.copyWith(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      pixelAspect: null == pixelAspect
          ? _value.pixelAspect
          : pixelAspect // ignore: cast_nullable_to_non_nullable
              as RatioU32,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SpecificMetadata_ImageImplCopyWith<$Res>
    implements $SpecificMetadataCopyWith<$Res> {
  factory _$$SpecificMetadata_ImageImplCopyWith(
          _$SpecificMetadata_ImageImpl value,
          $Res Function(_$SpecificMetadata_ImageImpl) then) =
      __$$SpecificMetadata_ImageImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int width, int height, RatioU32 pixelAspect});
}

/// @nodoc
class __$$SpecificMetadata_ImageImplCopyWithImpl<$Res>
    extends _$SpecificMetadataCopyWithImpl<$Res, _$SpecificMetadata_ImageImpl>
    implements _$$SpecificMetadata_ImageImplCopyWith<$Res> {
  __$$SpecificMetadata_ImageImplCopyWithImpl(
      _$SpecificMetadata_ImageImpl _value,
      $Res Function(_$SpecificMetadata_ImageImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
    Object? pixelAspect = null,
  }) {
    return _then(_$SpecificMetadata_ImageImpl(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      pixelAspect: null == pixelAspect
          ? _value.pixelAspect
          : pixelAspect // ignore: cast_nullable_to_non_nullable
              as RatioU32,
    ));
  }
}

/// @nodoc

class _$SpecificMetadata_ImageImpl extends SpecificMetadata_Image {
  const _$SpecificMetadata_ImageImpl(
      {required this.width, required this.height, required this.pixelAspect})
      : super._();

  @override
  final int width;
  @override
  final int height;
  @override
  final RatioU32 pixelAspect;

  @override
  String toString() {
    return 'SpecificMetadata.image(width: $width, height: $height, pixelAspect: $pixelAspect)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpecificMetadata_ImageImpl &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height) &&
            (identical(other.pixelAspect, pixelAspect) ||
                other.pixelAspect == pixelAspect));
  }

  @override
  int get hashCode => Object.hash(runtimeType, width, height, pixelAspect);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SpecificMetadata_ImageImplCopyWith<_$SpecificMetadata_ImageImpl>
      get copyWith => __$$SpecificMetadata_ImageImplCopyWithImpl<
          _$SpecificMetadata_ImageImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int width, int height, RatioU32 pixelAspect)
        image,
    required TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)
        video,
  }) {
    return image(width, height, pixelAspect);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult? Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
  }) {
    return image?.call(width, height, pixelAspect);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
    required TResult orElse(),
  }) {
    if (image != null) {
      return image(width, height, pixelAspect);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpecificMetadata_Image value) image,
    required TResult Function(SpecificMetadata_Video value) video,
  }) {
    return image(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpecificMetadata_Image value)? image,
    TResult? Function(SpecificMetadata_Video value)? video,
  }) {
    return image?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpecificMetadata_Image value)? image,
    TResult Function(SpecificMetadata_Video value)? video,
    required TResult orElse(),
  }) {
    if (image != null) {
      return image(this);
    }
    return orElse();
  }
}

abstract class SpecificMetadata_Image extends SpecificMetadata {
  const factory SpecificMetadata_Image(
      {required final int width,
      required final int height,
      required final RatioU32 pixelAspect}) = _$SpecificMetadata_ImageImpl;
  const SpecificMetadata_Image._() : super._();

  @override
  int get width;
  @override
  int get height;
  @override
  RatioU32 get pixelAspect;
  @override
  @JsonKey(ignore: true)
  _$$SpecificMetadata_ImageImplCopyWith<_$SpecificMetadata_ImageImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SpecificMetadata_VideoImplCopyWith<$Res>
    implements $SpecificMetadataCopyWith<$Res> {
  factory _$$SpecificMetadata_VideoImplCopyWith(
          _$SpecificMetadata_VideoImpl value,
          $Res Function(_$SpecificMetadata_VideoImpl) then) =
      __$$SpecificMetadata_VideoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int width,
      int height,
      RatioU32 pixelAspect,
      BigInt numberOfFrames,
      BigDecimal framesPerSecond});
}

/// @nodoc
class __$$SpecificMetadata_VideoImplCopyWithImpl<$Res>
    extends _$SpecificMetadataCopyWithImpl<$Res, _$SpecificMetadata_VideoImpl>
    implements _$$SpecificMetadata_VideoImplCopyWith<$Res> {
  __$$SpecificMetadata_VideoImplCopyWithImpl(
      _$SpecificMetadata_VideoImpl _value,
      $Res Function(_$SpecificMetadata_VideoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
    Object? pixelAspect = null,
    Object? numberOfFrames = null,
    Object? framesPerSecond = null,
  }) {
    return _then(_$SpecificMetadata_VideoImpl(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      pixelAspect: null == pixelAspect
          ? _value.pixelAspect
          : pixelAspect // ignore: cast_nullable_to_non_nullable
              as RatioU32,
      numberOfFrames: null == numberOfFrames
          ? _value.numberOfFrames
          : numberOfFrames // ignore: cast_nullable_to_non_nullable
              as BigInt,
      framesPerSecond: null == framesPerSecond
          ? _value.framesPerSecond
          : framesPerSecond // ignore: cast_nullable_to_non_nullable
              as BigDecimal,
    ));
  }
}

/// @nodoc

class _$SpecificMetadata_VideoImpl extends SpecificMetadata_Video {
  const _$SpecificMetadata_VideoImpl(
      {required this.width,
      required this.height,
      required this.pixelAspect,
      required this.numberOfFrames,
      required this.framesPerSecond})
      : super._();

  @override
  final int width;
  @override
  final int height;
  @override
  final RatioU32 pixelAspect;
  @override
  final BigInt numberOfFrames;
  @override
  final BigDecimal framesPerSecond;

  @override
  String toString() {
    return 'SpecificMetadata.video(width: $width, height: $height, pixelAspect: $pixelAspect, numberOfFrames: $numberOfFrames, framesPerSecond: $framesPerSecond)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SpecificMetadata_VideoImpl &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height) &&
            (identical(other.pixelAspect, pixelAspect) ||
                other.pixelAspect == pixelAspect) &&
            (identical(other.numberOfFrames, numberOfFrames) ||
                other.numberOfFrames == numberOfFrames) &&
            (identical(other.framesPerSecond, framesPerSecond) ||
                other.framesPerSecond == framesPerSecond));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, width, height, pixelAspect, numberOfFrames, framesPerSecond);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SpecificMetadata_VideoImplCopyWith<_$SpecificMetadata_VideoImpl>
      get copyWith => __$$SpecificMetadata_VideoImplCopyWithImpl<
          _$SpecificMetadata_VideoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int width, int height, RatioU32 pixelAspect)
        image,
    required TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)
        video,
  }) {
    return video(width, height, pixelAspect, numberOfFrames, framesPerSecond);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult? Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
  }) {
    return video?.call(
        width, height, pixelAspect, numberOfFrames, framesPerSecond);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int width, int height, RatioU32 pixelAspect)? image,
    TResult Function(int width, int height, RatioU32 pixelAspect,
            BigInt numberOfFrames, BigDecimal framesPerSecond)?
        video,
    required TResult orElse(),
  }) {
    if (video != null) {
      return video(width, height, pixelAspect, numberOfFrames, framesPerSecond);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SpecificMetadata_Image value) image,
    required TResult Function(SpecificMetadata_Video value) video,
  }) {
    return video(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SpecificMetadata_Image value)? image,
    TResult? Function(SpecificMetadata_Video value)? video,
  }) {
    return video?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SpecificMetadata_Image value)? image,
    TResult Function(SpecificMetadata_Video value)? video,
    required TResult orElse(),
  }) {
    if (video != null) {
      return video(this);
    }
    return orElse();
  }
}

abstract class SpecificMetadata_Video extends SpecificMetadata {
  const factory SpecificMetadata_Video(
          {required final int width,
          required final int height,
          required final RatioU32 pixelAspect,
          required final BigInt numberOfFrames,
          required final BigDecimal framesPerSecond}) =
      _$SpecificMetadata_VideoImpl;
  const SpecificMetadata_Video._() : super._();

  @override
  int get width;
  @override
  int get height;
  @override
  RatioU32 get pixelAspect;
  BigInt get numberOfFrames;
  BigDecimal get framesPerSecond;
  @override
  @JsonKey(ignore: true)
  _$$SpecificMetadata_VideoImplCopyWith<_$SpecificMetadata_VideoImpl>
      get copyWith => throw _privateConstructorUsedError;
}
