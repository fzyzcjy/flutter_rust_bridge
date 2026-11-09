import 'dart:io';

import 'package:path/path.dart' as path;

final _mixinHeader = RegExp(r'mixin _\$([A-Za-z0-9_]+) \{');
final _mixinGetter = RegExp(
  r'^([ \t]+)(.+?) get ([A-Za-z_]\w*) =>',
  multiLine: true,
);
final _variantGetter = RegExp(
  r'^[ \t]+(.+?) get ([A-Za-z_]\w*);',
  multiLine: true,
);
final _classHeader = RegExp(
  r'(?:abstract\s+|base\s+|final\s+|sealed\s+|interface\s+)?class\s+([A-Za-z_]\w*)\b([^{;]*)\{',
);
final _strictHeader = RegExp(
  r'^(?:\s*extends\s+([A-Za-z_]\w*))?(?:\s*implements\s+([A-Za-z_]\w*(?:\s*,\s*[A-Za-z_]\w*)*))?\s*$',
);

const _skippedDirectories = {
  '.git',
  '.dart_tool',
  'build',
  'node_modules',
  'target',
  'test',
};

class _TypeDecl {
  final String? superclass;
  final List<String> interfaces;

  const _TypeDecl({required this.superclass, required this.interfaces});

  bool sameAs(_TypeDecl other) {
    if (superclass != other.superclass) return false;
    if (interfaces.length != other.interfaces.length) return false;
    return interfaces.toSet().containsAll(other.interfaces);
  }
}

class _DeclarationIndex {
  final declarations = <String, _TypeDecl>{};
  final ambiguous = <String>{};

  void addSource(String source) {
    for (final match in _classHeader.allMatches(source)) {
      final name = match.group(1)!;
      final parsed = _parseDecl(match.group(2)!);
      if (parsed == null) {
        declarations.remove(name);
        ambiguous.add(name);
        continue;
      }
      if (ambiguous.contains(name)) continue;
      final existing = declarations[name];
      if (existing == null) {
        declarations[name] = parsed;
      } else if (!existing.sameAs(parsed)) {
        declarations.remove(name);
        ambiguous.add(name);
      }
    }
  }
}

/// Pins a freezed shared getter only for the confirmed ambiguous bound.
///
/// `leastUpperBound` is not unique when every variant type implements the same
/// set of two or more interfaces and each of those interfaces sits directly
/// under `Object`. Fresh generation then alternates between `Object` and one
/// of those interfaces. A unique common type is left unchanged.
void normalizeUnstableFreezedLeastUpperBounds({required String repoRootPath}) {
  final root = Directory(repoRootPath);
  if (!root.existsSync()) return;
  final index = _DeclarationIndex();
  _visitDartFiles(root, (file) => index.addSource(file.readAsStringSync()));
  _visitFreezedFiles(root, (file) {
    final original = file.readAsStringSync();
    final normalized = _normalize(original, index);
    if (normalized != original) file.writeAsStringSync(normalized);
  });
}

String normalizeUnstableFreezedLeastUpperBound(String source) {
  final index = _DeclarationIndex()..addSource(source);
  return _normalize(source, index);
}

String _normalize(String source, _DeclarationIndex index) {
  final matches = _mixinHeader.allMatches(source).toList();
  if (matches.isEmpty) return source;

  final edits = <(int, int, String)>[];
  for (final match in matches) {
    final openBrace = match.end - 1;
    final closeBrace = _matchingBrace(source, openBrace);
    if (closeBrace < 0) return source;

    final unionName = match.group(1)!;
    final variantTypes = _variantFieldTypes(source, unionName);
    final block = source.substring(match.start, closeBrace + 1);
    final rewritten = _rewriteMixinGetters(block, variantTypes, index);
    if (rewritten != block) {
      edits.add((match.start, closeBrace + 1, rewritten));
    }
  }
  if (edits.isEmpty) return source;

  final buffer = StringBuffer();
  var cursor = 0;
  for (final (start, end, replacement) in edits) {
    buffer
      ..write(source.substring(cursor, start))
      ..write(replacement);
    cursor = end;
  }
  buffer.write(source.substring(cursor));
  return buffer.toString();
}

