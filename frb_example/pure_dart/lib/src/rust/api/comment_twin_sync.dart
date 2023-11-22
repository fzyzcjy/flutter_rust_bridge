// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

/// Multiline comments are fine,
/// but they are not preferred in Rust nor in Dart.
/// Newlines are preserved.
void functionWithCommentsSlashStarStarTwinNormalTwinSync({dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsSlashStarStarTwinNormalTwinSync(hint: hint);

/// This is first line
/// This is second line
void functionWithCommentsTripleSlashMultiLineTwinNormalTwinSync(
        {dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashMultiLineTwinNormalTwinSync(hint: hint);

/// This is single line comment
void functionWithCommentsTripleSlashSingleLineTwinNormalTwinSync(
        {dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashSingleLineTwinNormalTwinSync(
            hint: hint);

/// Comments on structs
class StructWithCommentsTwinSync {
  /// Documentation on a struct field
  final int fieldWithComments;

  const StructWithCommentsTwinSync({
    required this.fieldWithComments,
  });

  /// Documentation on an instance method
  void instanceMethodTwinSync({dynamic hint}) =>
      RustLib.instance.api.structWithCommentsTwinSyncInstanceMethodTwinSync(
        that: this,
      );

  /// Documentation on a static method
  static void staticMethodTwinSync({dynamic hint}) => RustLib.instance.api
      .structWithCommentsTwinSyncStaticMethodTwinSync(hint: hint);
}
