import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/loader/loader.dart';
import 'package:flutter_rust_bridge/src/main_components/api.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/main_components/wire.dart';
import 'package:flutter_rust_bridge/src/opaque/dart_opaque.dart';
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

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  NativePortType get dropPort => _state.dropPortManager.dropPort;

  _EntrypointState<A> get _state =>
      __state ??
      (throw StateError('flutter_rust_bridge has not been initialized'));
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

    externalLibrary ??= await _loadDefaultExternalLibrary();
    final generalizedFrbRustBinding =
        GeneralizedFrbRustBinding(externalLibrary);
    final dropPortManager = DropPortManager(generalizedFrbRustBinding);
    api ??= _createDefaultApi(
        handler, generalizedFrbRustBinding, dropPortManager, externalLibrary);

    __state = _EntrypointState(
      generalizedFrbRustBinding: generalizedFrbRustBinding,
      dropPortManager: dropPortManager,
      api: api,
    );
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

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  ApiImplConstructor<AI, W> get apiImplConstructor;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  WireConstructor<W> get wireConstructor;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig;

  Future<ExternalLibrary> _loadDefaultExternalLibrary() async =>
      await loadExternalLibrary(defaultExternalLibraryLoaderConfig);

  A _createDefaultApi(
    BaseHandler? handler,
    GeneralizedFrbRustBinding generalizedFrbRustBinding,
    DropPortManager dropPortManager,
    ExternalLibrary externalLibrary,
  ) {
    return apiImplConstructor(
      handler: handler,
      generalizedFrbRustBinding: generalizedFrbRustBinding,
      dropPortManager: dropPortManager,
      wire: wireConstructor(externalLibrary),
    ) as A;
  }
}

class _EntrypointState<A extends BaseApi> {
  final GeneralizedFrbRustBinding generalizedFrbRustBinding;
  final DropPortManager dropPortManager;
  final A api;

  _EntrypointState({
    required this.generalizedFrbRustBinding,
    required this.dropPortManager,
    required this.api,
  }) {
    _setUpRustToDartCommunication(generalizedFrbRustBinding);
    _initializeApiDlData(generalizedFrbRustBinding);
    generalizedFrbRustBinding.initializeFrbRust();
  }

  void dispose() {
    dropPortManager.dispose();
  }
}

void _setUpRustToDartCommunication(
    GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  generalizedFrbRustBinding.storeDartPostCObject();
}

void _initializeApiDlData(GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  generalizedFrbRustBinding.initFrbDartApiDl();
}
