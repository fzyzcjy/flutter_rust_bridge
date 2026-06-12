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
  final context = await _QuickstartSmokeContext.create(
    package: package,
    target: target,
    deviceId: deviceId,
  );

  print(
    'Start quickstart smoke: package=${context.package}, '
    'target=${context.targetName}, device=${context.resolvedDeviceId}',
  );
  print('Quickstart smoke artifacts: ${context.absoluteArtifactDir.path}');

  _QuickstartSmokeFlutterRun? flutterRun;
  try {
    flutterRun = await _startQuickstartSmokeFlutterRun(context);
    final activeFlutterRun = flutterRun;
    final readyForScreenshot = await _waitForQuickstartSmokeFlutterRunReady(
      context: context,
      flutterRun: activeFlutterRun,
    );
    final screenshotEvidence = await _captureQuickstartSmokeEvidence(
      context: context,
      readyForScreenshot: readyForScreenshot,
    );
    final exitStatus = await _stopQuickstartSmokeFlutterRun(
      flutterRun: activeFlutterRun,
    );
    await activeFlutterRun.closeLog();
    _printQuickstartSmokeEvidence(context);
    _validateQuickstartSmokeResult(
      context: context,
      combinedOutput: activeFlutterRun.combinedOutput,
      screenshotEvidence: screenshotEvidence,
      exitStatus: exitStatus,
    );
  } finally {
    await flutterRun?.closeLog();
    context.deleteTemporaryFiles();
  }
}

class _QuickstartSmokeContext {
  static const artifactDir = 'target/quickstart_smoke';

  final String package;
  final QuickstartSmokeTarget target;
  final String targetName;
  final String resolvedDeviceId;
  final Directory absolutePackage;
  final Directory absoluteArtifactDir;
  final File screenshotFile;
  final File preprocessedScreenshotFile;
  final File ocrOutputFile;
  final File preprocessedOcrOutputFile;
  final File logFile;
  final File? chromeWrapper;

  const _QuickstartSmokeContext({
    required this.package,
    required this.target,
    required this.targetName,
    required this.resolvedDeviceId,
    required this.absolutePackage,
    required this.absoluteArtifactDir,
    required this.screenshotFile,
    required this.preprocessedScreenshotFile,
    required this.ocrOutputFile,
    required this.preprocessedOcrOutputFile,
    required this.logFile,
    required this.chromeWrapper,
  });

  static Future<_QuickstartSmokeContext> create({
    required String package,
    required QuickstartSmokeTarget target,
    required String? deviceId,
  }) async {
    final targetName = target.name;
    final absolutePackage = Directory(
      quickstartSmokePackagePathForTesting(package),
    );
    final absoluteArtifactDir = Directory(
      '${absolutePackage.path}/$artifactDir',
    );
    absoluteArtifactDir.createSync(recursive: true);

    final context = _QuickstartSmokeContext(
      package: package,
      target: target,
      targetName: targetName,
      resolvedDeviceId:
          deviceId ?? quickstartSmokeDefaultDeviceIdForTesting(target),
      absolutePackage: absolutePackage,
      absoluteArtifactDir: absoluteArtifactDir,
      screenshotFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName.png',
      ),
      preprocessedScreenshotFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName-processed.png',
      ),
      ocrOutputFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName.txt',
      ),
      preprocessedOcrOutputFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName-processed.txt',
      ),
      logFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName.log',
      ),
      chromeWrapper: await _createChromeWrapperIfNeeded(
        absoluteArtifactDir.path,
      ),
    );
    context.clearArtifacts();
    return context;
  }

  List<String> get flutterRunArgs => [
    'run',
    '-d',
    resolvedDeviceId,
    if (target == QuickstartSmokeTarget.web) ...[
      '--web-header=Cross-Origin-Opener-Policy=same-origin',
      '--web-header=Cross-Origin-Embedder-Policy=require-corp',
    ],
  ];

  Map<String, String> get environment => {
    if (Platform.isLinux) 'DISPLAY': Platform.environment['DISPLAY'] ?? ':99',
    if (chromeWrapper != null) 'CHROME_EXECUTABLE': chromeWrapper!.path,
  };

  void clearArtifacts() {
    for (final file in [
      screenshotFile,
      preprocessedScreenshotFile,
      ocrOutputFile,
      preprocessedOcrOutputFile,
      logFile,
    ]) {
      if (file.existsSync()) file.deleteSync();
    }
  }

  void deleteTemporaryFiles() {
    if (chromeWrapper != null && chromeWrapper!.existsSync()) {
      chromeWrapper!.deleteSync();
    }
  }
}

