import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

/// Base class for various kinds of tasks.
///
/// {@template flutter_rust_bridge.not_manually_create}
/// Normally you do not manually create instances of this task (or its brothers), but instead
/// it is generated automatically by the codegen.
/// {@endtemplate}
@immutable
abstract class BaseTask<S, E extends Object, WireSyncType> {
  /// Parse the returned data from the underlying function
  final BaseCodec<S, E, WireSyncType> codec;

  /// Metadata that does not change across different method calls.
  final TaskConstMeta constMeta;

  /// Arguments to be passed into the function call.
  final List<dynamic> argValues;

  /// The Api class that creates the task.
  final BaseApiImpl apiImpl;

  /// Create a new task.
  const BaseTask({
    required this.codec,
    required this.constMeta,
    required this.argValues,
    required this.apiImpl,
  });

  /// Arguments to be passed into the function call, provided in the format of a [Map]
  Map<String, dynamic> get argMap => Map.fromEntries([
        for (var i = 0; i < constMeta.argNames.length; ++i)
          MapEntry(constMeta.argNames[i], argValues[i])
      ]);
}

/// A task to call FFI function.
///
/// {@macro flutter_rust_bridge.not_manually_create}
@immutable
class NormalTask<S, E extends Object> extends BaseTask<S, E, dynamic> {
  /// The underlying function to call FFI function, usually the generated wire function
  final void Function(NativePortType port) callFfi;

  /// Create a new task.
  const NormalTask({
    required this.callFfi,
    required super.codec,
    required super.constMeta,
    required super.argValues,
    required super.apiImpl,
  });
}

/// A task to call FFI function, but it is synchronous.
///
/// {@macro flutter_rust_bridge.not_manually_create}
@immutable
class SyncTask<S, E extends Object, WireSyncType>
    extends BaseTask<S, E, WireSyncType> {
  /// The underlying function to call FFI function, usually the generated wire function
  final WireSyncType Function() callFfi;

  /// Create a new task.
  const SyncTask({
    required this.callFfi,
    required super.codec,
    required super.constMeta,
    required super.argValues,
    required super.apiImpl,
  });
}

/// Metadata that does not change across different method calls. Thus it is made `const` to save memory and speed up
@immutable
class TaskConstMeta {
  /// Used for debugging purposes only.
  final String debugName;

  /// A list of arguments to the task.
  final List<String> argNames;

  /// Create a new task metadata.
  const TaskConstMeta({
    required this.debugName,
    required this.argNames,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TaskConstMeta &&
          runtimeType == other.runtimeType &&
          debugName == other.debugName &&
          _listEquals(argNames, other.argNames);

  @override
  int get hashCode => debugName.hashCode ^ Object.hashAll(argNames);

  @override
  String toString() =>
      'TaskConstMeta{debugName: $debugName, argNames: $argNames}';
}

bool _listEquals<T>(List<T>? a, List<T>? b) {
  if (a == null) return b == null;
  if (b == null || a.length != b.length) return false;
  if (identical(a, b)) return true;
  for (var index = 0; index < a.length; index += 1) {
    if (a[index] != b[index]) return false;
  }
  return true;
}
