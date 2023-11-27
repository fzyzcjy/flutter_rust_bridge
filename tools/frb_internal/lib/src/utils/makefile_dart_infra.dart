// TODO merge with things in frb_util
import 'package:args/command_runner.dart';

Future<void> execute(String command) async {
  TODO;
}

extension ExtCommandRunner<T> on CommandRunner<T> {
  void addCommands(List<Command<T>> commands) {
    for (final command in commands) {
      addCommand(command);
    }
  }
}

class SimpleCommand extends Command<void> {
  @override
  final String name;
  @override
  final String description;
  final Future<void> Function() executor;

  SimpleCommand({
    required this.name,
    this.description = '',
    required this.executor,
  });

  @override
  Future<void> run() async => await executor();
}
