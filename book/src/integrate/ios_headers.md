# Using dummy headers

`flutter_rust_bridge_codegen` created a C header which lists all the
exported symbols from our library, then uses it so that XCode won't strip
the symbols.

Add `ios/Runner/bridge_generated.h` to the project, either by drag-and-dropping
it onto the project tree or via the **Add Files to "Runner"...** menu option.

Next, add this line to `ios/Runner/Runner-Bridging-Header.h`:

```diff
+#import "bridge_generated.h"
```

and in `ios/Runner/AppDelegate.swift`:

```diff
 override func application(
     _ application: UIApplication,
     didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
 ) -> Bool {
+    dummy_method_to_enforce_bundling()
     ..
 }
```

Finally, switch to the **Build Phases** tab and drag the **bridge_generated.h** file over
to the **Copy Bundle Resources** phase, if it isn't already present.
