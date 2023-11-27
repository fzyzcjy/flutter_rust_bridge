// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('misc-normalize-pubspec', miscNormalizePubspec),
    SimpleCommand('precommit-fast', precommitFast),
  ];
}

Future<void> miscNormalizePubspec() async {
  print('Execute miscNormalizePubspec');
  for (final package in kDartPackages) {
    final file = File('${exec.pwd}$package/pubspec.lock');
    file.writeAsStringSync(file.readAsStringSync().replaceAll('pub.flutter-io.cn', 'pub.dev'));
  }
}

Future<void> precommitFast() async {
  TODO_dart_format;
  TODO_rust_format;
  await miscNormalizePubspec();
}
