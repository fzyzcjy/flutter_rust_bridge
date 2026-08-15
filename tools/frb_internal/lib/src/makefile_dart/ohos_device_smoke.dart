// ignore_for_file: avoid_print

import 'dart:async';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_hap.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:path/path.dart' as path;

part 'ohos_device_smoke.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'ohos-device-smoke',
      ohosDeviceSmoke,
      _$populateOhosDeviceSmokeConfigParser,
      _$parseOhosDeviceSmokeConfigResult,
    ),
  ];
}

@CliOptions()
class OhosDeviceSmokeConfig {
  final String hap;
  final String bundle;

  @CliOption(defaultsTo: 'EntryAbility')
  final String ability;

  @CliOption(defaultsTo: 'FRB_OHOS_SMOKE_RESULT=PASS')
  final String expectedLog;

  @CliOption(defaultsTo: 30)
  final int timeoutSeconds;

  final String? deviceId;

  @CliOption(defaultsTo: 'target/ohos_device_smoke')
  final String artifactDir;

  const OhosDeviceSmokeConfig({
    required this.hap,
    required this.bundle,
    required this.ability,
    required this.expectedLog,
    required this.timeoutSeconds,
    required this.deviceId,
    required this.artifactDir,
  });
}

Future<void> ohosDeviceSmoke(OhosDeviceSmokeConfig config) async {
  if (config.timeoutSeconds <= 0) {
    throw ArgumentError.value(
      config.timeoutSeconds,
      'timeoutSeconds',
      'must be greater than zero',
    );
  }
  if (config.expectedLog.trim().isEmpty) {
    throw ArgumentError.value(
      config.expectedLog,
      'expectedLog',
      'must not be empty',
    );
  }
  final hdcTimeout = Duration(seconds: config.timeoutSeconds);

  final hapFile = File(path.absolute(config.hap));
  if (!hapFile.existsSync()) {
    throw StateError('Signed OHOS HAP does not exist: ${hapFile.path}');
  }
  final hapBundle = await readOhosHapBundleName(hapFile.path);
  if (hapBundle != config.bundle) {
    throw StateError(
      'OHOS HAP bundle `$hapBundle` does not match --bundle '
      '`${config.bundle}`. Refusing to install because cleanup could target '
      'the wrong application.',
    );
  }

  final targetsResult = await _runHdc(['list', 'targets'], timeout: hdcTimeout);
  _ensureProcessSucceeded(targetsResult, operation: '`hdc list targets`');
  final deviceId = resolveOhosDeviceIdForTesting(
    targetsResult.stdout as String,
    requestedDeviceId: config.deviceId,
  );
  final hdcPrefix = ['-t', deviceId];

  final bundlesResult = await _runHdc([
    ...hdcPrefix,
    'shell',
    'bm',
    'dump',
    '-a',
  ], timeout: hdcTimeout);
  _ensureProcessSucceeded(bundlesResult, operation: 'query installed bundles');
  if (ohosBundleAppearsInstalledForTesting(
    bundlesResult.stdout as String,
    bundle: config.bundle,
  )) {
    throw StateError(
      'Refusing to replace the already-installed bundle `${config.bundle}` '
      'on OHOS device $deviceId. Use a dedicated smoke-test bundle name.',
    );
  }

  final artifactDirectory = Directory(path.absolute(config.artifactDir));
  artifactDirectory.createSync(recursive: true);
  final safeDeviceId = deviceId.replaceAll(RegExp(r'[^A-Za-z0-9_.-]'), '_');
  final logFile = File(
    path.join(artifactDirectory.path, 'hilog-$safeDeviceId.log'),
  );

  var installedByThisRun = false;
  var installAttempted = false;
  String? successMessage;
  Object? operationError;
  StackTrace? operationStackTrace;
  try {
    final installResult = await _runHdc(
      ohosHdcInstallArgumentsForTesting(
        deviceId: deviceId,
        hapPath: hapFile.path,
      ),
      timeout: hdcTimeout,
      onStarted: () => installAttempted = true,
    );
    _ensureProcessSucceeded(installResult, operation: 'install signed HAP');
    installedByThisRun = true;
    final installOutput = '${installResult.stdout}\n${installResult.stderr}';
    if (!ohosHdcInstallSucceededForTesting(installOutput)) {
      throw StateError('OHOS HAP installation failed:\n$installOutput');
    }

    final markerPattern = RegExp.escape(config.expectedLog.trim());
    final baselineLogsResult = await _runHdc([
      ...hdcPrefix,
      'shell',
      'hilog',
      '-x',
      '-e',
      markerPattern,
    ], timeout: hdcTimeout);
    _ensureProcessSucceeded(
      baselineLogsResult,
      operation: 'capture OHOS smoke log baseline',
    );
    final baselineLogs =
        '${baselineLogsResult.stdout}\n${baselineLogsResult.stderr}';

    final launchResult = await _runHdc([
      ...hdcPrefix,
      'shell',
      'aa',
      'start',
      '-a',
      config.ability,
      '-b',
      config.bundle,
    ], timeout: hdcTimeout);
    _ensureProcessSucceeded(launchResult, operation: 'start OHOS ability');
    final launchOutput = '${launchResult.stdout}\n${launchResult.stderr}';
    if (!ohosHdcAbilityStartSucceededForTesting(launchOutput)) {
      throw StateError('OHOS ability launch failed:\n$launchOutput');
    }

    final deadline = DateTime.now().add(
      Duration(seconds: config.timeoutSeconds),
    );
    String? processId;
    var latestLogs = '';
    while (DateTime.now().isBefore(deadline)) {
      final pidTimeout = deadline.difference(DateTime.now());
      if (pidTimeout <= Duration.zero) break;
      final pidResult = await _runHdc([
        ...hdcPrefix,
        'shell',
        'pidof',
        config.bundle,
      ], timeout: pidTimeout);
      final candidate = (pidResult.stdout as String).trim();
      if (pidResult.exitCode == 0 && RegExp(r'^\d+$').hasMatch(candidate)) {
        processId = candidate;
      }

      if (processId != null) {
        final logsTimeout = deadline.difference(DateTime.now());
        if (logsTimeout <= Duration.zero) break;
        final logsResult = await _runHdc([
          ...hdcPrefix,
          'shell',
          'hilog',
          '-x',
          '-P',
          processId,
        ], timeout: logsTimeout);
        latestLogs = '${logsResult.stdout}\n${logsResult.stderr}';
        logFile.writeAsStringSync(latestLogs);
        if (ohosDeviceSmokeLogPassedForTesting(
          latestLogs,
          baselineLogs: baselineLogs,
          expectedLog: config.expectedLog,
        )) {
          successMessage =
              'OHOS device smoke passed: device=$deviceId, pid=$processId, '
              'marker=${config.expectedLog}, logs=${logFile.path}';
          break;
        }
      }

      await Future<void>.delayed(const Duration(seconds: 1));
    }

    if (successMessage == null) {
      throw StateError(
        'Timed out after ${config.timeoutSeconds}s waiting for OHOS smoke '
        'marker `${config.expectedLog}` on device $deviceId. '
        'Device logs: ${logFile.path}\n$latestLogs',
      );
    }
  } catch (error, stackTrace) {
    operationError = error;
    operationStackTrace = stackTrace;
  }

  Object? cleanupError;
  Object? cleanupProbeError;
  if (installAttempted && !installedByThisRun) {
    installedByThisRun = true;
    try {
      final bundlesAfterInstallResult = await _runHdc([
        ...hdcPrefix,
        'shell',
        'bm',
        'dump',
        '-a',
      ], timeout: hdcTimeout);
      _ensureProcessSucceeded(
        bundlesAfterInstallResult,
        operation: 'query installed bundles after failed install',
      );
      installedByThisRun = ohosBundleAppearsInstalledForTesting(
        bundlesAfterInstallResult.stdout as String,
        bundle: config.bundle,
      );
    } catch (error) {
      cleanupProbeError = error;
    }
  }
  if (installedByThisRun) {
    try {
      final uninstallResult = await _runHdc([
        ...hdcPrefix,
        'uninstall',
        config.bundle,
      ], timeout: hdcTimeout);
      final uninstallOutput =
          '${uninstallResult.stdout}\n${uninstallResult.stderr}';
      if (uninstallResult.exitCode != 0 ||
          !uninstallOutput.contains('uninstall bundle successfully')) {
        final uninstallError = StateError(
          'Failed to clean up OHOS smoke bundle '
          '`${config.bundle}` from device $deviceId:\n$uninstallOutput',
        );
        cleanupError = cleanupProbeError == null
            ? uninstallError
            : StateError('$cleanupProbeError\nAdditionally, $uninstallError');
      }
    } catch (error) {
      cleanupError = cleanupProbeError == null
          ? error
          : StateError('$cleanupProbeError\nAdditionally, $error');
    }
  }

  if (operationError != null) {
    if (cleanupError != null) {
      Error.throwWithStackTrace(
        StateError(
          '$operationError\nAdditionally, OHOS smoke cleanup failed: '
          '$cleanupError',
        ),
        operationStackTrace!,
      );
    }
    Error.throwWithStackTrace(operationError, operationStackTrace!);
  }
  if (cleanupError != null) throw cleanupError;
  if (successMessage == null) {
    throw StateError('OHOS smoke finished without a result');
  }
  print(successMessage);
}

