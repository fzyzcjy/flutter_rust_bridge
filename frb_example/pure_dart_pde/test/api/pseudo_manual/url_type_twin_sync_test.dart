// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `url_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/url_type_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('url::Url', () async {
    final uri = Uri.parse('https://example.com/a/path?x=1#frag');
    final output = await handleUrlTwinSync(url: uri);
    expect(output, uri);
  });

  test('Vec<url::Url>', () async {
    final uris = [
      Uri.parse('https://example.com/a'),
      Uri.parse('https://www.rust-lang.org/tools/install'),
    ];
    final outputs = await handleUrlsTwinSync(urls: uris);
    expect(outputs, uris);
  });

  test('nested url::Url types', () async {
    final uri = Uri.parse('https://example.com/nested');
    final wrapper = FeatureUrlTwinSync(one: uri);
    final output = await handleNestedUrlTwinSync(url: wrapper);
    expect(output.one, uri);
  });

  test("uriparse::URI<'static>", () async {
    final uri = Uri.parse('https://example.com/a/path?x=1#frag');
    final output = await handleUriparseUriTwinSync(uri: uri);
    expect(output, uri);
  });

  test("Vec<uriparse::URI<'static>>", () async {
    final uris = [
      Uri.parse('https://example.com/a'),
      Uri.parse('urn:isbn:0451450523'),
    ];
    final outputs = await handleUriparseUrisTwinSync(uris: uris);
    expect(outputs, uris);
  });

  test("nested uriparse::URI<'static> types", () async {
    final uri = Uri.parse('https://example.com/nested');
    final wrapper = FeatureUriparseUriTwinSync(one: uri);
    final output = await handleNestedUriparseUriTwinSync(uri: wrapper);
    expect(output.one, uri);
  });
}
