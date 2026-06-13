// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'post_release.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

T _$enumValueHelper<T>(Map<T, String> enumValues, String source) => enumValues
    .entries
    .singleWhere(
      (e) => e.value == source,
      orElse: () => throw ArgumentError(
        '`$source` is not one of the supported values: '
        '${enumValues.values.join(', ')}',
      ),
    )
    .key;

PostReleaseConfig _$parsePostReleaseConfigResult(ArgResults result) =>
    PostReleaseConfig(
      codegenInstallMode: _$enumValueHelper(
        _$CodegenInstallModeEnumMapBuildCli,
        result['codegen-install-mode'] as String,
      ),
      releaseChannel: _$enumValueHelper(
        _$ReleaseChannelEnumMapBuildCli,
        result['release-channel'] as String,
      ),
    );

const _$CodegenInstallModeEnumMapBuildCli = <CodegenInstallMode, String>{
  CodegenInstallMode.cargoInstall: 'cargo-install',
  CodegenInstallMode.cargoBinstall: 'cargo-binstall',
  CodegenInstallMode.scoop: 'scoop',
  CodegenInstallMode.homebrew: 'homebrew',
};

const _$ReleaseChannelEnumMapBuildCli = <ReleaseChannel, String>{
  ReleaseChannel.stable: 'stable',
  ReleaseChannel.unstable: 'unstable',
};

ArgParser _$populatePostReleaseConfigParser(ArgParser parser) => parser
  ..addOption(
    'codegen-install-mode',
    allowed: ['cargo-install', 'cargo-binstall', 'scoop', 'homebrew'],
  )
  ..addOption('release-channel', allowed: ['stable', 'unstable']);

final _$parserForPostReleaseConfig = _$populatePostReleaseConfigParser(
  ArgParser(),
);

PostReleaseConfig parsePostReleaseConfig(List<String> args) {
  final result = _$parserForPostReleaseConfig.parse(args);
  return _$parsePostReleaseConfigResult(result);
}
