import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_device_smoke.dart';
import 'package:test/test.dart';

void main() {
  test('OHOS device smoke resolves one connected target', () {
    expect(
      resolveOhosDeviceIdForTesting(
        '3QC0124C20001268\n',
        requestedDeviceId: null,
      ),
      '3QC0124C20001268',
    );
    expect(
      () => resolveOhosDeviceIdForTesting('[Empty]\n', requestedDeviceId: null),
      throwsStateError,
    );
    expect(
      () => resolveOhosDeviceIdForTesting(
        'device-a\ndevice-b\n',
        requestedDeviceId: null,
      ),
      throwsStateError,
    );
  });

  test('OHOS device smoke validates an explicitly selected target', () {
    expect(
      resolveOhosDeviceIdForTesting(
        'device-a\ndevice-b\n',
        requestedDeviceId: 'device-b',
      ),
      'device-b',
    );
    expect(
      () => resolveOhosDeviceIdForTesting(
        'device-a\n',
        requestedDeviceId: 'device-b',
      ),
      throwsStateError,
    );
  });

  test('OHOS device smoke protects existing bundles', () {
    const bundles = '''
bundleName: com.example.other
bundleName: com.example.smoke
''';
    expect(
      ohosBundleAppearsInstalledForTesting(
        bundles,
        bundle: 'com.example.smoke',
      ),
      isTrue,
    );
    expect(
      ohosBundleAppearsInstalledForTesting(
        bundles,
        bundle: 'com.example.smoke.extra',
      ),
      isFalse,
    );
  });

  test('OHOS device smoke install never replaces an existing bundle', () {
    expect(
      ohosHdcInstallArgumentsForTesting(
        deviceId: 'device-a',
        hapPath: '/tmp/smoke.hap',
      ),
      ['-t', 'device-a', 'install', '/tmp/smoke.hap'],
    );
  });

  test('OHOS device smoke recognizes hdc outcomes and Rust marker', () {
    expect(
      ohosHdcInstallSucceededForTesting('install bundle successfully.'),
      isTrue,
    );
    expect(
      ohosHdcInstallSucceededForTesting('fail to verify pkcs7 file'),
      isFalse,
    );
    expect(
      ohosHdcAbilityStartSucceededForTesting('start ability successfully.'),
      isTrue,
    );
    expect(
      ohosDeviceSmokeLogPassedForTesting('''
08-11 12:31:36.401 50093 50093 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
''', expectedLog: 'FRB_OHOS_SMOKE_RESULT=PASS'),
      isTrue,
    );
    expect(
      ohosDeviceSmokeLogPassedForTesting(
        '''
08-11 12:30:00.500 40000 40000 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
''',
        baselineLogs: '''
08-11 12:30:00.500 40000 40000 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
''',
        expectedLog: 'FRB_OHOS_SMOKE_RESULT=PASS',
      ),
      isFalse,
    );
    expect(
      ohosDeviceSmokeLogPassedForTesting(
        '''
08-11 12:30:00.500 40000 40000 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
08-11 12:31:36.401 50093 50093 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
''',
        baselineLogs: '''
08-11 12:30:00.500 40000 40000 W Flutter: FRB_OHOS_SMOKE_RESULT=PASS
''',
        expectedLog: 'FRB_OHOS_SMOKE_RESULT=PASS',
      ),
      isTrue,
    );
    expect(
      ohosDeviceSmokeLogPassedForTesting('''
08-11 12:31:36.401 50093 50093 W Flutter: FRB_OHOS_SMOKE_RESULT=PASSIVE
''', expectedLog: 'FRB_OHOS_SMOKE_RESULT=PASS'),
      isFalse,
    );
    expect(
      ohosDeviceSmokeLogPassedForTesting(
        'FRB_OHOS_SMOKE_RESULT=PASS',
        expectedLog: '   ',
      ),
      isFalse,
    );
  });
}
