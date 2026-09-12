import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:isolate';

import 'package:test/test.dart';

void main() {
  for (final change in [
    'build.rs',
    '.cargo/config.toml',
    'cargo clean',
    'library removal'
  ]) {
    test('cached simpleBuild rebuilds after $change', () async {
      final fixture = await Directory.systemTemp.createTemp('frb_hook_cache_');
      addTearDown(() => fixture.delete(recursive: true));
      await _prepareFixture(fixture);
      await _run(
          fixture, Platform.resolvedExecutable, ['pub', 'get', '--offline']);

      await _expectAnswer(fixture, 1);
      final output = await Directory.fromUri(
        fixture.uri.resolve('.dart_tool/hooks_runner/cache_probe/'),
      )
          .list(recursive: true)
          .where((entry) =>
              entry.path.endsWith('/output.json') ||
              entry.path.endsWith('\\output.json'))
          .single;
      final outputFile = File(output.path);
      var cached = false;
      for (var attempt = 0; attempt < 4; attempt++) {
        final before = await outputFile.lastModified();
        await _expectAnswer(fixture, 1);
        if (await outputFile.lastModified() == before) {
          cached = true;
          break;
        }
      }
      expect(cached, isTrue,
          reason: 'The regression must start from a cached hook');

      switch (change) {
        case 'build.rs':
          await File.fromUri(fixture.uri.resolve('rust/build.rs'))
              .writeAsString(
            'fn main() { println!("cargo:rustc-cfg=cache_changed"); }\n',
          );
        case '.cargo/config.toml':
          await Directory.fromUri(fixture.uri.resolve('rust/.cargo/')).create();
          await File.fromUri(fixture.uri.resolve('rust/.cargo/config.toml'))
              .writeAsString(
                  '[build]\nrustflags = ["--cfg", "cache_changed"]\n');
        case 'cargo clean':
          await _run(Directory.fromUri(fixture.uri.resolve('rust/')), 'cargo',
              ['clean']);
        case 'library removal':
          final name = Platform.isWindows
              ? 'cache_probe.dll'
              : Platform.isMacOS
                  ? 'libcache_probe.dylib'
                  : 'libcache_probe.so';
          await File.fromUri(fixture.uri.resolve('rust/target/release/$name'))
              .delete();
      }

      await _expectAnswer(fixture,
          change == 'build.rs' || change == '.cargo/config.toml' ? 2 : 1);
    }, timeout: const Timeout(Duration(minutes: 5)));
  }
}

Future<void> _prepareFixture(Directory fixture) async {
  final library = await Isolate.resolvePackageUri(
    Uri.parse(
        'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils.dart'),
  );
  final packageRoot = library!.resolve('../').toFilePath();
  await File.fromUri(fixture.uri.resolve('pubspec.yaml')).writeAsString('''
name: cache_probe
environment:
  sdk: '>=3.10.0 <4.0.0'
dependencies:
  flutter_rust_bridge_utils:
    path: ${jsonEncode(packageRoot)}
''');
  await Directory.fromUri(fixture.uri.resolve('hook/')).create();
  await File.fromUri(fixture.uri.resolve('hook/build.dart')).writeAsString('''
import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils.dart';
Future<void> main(List<String> args) => simpleBuild(args);
''');
  await Directory.fromUri(fixture.uri.resolve('rust/src/'))
      .create(recursive: true);
  await File.fromUri(fixture.uri.resolve('rust/Cargo.toml')).writeAsString('''
[package]
name = "cache_probe"
version = "0.1.0"
edition = "2021"
[lib]
crate-type = ["cdylib"]
''');
  await File.fromUri(fixture.uri.resolve('rust/src/lib.rs')).writeAsString('''
#[no_mangle]
pub extern "C" fn answer() -> i32 {
    if cfg!(cache_changed) { 2 } else { 1 }
}
''');
  await File.fromUri(fixture.uri.resolve('probe.dart')).writeAsString('''
import 'dart:ffi';
import 'dart:io';
void main() {
  final name = Platform.isWindows ? 'cache_probe.dll' :
      Platform.isMacOS ? 'libcache_probe.dylib' : 'libcache_probe.so';
  final library = DynamicLibrary.open('rust/target/release/\$name');
  final answer = library.lookupFunction<Int32 Function(), int Function()>('answer');
  stdout.writeln('answer=\${answer()}');
}
''');
}

Future<void> _expectAnswer(Directory fixture, int answer) async {
  final output =
      await _run(fixture, Platform.resolvedExecutable, ['run', 'probe.dart']);
  expect(output, contains('answer=$answer'));
}

Future<String> _run(
    Directory directory, String executable, List<String> args) async {
  stdout
      .writeln('EXEC: $executable ${args.join(' ')} (cwd: ${directory.path})');
  final process =
      await Process.start(executable, args, workingDirectory: directory.path);
  final output = process.stdout.transform(utf8.decoder).join();
  final errors = process.stderr.transform(utf8.decoder).join();
  final exitCode =
      await process.exitCode.timeout(const Duration(minutes: 3), onTimeout: () {
    process.kill();
    throw TimeoutException('Timed out: $executable ${args.join(' ')}');
  });
  final result = await output;
  expect(exitCode, 0, reason: '$result\n${await errors}');
  return result;
}
