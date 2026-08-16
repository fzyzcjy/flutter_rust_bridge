import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

void main(List<String> args) async {
  const cratePath = 'rust';
  const builder = FlutterRustBridgeNativeAssetsBuilder(cratePath: cratePath);
  await build(args, (input, output) async {
    await builder.run(input: input, output: output);
  });
}
