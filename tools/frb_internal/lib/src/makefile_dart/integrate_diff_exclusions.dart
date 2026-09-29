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
  'frb_example/flutter_via_create': _flutterViaCreateExclusions(
    'frb_example/flutter_via_create',
    needCompareOhos: needCompareOhos,
    hasRustBuilder: true,
  ),
  'frb_example/flutter_via_create_native_assets': _flutterViaCreateExclusions(
    'frb_example/flutter_via_create_native_assets',
    needCompareOhos: needCompareOhos,
    hasRustBuilder: false,
  ),
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

List<String> _flutterViaCreateExclusions(
  String package, {
  required bool needCompareOhos,
  required bool hasRustBuilder,
}) => [
  '$package/macos/Flutter/Flutter-Debug.xcconfig',
  '$package/macos/Flutter/Flutter-Release.xcconfig',
  '$package/pubspec.lock',
  '$package/pubspec.yaml',
  // The OHOS smoke API is an intentional addition to this checked-in
  // example; it is not part of the generic Flutter create template.
  if (package == 'frb_example/flutter_via_create') ...[
    '$package/lib/src/rust/api/ohos_smoke.dart',
    '$package/lib/src/rust/frb_generated.dart',
    '$package/lib/src/rust/frb_generated.io.dart',
    '$package/lib/src/rust/frb_generated.web.dart',
    '$package/rust/src/api/mod.rs',
    '$package/rust/src/api/ohos_smoke.rs',
    '$package/rust/src/frb_generated.rs',
  ],
  if (needCompareOhos) '$package/android/',
  if (needCompareOhos) '$package/macos/',
  if (needCompareOhos) '$package/windows/',
  if (!needCompareOhos) '$package/ohos/',
  if (!needCompareOhos && hasRustBuilder) '$package/rust_builder/ohos/',
  if (!needCompareOhos && hasRustBuilder) '$package/rust_builder/pubspec.yaml',
];

@visibleForTesting
String integrateDiffExclusionArgsForTesting(
  String package, {
  required bool needCompareOhos,
}) => integrateDiffExclusionArgs(package, needCompareOhos: needCompareOhos);

String gitExcludePathspecArgs(Iterable<String> paths) {
  return paths.map((path) => "':(exclude)$path'").join(' ');
}
