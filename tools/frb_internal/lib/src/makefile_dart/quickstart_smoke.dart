// ignore_for_file: avoid_print

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
  final preprocessedOcrOutputPath =
      '$artifactDir/quickstart-$targetName-processed.txt';
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
    preprocessedOcrOutputPath,
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

  final deadline = DateTime.now().add(
    quickstartSmokeFlutterRunReadyTimeoutForTesting(target),
  );
  var readyForScreenshot = false;
  while (DateTime.now().isBefore(deadline)) {
    await Future<void>.delayed(const Duration(seconds: 1));
    final output = outputBuffer.toString();
    if (quickstartSmokeOutputHasFailurePatternForTesting(output)) break;
    if (quickstartSmokeFlutterRunIsReadyForTesting(output)) {
      readyForScreenshot = true;
      break;
    }
    if (observedExitCode != null) break;
  }
  var ocrMatched = false;
  Exception? lastOcrException;
  if (readyForScreenshot) {
    final visibleTextDeadline = DateTime.now().add(
      quickstartSmokeVisibleTextTimeoutForTesting(target),
    );
    await Future<void>.delayed(const Duration(seconds: 5));
    do {
      await _captureAndOcrQuickstartSmokeScreenshot(
        target: target,
        screenshotFile: File('${absolutePackage.path}/$screenshotPath'),
        preprocessedScreenshotFile: File(
          '${absolutePackage.path}/$preprocessedScreenshotPath',
        ),
        ocrOutputFile: File('${absolutePackage.path}/$ocrOutputPath'),
        preprocessedOcrOutputFile: File(
          '${absolutePackage.path}/$preprocessedOcrOutputPath',
        ),
      );
      try {
        checkQuickstartSmokeOcrTextForTesting(
          File('${absolutePackage.path}/$ocrOutputPath').readAsStringSync(),
        );
        ocrMatched = true;
        break;
      } on Exception catch (exception) {
        lastOcrException = exception;
        if (!DateTime.now().isBefore(visibleTextDeadline)) break;
        await Future<void>.delayed(const Duration(seconds: 5));
      }
    } while (DateTime.now().isBefore(visibleTextDeadline));
  } else {
    await _captureAndOcrQuickstartSmokeScreenshot(
      target: target,
      screenshotFile: File('${absolutePackage.path}/$screenshotPath'),
      preprocessedScreenshotFile: File(
        '${absolutePackage.path}/$preprocessedScreenshotPath',
      ),
      ocrOutputFile: File('${absolutePackage.path}/$ocrOutputPath'),
      preprocessedOcrOutputFile: File(
        '${absolutePackage.path}/$preprocessedOcrOutputPath',
      ),
    );
  }

  final killedBySmoke = observedExitCode == null;
  if (killedBySmoke) {
    await _terminateQuickstartSmokeProcess(process);
  }
  final exitCode = killedBySmoke
      ? await _waitForQuickstartSmokeExit(exitCodeFuture)
      : await exitCodeFuture;
  await logSink.close();
  if (chromeWrapper != null && chromeWrapper.existsSync()) {
    chromeWrapper.deleteSync();
  }

  final combinedOutput = outputBuffer.toString();
  final screenshotFile = File('${absolutePackage.path}/$screenshotPath');
  final ocrOutputFile = File('${absolutePackage.path}/$ocrOutputPath');
  print('\n===== flutter run log ($targetName) =====');
  print(logFile.readAsStringSync());
  if (ocrOutputFile.existsSync()) {
    print('\n===== screenshot OCR ($targetName) =====');
    print(ocrOutputFile.readAsStringSync());
  }

  final failurePattern = quickstartSmokeOutputFailurePatternForTesting(
    combinedOutput,
  );
  if (failurePattern != null) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed '
      'with `$failurePattern`',
    );
  }

  if (!screenshotFile.existsSync() || screenshotFile.lengthSync() == 0) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test failed to capture '
      'a screenshot at `$screenshotPath`',
    );
  }

  if (!readyForScreenshot) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test did not observe '
      'Flutter run readiness before screenshot capture',
    );
  }

  if (!ocrOutputFile.existsSync()) {
    throw Exception(
      'flutter_via_create $targetName quickstart smoke test did not produce OCR '
      'output at `$ocrOutputPath`',
    );
  }
  if (!ocrMatched) {
    if (lastOcrException != null) throw lastOcrException;
    checkQuickstartSmokeOcrTextForTesting(ocrOutputFile.readAsStringSync());
  }

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
Duration quickstartSmokeFlutterRunReadyTimeoutForTesting(
  QuickstartSmokeTarget target,
) => switch (target) {
  QuickstartSmokeTarget.web => const Duration(seconds: 120),
  QuickstartSmokeTarget.desktop ||
  QuickstartSmokeTarget.android ||
  QuickstartSmokeTarget.ios => const Duration(minutes: 5),
};