class _QuickstartSmokeFlutterRun {
  final Process process;
  final IOSink logSink;
  final Future<int> exitCodeFuture;
  final StringBuffer outputBuffer = StringBuffer();
  var _logClosed = false;

  int? observedExitCode;

  _QuickstartSmokeFlutterRun({required this.process, required this.logSink})
    : exitCodeFuture = process.exitCode {
    unawaited(exitCodeFuture.then((exitCode) => observedExitCode = exitCode));
  }

  String get combinedOutput => outputBuffer.toString();

  void recordOutput(String text) {
    outputBuffer.write(text);
    logSink.write(text);
  }

  Future<void> closeLog() async {
    if (_logClosed) return;
    _logClosed = true;
    await logSink.close();
  }
}

class _QuickstartSmokeScreenshotEvidence {
  final bool readyForScreenshot;
  final bool ocrMatched;
  final Exception? lastOcrException;

  const _QuickstartSmokeScreenshotEvidence({
    required this.readyForScreenshot,
    required this.ocrMatched,
    required this.lastOcrException,
  });
}

class _QuickstartSmokeExitStatus {
  final bool killedBySmoke;
  final int? exitCode;

  const _QuickstartSmokeExitStatus({
    required this.killedBySmoke,
    required this.exitCode,
  });
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
  QuickstartSmokeTarget.android => const Duration(minutes: 5),
  QuickstartSmokeTarget.ios => const Duration(minutes: 10),
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

Future<_QuickstartSmokeFlutterRun> _startQuickstartSmokeFlutterRun(
  _QuickstartSmokeContext context,
) async {
  print(
    'Starting `flutter ${context.flutterRunArgs.join(' ')}` '
    'in ${context.absolutePackage.path}',
  );
  if (context.environment.isNotEmpty) {
    print('Quickstart smoke environment overrides: ${context.environment}');
  }

  final process = await Process.start(
    'flutter',
    context.flutterRunArgs,
    workingDirectory: context.absolutePackage.path,
    environment: context.environment,
    runInShell: Platform.isWindows,
  );
  print('Started flutter run pid=${process.pid}');

  final result = _QuickstartSmokeFlutterRun(
    process: process,
    logSink: context.logFile.openWrite(),
  );
  process.stdout.transform(systemEncoding.decoder).listen(result.recordOutput);
  process.stderr.transform(systemEncoding.decoder).listen(result.recordOutput);
  return result;
}

Future<bool> _waitForQuickstartSmokeFlutterRunReady({
  required _QuickstartSmokeContext context,
  required _QuickstartSmokeFlutterRun flutterRun,
}) async {
  final timeout = quickstartSmokeFlutterRunReadyTimeoutForTesting(
    context.target,
  );
  final deadline = DateTime.now().add(timeout);
  print('Waiting up to $timeout for flutter run readiness');

  while (DateTime.now().isBefore(deadline)) {
    await Future<void>.delayed(const Duration(seconds: 1));
    final output = flutterRun.combinedOutput;
    final failurePattern = quickstartSmokeOutputFailurePatternForTesting(
      output,
    );
    if (failurePattern != null) {
      print('Observed early flutter run failure pattern: $failurePattern');
      return false;
    }
    if (quickstartSmokeFlutterRunIsReadyForTesting(output)) {
      print('Observed flutter run readiness; screenshot capture can start');
      return true;
    }
    if (flutterRun.observedExitCode != null) {
      print(
        'flutter run exited before readiness '
        '(exitCode=${flutterRun.observedExitCode})',
      );
      return false;
    }
  }

  print('Timed out waiting for flutter run readiness');
  return false;
}

Future<_QuickstartSmokeScreenshotEvidence> _captureQuickstartSmokeEvidence({
  required _QuickstartSmokeContext context,
  required bool readyForScreenshot,
}) async {
  if (!readyForScreenshot) {
    print('Capturing one diagnostic screenshot before failing readiness check');
    await _captureAndOcrQuickstartSmokeScreenshotFromContext(context);
    return _QuickstartSmokeScreenshotEvidence(
      readyForScreenshot: readyForScreenshot,
      ocrMatched: false,
      lastOcrException: null,
    );
  }

  final visibleTextTimeout = quickstartSmokeVisibleTextTimeoutForTesting(
    context.target,
  );
  final visibleTextDeadline = DateTime.now().add(visibleTextTimeout);
  print(
    'Waiting up to $visibleTextTimeout for quickstart text to become OCR-visible',
  );
  await Future<void>.delayed(const Duration(seconds: 5));

  Exception? lastOcrException;
  var attempt = 0;
  do {
    attempt++;
    print('Capturing quickstart screenshot/OCR attempt #$attempt');
    await _captureAndOcrQuickstartSmokeScreenshotFromContext(context);
    try {
      checkQuickstartSmokeOcrTextForTesting(
        context.ocrOutputFile.readAsStringSync(),
      );
      print('Quickstart screenshot OCR contains expected text');
      return _QuickstartSmokeScreenshotEvidence(
        readyForScreenshot: readyForScreenshot,
        ocrMatched: true,
        lastOcrException: null,
      );
    } on Exception catch (exception) {
      lastOcrException = exception;
      print('Quickstart screenshot OCR did not match yet: $exception');
      if (!DateTime.now().isBefore(visibleTextDeadline)) break;
      await Future<void>.delayed(const Duration(seconds: 5));
    }
  } while (DateTime.now().isBefore(visibleTextDeadline));

  return _QuickstartSmokeScreenshotEvidence(
    readyForScreenshot: readyForScreenshot,
    ocrMatched: false,
    lastOcrException: lastOcrException,
  );
}

Future<_QuickstartSmokeExitStatus> _stopQuickstartSmokeFlutterRun({
  required _QuickstartSmokeFlutterRun flutterRun,
}) async {
  final killedBySmoke = flutterRun.observedExitCode == null;
  if (killedBySmoke) {
    print('Terminating flutter run after successful evidence capture');
    await _terminateQuickstartSmokeProcess(flutterRun.process);
  }

  final exitCode = killedBySmoke
      ? await _waitForQuickstartSmokeExit(flutterRun.exitCodeFuture)
      : await flutterRun.exitCodeFuture;
  print(
    'flutter run exit status: exitCode=$exitCode, killedBySmoke=$killedBySmoke',
  );
  return _QuickstartSmokeExitStatus(
    killedBySmoke: killedBySmoke,
    exitCode: exitCode,
  );
}

void _printQuickstartSmokeEvidence(_QuickstartSmokeContext context) {
  print('\n===== flutter run log (${context.targetName}) =====');
  print(context.logFile.readAsStringSync());
  if (context.ocrOutputFile.existsSync()) {
    print('\n===== screenshot OCR (${context.targetName}) =====');
    print(context.ocrOutputFile.readAsStringSync());
  }
}

void _validateQuickstartSmokeResult({
  required _QuickstartSmokeContext context,
  required String combinedOutput,
  required _QuickstartSmokeScreenshotEvidence screenshotEvidence,
  required _QuickstartSmokeExitStatus exitStatus,
}) {
  final failurePattern = quickstartSmokeOutputFailurePatternForTesting(
    combinedOutput,
  );
  if (failurePattern != null) {
    throw Exception(
      'flutter_via_create ${context.targetName} quickstart smoke test failed '
      'with `$failurePattern`',
    );
  }

  if (!context.screenshotFile.existsSync() ||
      context.screenshotFile.lengthSync() == 0) {
    throw Exception(
      'flutter_via_create ${context.targetName} quickstart smoke test failed '
      'to capture a screenshot at `${context.screenshotFile.path}`',
    );
  }

  if (!screenshotEvidence.readyForScreenshot) {
    throw Exception(
      'flutter_via_create ${context.targetName} quickstart smoke test did not '
      'observe Flutter run readiness before screenshot capture',
    );
  }

  if (!context.ocrOutputFile.existsSync()) {
    throw Exception(
      'flutter_via_create ${context.targetName} quickstart smoke test did not '
      'produce OCR output at `${context.ocrOutputFile.path}`',
    );
  }
  if (!screenshotEvidence.ocrMatched) {
    if (screenshotEvidence.lastOcrException != null) {
      throw screenshotEvidence.lastOcrException!;
    }
    checkQuickstartSmokeOcrTextForTesting(
      context.ocrOutputFile.readAsStringSync(),
    );
  }

  if (!exitStatus.killedBySmoke && exitStatus.exitCode != 0) {
    throw Exception(
      'flutter_via_create ${context.targetName} quickstart smoke test failed '
      'with unexpected exit code ${exitStatus.exitCode}',
    );
  }
}

Future<void> _captureAndOcrQuickstartSmokeScreenshotFromContext(
  _QuickstartSmokeContext context,
) async {
  await _captureAndOcrQuickstartSmokeScreenshot(
    target: context.target,
    screenshotFile: context.screenshotFile,
    preprocessedScreenshotFile: context.preprocessedScreenshotFile,
    ocrOutputFile: context.ocrOutputFile,
    preprocessedOcrOutputFile: context.preprocessedOcrOutputFile,
  );
}

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
