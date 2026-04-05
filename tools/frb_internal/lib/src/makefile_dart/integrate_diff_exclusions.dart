import 'package:meta/meta.dart';

const kIntegrateDiffExcludedPaths = <String>[
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig',
  'frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig',
];

const _kIntegrateSetExitIfChangedExtraArgsByPackage = <String, String>{
  'frb_example/flutter_via_create':
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig'",
  'frb_example/flutter_via_integrate':
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig'",
  'frb_example/flutter_package':
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig'",
};

String integrateDiffExclusionArgs(String package) {
  return _kIntegrateSetExitIfChangedExtraArgsByPackage[package] ?? '';
}

@visibleForTesting
String integrateDiffExclusionArgsForTesting(String package) =>
    integrateDiffExclusionArgs(package);

String gitExcludePathspecArgs(Iterable<String> paths) {
  return paths.map((path) => "':(exclude)$path'").join(' ');
}