Future<ProcessResult> _runHdc(
  List<String> arguments, {
  required Duration timeout,
  void Function()? onStarted,
}) async {
  Process process;
  try {
    process = await Process.start('hdc', arguments);
  } on ProcessException catch (error) {
    throw StateError(
      'Cannot execute `hdc`: ${error.message}. Install the HarmonyOS '
      'command-line tools and add `hdc` to PATH.',
    );
  }
  onStarted?.call();

  final stdout = StringBuffer();
  final stderr = StringBuffer();
  final stdoutSubscription = process.stdout
      .transform(systemEncoding.decoder)
      .listen(stdout.write);
  final stderrSubscription = process.stderr
      .transform(systemEncoding.decoder)
      .listen(stderr.write);
  final stdoutDone = stdoutSubscription.asFuture<void>();
  final stderrDone = stderrSubscription.asFuture<void>();
  final exitCodeFuture = process.exitCode;
  try {
    final result = await Future.wait<Object?>([
      exitCodeFuture,
      stdoutDone,
      stderrDone,
    ]).timeout(timeout);
    return ProcessResult(
      process.pid,
      result.first! as int,
      stdout.toString(),
      stderr.toString(),
    );
  } on TimeoutException {
    process.kill();
    final exitCode = await exitCodeFuture.timeout(
      const Duration(seconds: 5),
      onTimeout: () => -1,
    );
    if (exitCode == -1) {
      try {
        process.kill(ProcessSignal.sigkill);
      } on UnsupportedError {
        process.kill();
      }
      await exitCodeFuture.timeout(
        const Duration(seconds: 1),
        onTimeout: () => -1,
      );
    }
    await Future.wait<void>([
      stdoutSubscription.cancel(),
      stderrSubscription.cancel(),
    ]).timeout(const Duration(seconds: 1), onTimeout: () => <void>[]);
    throw StateError(
      'Timed out after ${timeout.inMilliseconds}ms executing '
      '`hdc ${arguments.join(' ')}`:\n$stdout\n$stderr',
    );
  }
}

