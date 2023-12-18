//
//  Generated code. Do not modify.
//  source: protobuf_for_benchmark.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class BinaryTreeProtobuf extends $pb.GeneratedMessage {
  factory BinaryTreeProtobuf({
    $core.String? name,
    BinaryTreeProtobuf? left,
    BinaryTreeProtobuf? right,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (left != null) {
      $result.left = left;
    }
    if (right != null) {
      $result.right = right;
    }
    return $result;
  }
  BinaryTreeProtobuf._() : super();
  factory BinaryTreeProtobuf.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory BinaryTreeProtobuf.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BinaryTreeProtobuf',
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOM<BinaryTreeProtobuf>(2, _omitFieldNames ? '' : 'left',
        subBuilder: BinaryTreeProtobuf.create)
    ..aOM<BinaryTreeProtobuf>(3, _omitFieldNames ? '' : 'right',
        subBuilder: BinaryTreeProtobuf.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  BinaryTreeProtobuf clone() => BinaryTreeProtobuf()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  BinaryTreeProtobuf copyWith(void Function(BinaryTreeProtobuf) updates) =>
      super.copyWith((message) => updates(message as BinaryTreeProtobuf))
          as BinaryTreeProtobuf;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BinaryTreeProtobuf create() => BinaryTreeProtobuf._();
  BinaryTreeProtobuf createEmptyInstance() => create();
  static $pb.PbList<BinaryTreeProtobuf> createRepeated() =>
      $pb.PbList<BinaryTreeProtobuf>();
  @$core.pragma('dart2js:noInline')
  static BinaryTreeProtobuf getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BinaryTreeProtobuf>(create);
  static BinaryTreeProtobuf? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  BinaryTreeProtobuf get left => $_getN(1);
  @$pb.TagNumber(2)
  set left(BinaryTreeProtobuf v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasLeft() => $_has(1);
  @$pb.TagNumber(2)
  void clearLeft() => clearField(2);
  @$pb.TagNumber(2)
  BinaryTreeProtobuf ensureLeft() => $_ensure(1);

  @$pb.TagNumber(3)
  BinaryTreeProtobuf get right => $_getN(2);
  @$pb.TagNumber(3)
  set right(BinaryTreeProtobuf v) {
    setField(3, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasRight() => $_has(2);
  @$pb.TagNumber(3)
  void clearRight() => clearField(3);
  @$pb.TagNumber(3)
  BinaryTreeProtobuf ensureRight() => $_ensure(2);
}

class BlobProtobuf extends $pb.GeneratedMessage {
  factory BlobProtobuf({
    $core.List<$core.int>? first,
    $core.List<$core.int>? second,
    $core.List<$core.int>? third,
  }) {
    final $result = create();
    if (first != null) {
      $result.first = first;
    }
    if (second != null) {
      $result.second = second;
    }
    if (third != null) {
      $result.third = third;
    }
    return $result;
  }
  BlobProtobuf._() : super();
  factory BlobProtobuf.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory BlobProtobuf.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'BlobProtobuf',
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'first', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        2, _omitFieldNames ? '' : 'second', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(
        3, _omitFieldNames ? '' : 'third', $pb.PbFieldType.OY)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  BlobProtobuf clone() => BlobProtobuf()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  BlobProtobuf copyWith(void Function(BlobProtobuf) updates) =>
      super.copyWith((message) => updates(message as BlobProtobuf))
          as BlobProtobuf;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BlobProtobuf create() => BlobProtobuf._();
  BlobProtobuf createEmptyInstance() => create();
  static $pb.PbList<BlobProtobuf> createRepeated() =>
      $pb.PbList<BlobProtobuf>();
  @$core.pragma('dart2js:noInline')
  static BlobProtobuf getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<BlobProtobuf>(create);
  static BlobProtobuf? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get first => $_getN(0);
  @$pb.TagNumber(1)
  set first($core.List<$core.int> v) {
    $_setBytes(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasFirst() => $_has(0);
  @$pb.TagNumber(1)
  void clearFirst() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get second => $_getN(1);
  @$pb.TagNumber(2)
  set second($core.List<$core.int> v) {
    $_setBytes(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasSecond() => $_has(1);
  @$pb.TagNumber(2)
  void clearSecond() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get third => $_getN(2);
  @$pb.TagNumber(3)
  set third($core.List<$core.int> v) {
    $_setBytes(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasThird() => $_has(2);
  @$pb.TagNumber(3)
  void clearThird() => clearField(3);
}

const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