@visibleForTesting
Duration quickstartSmokeVisibleTextTimeoutForTesting(
  QuickstartSmokeTarget target,
) => switch (target) {
  QuickstartSmokeTarget.web => const Duration(seconds: 45),
  QuickstartSmokeTarget.desktop ||
  QuickstartSmokeTarget.android ||
  QuickstartSmokeTarget.ios => const Duration(seconds: 90),
};

@visibleForTesting
String? quickstartSmokeOutputFailurePatternForTesting(String output) {
  const failurePatterns = [
    'DataCloneError',
    'Failed to execute \'postMessage\' on \'Worker\'',
    'fail to create WorkerPool',
    'WebAssembly.instantiate',
  ];
  for (final pattern in failurePatterns) {
    if (output.contains(pattern)) return pattern;
  }
  return null;
}

@visibleForTesting
bool quickstartSmokeOutputHasFailurePatternForTesting(String output) =>
    quickstartSmokeOutputFailurePatternForTesting(output) != null;

@visibleForTesting
bool quickstartSmokeFlutterRunIsReadyForTesting(String output) {
  const readyPatterns = [
    'Flutter run key commands.',
    'Debug service listening on',
    'A Dart VM Service',
  ];
  return readyPatterns.any(output.contains);
}

@visibleForTesting
String quickstartSmokePackagePathForTesting(
  String package, {
  String? repoRootPath,
}) => Directory('${repoRootPath ?? exec.pwd}$package').absolute.path;

Future<File?> _createChromeWrapperIfNeeded(String artifactDir) async {
  if (!Platform.isLinux || !await _isRunningAsRoot()) {
    return null;
  }
  final file = File('$artifactDir/chrome-no-sandbox-wrapper');
  await file.writeAsString(
    '#!/bin/sh\nexec google-chrome --no-sandbox "\$@"\n',
  );
  await Process.run('chmod', ['+x', file.path]);
  return file;
}

Future<bool> _isRunningAsRoot() async {
  final result = await Process.run('id', [
    '-u',
  ], stdoutEncoding: systemEncoding);
  return result.exitCode == 0 && result.stdout.toString().trim() == '0';
}

Future<void> _terminateQuickstartSmokeProcess(Process process) async {
  if (Platform.isWindows) {
    await Process.run('taskkill', [
      '/PID',
      process.pid.toString(),
      '/T',
      '/F',
    ], stderrEncoding: systemEncoding);
    return;
  }
  process.kill();
}

Future<int?> _waitForQuickstartSmokeExit(Future<int> exitCodeFuture) async {
  try {
    return await exitCodeFuture.timeout(const Duration(seconds: 15));
  } on TimeoutException {
    return null;
  }
}

Future<void> _captureAndOcrQuickstartSmokeScreenshot({
  required QuickstartSmokeTarget target,
  required File screenshotFile,
  required File preprocessedScreenshotFile,
  required File ocrOutputFile,
  required File preprocessedOcrOutputFile,
}) async {
  await _captureQuickstartSmokeScreenshot(
    target: target,
    screenshotFile: screenshotFile,
  );
  await _preprocessQuickstartSmokeScreenshot(
    screenshotFile: screenshotFile,
    preprocessedScreenshotFile: preprocessedScreenshotFile,
  );

  await _runQuickstartSmokeOcr(
    imageFile: screenshotFile,
    ocrOutputFile: ocrOutputFile,
  );
  await _runQuickstartSmokeOcr(
    imageFile: preprocessedScreenshotFile,
    ocrOutputFile: preprocessedOcrOutputFile,
  );
  ocrOutputFile.writeAsStringSync(
    [
      '===== original screenshot OCR =====',
      if (ocrOutputFile.existsSync()) ocrOutputFile.readAsStringSync(),
      '===== preprocessed screenshot OCR =====',
      if (preprocessedOcrOutputFile.existsSync())
        preprocessedOcrOutputFile.readAsStringSync(),
    ].join('\n'),
  );
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
    _ when Platform.isWindows => await _captureWindowsQuickstartSmokeScreenshot(
      screenshotFile,
    ),
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

Future<ProcessResult> _captureWindowsQuickstartSmokeScreenshot(
  File screenshotFile,
) async {
  final scriptFile = File(
    '${screenshotFile.parent.path}/capture-quickstart-screenshot.ps1',
  );
  await scriptFile.writeAsString(r'''
param([string]$OutputPath)
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$bitmap.Save($OutputPath, [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
''');
  try {
    return await Process.run('powershell', [
      '-NoProfile',
      '-ExecutionPolicy',
      'Bypass',
      '-File',
      scriptFile.path,
      screenshotFile.path,
    ], stderrEncoding: systemEncoding);
  } finally {
    if (await scriptFile.exists()) {
      await scriptFile.delete();
    }
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
  required File imageFile,
  required File ocrOutputFile,
}) async {
  final outputBase = ocrOutputFile.path.replaceAll(RegExp(r'\.txt$'), '');
  final result = await Process.run('tesseract', [
    imageFile.path,
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
