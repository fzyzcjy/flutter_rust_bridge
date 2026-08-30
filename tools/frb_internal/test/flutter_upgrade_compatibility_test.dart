import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  for (final package in [
    'frb_example/gallery',
    'frb_example/integrate_third_party',
    'frb_example/rust_ui_counter/ui',
    'frb_example/rust_ui_todo_list/ui',
  ]) {
    test('$package uses Flutter 3.47 platform minimums', () {
      final packageDirectory = path.join(exec.pwd!, package);
      final androidSettings = _read(
        packageDirectory,
        'android/settings.gradle',
      );
      final kotlinConfiguration = package.endsWith('gallery')
          ? _read(packageDirectory, 'android/build.gradle')
          : androidSettings;

      expect(
        _read(
          packageDirectory,
          'android/gradle/wrapper/gradle-wrapper.properties',
        ),
        contains('gradle-9.3.1-all.zip'),
      );
      expect(
        androidSettings,
        contains('com.android.application" version "9.1.0'),
      );
      expect(kotlinConfiguration, contains('2.4.0'));
      expect(
        _read(packageDirectory, 'android/gradle.properties'),
        allOf(
          contains('android.builtInKotlin=false'),
          contains('android.newDsl=false'),
        ),
      );
      expect(
        _read(packageDirectory, 'ios/Runner.xcodeproj/project.pbxproj'),
        contains('IPHONEOS_DEPLOYMENT_TARGET = 15.0;'),
      );
      expect(
        _read(packageDirectory, 'ios/Podfile'),
        contains("platform :ios, '15.0'"),
      );
      expect(
        _read(packageDirectory, 'ios/Flutter/AppFrameworkInfo.plist'),
        isNot(contains('MinimumOSVersion')),
      );
      expect(
        _read(packageDirectory, 'macos/Runner.xcodeproj/project.pbxproj'),
        contains('MACOSX_DEPLOYMENT_TARGET = 12.0;'),
      );
      expect(
        _read(packageDirectory, 'macos/Podfile'),
        contains("platform :osx, '12.0'"),
      );
    });
  }
}

String _read(String packageDirectory, String relativePath) =>
    File(path.join(packageDirectory, relativePath)).readAsStringSync();
