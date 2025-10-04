/// Arguments of `loadExternalLibrary`
class ExternalLibraryLoaderConfig {
  /// The file stem
  final String stem;

  /// In io (native), the directory that may find the dynamic library
  final String? ioDirectory;

  /// In Web, the prefix path for the wasm
  final String? webPrefix;

  /// The name of the wasm_bindgen module.
  final String wasmBindgenName;

  /// Arguments of `loadExternalLibrary`
  const ExternalLibraryLoaderConfig({
    required this.stem,
    required this.ioDirectory,
    required this.webPrefix,
    this.wasmBindgenName = 'wasm_bindgen',
  });
}
