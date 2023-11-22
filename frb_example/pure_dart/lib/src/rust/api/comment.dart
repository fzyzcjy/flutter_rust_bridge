// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

/// Multiline comments are fine,
/// but they are not preferred in Rust nor in Dart.
/// Newlines are preserved.
Future<void> functionWithCommentsSlashStarStarTwinNormal({dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsSlashStarStarTwinNormal(hint: hint);

/// This is first line
/// This is second line
Future<void> functionWithCommentsTripleSlashMultiLineTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashMultiLineTwinNormal(hint: hint);

/// This is single line comment
Future<void> functionWithCommentsTripleSlashSingleLineTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashSingleLineTwinNormal(hint: hint);

/// Comments on structs
class StructWithCommentsTwinNormal {
  /// Documentation on a struct field
  final int fieldWithComments;

  const StructWithCommentsTwinNormal({
    required this.fieldWithComments,
  });

  /// Documentation on an instance method
  Future<void> instanceMethod({dynamic hint}) =>
      RustLib.instance.api.structWithCommentsTwinNormalInstanceMethod(
        that: this,
      );

  /// Documentation on a static method
  static Future<void> staticMethod({dynamic hint}) =>
      RustLib.instance.api.structWithCommentsTwinNormalStaticMethod(hint: hint);
}
