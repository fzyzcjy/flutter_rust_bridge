import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/serve_web/run_server.dart';

part 'serve_web_command.g.dart';

class ServeWebCommand extends _$ConfigCommand<void> {
  @override
  String get name => 'serve-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    await runServer(_options);
  }
}

@CliOptions(createCommand: true)
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

  @CliOption(help: 'Run tests in headless Chromium', negatable: false)
  late bool runTests;

  @CliOption(help: 'Open the webpage in a browser', defaultsTo: true)
  late bool open;
}
