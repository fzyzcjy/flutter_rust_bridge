import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:recase/recase.dart';

class RustGenerator extends BaseGenerator {
  RustGenerator({required super.packageRootDir, required super.interestDir});

  @override
  Future<void> executeFormat() =>
      executeRustFormat(pwd: packageRootDir.toFilePath());

  @override
  Set<String> get duplicatorBlacklistNames => {'mod.rs'};

  @override
  Map<String, String> generateDirectSources() => generateRustDirectSources();

  @override
  String get extension => 'rs';

  @override
  String generateDuplicateCode(String inputText, DuplicatorMode mode) {
    const sse = '#[flutter_rust_bridge::frb(serialize)]';
    String prefix(String raw) => switch (mode) {
          DuplicatorMode.sync => '#[flutter_rust_bridge::frb(sync)] $raw',
          DuplicatorMode.rustAsync => 'pub async fn',
          DuplicatorMode.sse => '$sse $raw',
          DuplicatorMode.syncSse =>
            '$sse #[flutter_rust_bridge::frb(sync)] $raw',
          DuplicatorMode.rustAsyncSse => '$sse pub async fn',
        };

    var ans = inputText
        .replaceAllMapped(
          RegExp(r'(pub (async )?fn) ([a-zA-Z0-9_-]+?)(_twin_normal)?\('),
          (m) => '${prefix(m.group(1)!)} ${m.group(3)}${mode.postfix}(',
        )
        // struct name, etc
        .replaceAll('TwinNormal', ReCase(mode.postfix).pascalCase)
        .replaceAll('_twin_normal', mode.postfix)
        .replaceAllMapped(
            RegExp(r'use crate::api::([a-zA-Z0-9_]+)::'),
            (m) =>
                'use crate::api::pseudo_manual::${m.group(1)}${mode.postfix}::');
    if (mode.enableSse) {
      // quick hack, since we are merely generating tests
      ans = ans
          .replaceAllMapped(
              RegExp(r'StreamSink<Vec<(.*?)>>'),
              (m) =>
                  'StreamSink<Vec<${m.group(1)}>, flutter_rust_bridge::SseCodec>')
          .replaceAllMapped(
              RegExp(r'StreamSink<(.*?)>'),
              (m) => m.group(1)!.startsWith('Vec')
                  ? m.group(0)!
                  : 'StreamSink<${m.group(1)}, flutter_rust_bridge::SseCodec>');
    }
    return ans;
  }

  @override
  String generateDuplicateFileStem(String inputStem, DuplicatorMode mode) =>
      inputStem + mode.postfix;
}
