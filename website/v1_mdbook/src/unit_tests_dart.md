# Unit tests in dart

To run `flutter` or `dart test` with the bridge you need to load the library on your own development machine (Windows/MacOS/Linux/CI). For that use `loadLibForFlutter` or `loadLibForDart`, for example:

```dart
BridgeImpl initializeExternalLibrary(String path) => BridgeImpl(loadLibForDart(path));
```

Note however, that you need to build the library for your IDE's Operating System. `cargo build` should normally handle that.

**Do not change the target to your OS only**, as otherwise you will not be able to build for your target platform anymore.

## Example setup (verified on MacOS)
```
project
|- lib
|- test
|-- ffi.test.dart
|-- bridge_test.dart
|- rust
|-- src
|--- api.rs
|-- target
```

Where `ffi.test.dart` has the following content:
```dart
import 'package:basis_hybrid/bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

BridgeImpl initializeExternalLibrary(String path) {
  return BridgeImpl(
    loadLibForFlutter(path)
  );
}
```

and then `bridge_test.dart` has the following content:
```dart
import 'package:basis_hybrid/bridge_definitions.dart';
import 'package:flutter_test/flutter_test.dart';

import 'ffi.test.dart';

Future<void> main() async {
  final api = initializeExternalLibrary('rust/target/debug/librustbridge.dylib');
  await api.init(sqlPath: 'test.db', kvPath: 'test.kv');

  test('User save/load', () async {
     await api.saveUser();
     var user = await api.readUser();
     expect(user, isNotNull);
   });
}
``` 

Ensure that you have your IDE's system target installed (`rustup`) according to [Creating a new project](https://cjycode.com/flutter_rust_bridge/template/setup.html), after running `cargo build`  you should've a library in `rust/target/debug/`