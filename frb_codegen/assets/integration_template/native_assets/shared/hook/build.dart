import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    const cratePath = 'REPLACE_ME_RUST_CRATE_DIR';
    const builder = FlutterRustBridgeNativeAssetsBuilder(cratePath: cratePath);
    await builder.run(input: input, output: output);
  });
}
