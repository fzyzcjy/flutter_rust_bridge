import 'structs.dart';

const _githubHostedDesktopImages = [
  'windows-2025',
  'macos-15-intel',
  'ubuntu-24.04',
];

const _exampleDartPackages = [
  'frb_example--dart_minimal',
  'frb_example--pure_dart',
  'frb_example--pure_dart_pde',
];

const _flutterNativePackages = [
  'frb_example--flutter_via_create',
  'frb_example--flutter_via_create_native_assets',
  'frb_example--flutter_package--example',
  'frb_example--flutter_package_native_assets--example',
  'frb_example--rust_ui_counter--ui',
  'frb_example--rust_ui_todo_list--ui',
];

final kCiJobs = [
  const CiJob('deploy_website'),
  const CiJob('lint_rust_primary'),
  const CiJob('lint_dart_primary'),
  const CiJob('lint_rust_feature_flag'),
  CiJob(
    'generate_run_frb_codegen_command_generate',
    matrix: CiMatrix([
      for (final image in _githubHostedDesktopImages)
        for (final package in [
          'frb_example--dart_minimal',
          'frb_example--pure_dart',
          'frb_example--pure_dart_pde',
          'frb_example--dart_build_rs',
          'frb_example--deliberate_bad',
          'frb_example--integrate_third_party',
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_integrate',
          'frb_example--flutter_package',
          'frb_example--flutter_via_create_native_assets',
          'frb_example--flutter_via_integrate_native_assets',
          'frb_example--flutter_package_native_assets',
          'frb_example--rust_ui_counter--ui',
          'frb_example--rust_ui_todo_list--ui',
        ])
          if (!_isExcludedGenerateCommandGenerate(
            image: image,
            package: package,
          ))
            {'image': image, 'package': package},
    ]),
  ),
  CiJob(
    'generate_run_frb_codegen_command_integrate',
    matrix: CiMatrix([
      for (final image in ['macos-15-intel', 'windows-2025', 'ubuntu-24.04'])
        for (final package in [
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_integrate',
          'frb_example--flutter_package',
          'frb_example--flutter_via_create_native_assets',
          'frb_example--flutter_via_integrate_native_assets',
          'frb_example--flutter_package_native_assets',
        ])
          {'image': image, 'package': package, 'platforms': 'default'},
      {
        'image': 'ubuntu-24.04',
        'package': 'frb_example--flutter_via_create',
        'platforms': 'ohos',
      },
    ]),
  ),
  const CiJob('generate_apple_scaffold'),
  const CiJob('generate_internal'),
  CiJob(
    'bench_dart_native',
    matrix: CiMatrix([
      for (final image in _githubHostedDesktopImages) {'image': image},
    ]),
  ),
  const CiJob('bench_upload'),
  CiJob(
    'build_flutter',
    matrix: CiMatrix([
      for (final info in [
        for (final package in [
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_create_native_assets',
        ])
          for (final target in [
            {'image': 'windows-2025', 'target': 'windows'},
            {'image': 'windows-11-arm', 'target': 'windows'},
            {'image': 'macos-15-intel', 'target': 'macos'},
            {'image': 'ubuntu-latest', 'target': 'linux'},
            {'image': 'ubuntu-latest', 'target': 'android-aab'},
            {'image': 'ubuntu-latest', 'target': 'android-apk'},
            {'image': 'macos-15-intel', 'target': 'ios'},
          ])
            {
              ...target,
              'package': package,
              'package_path': package.replaceAll('--', '/'),
            },
        {
          'image': 'ubuntu-latest',
          'target': 'ohos',
          'package': 'frb_example--flutter_via_create',
          'package_path': 'frb_example/flutter_via_create',
        },
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_mimic_quickstart',
    matrix: CiMatrix([
      for (final image in ['windows-2025', 'macos-15-intel', 'ubuntu-latest'])
        {'image': image},
    ]),
  ),
  CiJob(
    'test_rust',
    matrix: CiMatrix([
      for (final info in [
        {'image': 'macos-15-intel', 'version': ''},
        {'image': 'windows-2025', 'version': ''},
        {'image': 'ubuntu-latest', 'version': ''},
        {'image': 'ubuntu-latest', 'version': 'nightly'},
        {'image': 'ubuntu-latest', 'version': '1.85.0'},
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_dart_native',
    matrix: CiMatrix([
      for (final image in _githubHostedDesktopImages)
        for (final package in [
          'frb_dart',
          'frb_hooks',
          'frb_utils',
          'tools--frb_internal',
          ..._exampleDartPackages,
          'frb_example--dart_build_rs',
        ])
          if (!_isExcludedTestDartNative(image: image, package: package))
            {'image': image, 'package': package},
    ]),
  ),
  CiJob(
    'test_dart_web',
    matrix: CiMatrix([
      for (final package in ['frb_dart', ..._exampleDartPackages])
        {'package': package},
    ]),
  ),
  CiJob(
    'test_dart_valgrind',
    matrix: CiMatrix([
      for (final package in _exampleDartPackages) {'package': package},
    ]),
  ),
  CiJob(
    'test_dart_sanitizer',
    matrix: CiMatrix([
      for (final sanitizer in ['asan', 'lsan'])
        for (final package in _exampleDartPackages)
          {'sanitizer': sanitizer, 'package': package},
    ]),
  ),
  CiJob(
    'test_flutter_native_android',
    matrix: CiMatrix([
      for (final package in _flutterNativePackages)
        {'package': package, 'device': 'pixel', 'api-level': 35},
    ]),
  ),
  CiJob(
    'test_flutter_native_ios',
    matrix: CiMatrix([
      for (final package in _flutterNativePackages)
        {'package': package, 'device': 'iPhone 16 Pro Max Simulator (18.6)'},
    ]),
  ),
  CiJob(
    'test_flutter_native_desktop',
    matrix: CiMatrix([
      for (final info in [
        for (final package in _flutterNativePackages)
          ..._flutterDesktopPackageEntries(package),
        ..._linuxFlutterDesktopPackageEntries([
          'frb_example--flutter_via_integrate',
          'frb_example--flutter_via_integrate_native_assets',
          'frb_example--gallery',
          'frb_example--integrate_third_party',
        ]),
      ])
        {'info': info},
    ]),
  ),
  CiJob(
    'test_flutter_web',
    matrix: CiMatrix([
      for (final package in [
        'frb_example--flutter_via_create',
        'frb_example--flutter_via_create_native_assets',
        'frb_example--gallery',
      ])
        {'package': package},
    ]),
  ),
  CiJob(
    'test_flutter_quickstart_smoke',
    matrix: CiMatrix([
      for (final info in [
        for (final package in [
          'frb_example--flutter_via_create',
          'frb_example--flutter_via_create_native_assets',
        ])
          ..._quickstartSmokeEntries(package),
      ])
        {'info': info},
    ]),
  ),
  const CiJob('misc_codecov'),
];

bool _isExcludedGenerateCommandGenerate({
  required String image,
  required String package,
}) =>
    (image == 'windows-2025' || image == 'macos-15-intel') &&
    {
      'frb_example--deliberate_bad',
      'frb_example--integrate_third_party',
      'frb_example--flutter_via_integrate',
      'frb_example--flutter_via_integrate_native_assets',
    }.contains(package);

bool _isExcludedTestDartNative({
  required String image,
  required String package,
}) =>
    (image == 'windows-2025' || image == 'macos-15-intel') &&
    {'frb_utils', 'tools--frb_internal'}.contains(package);

List<Map<String, Object?>> _flutterDesktopPackageEntries(String package) => [
  {'image': 'windows-2025', 'platform': 'windows', 'package': package},
  {'image': 'macos-15-intel', 'platform': 'macos', 'package': package},
  {'image': 'ubuntu-latest', 'platform': 'linux', 'package': package},
];

List<Map<String, Object?>> _linuxFlutterDesktopPackageEntries(
  List<String> packages,
) => [
  for (final package in packages)
    {'image': 'ubuntu-latest', 'platform': 'linux', 'package': package},
];

List<Map<String, Object?>> _quickstartSmokeEntries(String package) {
  final packagePath = package.replaceAll('--', '/');
  return [
    {
      'image': 'ubuntu-latest',
      'platform': 'web',
      'target': 'web',
      'device': 'chrome',
      'package': package,
      'package_path': packagePath,
    },
    {
      'image': 'ubuntu-latest',
      'platform': 'android',
      'target': 'android',
      'device': 'pixel',
      'api-level': 35,
      'package': package,
      'package_path': packagePath,
    },
    {
      'image': 'macos-latest',
      'platform': 'ios',
      'target': 'ios',
      'device': 'iPhone 16 Pro Max Simulator (18.6)',
      'package': package,
      'package_path': packagePath,
    },
    {
      'image': 'ubuntu-latest',
      'platform': 'linux',
      'target': 'desktop',
      'device': 'linux',
      'package': package,
      'package_path': packagePath,
    },
    {
      'image': 'macos-15-intel',
      'platform': 'macos',
      'target': 'desktop',
      'device': 'macos',
      'package': package,
      'package_path': packagePath,
    },
    {
      'image': 'windows-2025',
      'platform': 'windows',
      'target': 'desktop',
      'device': 'windows',
      'package': package,
      'package_path': packagePath,
    },
  ];
}
