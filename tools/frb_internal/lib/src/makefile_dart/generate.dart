// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';

// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/utils/codecov_transformer.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:path/path.dart' as path;
import 'package:yaml/yaml.dart';

part 'generate.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('generate-internal', generateInternal,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-generate',
        generateRunFrbCodegenCommandGenerate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-integrate',
        generateRunFrbCodegenCommandIntegrate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    // more detailed command, can be used to execute just a portion of the main command
    SimpleConfigCommand(
        'generate-internal-frb-example-pure-dart',
        generateInternalFrbExamplePureDart,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
    SimpleConfigCommand('generate-internal-rust', generateInternalRust,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand('generate-internal-book-help', generateInternalBookHelp,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal-contributor',
        generateInternalContributor,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
    SimpleConfigCommand('generate-internal-readme', generateInternalReadme,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal-dart-source',
        generateInternalDartSource,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-website',
        generateWebsite,
        _$populateGenerateWebsiteConfigParser,
        _$parseGenerateWebsiteConfigResult),
    SimpleConfigCommand(
        'generate-website-build',
        generateWebsiteBuild,
        _$populateGenerateWebsiteConfigParser,
        _$parseGenerateWebsiteConfigResult),
    SimpleCommand('generate-website-merge', generateWebsiteMerge),
    SimpleCommand('generate-website-serve', generateWebsiteServe),
  ];
}

@CliOptions()
class GenerateConfig {
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;
  final bool coverage;

  const GenerateConfig({
    required this.setExitIfChanged,
    required this.coverage,
  });
}

@CliOptions()
class GeneratePackageConfig implements GenerateConfig {
  @override
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;
  @CliOption(convert: convertConfigPackage)
  final String package;
  @override
  final bool coverage;

  const GeneratePackageConfig({
    required this.setExitIfChanged,
    required this.package,
    required this.coverage,
  });
}

@CliOptions()
class GenerateWebsiteConfig {
  final bool coverage;

  const GenerateWebsiteConfig({
    required this.coverage,
  });
}

Future<void> generateInternal(GenerateConfig config,
    {bool canSkipAllContributor = false}) async {
  await generateInternalFrbExamplePureDart(config);
  await generateInternalRust(config);
  await generateInternalBookHelp(config);
  await generateInternalDartSource(config);
  await generateInternalBuildRunner(config);
  await _maybeAllowFailureAndSkip(canSkip: canSkipAllContributor, () async {
    await generateInternalContributor(config);
  });
  await generateInternalReadme(config);
}

Future<void> generateInternalFrbExamplePureDart(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await frb_example_pure_dart_generator.generate();
  });
}

Future<void> generateInternalDartSource(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final path = randomTempDir();
    await exec('''
    #!/usr/bin/env bash
    set -eux
    mkdir -p $path && cd $path

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    (cd sdk && git sparse-checkout set runtime/include)
    cp -rf ./sdk/runtime/include/* ${exec.pwd}frb_rust/src/dart_api/
    rm -rf sdk
  ''');
  });
}

Future<void> generateInternalRust(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartPackages) {
      await runPubGetIfNotRunYet(package);
    }

    await executeFrbCodegen(
      'internal-generate',
      relativePwd: 'frb_codegen',
      coverage: config.coverage,
      coverageName: 'GenerateInternalRust',
      // cbindgen needs this (e.g. https://github.com/mozilla/cbindgen/issues/674)
      nightly: true,
    );
  });
}

Future<void> generateInternalBookHelp(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final (cmd, extraArgs) in [
      ('', ''),
      ('generate', ''),
      ('create', ''),
      ('integrate', ''),
      ('build-web', '--dart-root ${exec.pwd}frb_example/pure_dart'),
    ]) {
      final resp = await executeFrbCodegen(
        '$cmd $extraArgs --help',
        relativePwd: 'frb_codegen',
        coverage: config.coverage,
        coverageName: 'GenerateInternalBookHelp',
      );
      File('${exec.pwd}website/docs/generated/_frb-codegen-command-${cmd.isEmpty ? "main" : cmd}.mdx')
          .writeAsStringSync('```\n${resp.stdout}```');
    }
  });
}

