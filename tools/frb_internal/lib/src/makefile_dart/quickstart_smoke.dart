// ignore_for_file: avoid_print

import 'dart:async';
import 'dart:convert';
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
  final File vmServiceOutputFile;
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
    required this.vmServiceOutputFile,
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
      vmServiceOutputFile: File(
        '${absolutePackage.path}/$artifactDir/quickstart-$targetName-vm-service.txt',
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
      vmServiceOutputFile,
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
  final bool vmServiceMatched;
  final Exception? lastOcrException;

  const _QuickstartSmokeScreenshotEvidence({
    required this.readyForScreenshot,
    required this.ocrMatched,
    required this.vmServiceMatched,
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
Uri? quickstartSmokeVmServiceUriForTesting(String output) {
  final match = RegExp(
    r'A Dart VM Service.*? at: (http://\S+/)',
  ).firstMatch(output);
  if (match == null) return null;
  return Uri.parse(match.group(1)!);
}

@visibleForTesting
String quickstartSmokeInspectorTextForTesting(Object? json) {
  final buffer = StringBuffer();
  _writeQuickstartSmokeInspectorText(buffer, json);
  return buffer.toString();
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
  required _QuickstartSmokeFlutterRun flutterRun,
}) async {
  if (!readyForScreenshot) {
    print('Capturing one diagnostic screenshot before failing readiness check');
    await _captureAndOcrQuickstartSmokeScreenshotFromContext(context);
    return _QuickstartSmokeScreenshotEvidence(
      readyForScreenshot: readyForScreenshot,
      ocrMatched: false,
      vmServiceMatched: false,
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
        vmServiceMatched: false,
        lastOcrException: null,
      );
    } on Exception catch (exception) {
      lastOcrException = exception;
      print('Quickstart screenshot OCR did not match yet: $exception');
      if (!DateTime.now().isBefore(visibleTextDeadline)) break;
      await Future<void>.delayed(const Duration(seconds: 5));
    }
  } while (DateTime.now().isBefore(visibleTextDeadline));

  final vmServiceMatched = await _captureQuickstartMacosVmServiceEvidence(
    context: context,
    flutterRun: flutterRun,
  );

  return _QuickstartSmokeScreenshotEvidence(
    readyForScreenshot: readyForScreenshot,
    ocrMatched: false,
    vmServiceMatched: vmServiceMatched,
    lastOcrException: lastOcrException,
  );
}

Future<bool> _captureQuickstartMacosVmServiceEvidence({
  required _QuickstartSmokeContext context,
  required _QuickstartSmokeFlutterRun flutterRun,
}) async {
  if (context.target != QuickstartSmokeTarget.desktop || !Platform.isMacOS) {
    return false;
  }

  print('Trying macOS Flutter VM service evidence after screenshot OCR miss');
  try {
    final text = await _readQuickstartMacosVmServiceText(
      flutterRun.combinedOutput,
    );
    context.vmServiceOutputFile.writeAsStringSync(text);
    checkQuickstartSmokeOcrTextForTesting(text);
    print('macOS Flutter VM service evidence contains expected text');
    return true;
  } on Exception catch (exception) {
    context.vmServiceOutputFile.writeAsStringSync(exception.toString());
    print('macOS Flutter VM service evidence did not match: $exception');
    return false;
  }
}

Future<String> _readQuickstartMacosVmServiceText(
  String flutterRunOutput,
) async {
  final vmServiceUri = quickstartSmokeVmServiceUriForTesting(flutterRunOutput);
  if (vmServiceUri == null) {
    throw Exception('Could not find Flutter VM Service URL in flutter run log');
  }

  final vm = await _getQuickstartSmokeVmServiceJson(
    vmServiceUri.resolve('getVM'),
  );
  final isolates = (vm['result'] as Map<String, Object?>)['isolates'] as List;
  if (isolates.isEmpty) {
    throw Exception('Flutter VM Service reported no app isolates');
  }
  final isolateId = (isolates.single as Map<String, Object?>)['id'] as String;

  final tree = await _getQuickstartSmokeVmServiceJson(
    vmServiceUri.replace(
      path:
          '${vmServiceUri.path}ext.flutter.inspector.getRootWidgetSummaryTree',
      queryParameters: {
        'isolateId': isolateId,
        'objectGroup': 'quickstartSmoke',
        'subtreeDepth': '20',
      },
    ),
  );
  final textWidgetIds = _findQuickstartSmokeTextWidgetIds(tree);
  if (textWidgetIds.isEmpty) {
    throw Exception('Flutter inspector did not report any app Text widgets');
  }

  final buffer = StringBuffer();
  for (final textWidgetId in textWidgetIds) {
    final properties = await _getQuickstartSmokeVmServiceJson(
      vmServiceUri.replace(
        path: '${vmServiceUri.path}ext.flutter.inspector.getProperties',
        queryParameters: {
          'isolateId': isolateId,
          'objectGroup': 'quickstartSmoke',
          'arg': textWidgetId,
        },
      ),
    );
    buffer.writeln(quickstartSmokeInspectorTextForTesting(properties));
  }
  return buffer.toString();
}

Future<Map<String, Object?>> _getQuickstartSmokeVmServiceJson(Uri uri) async {
  final client = HttpClient();
  try {
    final request = await client.getUrl(uri);
    final response = await request.close();
    final body = await response.transform(utf8.decoder).join();
    if (response.statusCode != HttpStatus.ok) {
      throw Exception(
        'Flutter VM Service request failed '
        '(status=${response.statusCode}, uri=$uri, body=$body)',
      );
    }
    return jsonDecode(body) as Map<String, Object?>;
  } finally {
    client.close(force: true);
  }
}

List<String> _findQuickstartSmokeTextWidgetIds(Object? json) {
  final result = <String>[];
  _collectQuickstartSmokeTextWidgetIds(result, json);
  return result;
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
  if (context.vmServiceOutputFile.existsSync()) {
    print('\n===== VM service evidence (${context.targetName}) =====');
    print(context.vmServiceOutputFile.readAsStringSync());
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
  if (!screenshotEvidence.ocrMatched && !screenshotEvidence.vmServiceMatched) {
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
    deviceId: context.resolvedDeviceId,
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

void _collectQuickstartSmokeTextWidgetIds(List<String> result, Object? json) {
  if (json is List) {
    for (final item in json) {
      _collectQuickstartSmokeTextWidgetIds(result, item);
    }
    return;
  }
  if (json is! Map) return;

  if (json['widgetRuntimeType'] == 'Text' &&
      json['createdByLocalProject'] == true &&
      json['valueId'] is String) {
    result.add(json['valueId'] as String);
  }
  for (final value in json.values) {
    _collectQuickstartSmokeTextWidgetIds(result, value);
  }
}

void _writeQuickstartSmokeInspectorText(StringBuffer buffer, Object? json) {
  if (json is List) {
    for (final item in json) {
      _writeQuickstartSmokeInspectorText(buffer, item);
    }
    return;
  }
  if (json is! Map) return;

  if (json['type'] == 'StringProperty' && json['value'] is String) {
    buffer.writeln(json['value']);
  }
  for (final value in json.values) {
    _writeQuickstartSmokeInspectorText(buffer, value);
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
