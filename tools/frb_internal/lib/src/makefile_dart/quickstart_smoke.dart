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
    if (Platform.isMacOS && context.target == QuickstartSmokeTarget.desktop) {
      await _recordMacosQuickstartSmokeDiagnostics(context, label: 'start');
    }
    flutterRun = await _startQuickstartSmokeFlutterRun(context);
    final activeFlutterRun = flutterRun;
    final readyForScreenshot = await _waitForQuickstartSmokeFlutterRunReady(
      context: context,
      flutterRun: activeFlutterRun,
    );
    final screenshotEvidence = await _captureQuickstartSmokeEvidence(
      context: context,
      readyForScreenshot: readyForScreenshot,
      flutterRun: activeFlutterRun,
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

  File get macosDiagnosticsFile => File(
    '${absoluteArtifactDir.path}/quickstart-$targetName-macos-diagnostics.txt',
  );

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

  List<String> get flutterRunArgs => quickstartSmokeFlutterRunArgsForTesting(
    target: target,
    deviceId: resolvedDeviceId,
  );

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
      macosDiagnosticsFile,
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

@visibleForTesting
List<String> quickstartSmokeFlutterRunArgsForTesting({
  required QuickstartSmokeTarget target,
  required String deviceId,
}) => [
  'run',
  '-d',
  deviceId,
  if (target == QuickstartSmokeTarget.android) '--no-dds',
  if (target == QuickstartSmokeTarget.web) ...[
    '--web-header=Cross-Origin-Opener-Policy=same-origin',
    '--web-header=Cross-Origin-Embedder-Policy=require-corp',
  ],
];

class _QuickstartSmokeFlutterRun {
  final Process process;
  final IOSink logSink;
  final Future<int> exitCodeFuture;
  final StringBuffer outputBuffer = StringBuffer();
  final List<StreamSubscription<String>> _outputSubscriptions = [];
  final List<Future<void>> _outputDoneFutures = [];
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

  void listenToOutput() {
    for (final stream in [process.stdout, process.stderr]) {
      final subscription = stream
          .transform(systemEncoding.decoder)
          .listen(recordOutput);
      _outputSubscriptions.add(subscription);
      _outputDoneFutures.add(subscription.asFuture<void>());
    }
  }

  Future<void> closeLog() async {
    if (_logClosed) return;
    _logClosed = true;
    try {
      await Future.any<void>([
        Future.wait(_outputDoneFutures).then((_) {}),
        Future<void>.delayed(const Duration(seconds: 1)),
      ]);
    } finally {
      await Future.wait(
        _outputSubscriptions.map((subscription) => subscription.cancel()),
      );
      await logSink.close();
    }
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
  QuickstartSmokeTarget target, {
  bool? isMacOS,
}) {
  final effectiveIsMacOS = isMacOS ?? Platform.isMacOS;
  return switch (target) {
    QuickstartSmokeTarget.web => const Duration(seconds: 120),
    QuickstartSmokeTarget.desktop when effectiveIsMacOS => const Duration(
      minutes: 10,
    ),
    QuickstartSmokeTarget.desktop => const Duration(minutes: 5),
    // Gradle downloads and unzips the NDK on every CI run before the app
    // starts, which alone regularly takes 3-4 minutes.
    QuickstartSmokeTarget.android ||
    QuickstartSmokeTarget.ios => const Duration(minutes: 10),
  };
}

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

@visibleForTesting
List<String> quickstartSmokeAndroidScreenshotArgsForTesting(String deviceId) =>
    ['-s', deviceId, 'exec-out', 'screencap', '-p'];

@visibleForTesting
List<String> quickstartSmokeIosScreenshotArgsForTesting({
  required String deviceId,
  required String screenshotPath,
}) => ['simctl', 'io', deviceId, 'screenshot', screenshotPath];

@visibleForTesting
List<String> quickstartSmokeMacosScreenshotArgsForTesting(
  String screenshotPath,
) => ['-x', '-T', '1', screenshotPath];

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
  result.listenToOutput();
  return result;
}

Future<bool> _waitForQuickstartSmokeFlutterRunReady({
  required _QuickstartSmokeContext context,
  required _QuickstartSmokeFlutterRun flutterRun,
}) async {
  final timeout = quickstartSmokeFlutterRunReadyTimeoutForTesting(
    context.target,
    isMacOS: Platform.isMacOS,
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
  required _QuickstartSmokeFlutterRun flutterRun,
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

  if (Platform.isMacOS && context.target == QuickstartSmokeTarget.desktop) {
    await _dumpQuickstartSmokeFlutterTrees(flutterRun);
  }

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
      '${context.package} ${context.targetName} quickstart smoke test failed '
      'with `$failurePattern`',
    );
  }

  if (!context.screenshotFile.existsSync() ||
      context.screenshotFile.lengthSync() == 0) {
    throw Exception(
      '${context.package} ${context.targetName} quickstart smoke test failed '
      'to capture a screenshot at `${context.screenshotFile.path}`',
    );
  }

  if (!screenshotEvidence.readyForScreenshot) {
    throw Exception(
      '${context.package} ${context.targetName} quickstart smoke test did not '
      'observe Flutter run readiness before screenshot capture',
    );
  }

  if (!context.ocrOutputFile.existsSync()) {
    throw Exception(
      '${context.package} ${context.targetName} quickstart smoke test did not '
      'produce OCR output at `${context.ocrOutputFile.path}`',
    );
  }
  if (!screenshotEvidence.ocrMatched) {
    if (screenshotEvidence.lastOcrException != null) {
      throw screenshotEvidence.lastOcrException!;
    }
    checkQuickstartSmokeOcrTextForTesting(
      context.ocrOutputFile.readAsStringSync(),
      package: context.package,
    );
  }

  if (!exitStatus.killedBySmoke && exitStatus.exitCode != 0) {
    throw Exception(
      '${context.package} ${context.targetName} quickstart smoke test failed '
      'with unexpected exit code ${exitStatus.exitCode}',
    );
  }
}

