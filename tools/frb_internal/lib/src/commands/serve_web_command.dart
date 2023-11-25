import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/serve_web/run_server.dart';

class ServeWebCommand extends Command<void> {
  @override
  String get name => 'serve-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    await runServer(parseConfig());
  }
}
