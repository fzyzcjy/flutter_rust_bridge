import 'package:meta/meta.dart';

const _kIntegrateSetExitIfChangedExtraArgsByPackage = <String, String>{
  'frb_example/flutter_via_create':
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/rust/Cargo.lock'",
  'frb_example/flutter_via_integrate':
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/rust/Cargo.lock'",
  'frb_example/flutter_package':
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)frb_example/flutter_package/rust/Cargo.lock'",
};

String integrateSetExitIfChangedExtraArgs(String package) {
  return _kIntegrateSetExitIfChangedExtraArgsByPackage[package] ?? '';
}

@visibleForTesting
String integrateSetExitIfChangedExtraArgsForTesting(String package) =>
    integrateSetExitIfChangedExtraArgs(package);
