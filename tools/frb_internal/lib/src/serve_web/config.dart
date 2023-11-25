// ignore_for_file: avoid_print

import 'package:build_cli_annotations/build_cli_annotations.dart';

part 'config.g.dart';

@CliOptions()
class Config {
  @CliOption(
    abbr: 'r',
    help: 'Root of the directory to be served',
    valueHelp: 'ROOT',
  )
  late String webRoot;

  @CliOption(
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: 8080,
  )
  late int port;
}
