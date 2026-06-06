import 'dart:async';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:meta/meta.dart';

enum QuickstartSmokeTarget { web, desktop, android, ios }

Future<void> runFlutterViaCreateQuickstartSmokeTest({
  required String package,
  required QuickstartSmokeTarget target,
  required String? deviceId,
}) async {
  const artifactDir = 'target/quickstart_smoke';
  final targetName = target.name;
  final screenshotPath = '$artifactDir/quickstart-$targetName.png';
  final preprocessedScreenshotPath =
      '$artifactDir/quickstart-$targetName-processed.png';
  final ocrOutputPath = '$artifactDir/quickstart-$targetName.txt';
  final logPath = '$artifactDir/quickstart-$targetName.log';
  final absolutePackage = Directory(
    quickstartSmokePackagePathForTesting(package),
  );
  final absoluteArtifactDir = Directory('${absolutePackage.path}/$artifactDir');
  absoluteArtifactDir.createSync(recursive: true);
  for (final relativePath in [
    screenshotPath,
    preprocessedScreenshotPath,
    ocrOutputPath,
    logPath,
  ]) {
    final file = File('${absolutePackage.path}/$relativePath');
    if (file.existsSync()) file.deleteSync();
  }

  final logFile = File('${absolutePackage.path}/$logPath');
  final logSink = logFile.openWrite();
  final flutterRunArgs = [
    'run',
    '-d',
    deviceId ?? quickstartSmokeDefaultDeviceIdForTesting(target),
    if (target == QuickstartSmokeTarget.web) ...[
      '--web-header=Cross-Origin-Opener-Policy=same-origin',
      '--web-header=Cross-Origin-Embedder-Policy=require-corp',
    ],
  ];
  final environment = <String, String>{
    if (Platform.isLinux) 'DISPLAY': Platform.environment['DISPLAY'] ?? ':99',
  };
  final chromeWrapper = await _createChromeWrapperIfNeeded(
    absoluteArtifactDir.path,
  );
  if (chromeWrapper != null) {
    environment['CHROME_EXECUTABLE'] = chromeWrapper.path;
  }

  final process = await Process.start(
    'flutter',
    flutterRunArgs,
    workingDirectory: absolutePackage.path,
    environment: environment,
    runInShell: Platform.isWindows,
  );
  int? observedExitCode;
  final exitCodeFuture = process.exitCode;
  unawaited(exitCodeFuture.then((exitCode) => observedExitCode = exitCode));
  final outputBuffer = StringBuffer();
  process.stdout.transform(systemEncoding.decoder).listen((text) {
    outputBuffer.write(text);
    logSink.write(text);
  });
  process.stderr.transform(systemEncoding.decoder).listen((text) {
    outputBuffer.write(text);
    logSink.write(text);
  });

  final deadline = DateTime.now().add(const Duration(seconds: 75));
  const failurePatterns = [
    'DataCloneError',
    'Failed to execute \'postMessage\' on \'Worker\'',
    'fail to create WorkerPool',
    'Failed to initialize',
    'WebAssembly.instantiate',
  ];
  while (DateTime.now().isBefore(deadline)) {
    await Future<void>.delayed(const Duration(seconds: 1));
    final output = outputBuffer.toString();
    if (failurePatterns.any(output.contains)) break;
    if (observedExitCode != null) break;
  }

  final screenshotFile = File('${absolutePackage.path}/$screenshotPath');
  await _captureQuickstartSmokeScreenshot(
    target: target,
    screenshotFile: screenshotFile,
  );
  final preprocessedScreenshotFile = File(
    '${absolutePackage.path}/$preprocessedScreenshotPath',
  );
  await _preprocessQuickstartSmokeScreenshot(
    screenshotFile: screenshotFile,
    preprocessedScreenshotFile: preprocessedScreenshotFile,
  );
  final ocrOutputFile = File('${absolutePackage.path}/$ocrOutputPath');
  await _runQuickstartSmokeOcr(
    preprocessedScreenshotFile: preprocessedScreenshotFile,
    ocrOutputFile: ocrOutputFile,
  );

  final killedBySmoke = observedExitCode == null;
  if (killedBySmoke) {
    process.kill();
  }
  final exitCode = await exitCodeFuture;
  await logSink.close();
  if (chromeWrapper != null && chromeWrapper.existsSync()) {
    chromeWrapper.deleteSync();
  }

  final combinedOutput = outputBuffer.toString();
  print('\n===== flutter run log ($targetName) =====');
  print(logFile.readAsStringSync());
  if (ocrOutputFile.existsSync()) {
    print('\n===== screenshot OCR ($targetName) =====');
    print(ocrOutputFile.readAsStringSync());
  }

  for (final pattern in failurePatterns) {
    if (combinedOutput.contains(pattern)) {
      throw Exception(
        'flutter_via_create $targetName quickstart smoke test failed '
        'with `$pattern`',
      );
    }
  }

  if (!screenshotFile.existsSync() || screenshotFile.lengthSync() == 0) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed to capture '
      'a screenshot at `$screenshotPath`',
    );
  }

  if (!ocrOutputFile.existsSync()) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test did not produce OCR '
      'output at `$ocrOutputPath`',
    );
  }
  checkQuickstartSmokeOcrTextForTesting(ocrOutputFile.readAsStringSync());

  if (!killedBySmoke && exitCode != 0) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed with '
      'unexpected exit code $exitCode',
    );
  }
}