void _ensureProcessSucceeded(
  ProcessResult result, {
  required String operation,
}) {
  if (result.exitCode != 0) {
    throw StateError(
      'Failed to $operation (exit code ${result.exitCode}):\n'
      '${result.stdout}\n${result.stderr}',
    );
  }
}

String resolveOhosDeviceIdForTesting(
  String targetsOutput, {
  required String? requestedDeviceId,
}) {
  final targets = targetsOutput
      .split(RegExp(r'\r?\n'))
      .map((line) => line.trim().split(RegExp(r'\s+')).first)
      .where((target) => target.isNotEmpty && target != '[Empty]')
      .toSet()
      .toList();

  if (requestedDeviceId != null && requestedDeviceId.trim().isNotEmpty) {
    if (!targets.contains(requestedDeviceId)) {
      throw StateError(
        'Requested OHOS device `$requestedDeviceId` is not connected. '
        'Connected targets: ${targets.join(', ')}',
      );
    }
    return requestedDeviceId;
  }
  if (targets.isEmpty) {
    throw StateError(
      'No OHOS device is connected. Enable developer mode and verify '
      '`hdc list targets`.',
    );
  }
  if (targets.length > 1) {
    throw StateError(
      'Multiple OHOS devices are connected (${targets.join(', ')}). '
      'Pass `--device-id` explicitly.',
    );
  }
  return targets.single;
}

bool ohosBundleAppearsInstalledForTesting(
  String bundlesOutput, {
  required String bundle,
}) => RegExp(
  '(^|[^A-Za-z0-9_.-])${RegExp.escape(bundle)}([^A-Za-z0-9_.-]|\$)',
  multiLine: true,
).hasMatch(bundlesOutput);

bool ohosHdcInstallSucceededForTesting(String output) =>
    output.contains('install bundle successfully');

List<String> ohosHdcInstallArgumentsForTesting({
  required String deviceId,
  required String hapPath,
}) => ['-t', deviceId, 'install', hapPath];

bool ohosHdcAbilityStartSucceededForTesting(String output) =>
    output.toLowerCase().contains('start ability successfully');

bool ohosDeviceSmokeLogPassedForTesting(
  String logs, {
  String baselineLogs = '',
  required String expectedLog,
}) {
  final marker = expectedLog.trim();
  if (marker.isEmpty) return false;

  final markerAtEndOfLine = RegExp('${RegExp.escape(marker)}\\s*\$');
  final baselineMarkerCounts = <String, int>{};
  for (final line in baselineLogs.split(RegExp(r'\r?\n'))) {
    if (markerAtEndOfLine.hasMatch(line)) {
      baselineMarkerCounts.update(
        line,
        (count) => count + 1,
        ifAbsent: () => 1,
      );
    }
  }

  for (final line in logs.split(RegExp(r'\r?\n'))) {
    if (!markerAtEndOfLine.hasMatch(line)) continue;
    final baselineCount = baselineMarkerCounts[line] ?? 0;
    if (baselineCount == 0) return true;
    baselineMarkerCounts[line] = baselineCount - 1;
  }
  return false;
}