Future<void> generateInternalContributor(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final customPath = '${exec.pwd}/.all-contributors-custom.yaml';
    final customRaw = loadYaml(File(customPath).readAsStringSync());
    final customConverted = [
      for (final item in customRaw)
        {
          'login': (item as Map<dynamic, dynamic>).keys.single,
          'customMessage': item.values.single,
        }
    ];
    print('customConverted=$customConverted');

    final fileAllContributorsrc = File('${exec.pwd}/.all-contributorsrc');
    final allContributorsrcOld =
        jsonDecode(fileAllContributorsrc.readAsStringSync());

    final contributorNamesNew = [
      'fzyzcjy',
      for (final item in customConverted) item['login'],
    ];
    final allContributorsrcNew = {
      ...allContributorsrcOld,
      'contributors': [
        for (final login in contributorNamesNew)
          allContributorsrcOld['contributors']
              .where((x) => x['login'] == login)
              .single
      ]
    };

    if (allContributorsrcNew['contributors'].length !=
        allContributorsrcOld['contributors'].length) {
      throw Exception(
          'num contributors does not agree, maybe you forget to put contributors in $customPath?');
    }

    fileAllContributorsrc.writeAsStringSync(
        const JsonEncoder.withIndent('  ').convert(allContributorsrcNew));

    final messageTextNew = [
      for (final item in customConverted)
        '* [${item["login"]}](https://github.com/${item["login"]}): ${item["customMessage"]}\n',
    ].join('');

    _replaceCustomMessageText('\n$messageTextNew');

    final numContributors = allContributorsrcNew['contributors'].length;
    simpleReplaceFile(
      '${exec.pwd}README.md',
      RegExp(r'https://img.shields.io/badge/all_contributors-(\d+)-orange.svg'),
      'https://img.shields.io/badge/all_contributors-$numContributors-orange.svg',
    );

    await exec('all-contributors generate');
  });

  await generateInternalReadme(config);
}

void _replaceCustomMessageText(String customMessageText) {
  simpleActFile(
    '${exec.pwd}README.md',
    (raw) => simpleReplaceSection(
      raw,
      prelude:
          '<!-- CUSTOM-MESSAGE:START - Do not remove or modify this section -->',
      postlude: '<!-- CUSTOM-MESSAGE:END -->',
      inside: customMessageText,
    ),
  );
}

Future<void> generateInternalReadme(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final readmeText = File('${exec.pwd}README.md').readAsStringSync();

    {
      const kPrelude = '''---
title: Introduction
hide_title: true
---

<!-- AUTO-GENERATED FILE - DO NOT EDIT -->

''';

//       const kShowMeTheCode = '''
// import ShowMeTheCode from "@site/src/components/ShowMeTheCode";
//
// <ShowMeTheCode/>
// ''';

      final text = kPrelude + readmeText;
      // simpleReplaceSection(
      //   readmeText,
      //   prelude: '<!-- SHOW-ME-THE-CODE:START -->',
      //   postlude: '<!-- SHOW-ME-THE-CODE:END -->',
      //   inside: kShowMeTheCode,
      // );

      File('${exec.pwd}/website/docs/index.md').writeAsStringSync(text);
    }
  });
}

Future<void> generateInternalBuildRunner(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartNonExamplePackages) {
      await runPubGetIfNotRunYet(package);
      await exec('dart run build_runner build --delete-conflicting-outputs',
          relativePwd: package);
    }
  });
}

Future<void> generateRunFrbCodegenCommandGenerate(
    GeneratePackageConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await runPubGetIfNotRunYet(config.package);
    await executeFrbCodegen(
      'generate',
      relativePwd: config.package,
      coverage: config.coverage,
      coverageName: 'GenerateRunFrbCodegenCommandGenerate',
    );
  });
}

