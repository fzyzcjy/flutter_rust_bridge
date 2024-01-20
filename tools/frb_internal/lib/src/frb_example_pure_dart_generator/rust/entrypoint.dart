import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:recase/recase.dart';

class RustGenerator extends BaseGenerator {
  RustGenerator({
    required super.packageRootDir,
    required super.interestDir,
    required super.package,
  });

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
    String prefix(String raw) {
      var ans = raw;
      for (final component in mode.components) {
        switch (component) {
          case DuplicatorComponentMode.sync:
            ans = '#[flutter_rust_bridge::frb(sync)] $ans';
          case DuplicatorComponentMode.rustAsync:
            ans = 'pub async fn';
          case DuplicatorComponentMode.sse:
            ans = '#[flutter_rust_bridge::frb(serialize)] $ans';
          case DuplicatorComponentMode.moi:
            ans = '#[flutter_rust_bridge::frb(rust_opaque_codec_moi)] $ans';
        }
      }
      return ans;
    }

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
                'use crate::api::pseudo_manual::${m.group(1)}${mode.postfix}::')
        .replaceAll(
            'super::rust_opaque::', 'super::rust_opaque${mode.postfix}::')
        .replaceAll('super::basic::', 'super::basic${mode.postfix}::');

    if (mode.components.any((e) => e == DuplicatorComponentMode.sse)) {
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

    if (mode.components.any((e) => e == DuplicatorComponentMode.moi)) {
      // hack, otherwise `i32` is considered as Nom, and will ignore requests of using Moi codec
      // anyway this hack only affects how tests are auto generated, so no problem
      ans = ans.replaceAll(RegExp(r'RustOpaque<i32>'),
          'crate::frb_generated::RustOpaqueMoi<i16>');
      ans = ans.replaceAllMapped(
          RegExp(r'Rust(Auto)?Opaque(Nom)?(<|::)'),
          (m) =>
              'crate::frb_generated::Rust${m.group(1) ?? ""}OpaqueMoi${m.group(3)}');
    }

    return ans;
  }

  @override
  String generateDuplicateFileStem(String inputStem, DuplicatorMode mode) =>
      inputStem + mode.postfix;
}