_TypeDecl? _parseDecl(String header) {
  final match = _strictHeader.firstMatch(header);
  if (match == null) return null;
  final interfaces = match.group(2);
  return _TypeDecl(
    superclass: match.group(1),
    interfaces: interfaces == null
        ? const []
        : interfaces.split(',').map((type) => type.trim()).toList(),
  );
}

void _visitDartFiles(Directory directory, void Function(File file) visit) {
  for (final entity in directory.listSync(followLinks: false)) {
    if (entity is Directory) {
      if (_skippedDirectories.contains(path.basename(entity.path))) continue;
      _visitDartFiles(entity, visit);
      continue;
    }
    if (entity is File && entity.path.endsWith('.dart')) visit(entity);
  }
}

void _visitFreezedFiles(Directory directory, void Function(File file) visit) {
  for (final entity in directory.listSync(followLinks: false)) {
    if (entity is Directory) {
      if (_skippedDirectories.contains(path.basename(entity.path))) continue;
      _visitFreezedFiles(entity, visit);
      continue;
    }
    if (entity is File && entity.path.endsWith('.freezed.dart')) visit(entity);
  }
}

String _rewriteMixinGetters(
  String block,
  Map<String, Set<String>> variantTypes,
  _DeclarationIndex index,
) {
  return block.replaceAllMapped(_mixinGetter, (match) {
    final field = match.group(3)!;
    final types = variantTypes[field];
    final ambiguous = types == null ? null : _ambiguousInterfaces(types, index);
    final declared = match.group(2)!.trim();
    if (ambiguous == null || !ambiguous.contains(declared)) {
      return match.group(0)!;
    }
    return '${match.group(1)}Object get $field =>';
  });
}

Set<String>? _ambiguousInterfaces(
  Set<String> variantTypes,
  _DeclarationIndex index,
) {
  if (variantTypes.length < 2) return null;
  Set<String>? shared;
  for (final type in variantTypes) {
    if (index.ambiguous.contains(type)) return null;
    final decl = index.declarations[type];
    if (decl == null || decl.interfaces.length < 2) return null;
    final current = decl.interfaces.toSet();
    if (current.length != decl.interfaces.length) return null;
    if (shared == null) {
      shared = current;
    } else if (shared.length != current.length ||
        !shared.containsAll(current)) {
      return null;
    }
  }
  if (shared == null) return null;
  for (final interface in shared) {
    if (!_isDepthOneInterface(interface, index)) return null;
  }
  return shared;
}

bool _isDepthOneInterface(String name, _DeclarationIndex index) {
  if (index.ambiguous.contains(name)) return false;
  final decl = index.declarations[name];
  if (decl == null) return false;
  return decl.superclass == null && decl.interfaces.isEmpty;
}

Map<String, Set<String>> _variantFieldTypes(String source, String unionName) {
  final classHeader = RegExp(
    'abstract class ${RegExp.escape(unionName)}_Variant\\d+\\b',
  );
  final types = <String, Set<String>>{};
  for (final match in classHeader.allMatches(source)) {
    final openBrace = source.indexOf('{', match.end);
    if (openBrace < 0) continue;
    final closeBrace = _matchingBrace(source, openBrace);
    if (closeBrace < 0) continue;
    final body = source.substring(openBrace, closeBrace);
    for (final getter in _variantGetter.allMatches(body)) {
      types
          .putIfAbsent(getter.group(2)!, () => {})
          .add(getter.group(1)!.trim());
    }
  }
  return types;
}

int _matchingBrace(String source, int openBrace) {
  var depth = 0;
  for (var index = openBrace; index < source.length; index++) {
    final char = source[index];
    if (char == '{') {
      depth++;
    } else if (char == '}') {
      depth--;
      if (depth == 0) return index;
    }
  }
  return -1;
}
