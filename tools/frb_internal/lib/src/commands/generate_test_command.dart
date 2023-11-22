import 'package:args/command_runner.dart';

class GenerateTestCommand extends Command<void> {
  @override
  String get name => 'generate-test';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    print('TODO');
  }
}
