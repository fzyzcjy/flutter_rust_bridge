import 'dart:async';

abstract class WasmModule {
  Object call([String? moduleName]);

  WasmModule bind(dynamic thisArg, String moduleName);

  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }

  static FutureOr<WasmModule> initialize({required Modules kind, WasmModule Function()? module}) =>
      throw UnimplementedError();
}

abstract class Modules {
  const Modules();

  const factory Modules.noModules({required String root}) = _WasmBindgenNoModules;

  FutureOr<WasmModule> initializeModule(WasmModule Function()? module);
}

class _WasmBindgenNoModules extends Modules {
  final String root;

  const _WasmBindgenNoModules({required this.root});

  @override
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module) => throw UnimplementedError();
}
