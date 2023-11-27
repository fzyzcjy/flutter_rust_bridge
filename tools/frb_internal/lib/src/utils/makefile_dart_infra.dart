// TODO merge with things in frb_util
import 'package:args/args.dart';
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

  SimpleCommand(
    this.name,
    this.executor, {
    this.description = '',
  });

  @override
  Future<void> run() async => await executor();
}

class SimpleConfigCommand<T> extends Command<void> {
  @override
  final String name;
  @override
  final String description;
  final Future<void> Function(T config) executor;

  final void Function(ArgParser) populateConfigParser;
  final T Function(ArgResults) parseConfigResult;

  SimpleConfigCommand(
    this.name,
    this.executor,
    this.populateConfigParser,
    this.parseConfigResult, {
    this.description = '',
  }) {
    populateConfigParser(argParser);
  }

  @override
  Future<void> run() async => await executor(parseConfigResult(argResults!));
}
