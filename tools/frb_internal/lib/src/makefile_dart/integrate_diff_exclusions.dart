import 'package:meta/meta.dart';

const kIntegrateDiffExcludedPaths = <String>[
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig',
];

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
  'frb_example/flutter_via_create': [
    'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
    'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
    'frb_example/flutter_via_create/pubspec.lock',
    'frb_example/flutter_via_create/pubspec.yaml',
    if (!needCompareOhos) 'frb_example/flutter_via_create/ohos/',
    if (!needCompareOhos) 'frb_example/flutter_via_create/rust_builder/ohos/',
    if (!needCompareOhos)
      'frb_example/flutter_via_create/rust_builder/pubspec.yaml',
  ],
  'frb_example/flutter_via_integrate': [
    'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig',
    'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig',
  ],
  'frb_example/flutter_package': [
    'frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig',
    'frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig',
  ],
};

@visibleForTesting
String integrateDiffExclusionArgsForTesting(
  String package, {
  required bool needCompareOhos,
}) => integrateDiffExclusionArgs(package, needCompareOhos: needCompareOhos);

String gitExcludePathspecArgs(Iterable<String> paths) {
  return paths.map((path) => "':(exclude)$path'").join(' ');
}
