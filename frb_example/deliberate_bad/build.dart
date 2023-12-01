import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils.dart';
import 'package:frb_example_deliberate_bad/src/misc.dart';

void main(List<String> args) async => simpleBuild(
      args,
      runCargoBuild: (pwd) async => runCommand(
        'cargo',
        [
          '+nightly',
          'build',
          '--release',
          ...CargoBuildAsanInfo.kExtraArgs,
        ],
        pwd: pwd,
        env: CargoBuildAsanInfo.kExtraEnv,
      ),
    );
