// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

/// Multiline comments are fine,
/// but they are not preferred in Rust nor in Dart.
/// Newlines are preserved.
void functionWithCommentsSlashStarStarTwinSync({dynamic hint}) =>
    RustLib.instance.api.functionWithCommentsSlashStarStarTwinSync(hint: hint);

/// This is first line
/// This is second line
void functionWithCommentsTripleSlashMultiLineTwinSync({dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashMultiLineTwinSync(hint: hint);

/// This is single line comment
void functionWithCommentsTripleSlashSingleLineTwinSync({dynamic hint}) =>
    RustLib.instance.api
        .functionWithCommentsTripleSlashSingleLineTwinSync(hint: hint);

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
