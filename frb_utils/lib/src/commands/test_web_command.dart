import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_utils/src/serve_web/run_server.dart';

part 'test_web_command.g.dart';

class TestWebCommand extends _$TestWebConfigCommand<void> {
  @override
  String get name => 'test-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    TODO;
  }
}

@CliOptions(createCommand: true)
class TestWebConfig {
  @CliOption(
    abbr: 'd',
    help: 'Dart file to be tested',
  )
  late String entrypoint;
}
