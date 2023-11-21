import 'package:native_assets_cli/native_assets_cli.dart';

// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
void main(List<String> args) async {
  print('hi $args');

  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  buildOutput.assets.add(Asset(
    id: assetId,
    linkMode: LinkMode.dynamic,
    target: buildConfig.target,
    path: AssetAbsolutePath(libUri),
  ));

  buildOutput.dependencies.dependencies.addAll({
    // Note: We use a Set here to deduplicate the dependencies.
    ...sources,
    ...includeFiles,
    ...dartBuildFiles,
  });
}