@visibleForTesting
String quickstartSmokeDefaultDeviceIdForTesting(QuickstartSmokeTarget target) {
  if (target == QuickstartSmokeTarget.web) return 'chrome';
  if (target == QuickstartSmokeTarget.android) return 'emulator-5554';
  if (target == QuickstartSmokeTarget.desktop) {
    if (Platform.isWindows) return 'windows';
    if (Platform.isMacOS) return 'macos';
    if (Platform.isLinux) return 'linux';
  }
  throw Exception('No default quickstart smoke device for `$target`');
}

@visibleForTesting
String quickstartSmokePackagePathForTesting(
  String package, {
  String? repoRootPath,
}) => Directory('${repoRootPath ?? exec.pwd}$package').absolute.path;

Future<File?> _createChromeWrapperIfNeeded(String artifactDir) async {
  if (!Platform.isLinux || Platform.environment['USER'] != 'root') {
    return null;
  }
  final file = File('$artifactDir/chrome-no-sandbox-wrapper');
  await file.writeAsString(
    '#!/bin/sh\nexec google-chrome --no-sandbox "\$@"\n',
  );
  await Process.run('chmod', ['+x', file.path]);
  return file;
}

Future<void> _captureQuickstartSmokeScreenshot({
  required QuickstartSmokeTarget target,
  required File screenshotFile,
}) async {
  final result = switch (target) {
    QuickstartSmokeTarget.android => await Process.run(
      'adb',
      ['exec-out', 'screencap', '-p'],
      stdoutEncoding: null,
      stderrEncoding: systemEncoding,
    ),
    QuickstartSmokeTarget.ios => await Process.run('xcrun', [
      'simctl',
      'io',
      'booted',
      'screenshot',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ when Platform.isMacOS => await Process.run('screencapture', [
      '-x',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ when Platform.isWindows => await Process.run('powershell', [
      '-NoProfile',
      '-Command',
      r'''
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$bitmap.Save($args[0], [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
''',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
    _ => await Process.run('import', [
      '-window',
      'root',
      screenshotFile.path,
    ], stderrEncoding: systemEncoding),
  };
  if (target == QuickstartSmokeTarget.android && result.exitCode == 0) {
    screenshotFile.writeAsBytesSync(result.stdout as List<int>);
  }
  if (result.exitCode != 0) {
    throw Exception(
      'Failed to capture quickstart screenshot (exitCode=${result.exitCode}, '
      'stderr=${result.stderr})',
    );
  }
}

Future<void> _preprocessQuickstartSmokeScreenshot({
  required File screenshotFile,
  required File preprocessedScreenshotFile,
}) async {
  final result = await Process.run(Platform.isWindows ? 'magick' : 'convert', [
    screenshotFile.path,
    '-resize',
    '300%',
    '-colorspace',
    'Gray',
    '-normalize',
    preprocessedScreenshotFile.path,
  ], stderrEncoding: systemEncoding);
  if (result.exitCode != 0 || !preprocessedScreenshotFile.existsSync()) {
    screenshotFile.copySync(preprocessedScreenshotFile.path);
  }
}

Future<void> _runQuickstartSmokeOcr({
  required File preprocessedScreenshotFile,
  required File ocrOutputFile,
}) async {
  final outputBase = ocrOutputFile.path.replaceAll(RegExp(r'\.txt$'), '');
  final result = await Process.run('tesseract', [
    preprocessedScreenshotFile.path,
    outputBase,
  ], stderrEncoding: systemEncoding);
  if (result.exitCode != 0) {
    throw Exception(
      'Failed to OCR quickstart screenshot (exitCode=${result.exitCode}, '
      'stderr=${result.stderr})',
    );
  }
}

@visibleForTesting
String normalizeQuickstartSmokeOcrTextForTesting(String text) => text
    .toLowerCase()
    .replaceAll(RegExp(r'[^a-z0-9]+'), ' ')
    .replaceAll(RegExp(r'\s+'), ' ')
    .trim();

@visibleForTesting
void checkQuickstartSmokeOcrTextForTesting(String text) {
  final normalized = normalizeQuickstartSmokeOcrTextForTesting(text);
  final hasExpectedText =
      normalized.contains('hello') && normalized.contains('tom');
  if (!hasExpectedText) {
    throw Exception(
      'flutter_via_create quickstart smoke test screenshot OCR did not '
      'contain expected text `Hello, Tom` (normalized OCR: `$normalized`)',
    );
  }
}
