import 'dart:io';

import 'package:build_tool/src/artifacts_provider.dart';
import 'package:build_tool/src/target.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  group('artifact naming', () {
    test('windows remote artifacts exclude pdb files', () {
      final target = Target.forRustTriple('x86_64-pc-windows-msvc')!;

      expect(
        getArtifactNames(
          target: target,
          libraryName: 'demo',
          remote: true,
        ),
        ['demo.dll', 'demo.dll.lib'],
      );
    });

    test('windows local artifacts include pdb files', () {
      final target = Target.forRustTriple('x86_64-pc-windows-msvc')!;

      expect(
        getArtifactNames(
          target: target,
          libraryName: 'demo',
          remote: false,
        ),
        ['demo.dll', 'demo.dll.lib', 'demo.pdb'],
      );
    });

    test('darwin artifacts default to static libraries', () {
      final target = Target.forRustTriple('aarch64-apple-darwin')!;

      expect(
        getArtifactNames(
          target: target,
          libraryName: 'demo',
          remote: true,
        ),
        ['libdemo.a'],
      );
    });
  });

  group('artifact materialization', () {
    late Directory tempDir;

    setUp(() {
      tempDir = Directory.systemTemp.createTempSync('artifact_materializer_');
    });

    tearDown(() {
      tempDir.deleteSync(recursive: true);
    });

    test('copyDynamicLibraries only copies dynamic artifacts', () {
      final sourceDir = Directory(path.join(tempDir.path, 'source'))
        ..createSync(recursive: true);
      final outputDir = path.join(tempDir.path, 'output');

      final dylib = File(path.join(sourceDir.path, 'libdemo.so'))
        ..writeAsStringSync('dynamic');
      final staticlib = File(path.join(sourceDir.path, 'libdemo.a'))
        ..writeAsStringSync('static');

      final copied = ArtifactMaterializer.copyDynamicLibraries(
        [
          Artifact(path: dylib.path, finalFileName: 'libdemo.so'),
          Artifact(path: staticlib.path, finalFileName: 'libdemo.a'),
        ],
        outputDir: outputDir,
      );

      expect(copied, [path.join(outputDir, 'libdemo.so')]);
      expect(File(path.join(outputDir, 'libdemo.so')).readAsStringSync(),
          'dynamic');
      expect(File(path.join(outputDir, 'libdemo.a')).existsSync(), isFalse);
    });

    test('flattenForType collects artifacts across targets', () {
      final linuxTarget = Target.forRustTriple('x86_64-unknown-linux-gnu')!;
      final windowsTarget = Target.forRustTriple('x86_64-pc-windows-msvc')!;

      final flattened = ArtifactMaterializer.flattenForType(
        {
          linuxTarget: [
            Artifact(path: '/tmp/libdemo.so', finalFileName: 'libdemo.so'),
          ],
          windowsTarget: [
            Artifact(path: '/tmp/demo.dll', finalFileName: 'demo.dll'),
            Artifact(path: '/tmp/demo.lib', finalFileName: 'demo.lib'),
          ],
        },
        type: ArtifactType.dylib,
      );

      expect(flattened.map((artifact) => artifact.finalFileName), [
        'libdemo.so',
        'demo.dll',
      ]);
    });
  });
}
