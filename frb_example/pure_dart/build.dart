import 'package:native_assets_cli/native_assets_cli.dart';

void main(List<String> args) async {
  print('hi $args');

  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  print('hi $buildConfig $buildOutput');
  throw Exception('TODO');
}
