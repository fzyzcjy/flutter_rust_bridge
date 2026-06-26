import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

void main() {
  const builder = FlutterRustBridgeNativeAssetsBuilder(
    assetName: 'src/rust/frb_generated.io.dart',
    cratePath: 'rust',
    buildMode: FlutterRustBridgeBuildMode.release,
    features: <String>[],
    assetRouting: <AssetRouting>[ToAppBundle()],
  );

  assert(builder.assetName.isNotEmpty);
}
