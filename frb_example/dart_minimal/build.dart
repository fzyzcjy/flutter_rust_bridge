import 'package:flutter_rust_bridge/src/build_utils/simple_build_utils.dart';
import 'package:native_assets_cli/native_assets_cli.dart';

void main(List<String> args) async {
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  final dependencies = await simpleBuild(packageRoot: buildConfig.packageRoot);
  buildOutput.dependencies.dependencies.addAll(dependencies);

  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
