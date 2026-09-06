import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_common.dart';
import 'package:flutter_rust_bridge/src/consts.dart' show kIsWeb;
import 'package:flutter_rust_bridge/src/misc/version.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/entrypoint.dart'
    as build_web;
import 'package:test/test.dart';

void main() {
  test('initialization rejects state installed during readiness', () async {
    final entrypoint = _FakeBaseEntrypointWithCodegenVersion(
      kFlutterRustBridgeRuntimeVersion,
    );
    final pending = entrypoint._initialize();
    final api = _FakeApi();
    entrypoint._mock(api);

    await expectLater(
      pending,
      throwsA(
        isA<StateError>().having(
          (error) => error.message,
          'message',
          'Should not initialize flutter_rust_bridge twice',
        ),
      ),
    );
    expect(entrypoint.api, same(api));
  });

  test('Should be ready when initMock is called', () async {
    final entrypoint = _FakeBaseEntrypoint();

    // ignore: invalid_use_of_protected_member
    entrypoint.initMockImpl(api: _FakeApi());

    expect(entrypoint.initialized, true);
    expect(entrypoint.api, isA<_FakeApi>());
  });

  test(
    'disposing a mock entrypoint clears state and permits reinitialization',
    () {
      final entrypoint = _FakeBaseEntrypoint();
      final firstApi = _FakeApi();

      // ignore: invalid_use_of_protected_member
      entrypoint.initMockImpl(api: firstApi);
      // ignore: invalid_use_of_protected_member
      entrypoint.disposeImpl();

      expect(entrypoint.initialized, isFalse);
      expect(() => entrypoint.api, throwsA(isA<StateError>()));

      final secondApi = _FakeApi();
      // ignore: invalid_use_of_protected_member
      entrypoint.initMockImpl(api: secondApi);
      expect(entrypoint.api, same(secondApi));
    },
  );

  test('Codegen version check', () {
    final entrypoint = _FakeBaseEntrypointWithCodegenVersion('999.999.999');

    // Version does not match, will throw a [StateError].
    expectLater(
      // ignore: invalid_use_of_protected_member
      entrypoint.initImpl(api: _FakeApi(), forceSameCodegenVersion: true),
      throwsA(isA<StateError>()),
    );

    // Version matched but the stem is fake, will throw an [ArgumentError].
    expectLater(
      // ignore: invalid_use_of_protected_member
      entrypoint.initImpl(api: _FakeApi(), forceSameCodegenVersion: false),
      throwsA(isA<ArgumentError>()),
    );
  }, skip: kIsWeb);

  test('build-web parser applies generated defaults', () {
    final config = build_web.parseConfig([]);

    expect(config.dartRoot, isNull);
    expect(config.rustRoot, 'rust');
    expect(config.output, isNull);
    expect(config.release, isFalse);
    expect(config.verbose, isFalse);
    expect(config.cargoBuildArgs, isEmpty);
    expect(config.wasmBindgenArgs, isEmpty);
    expect(config.wasmPackRustupToolchain, isNull);
    expect(config.wasmPackRustflags, isNull);
    expect(config.dartCompileJsEntrypoint, isNull);
  });

  test('build-web parser maps every supported option', () {
    final config = build_web.parseConfig([
      '--dart-root',
      'dart_package',
      '-c',
      'rust_package',
      '-o',
      'web_output',
      '--release',
      '-v',
      '--cargo-build-args=--features=feature_a',
      '--cargo-build-args=--locked',
      '--wasm-bindgen-args=--weak-refs',
      '--wasm-bindgen-args=--reference-types',
      '--wasm-pack-rustup-toolchain',
      'nightly-2025-02-01',
      '--wasm-pack-rustflags',
      '-C target-feature=+atomics',
      '--dart-compile-js-entrypoint',
      'lib/main.dart',
    ]);

    expect(config.dartRoot, 'dart_package');
    expect(config.rustRoot, 'rust_package');
    expect(config.output, 'web_output');
    expect(config.release, isTrue);
    expect(config.verbose, isTrue);
    expect(config.cargoBuildArgs, ['--features=feature_a', '--locked']);
    expect(config.wasmBindgenArgs, ['--weak-refs', '--reference-types']);
    expect(config.wasmPackRustupToolchain, 'nightly-2025-02-01');
    expect(config.wasmPackRustflags, '-C target-feature=+atomics');
    expect(config.dartCompileJsEntrypoint, 'lib/main.dart');
  });

  test('build-web command exposes generated command metadata', () {
    final command = build_web.BuildWebCommand();

    expect(command.name, 'build-web');
    expect(command.description, 'Build for web platform');
    expect(
      command.argParser.options.keys,
      containsAll([
        'dart-root',
        'rust-root',
        'output',
        'release',
        'verbose',
        'cargo-build-args',
        'wasm-bindgen-args',
        'wasm-pack-rustup-toolchain',
        'wasm-pack-rustflags',
        'dart-compile-js-entrypoint',
      ]),
    );
  });
}

class _FakeBaseEntrypointWithCodegenVersion extends _FakeBaseEntrypoint {
  _FakeBaseEntrypointWithCodegenVersion(this.codegenVersion);

  Future<void> _initialize() => initImpl(api: _FakeApi());

  void _mock(BaseApi api) => initMockImpl(api: api);

  @override
  final String codegenVersion;

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      const ExternalLibraryLoaderConfig(
        stem: 'fake_codegen_version',
        ioDirectory: 'fake_dir',
        webPrefix: 'fake',
        wasmBindgenName: 'wasm_bindgen',
      );
}

class _FakeBaseEntrypoint extends BaseEntrypoint {
  // We do not care about these functions in this test (and they should not be called as well)
  // frb-coverage:ignore-start
  @override
  get apiImplConstructor => throw UnimplementedError();

  @override
  String get codegenVersion => throw UnimplementedError();

  @override
  get defaultExternalLibraryLoaderConfig => throw UnimplementedError();

  @override
  Future<void> executeRustInitializers() => throw UnimplementedError();

  @override
  int get rustContentHash => throw UnimplementedError();

  @override
  get wireConstructor => throw UnimplementedError();
  // frb-coverage:ignore-end
}

class _FakeApi implements BaseApi {}
