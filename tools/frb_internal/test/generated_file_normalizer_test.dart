import 'package:flutter_rust_bridge_internal/src/makefile_dart/generated_file_normalizer.dart';
import 'package:test/test.dart';

void main() {
  test('generated file normalization removes extra trailing blank lines', () {
    expect(normalizeSingleTrailingNewlineText('body\n\n'), 'body\n');
    expect(normalizeSingleTrailingNewlineText('body\n\n\n'), 'body\n');
  });

  test('generated file normalization keeps exactly one trailing newline', () {
    expect(normalizeSingleTrailingNewlineText('body'), 'body\n');
    expect(normalizeSingleTrailingNewlineText('body\n'), 'body\n');
  });
}
