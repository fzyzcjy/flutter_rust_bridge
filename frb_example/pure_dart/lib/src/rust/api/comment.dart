// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

/// Multiline comments are fine,
/// but they are not preferred in Rust nor in Dart.
/// Newlines are preserved.
Future<void> functionWithCommentsSlashStarStar({dynamic hint}) =>
    RustLib.instance.api.functionWithCommentsSlashStarStar(hint: hint);

/// This is first line
/// This is second line
Future<void> functionWithCommentsTripleSlashMultiLine({dynamic hint}) =>
    RustLib.instance.api.functionWithCommentsTripleSlashMultiLine(hint: hint);

/// This is single line comment
Future<void> functionWithCommentsTripleSlashSingleLine({dynamic hint}) =>
    RustLib.instance.api.functionWithCommentsTripleSlashSingleLine(hint: hint);
