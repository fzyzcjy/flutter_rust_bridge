// Copyright (c) 2023, the Dart project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

import 'package:logging/logging.dart';
import 'package:native_assets_cli/native_assets_cli.dart';
import 'package:native_toolchain_c/native_toolchain_c.dart';

const packageName = 'native_add_library';

/// Implements the protocol from `package:native_assets_cli` by building
/// the C code in `src/` and reporting what native assets it built.
void main(List<String> args) async {
  // Parse the build configuration passed to this CLI from Dart or Flutter.
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  // Configure `package:native_toolchain_c` to build the C code for us.
  final cbuilder = CBuilder.library(
    name: packageName,
    assetId: 'package:$packageName/${packageName}.dart',
    sources: [
      'src/$packageName.c',
    ],
  );
  await cbuilder.run(
    buildConfig: buildConfig,
    // `package:native_toolchain_c` will output the dynamic or static libraries it built,
    // what files it accessed (for caching the build), etc.
    buildOutput: buildOutput,
    logger: Logger('')
      ..level = Level.ALL
      ..onRecord.listen((record) => print(record.message)),
  );

  // Write the output according to the native assets protocol so that Dart or
  // Flutter can find the native assets produced by this script.
  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
