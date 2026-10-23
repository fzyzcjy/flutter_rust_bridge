/// {@macro flutter_rust_bridge.internal}
const wasmBindgenRegisteredEventName = 'wasm_bindgen_registered';

/// JavaScript appended by `build-web` to the generated no-modules bindings.
///
/// {@macro flutter_rust_bridge.internal}
const wasmBindgenRegisteredEventSnippet =
    '''

// This snippet dispatches the '$wasmBindgenRegisteredEventName' event after the wasm_bindgen object is available.
// It's used to notify any listeners that the wasm_bindgen object is now available.
// Used in WASM bridge initialization to signal that the wasm_bindgen object is ready and can be used to start interacting with the WASM module.
// Web workers also `importScripts` this file but have no `window`, so the event is only dispatched on the main thread.

if (typeof window !== 'undefined') {
  const wasmBindgenRegisteredEvent = new Event('$wasmBindgenRegisteredEventName');
  wasmBindgenRegisteredEvent.wasm_bindgen = wasm_bindgen;
  window.dispatchEvent(wasmBindgenRegisteredEvent);
}
''';
