import 'dart:io';

import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart'
    as native_toolchain_rust;
import 'package:test/test.dart';

void main() {
  test(
    'builder defaults to generated IO asset, rust crate path, and release build mode',
    () {
      const builder = FlutterRustBridgeNativeAssetsBuilder();

      expect(builder.assetName, 'src/rust/frb_generated.io.dart');
      expect(builder.cratePath, 'rust');
      expect(builder.buildMode, native_toolchain_rust.BuildMode.release);
    },
  );

  test('builder keeps explicit build mode override', () {
    const builder = FlutterRustBridgeNativeAssetsBuilder(
      buildMode: native_toolchain_rust.BuildMode.debug,
    );

    expect(builder.buildMode, native_toolchain_rust.BuildMode.debug);
  });

  test('buildInputForHost uses short Windows output directory', () async {
    final input = _createBuildInput(outputDirectoryShared: '/tmp/frb-long');
    final adjusted = await buildInputForHost(isWindows: true, input: input);
    final shortOutputDirectoryShared = adjusted.outputDirectoryShared;

    expect(
      shortOutputDirectoryShared.path,
      startsWith('${Directory.systemTemp.uri.path}frb_native_assets_'),
    );
    expect(shortOutputDirectoryShared, isNot(input.outputDirectoryShared));
    expect(adjusted.json, {
      ...input.json,
      'out_dir_shared': Directory.fromUri(shortOutputDirectoryShared).path,
    });
    expect(
      adjusted.outputDirectory.path,
      startsWith(Directory.fromUri(shortOutputDirectoryShared).path),
    );
  });

  test('buildInputForHost keeps non-Windows input unchanged', () async {
    final input = _createBuildInput(outputDirectoryShared: '/tmp/frb-long');

    expect(
      await buildInputForHost(isWindows: false, input: input),
      same(input),
    );
  });
}

BuildInput _createBuildInput({required String outputDirectoryShared}) {
  final builder = BuildInputBuilder()
    ..setupShared(
      packageRoot: Uri.directory('/tmp/frb-package/'),
      packageName: 'frb_package',
      outputDirectoryShared: Uri.directory('$outputDirectoryShared/'),
      outputFile: Uri.file('/tmp/frb-output.json'),
    )
    ..setupBuildInput();
  builder.config.setupBuild(linkingEnabled: false);
  return builder.build();
}
