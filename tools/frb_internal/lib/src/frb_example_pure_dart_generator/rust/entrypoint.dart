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
          (m) {
            final addPostfix = m.group(3) != 'call';
            return '${prefix(m.group(1)!)} ${m.group(3)}${addPostfix ? mode.postfix : ""}(';
          },
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
      // Find matching StreamSink<...> and add `flutter_rust_bridge::SseCodec` to it
      ans = replaceAllGenericInnerTypeMapped(
        ans,
        'StreamSink',
        (innerType) => '$innerType, flutter_rust_bridge::SseCodec',
      );
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

/// Find matching [genericType]<[innerType]> and call [mapper] to
/// transform the [innerType] to a new type.
///
/// Returns the transformed string.
String replaceAllGenericInnerTypeMapped(
  String input,
  String genericType,
  String Function(String innerType) mapper,
) {
  var cursor = 0;
  final genericTypeSearch = '$genericType<';
  while (cursor < input.length) {
    cursor = input.indexOf(genericTypeSearch, cursor);
    // There is no more `StreamSink<` to replace
    if (cursor == -1) break;
    cursor += genericTypeSearch.length;
    final innerTypeStart = cursor;

    var openBrackets = 1;

    // Iterate until we find the matching closing bracket of StreamSink<...>
    while (openBrackets > 0 && cursor < input.length) {
      if (input[cursor] == '<') {
        openBrackets++;
      } else if (input[cursor] == '>') {
        openBrackets--;
      }
      cursor++;
      if (openBrackets == 0) break;
    }

    final innerTypeEnd = cursor - 1;
    final innerType = input.substring(innerTypeStart, innerTypeEnd);
    final newInnerType = mapper(innerType);
    input = input.replaceRange(innerTypeStart, innerTypeEnd, newInnerType);
    cursor = innerTypeStart + newInnerType.length;
  }

  return input;
}
