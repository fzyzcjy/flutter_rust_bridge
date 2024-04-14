import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/loader/loader.dart';
import 'package:flutter_rust_bridge/src/main_components/api.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/main_components/port_manager.dart';
import 'package:flutter_rust_bridge/src/main_components/wire.dart';
import 'package:flutter_rust_bridge/src/misc/version.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

/// This is the main entrypoint.
/// For example, users call `init` on it, and auto-generated code call `api` on it.
///
/// This class is like "service locator" (e.g. the get_it package) for all services related to flutter_rust_bridge.
///
/// This should be a singleton per flutter_rust_bridge usage (enforced via generated subclass code).
abstract class BaseEntrypoint<A extends BaseApi, AI extends BaseApiImpl,
    W extends BaseWire> {
  /// Whether the system has been initialized.
  bool get initialized => __state != null;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  A get api => _state.api;

  _EntrypointState<A> get _state =>
      __state ??
      (throw StateError('flutter_rust_bridge has not been initialized. '
          'Did you forget to call `await RustLib.init();`? '
          '(If you have configured a different lib name, change `RustLib` to your name.)'));
  _EntrypointState<A>? __state;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  Future<void> initImpl({
    A? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    if (__state != null) {
      throw StateError('Should not initialize flutter_rust_bridge twice');
    }

    _sanityCheckCodegenVersion();

    externalLibrary ??= await _loadDefaultExternalLibrary();
    handler ??= BaseHandler();
    final generalizedFrbRustBinding =
        GeneralizedFrbRustBinding(externalLibrary);
    _sanityCheckContentHash(generalizedFrbRustBinding);
    final portManager = PortManager(generalizedFrbRustBinding, handler);
    api ??= _createDefaultApi(
        handler, generalizedFrbRustBinding, portManager, externalLibrary);

    __state = _EntrypointState(
      generalizedFrbRustBinding: generalizedFrbRustBinding,
      portManager: portManager,
      api: api,
    );

    await executeRustInitializers();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  void disposeImpl() {
    __state!.dispose();
  }

  /// {@macro flutter_rust_bridge.internal}
  @internal
  @visibleForTesting
  void resetState() {
    // ignore: avoid_print
    print(
        'WARN: resetState() (should only be used in internal tests, never be used by normal users)');
    __state = null;
  }

  void _sanityCheckCodegenVersion() {
    if (codegenVersion != kFlutterRustBridgeRuntimeVersion) {
      throw StateError(
        "flutter_rust_bridge's codegen version ($codegenVersion) should be the same as runtime version ($kFlutterRustBridgeRuntimeVersion). "
        "See https://cjycode.com/flutter_rust_bridge/guides/miscellaneous/upgrade/regular for details.",
      );
    }
  }

  void _sanityCheckContentHash(
      GeneralizedFrbRustBinding generalizedFrbRustBinding) {
    final rustSideRustContentHash =
        generalizedFrbRustBinding.getRustContentHash();
    if (rustContentHash != rustSideRustContentHash) {
      throw StateError(
        "Content hash on Dart side ($rustContentHash) is different from Rust side ($rustSideRustContentHash), indicating out-of-sync code. "
        "This may happen when, for example, the Dart code is hot-restarted/hot-reloaded without recompiling Rust code. "
        "(Note: This is just a sanity check. Even if content hash does not change, the code may still change and needs to be recompiled)",
      );
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  ApiImplConstructor<AI, W> get apiImplConstructor;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  WireConstructor<W> get wireConstructor;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  String get codegenVersion;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  int get rustContentHash;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  Future<void> executeRustInitializers();

  Future<ExternalLibrary> _loadDefaultExternalLibrary() async =>
      await loadExternalLibrary(defaultExternalLibraryLoaderConfig);

  A _createDefaultApi(
    BaseHandler handler,
    GeneralizedFrbRustBinding generalizedFrbRustBinding,
    PortManager portManager,
    ExternalLibrary externalLibrary,
  ) {
    return apiImplConstructor(
      handler: handler,
      generalizedFrbRustBinding: generalizedFrbRustBinding,
      portManager: portManager,
      wire: wireConstructor(externalLibrary),
    ) as A;
  }
}

class _EntrypointState<A extends BaseApi> {
  final GeneralizedFrbRustBinding generalizedFrbRustBinding;
  final PortManager portManager;
  final A api;

  _EntrypointState({
    required this.generalizedFrbRustBinding,
    required this.portManager,
    required this.api,
  }) {
    _setUpRustToDartCommunication(generalizedFrbRustBinding);
    _initializeApiDlData(generalizedFrbRustBinding);
  }

  void dispose() {
    portManager.dispose();
  }
}

void _setUpRustToDartCommunication(
    GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  generalizedFrbRustBinding.storeDartPostCObject();
}

void _initializeApiDlData(GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  generalizedFrbRustBinding.initFrbDartApiDl();
}
