import 'ffi/io.dart' if (dart.library.html) 'ffi/web.dart';

export 'ffi/stub.dart' if (dart.library.io) 'ffi/io.dart' if (dart.library.html) 'ffi/web.dart';

typedef DropFnType = void Function(PlatformPointer);
typedef ShareFnType = PlatformPointer Function(PlatformPointer);

/// Rust SyncReturn<usize> type is forced cast to u64.
const syncReturnPointerLength = 8;