Future<void> generateRunFrbCodegenCommandIntegrate(
    GeneratePackageConfig config) async {
  await _wrapMaybeSetExitIfChanged(config,
      extraArgs:
          "':(exclude)*Podfile' ':(exclude)*.xcconfig' ':(exclude)pubspec.lock' ':(exclude)*Cargo.lock'",
      () async {
    final dirPackage = path.join(exec.pwd!, config.package);

    // Use temp dir within the repo. If use system-wide temp directory,
    // may see "OS Error: Cross-device link, errno = 18" and cannot use the
    // cheap "move directory" operation.
    final dirTemp = path.join(exec.pwd!, 'target',
        'GenerateRunFrbCodegenCommandIntegrate', randomTempDirName());
    print('Pick temporary directory: $dirTemp');
    await Directory(dirTemp).create(recursive: true);

    // We move instead of delete folder for extra safety of this script
    final dirTempOriginal = path.join(dirTemp, 'original');
    if (await Directory(dirPackage).exists()) {
      await Directory(dirPackage).rename(dirTempOriginal);
    }

    switch (config.package) {
      case 'frb_example/flutter_via_create':
        await executeFrbCodegen(
          'create flutter_via_create --local',
          relativePwd: 'frb_example',
          coverage: config.coverage,
          coverageName: 'GenerateRunFrbCodegenCommandIntegrate',
        );

      case 'frb_example/flutter_via_integrate':
        await exec('flutter create flutter_via_integrate',
            relativePwd: 'frb_example');
        await executeFrbCodegen(
          'integrate --local',
          relativePwd: config.package,
          coverage: config.coverage,
          coverageName: 'GenerateRunFrbCodegenCommandIntegrate',
        );
      case 'frb_example/flutter_package':
        await executeFrbCodegen(
          'create --local --template plugin flutter_package',
          relativePwd: 'frb_example',
          coverage: config.coverage,
          coverageName: 'GenerateRunFrbCodegenCommandIntegrate',
        );
      default:
        throw Exception('Do not know how to handle package ${config.package}');
    }

    // move back compilation cache to speed up future usage
    for (final subPath in ['build', 'rust/target']) {
      await _renameDirIfExists(
          path.join(dirTempOriginal, subPath), path.join(dirPackage, subPath));
    }
  });
}

Future<RunCommandOutput> executeFrbCodegen(
  String cmd, {
  required String relativePwd,
  required bool coverage,
  bool postRelease = false,
  required String coverageName,
  bool nightly = false,
}) async {
  if (postRelease) {
    assert(!coverage);
    return await exec('flutter_rust_bridge_codegen $cmd',
        relativePwd: relativePwd);
  } else {
    final outputCodecovPath = '${getCoverageDir(coverageName)}/codecov.json';
    final ans = await exec(
      'cargo ${nightly ? "+nightly" : ""} ${coverage ? "llvm-cov run --codecov --output-path $outputCodecovPath" : "run"} --manifest-path ${exec.pwd}frb_codegen/Cargo.toml -- $cmd',
      relativePwd: relativePwd,
      extraEnv: {'RUST_BACKTRACE': '1'},
    );
    if (coverage) transformCodecovReport(outputCodecovPath);
    return ans;
  }
}

Future<void> _renameDirIfExists(String src, String dst) async {
  if (!await Directory(src).exists()) return;
  await Directory(src).rename(dst);
}

Future<void> _wrapMaybeSetExitIfChanged(
    GenerateConfig config, Future<void> Function() inner,
    {String? extraArgs}) async {
  await wrapMaybeSetExitIfChangedRaw(config.setExitIfChanged, inner,
      extraArgs: extraArgs);
}

Future<void> wrapMaybeSetExitIfChangedRaw(
  bool enable,
  Future<void> Function() inner, {
  String? extraArgs,
}) async {
  // Before actually executing anything, check whether git repository is already dirty
  await _maybeSetExitIfChanged(enable, extraArgs: extraArgs);
  await inner();
  // The real check
  await _maybeSetExitIfChanged(enable, extraArgs: extraArgs);
}

