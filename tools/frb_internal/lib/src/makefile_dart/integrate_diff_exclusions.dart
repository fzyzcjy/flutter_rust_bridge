import 'package:meta/meta.dart';

const kIntegrateDiffExcludedPaths = <String>[
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_create/ohos/',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig',
];

const _kIntegrateSetExitIfChangedExcludedPathsByPackage = <String, List<String>>{
  'frb_example/flutter_via_create': [
    'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
    'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
    'frb_example/flutter_via_create/ohos/',
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

String integrateDiffExclusionArgs(String package, {bool includeOhos = false}) {
  var paths = _kIntegrateSetExitIfChangedExcludedPathsByPackage[package];
  if (paths == null) return '';
  if (includeOhos) {
    paths = paths
        .where((path) => path != 'frb_example/flutter_via_create/ohos/')
        .toList();
  }
  return gitExcludePathspecArgs(paths);
}

@visibleForTesting
String integrateDiffExclusionArgsForTesting(
  String package, {
  bool includeOhos = false,
}) => integrateDiffExclusionArgs(package, includeOhos: includeOhos);

String gitExcludePathspecArgs(Iterable<String> paths) {
  return paths.map((path) => "':(exclude)$path'").join(' ');
}
