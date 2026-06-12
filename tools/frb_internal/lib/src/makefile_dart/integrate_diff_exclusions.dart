import 'package:meta/meta.dart';

const kIntegrateDiffExcludedPaths = <String>[
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_create_native_assets/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_create_native_assets/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_integrate_native_assets/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_integrate_native_assets/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_package_native_assets/example/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_package_native_assets/example/macos/Flutter/Flutter-Release.xcconfig',
];

String generateDiffExclusionArgs(String package) {
  final paths = _generateSetExitIfChangedExcludedPathsByPackage()[package];
  if (paths == null) return '';
  return gitExcludePathspecArgs(paths);
}

String integrateDiffExclusionArgs(
  String package, {
  required bool needCompareOhos,
}) {
  final paths = _integrateSetExitIfChangedExcludedPathsByPackage(
    needCompareOhos: needCompareOhos,
  )[package];
  if (paths == null) return '';
  return gitExcludePathspecArgs(paths);
}

Map<String, List<String>> _integrateSetExitIfChangedExcludedPathsByPackage({
  required bool needCompareOhos,
}) => <String, List<String>>{
  for (final package in [
    'frb_example/flutter_via_create',
    'frb_example/flutter_via_create_native_assets',
  ])
    package: [
      '$package/macos/Flutter/Flutter-Debug.xcconfig',
      '$package/macos/Flutter/Flutter-Release.xcconfig',
      '$package/pubspec.lock',
      '$package/pubspec.yaml',
      if (needCompareOhos) '$package/android/',
      if (needCompareOhos) '$package/macos/',
      if (needCompareOhos) '$package/windows/',
      if (!needCompareOhos) '$package/ohos/',
      if (!needCompareOhos) '$package/rust_builder/ohos/',
      '$package/rust_builder/pubspec.yaml',
    ],
  for (final package in [
    'frb_example/flutter_via_integrate',
    'frb_example/flutter_via_integrate_native_assets',
  ])
    package: [
      '$package/macos/Flutter/Flutter-Debug.xcconfig',
      '$package/macos/Flutter/Flutter-Release.xcconfig',
    ],
  for (final package in [
    'frb_example/flutter_package',
    'frb_example/flutter_package_native_assets',
  ])
    package: [
      '$package/example/macos/Flutter/Flutter-Debug.xcconfig',
      '$package/example/macos/Flutter/Flutter-Release.xcconfig',
    ],
};

Map<String, List<String>> _generateSetExitIfChangedExcludedPathsByPackage() =>
    <String, List<String>>{
      for (final package in [
        'frb_example/flutter_via_create',
        'frb_example/flutter_via_integrate',
        'frb_example/flutter_via_create_native_assets',
        'frb_example/flutter_via_integrate_native_assets',
      ])
        package: [
          '$package/macos/Flutter/Flutter-Debug.xcconfig',
          '$package/macos/Flutter/Flutter-Release.xcconfig',
        ],
      for (final package in [
        'frb_example/flutter_package',
        'frb_example/flutter_package_native_assets',
      ])
        package: [
          '$package/example/macos/Flutter/Flutter-Debug.xcconfig',
          '$package/example/macos/Flutter/Flutter-Release.xcconfig',
        ],
    };

@visibleForTesting
String generateDiffExclusionArgsForTesting(String package) =>
    generateDiffExclusionArgs(package);

@visibleForTesting
String integrateDiffExclusionArgsForTesting(
  String package, {
  required bool needCompareOhos,
}) => integrateDiffExclusionArgs(package, needCompareOhos: needCompareOhos);

String gitExcludePathspecArgs(Iterable<String> paths) {
  return paths.map((path) => "':(exclude)$path'").join(' ');
}
