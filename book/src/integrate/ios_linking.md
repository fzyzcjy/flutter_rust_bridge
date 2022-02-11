# Linking the library

Hit **⌘B** to build the project. This will invoke the debug build process and
attempt to build your library. At this point, the process is most likely going to fail,
but if it succeeds you need to temporarily disable the **Remove Phase**.

When the build is finished, navigate to **Product > Show Build Folder in Finder**.
From the newly opened Finder, navigate to
**Build > Intermediates.noindex > Runner.build > Debug-iphonesimulator > Runner.build > DerivedSources**.
This is the `${DERIVED_FILE_DIR}` that we passed to the library search path earlier.

You should be able to find `lib$crate.a` here assuming everything went fine. If you did,
go ahead and drag it onto the project tree. Once you have a handle on the static library,
you can go ahead and re-enable the **Remove Phase**.

Finally, we can now link our library to the binary. Go to the **Build Phases** tab,
expand **Link Binary With Binaries** and drag `lib$crate.a` from the project tree
onto the phase. With this, the library should be present when `DynamicLibrary.executable()`
is called by Dart.
