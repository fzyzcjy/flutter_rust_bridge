import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';

part 'test_web_command.g.dart';

class TestWebCommand extends _$TestWebConfigCommand<void> {
  @override
  String get name => 'test-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    executeTestWeb(_options);
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
