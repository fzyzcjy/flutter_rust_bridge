# Using the dynamic library

If everything went well, running `flutter run` will now build your Rust library,
the Flutter binary and link the two together. Now the only thing left to do is
to actually use it!

Download [this file](https://raw.githubusercontent.com/Desdaemon/flutter_rust_bridge_template/main/lib/ffi.dart)
to `lib/ffi.dart`, then modify its contents:

```diff
 // Re-export the bridge so it is only necessary to import this file.
 export 'bridge_generated.dart';
 import 'dart:io' as io;

-const _base = 'native';
+const _base = '$crate';

 // On MacOS, the dynamic library is not bundled with the binary,
 // but rather directly **linked** against the binary.
 final _dylib = io.Platform.isWindows ? '$_base.dll' : 'lib$_base.so';
```
