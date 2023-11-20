import 'package:meta/meta.dart';

/// Base class for various kinds of tasks.
/// Note: Normally you do not manually create instances of this task (or its brothers), but instead
/// it is generated automatically by the codegen.
@immutable
abstract class BaseTask<S, E extends Object> {
  /// Parse the returned data from the underlying function
  final S Function(dynamic) parseSuccessData;

  /// Parse the returned error data from the underlying function
  final E Function(dynamic)? parseErrorData;

  /// Metadata that does not change across different method calls.
  final TaskConstMeta constMeta;

  /// Arguments to be passed into the function call.
  final List<dynamic> argValues;

  /// Transparent hint given by the caller of the method
  final dynamic hint;

  /// Create a new task.
  const BaseTask({
    required this.parseSuccessData,
    required this.parseErrorData,
    required this.constMeta,
    required this.argValues,
    required this.hint,
  });

  /// Name usually used for logging or debugging
  String get debugName => constMeta.debugName;

  /// Arguments to be passed into the function call, provided in the format of a series of [MapEntry]s
  Iterable<MapEntry<String, dynamic>> get argMapEntries sync* {
    for (var i = 0; i < constMeta.argNames.length; ++i) {
      yield MapEntry(constMeta.argNames[i], argValues[i]);
    }
  }

  /// Arguments to be passed into the function call, provided in the format of a [Map]
  Map<String, dynamic> get argMap => Map.fromEntries(argMapEntries);
}

/// A task to call FFI function.
@immutable
class FlutterRustBridgeTask<S, E extends Object> extends BaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final void Function(NativePortType port) callFfi;

  const FlutterRustBridgeTask({
    required this.callFfi,
    required super.parseSuccessData,
    required super.parseErrorData,
    required super.constMeta,
    required super.argValues,
    required super.hint,
  });
}

/// A task to call FFI function, but it is synchronous.
@immutable
class FlutterRustBridgeSyncTask<S, E extends Object> extends BaseTask {
  /// The underlying function to call FFI function, usually the generated wire function
  final WireSyncReturn Function() callFfi;

  const FlutterRustBridgeSyncTask({
    required this.callFfi,
    required super.parseSuccessData,
    required super.parseErrorData,
    required super.constMeta,
    required super.argValues,
    required super.hint,
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
  String toString() => 'TaskConstMeta{debugName: $debugName, argNames: $argNames}';
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
