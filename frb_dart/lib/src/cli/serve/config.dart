// TODO
// // ignore_for_file: avoid_print
//
// import 'dart:io';
//
// import 'package:build_cli_annotations/build_cli_annotations.dart';
// import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
// import 'package:path/path.dart' as p;
//
// part 'config.g.dart';
//
// /// {@macro flutter_rust_bridge.cli}
// @CliOptions()
// class Opts {
//   /// {@macro flutter_rust_bridge.cli}
//   @CliOption(
//     abbr: 'd',
//     help: 'Run "dart compile" with the specified input instead of "flutter build"',
//     valueHelp: 'ENTRY',
//   )
//   late String? dartInput;
//
//   /// {@macro flutter_rust_bridge.cli}
//   @CliOption(abbr: 'v', help: 'Display more verbose information')
//   late bool verbose;
//
//   /// {@macro flutter_rust_bridge.cli}
//   @CliOption(
//     help: 'Set COEP to credentialless\n'
//         'Defaults to true for Flutter',
//   )
//   late bool relaxCoep;
//
//   /// {@macro flutter_rust_bridge.cli}
//   late bool relaxCoepWasParsed;
//
//
//   /// {@macro flutter_rust_bridge.cli}
//   @CliOption(help: 'Whether to build the library.', defaultsTo: true)
//   late bool build;
// }
//
// /// {@macro flutter_rust_bridge.internal}
// extension ExtOpts on Opts {
//   /// {@macro flutter_rust_bridge.internal}
//   bool get shouldRunBindgen => weakRefs || referenceTypes;
//
//   /// If not set by user, relax COEP on Flutter.
//   bool get shouldRelaxCoep => relaxCoep || (!relaxCoepWasParsed && dartInput == null);
// }
//
// /// {@macro flutter_rust_bridge.internal}
// class Config {
//   /// {@macro flutter_rust_bridge.internal}
//   final Opts cliOpts;
//
//   /// {@macro flutter_rust_bridge.internal}
//   final String root;
//
//   /// {@macro flutter_rust_bridge.internal}
//   final String wasmOutput;
//
//   /// {@macro flutter_rust_bridge.internal}
//   final List<String> restArgs;
//
//   /// {@macro flutter_rust_bridge.internal}
//   const Config({
//     required this.cliOpts,
//     required this.root,
//     required this.wasmOutput,
//     required this.restArgs,
//   });
// }
//
// /// {@macro flutter_rust_bridge.internal}
// Config parseConfig(List<String> args) {
//   final opts = parseOpts(args);
//   if (opts.help) _printHelpAndExit();
//
//   final extra = _parseExtra(opts);
//
//   return Config(
//     cliOpts: opts,
//     root: extra.root,
//     wasmOutput: extra.wasmOutput,
//     restArgs: _$parserForOpts.parse(args).rest,
//   );
// }
//
// ({String root, String wasmOutput}) _parseExtra(Opts opts) {
//   if (opts.dartInput != null) {
//     if (opts.root == null) {
//       bail('The --root option is required when building plain Dart projects.');
//     }
//     final root = p.canonicalize(opts.root!);
//     return (
//       root: root,
//       wasmOutput: p.canonicalize(opts.wasmOutput ?? '$root/pkg'),
//     );
//   } else {
//     return (
//       root: p.canonicalize(opts.root ?? 'build/web'),
//       wasmOutput: p.canonicalize(opts.wasmOutput ?? 'web/pkg'),
//     );
//   }
// }
