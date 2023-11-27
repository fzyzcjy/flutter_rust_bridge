// TODO merge with things in frb_util
import 'package:args/command_runner.dart';

Future<void> execute(String command) async {
  TODO;
}

List<Command<void>> createCommands() {
  return TODO;
}

extension ExtCommandRunner<T> on CommandRunner<T> {
  void addCommands(List<Command<T>> commands) {
    for (final command in commands) {
      addCommand(command);
    }
  }
}