Future<void> _maybeSetExitIfChanged(bool enable, {String? extraArgs}) async {
  if (enable) {
    await exec('git diff --exit-code ${extraArgs ?? ""}');
  }
}

Future<void> generateWebsite(GenerateWebsiteConfig config) async {
  await generateWebsiteBuild(config);
  await generateWebsiteMerge();
}

Future<void> generateWebsiteBuild(GenerateWebsiteConfig config) async {
  await exec('yarn install --frozen-lockfile', relativePwd: 'website');
  await exec('yarn build', relativePwd: 'website');

  await executeFrbCodegen(
    'build-web --release',
    relativePwd: 'frb_example/gallery',
    coverage: config.coverage,
    coverageName: 'GenerateWebsiteBuild',
  );
  await exec(
      'flutter build web '
      '--base-href /flutter_rust_bridge/demo/ '
      // Pwa seems to have conflict with the enable-threads.js hack
      // enable-threads.js: https://github.com/orgs/community/discussions/13309
      '--pwa-strategy none',
      relativePwd: 'frb_example/gallery');

  await exec('mdbook build .', relativePwd: 'website/v1_mdbook');
}

const _kWebsiteDir = 'website/merged_target/flutter_rust_bridge';

Future<void> generateWebsiteMerge() async {
  await exec('rm -rf website/merged_target');
  await exec('mkdir -p website/merged_target');

  await exec('cp -r website/build/ $_kWebsiteDir');

  await exec('cp -r website/v1_mdbook/book/ $_kWebsiteDir/v1');

  await exec('rm $_kWebsiteDir/demo.html');
  await exec('mkdir $_kWebsiteDir/demo');
  await exec('cp -r frb_example/gallery/build/web/* $_kWebsiteDir/demo');
  await exec('rm $_kWebsiteDir/demo/pkg/.gitignore');
  await exec(
      'cp ${exec.pwd}website/build/demo.html ${exec.pwd}$_kWebsiteDir/demo/index.html');
  // _generateWebsiteMergeDemoIndexHtml();

  await exec('mkdir -p $_kWebsiteDir/dev/bench');
  for (final name in ['data.js', 'index.html']) {
    await exec(
        'curl https://raw.githubusercontent.com/fzyzcjy/flutter_rust_bridge/gh-pages/dev/bench/$name -o $_kWebsiteDir/dev/bench/$name');
  }

  await exec('ls -al $_kWebsiteDir ; ls -al $_kWebsiteDir/demo');
}

// TODO rm
// void _generateWebsiteMergeDemoIndexHtml() {
//   // https://docs.flutter.dev/deployment/web#hostelement
//   const headCode = '''
//   <script src="enable-threads.js"></script>
//   <script src="flutter.js" defer></script>
//   ''';
//   const bodyCode = '''
//     <script>
//       window.addEventListener("load", function (ev) {
//         _flutter.loader.loadEntrypoint({
//           onEntrypointLoaded: async function(engineInitializer) {
//             let appRunner = await engineInitializer.initializeEngine({
//               // Pass a reference to "div#flutter_host" into the Flutter engine.
//               hostElement: document.querySelector("#flutter_host")
//             });
//             await appRunner.runApp();
//           }
//         });
//       });
//     </script>
//   ''';
//
//   final htmlDocusaurus =
//       File('${exec.pwd}/website/build/demo/index.html').readAsStringSync();
//   final ans = htmlDocusaurus
//       .replaceFirst('</head>', '$headCode</head>')
//       .replaceFirst('</body>', '$bodyCode</body>');
//   File('${exec.pwd}/$_kWebsiteDir/demo/index.html').writeAsStringSync(ans);
// }

Future<void> generateWebsiteServe() async {
  await exec('python -m http.server 8765',
      relativePwd: 'website/merged_target');
}

Future<void> _maybeAllowFailureAndSkip(
  Future<void> Function() run, {
  required bool canSkip,
}) async {
  try {
    await run();
  } catch (e, s) {
    if (canSkip) {
      print('See error but ignore it: $e $s');
    } else {
      rethrow;
    }
  }
}