Future<void> _captureAndOcrQuickstartSmokeScreenshotFromContext(
  _QuickstartSmokeContext context,
) async {
  if (Platform.isMacOS && context.target == QuickstartSmokeTarget.desktop) {
    await _activateMacosQuickstartSmokeApp(context.absolutePackage);
  }

  await _captureAndOcrQuickstartSmokeScreenshot(
    target: context.target,
    deviceId: context.resolvedDeviceId,
    screenshotFile: context.screenshotFile,
    preprocessedScreenshotFile: context.preprocessedScreenshotFile,
    ocrOutputFile: context.ocrOutputFile,
    preprocessedOcrOutputFile: context.preprocessedOcrOutputFile,
  );

  if (Platform.isMacOS && context.target == QuickstartSmokeTarget.desktop) {
    await _recordMacosQuickstartSmokeDiagnostics(
      context,
      label: 'after capture',
    );
  }
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
  required String deviceId,
  required File screenshotFile,
  required File preprocessedScreenshotFile,
  required File ocrOutputFile,
  required File preprocessedOcrOutputFile,
}) async {
  await _captureQuickstartSmokeScreenshot(
    target: target,
    deviceId: deviceId,
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
  required String deviceId,
  required File screenshotFile,
}) async {
  final result = switch (target) {
    QuickstartSmokeTarget.android => await Process.run(
      'adb',
      quickstartSmokeAndroidScreenshotArgsForTesting(deviceId),
      stdoutEncoding: null,
      stderrEncoding: systemEncoding,
    ),
    QuickstartSmokeTarget.ios => await Process.run(
      'xcrun',
      quickstartSmokeIosScreenshotArgsForTesting(
        deviceId: deviceId,
        screenshotPath: screenshotFile.path,
      ),
      stderrEncoding: systemEncoding,
    ),
    _ when Platform.isMacOS => await _captureMacosQuickstartSmokeScreenshot(
      screenshotFile,
    ),
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

Future<ProcessResult> _captureMacosQuickstartSmokeScreenshot(
  File screenshotFile,
) async {
  return Process.run(
    'screencapture',
    quickstartSmokeMacosScreenshotArgsForTesting(screenshotFile.path),
    stderrEncoding: systemEncoding,
  );
}

/// Captures in CI sometimes come out fully transparent. Record the session
/// state alongside every capture so a blank one can be compared with a working
/// one.
Future<void> _recordMacosQuickstartSmokeDiagnostics(
  _QuickstartSmokeContext context, {
  required String label,
}) async {
  final screenshot = context.screenshotFile;
  final result = await Process.run('/bin/sh', [
    '-c',
    quickstartSmokeMacosDiagnosticsScriptForTesting,
    'sh',
    screenshot.existsSync() ? screenshot.path : '',
  ], stderrEncoding: systemEncoding);
  final report =
      '===== macOS diagnostics: $label (${DateTime.now().toUtc()}) =====\n'
      '${result.stdout}${result.stderr}';
  print(report);
  context.macosDiagnosticsFile.writeAsStringSync(
    '$report\n',
    mode: FileMode.append,
  );
}

/// The macOS app window sometimes stays black even though the desktop captures
/// fine. Ask `flutter run` to dump the app's trees into its log, to tell apart
/// a UI that was never built from one that was built but never presented.
Future<void> _dumpQuickstartSmokeFlutterTrees(
  _QuickstartSmokeFlutterRun flutterRun,
) async {
  // flutter run ignores keys while a previous one is still being handled.
  for (final (key, tree) in quickstartSmokeFlutterTreeDumpKeysForTesting) {
    print('Asking flutter run to dump the $tree tree');
    flutterRun.recordOutput('\n===== $tree tree dump =====\n');
    flutterRun.process.stdin.write('$key\n');
    await flutterRun.process.stdin.flush();
    await Future<void>.delayed(const Duration(seconds: 5));
  }
}

@visibleForTesting
const quickstartSmokeFlutterTreeDumpKeysForTesting = [
  ('w', 'widget'),
  ('t', 'render'),
  ('L', 'layer'),
];

@visibleForTesting
const quickstartSmokeMacosDiagnosticsScriptForTesting = r'''
run() { echo "\$ $*"; "$@" 2>&1; }
run uptime
run sysctl -n kern.boottime
ioreg -c IOHIDSystem | grep -m1 HIDIdleTime
ioreg -n Root -d1 | grep IOConsoleUsers
run pmset -g
run pmset -g assertions
run defaults -currentHost read com.apple.screensaver
run defaults read com.apple.screensaver
run pgrep -l 'ScreenSaverEngine|loginwindow'
run system_profiler SPDisplaysDataType
if [ -n "$1" ]; then
  run convert "$1" -format 'alpha max=%[fx:maxima.a] mean=%[fx:mean]\n' info:
fi
''';

Future<void> _activateMacosQuickstartSmokeApp(
  Directory packageDirectory,
) async {
  final appPath = quickstartSmokeMacosAppPathForTesting(packageDirectory);
  print('Activating macOS quickstart app: $appPath');
  final result = await Process.run('osascript', [
    '-e',
    'on run argv',
    '-e',
    'tell application (item 1 of argv) to activate',
    '-e',
    'end run',
    appPath,
  ], stderrEncoding: systemEncoding);
  if (result.exitCode != 0) {
    print(
      'Failed to activate macOS quickstart app before screenshot '
      '(exitCode=${result.exitCode}, stderr=${result.stderr})',
    );
  }
}

@visibleForTesting
String quickstartSmokeMacosAppPathForTesting(Directory packageDirectory) {
  final products = Directory(
    '${packageDirectory.path}/build/macos/Build/Products/Debug',
  );
  final apps = products
      .listSync()
      .whereType<Directory>()
      .where((directory) => directory.path.endsWith('.app'))
      .toList();
  if (apps.length != 1) {
    throw StateError(
      'Expected one macOS quickstart app in ${products.path}, '
      'found ${apps.length}',
    );
  }
  return apps.single.absolute.path;
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
void checkQuickstartSmokeOcrTextForTesting(String text, {String? package}) {
  final subject = package ?? 'quickstart smoke test';
  final normalized = normalizeQuickstartSmokeOcrTextForTesting(text);
  final hasExpectedText =
      normalized.contains('hello') && normalized.contains('tom');
  if (!hasExpectedText) {
    throw Exception(
      '$subject screenshot OCR did not '
      'contain expected text `Hello, Tom` (normalized OCR: `$normalized`)',
    );
  }
}
