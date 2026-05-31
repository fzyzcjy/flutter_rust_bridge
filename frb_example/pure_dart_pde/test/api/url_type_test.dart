// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/url_type.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('url::Url', () async {
    final uri = Uri.parse('https://example.com/a/path?x=1#frag');
    final output = await handleUrlTwinNormal(url: uri);
    expect(output, uri);
  });

  test('Vec<url::Url>', () async {
    final uris = [
      Uri.parse('https://example.com/a'),
      Uri.parse('https://www.rust-lang.org/tools/install'),
    ];
    final outputs = await handleUrlsTwinNormal(urls: uris);
    expect(outputs, uris);
  });

  test('nested url::Url types', () async {
    final uri = Uri.parse('https://example.com/nested');
    final wrapper = FeatureUrlTwinNormal(one: uri);
    final output = await handleNestedUrlTwinNormal(url: wrapper);
    expect(output.one, uri);
  });

  test("uriparse::URI<'static>", () async {
    final uri = Uri.parse('https://example.com/a/path?x=1#frag');
    final output = await handleUriparseUriTwinNormal(uri: uri);
    expect(output, uri);
  });

  test("Vec<uriparse::URI<'static>>", () async {
    final uris = [
      Uri.parse('https://example.com/a'),
      Uri.parse('urn:isbn:0451450523'),
    ];
    final outputs = await handleUriparseUrisTwinNormal(uris: uris);
    expect(outputs, uris);
  });

  test("nested uriparse::URI<'static> types", () async {
    final uri = Uri.parse('https://example.com/nested');
    final wrapper = FeatureUriparseUriTwinNormal(one: uri);
    final output = await handleNestedUriparseUriTwinNormal(uri: wrapper);
    expect(output.one, uri);
  });
}
