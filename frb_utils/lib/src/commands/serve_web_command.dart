import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_utils/src/serve_web/run_server.dart';

part 'serve_web_command.g.dart';

class ServeWebCommand extends _$ServeWebConfigCommand<void> {
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
class ServeWebConfig {
  @CliOption(
    abbr: 'r',
    help: 'Root of the directory to be served',
    valueHelp: 'ROOT',
  )
  final String webRoot;

  static const kDefaultPort = 8080;

  @CliOption(
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: kDefaultPort,
  )
  final int port;

  @CliOption(help: 'Open the webpage in a browser', defaultsTo: true)
  final bool open;

// migrate to `test-web`
// @CliOption(help: 'Run tests in headless Chromium', negatable: false)
// late bool runTests;

  const ServeWebConfig({
    required this.webRoot,
    required this.port,
    required this.open,
  });
}
