@TestOn('browser')
import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/wasm_bindgen_registration.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

const _kWasmBindgenInvokedKey = '__frbTestWasmBindgenInvoked';

void main() {
  test('wasm module initializer is available to browser callers', () {
    const Future<void> Function({required String root, String wasmBindgenName})
    initializer = initializeWasmModule;

    expect(initializer, isNotNull);
  });

  test('initializeWasmModule takes wasm_bindgen from the registration event '
      'without evaluating JavaScript', () async {
    addTearDown(() {
      web.window.delete('wasm_bindgen'.toJS);
      web.window.delete(_kWasmBindgenInvokedKey.toJS);
    });
    // The `#` keeps `$root.js` pointing at the blob.
    final root = '${_objectUrl(_fakeBindings())}#';
    final dynamicJsCalls = <String>[];
    _trapGlobal('Function', dynamicJsCalls);
    _trapGlobal('eval', dynamicJsCalls);

    await initializeWasmModule(root: root);

    expect(dynamicJsCalls, isEmpty);
    final wasmBindgen = web.window.getProperty<JSObject>('wasm_bindgen'.toJS);
    expect(
      wasmBindgen.getProperty<JSString>('frbTestMarker'.toJS).toDart,
      'fake',
    );
    expect(
      web.window.getProperty<JSBoolean?>(_kWasmBindgenInvokedKey.toJS)?.toDart,
      isTrue,
    );
  });

  test('wasm_bindgen registration can be imported by web workers', () async {
    // The worker pool `importScripts` the same generated bindings file.
    final bindingsUrl = _objectUrl(
      'let wasm_bindgen = function () {};\n$wasmBindgenRegisteredEventSnippet',
    );
    final workerUrl = _objectUrl(
      "importScripts('$bindingsUrl');\npostMessage('imported');",
    );
    final worker = web.Worker(workerUrl.toJS);
    addTearDown(() => worker.terminate());

    final result = Completer<String>();
    worker.onmessage = ((web.MessageEvent event) {
      result.complete((event.data as JSString).toDart);
    }).toJS;
    worker.onerror = ((web.ErrorEvent event) {
      event.preventDefault();
      result.complete('error: ${event.message}');
    }).toJS;

    expect(await result.future, 'imported');
  });
}

String _fakeBindings() =>
    '''
{
  let wasm_bindgen = Object.assign(function () {
    globalThis.$_kWasmBindgenInvokedKey = true;
    return Promise.resolve();
  }, { frbTestMarker: 'fake' });
  $wasmBindgenRegisteredEventSnippet
}
''';

String _objectUrl(String javascript) {
  final url = web.URL.createObjectURL(
    web.Blob(
      [javascript.toJS].toJS,
      web.BlobPropertyBag(type: 'text/javascript'),
    ),
  );
  addTearDown(() => web.URL.revokeObjectURL(url));
  return url;
}

void _trapGlobal(String name, List<String> calls) {
  final original = globalContext.getProperty<JSAny?>(name.toJS);
  JSAny? trap(JSAny? _) {
    calls.add(name);
    throw StateError('Unexpected dynamic JavaScript via $name');
  }

  globalContext.setProperty(name.toJS, trap.toJS);
  addTearDown(() => globalContext.setProperty(name.toJS, original));
}
